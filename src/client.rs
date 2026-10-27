use crate::{
    config::{self, Connection},
    error::{CommandError, invalid, network_error},
    validation,
};
use anyhow::Result;
use reqwest::{
    blocking::{Client as Http, Response},
    header,
};
use serde_json::{Value, json};
use std::time::{Duration, Instant};

pub struct Client {
    pub config: Connection,
    http: Http,
}
impl Client {
    pub fn new(config: Connection) -> Result<Self> {
        let http = Http::builder()
            .timeout(Duration::from_secs(config.timeout))
            .redirect(reqwest::redirect::Policy::none())
            .user_agent(concat!("canvatopsd-cli/", env!("CARGO_PKG_VERSION")))
            .build()
            .map_err(network_error)?;
        Ok(Self { config, http })
    }
    pub fn post(
        &mut self,
        path: &str,
        body: &Value,
        operator: bool,
        deadline: Option<Instant>,
    ) -> Result<Response> {
        if !regex::Regex::new(r"^/[a-z][a-z0-9/-]*$")?.is_match(path) {
            return Err(invalid("Use a command path such as /task/status"));
        }
        let mut request = self
            .http
            .post(format!("{}{path}", self.config.base_url))
            .header(header::ORIGIN, &self.config.origin)
            .json(body);
        if let Some(deadline) = deadline {
            let remaining = deadline
                .checked_duration_since(Instant::now())
                .ok_or_else(|| {
                    CommandError::new("wait_timeout", "Conversion wait deadline reached")
                })?;
            request = request.timeout(remaining.min(Duration::from_secs(self.config.timeout)));
        }
        if operator {
            request =
                request.bearer_auth(self.config.operator_token.as_ref().ok_or_else(|| {
                    invalid("Set CANVATOPSD_OPERATOR_TOKEN for operator commands")
                })?);
        } else if path != "/task/create"
            && path != "/task/access/exchange"
            && path != "/task/preview/link/open"
        {
            if let Some(cookie) = self.task_cookie(body.get("taskId").and_then(Value::as_str)) {
                request = request.header(header::COOKIE, cookie);
            }
        }
        if let Some(token) = &self.config.proxy_token {
            request = request.header("X-CanvaToPSD-Proxy-Token", token);
        }
        let response = request.send().map_err(network_error)?;
        let status = response.status();
        if !status.is_success() {
            let code = if is_json(&response) {
                response
                    .json::<Value>()
                    .map_err(network_error)?
                    .get("code")
                    .and_then(Value::as_str)
                    .map(str::to_owned)
            } else {
                None
            };
            let mut error = CommandError::new(
                &code.unwrap_or_else(|| format!("http_{}", status.as_u16())),
                "API command failed",
            );
            error.message = error.code.clone();
            error.status = Some(status.as_u16());
            return Err(error.into());
        }
        let mut cookies = config::Entries::new();
        let pattern =
            regex::Regex::new(r"^(__Host-canvatopsd-task(?:-[a-f0-9-]+)?)=([a-f0-9]{64})(?:;|$)")?;
        for header in response.headers().get_all(header::SET_COOKIE) {
            if let Some(c) = pattern.captures(header.to_str()?) {
                cookies.insert(c[1].into(), c[2].into());
            }
        }
        cookies.retain(|k, v| self.config.cookies.get(k) != Some(v));
        if !cookies.is_empty() {
            config::update(|settings| {
                settings
                    .sessions
                    .entry(self.config.base_url.clone())
                    .or_default()
                    .extend(cookies.clone());
                Ok(())
            })?;
            self.config.cookies.extend(cookies);
        }
        Ok(response)
    }
    pub fn json(&mut self, path: &str, body: &Value) -> Result<Value> {
        self.json_with(path, body, false, None)
    }
    pub fn json_with(
        &mut self,
        path: &str,
        body: &Value,
        operator: bool,
        deadline: Option<Instant>,
    ) -> Result<Value> {
        let response = self.post(path, body, operator, deadline)?;
        if !is_json(&response) {
            return Err(CommandError::new(
                "expected_json_response",
                "Expected an application/json response",
            )
            .into());
        }
        let result: Value = response.json().map_err(network_error)?;
        let children: Vec<String> = match path {
            "/task/page-request/create" => result
                .get("tasks")
                .and_then(Value::as_array)
                .map(|items| {
                    items
                        .iter()
                        .filter_map(|v| v["taskId"].as_str().map(str::to_owned))
                        .collect()
                })
                .unwrap_or_default(),
            "/task/source-page/list" => result
                .get("autoTaskId")
                .and_then(Value::as_str)
                .map(|id| vec![id.into()])
                .unwrap_or_default(),
            "/task/status" => result
                .get("pageTasks")
                .and_then(Value::as_array)
                .map(|items| {
                    items
                        .iter()
                        .filter_map(|v| v["taskId"].as_str().map(str::to_owned))
                        .collect()
                })
                .unwrap_or_default(),
            _ => vec![],
        };
        if !children.is_empty() {
            let parent = validation::uuid(validation::field(body, "taskId")?)?;
            let mut links = config::Entries::new();
            for child in children {
                validation::uuid(&child)?;
                if self.config.parents.get(&child) != Some(&parent) {
                    links.insert(child, parent.clone());
                }
            }
            if !links.is_empty() {
                config::update(|settings| {
                    settings
                        .task_parents
                        .entry(self.config.base_url.clone())
                        .or_default()
                        .extend(links.clone());
                    Ok(())
                })?;
                self.config.parents.extend(links);
            }
        }
        Ok(result)
    }
    pub fn download(&self, url: &str) -> Result<Response> {
        validation::secure_url(url)?;
        let response = self.http.get(url).send().map_err(network_error)?;
        if !response.status().is_success() {
            return Err(CommandError::new(
                "download_failed",
                format!("Download failed: HTTP {}", response.status().as_u16()),
            )
            .into());
        }
        Ok(response)
    }
    fn task_cookie(&self, task: Option<&str>) -> Option<String> {
        let task = task?;
        let direct = format!("__Host-canvatopsd-task-{task}");
        let name = if self.config.cookies.contains_key(&direct) {
            direct
        } else if let Some(parent) = self.config.parents.get(task) {
            format!("__Host-canvatopsd-task-{parent}")
        } else {
            "__Host-canvatopsd-task".into()
        };
        self.config
            .cookies
            .get(&name)
            .map(|token| format!("{name}={token}"))
    }
}
fn is_json(response: &Response) -> bool {
    response
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .is_some_and(|v| v.contains("application/json"))
}
pub fn submit(
    client: &mut Client,
    path: &str,
    input: &Value,
    deadline: Option<Instant>,
) -> Result<Value> {
    let key = validation::field(input, "submitKey")?;
    eprintln!("{}", json!({"submitKey":key}));
    let mut result = client
        .json_with(path, input, false, deadline)
        .map_err(|e| crate::error::submission_error(e, key, None))?;
    result["submitKey"] = json!(key);
    Ok(result)
}
