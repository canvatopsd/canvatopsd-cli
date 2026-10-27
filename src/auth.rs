use crate::{
    args::Auth,
    client::Client,
    config::{self, Connection},
    error::invalid,
    validation::{self, field},
};
use anyhow::Result;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::io::Write;
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Credential {
    format: String,
    base_url: String,
    origin: String,
    task_id: String,
    token: String,
}
pub fn submission_token(config: &Connection, key: &str) -> Result<String> {
    config::update(|settings| {
        let tokens = settings
            .submission_tokens
            .entry(config.base_url.clone())
            .or_default();
        let token = tokens.entry(key.into()).or_insert_with(|| {
            let mut bytes = [0u8; 32];
            rand::rng().fill_bytes(&mut bytes);
            hex::encode(bytes)
        });
        validation::token(token)
    })
}
pub fn run(client: &mut Client, command: &Auth) -> Result<Value> {
    match command {
        Auth::Status => {
            let settings = config::read()?;
            Ok(
                json!({"submissionKeys":settings.submission_tokens.get(&client.config.base_url).map(|v| v.keys().collect::<Vec<_>>()).unwrap_or_default(),"baseUrl":client.config.base_url,"taskSessions":client.config.cookies.keys().map(|k| k.trim_start_matches("__Host-canvatopsd-task-")).collect::<Vec<_>>(),"operatorAuth":if client.config.operator_token.is_some() { "env" } else { "missing" }}),
            )
        }
        Auth::Clear => {
            config::update(|settings| {
                settings.sessions.remove(&client.config.base_url);
                settings.submission_tokens.remove(&client.config.base_url);
                settings.task_parents.remove(&client.config.base_url);
                Ok(())
            })?;
            Ok(json!({"cleared":true,"baseUrl":client.config.base_url}))
        }
        Auth::Export { task, file } => {
            validation::uuid(task)?;
            let token = client.config.cookies.get(&format!("__Host-canvatopsd-task-{task}")).ok_or_else(|| invalid("No direct credential; export the parent task when it provides authorization"))?;
            validation::token(token)?;
            let credential = Credential {
                format: "canvatopsd-task-access-v1".into(),
                base_url: client.config.base_url.clone(),
                origin: client.config.origin.clone(),
                task_id: task.clone(),
                token: token.clone(),
            };
            let path = std::path::absolute(file)?;
            std::fs::create_dir_all(
                path.parent()
                    .ok_or_else(|| invalid("Invalid credential output path"))?,
            )?;
            let mut output = std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&path)?;
            config::private_permissions_file(&output, 0o600)?;
            writeln!(output, "{}", serde_json::to_string_pretty(&credential)?)?;
            Ok(json!({"taskId":task,"path":path}))
        }
        Auth::Import { file } => {
            let content = std::fs::read_to_string(file)?;
            let mut path = "/task/access/exchange";
            let input = if content.trim_start().starts_with('{') {
                let saved: Credential = serde_json::from_str(&content)?;
                if saved.format != "canvatopsd-task-access-v1"
                    || saved.base_url != client.config.base_url
                    || saved.origin != client.config.origin
                {
                    return Err(invalid(
                        "Credential format or endpoint does not match the selected service",
                    ));
                }
                json!({"taskId":validation::uuid(&saved.task_id)?,"token":validation::token(&saved.token)?})
            } else {
                let url = url::Url::parse(content.trim())?;
                let task = validation::task_link(&url)?
                    .ok_or_else(|| invalid("Expected a private task link"))?;
                if url.origin().ascii_serialization() != client.config.origin {
                    return Err(invalid("Private task link belongs to another endpoint"));
                }
                let fragment: std::collections::BTreeMap<_, _> =
                    url::form_urlencoded::parse(url.fragment().unwrap_or("").as_bytes())
                        .into_owned()
                        .collect();
                if let Some(ticket) = fragment.get("preview") {
                    path = "/task/preview/link/open";
                    json!({"taskId":task,"ticket":ticket})
                } else {
                    let token = fragment
                        .get("access")
                        .ok_or_else(|| invalid("Private task link is missing access"))?;
                    let mut value = json!({"taskId":task,"token":validation::token(token)?});
                    if let Some(event) = fragment.get("email") {
                        value["emailEventId"] = json!(event);
                    }
                    value
                }
            };
            let result = client.json(path, &input)?;
            client.json("/task/status", &json!({"taskId":field(&input,"taskId")?}))?;
            Ok(result)
        }
    }
}
