use crate::error::invalid;
use anyhow::Result;
use regex::Regex;
use serde_json::{Value, json};
use url::Url;

pub fn uuid(value: &str) -> Result<String> {
    let parsed = uuid::Uuid::parse_str(value).map_err(|_| invalid("Expected a UUID"))?;
    if parsed.to_string() != value.to_lowercase() {
        return Err(invalid("Expected a hyphenated UUID"));
    }
    Ok(value.into())
}
pub fn token(value: &str) -> Result<String> {
    if value.len() != 64
        || !value
            .bytes()
            .all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
    {
        return Err(invalid(
            "Expected a task access token containing 64 lowercase hexadecimal characters",
        ));
    }
    Ok(value.into())
}
pub fn pages(value: &str) -> Result<Vec<u32>> {
    let mut pages = value
        .split(',')
        .map(|v| {
            v.parse::<u32>()
                .ok()
                .filter(|v| *v > 0)
                .ok_or_else(|| invalid("Pages must be comma-separated positive integers"))
        })
        .collect::<Result<Vec<_>>>()?;
    pages.sort_unstable();
    pages.dedup();
    Ok(pages)
}
pub fn canva_link(value: &str) -> Result<Value> {
    if value.len() > 2048 {
        return Err(invalid("Canva URL exceeds 2048 characters"));
    }
    let mut url = Url::parse(value).map_err(|_| invalid("Enter a valid Canva public view URL"))?;
    if url.scheme() != "https"
        || url.port().is_some()
        || !url.username().is_empty()
        || url.password().is_some()
    {
        return Err(invalid(
            "Use an HTTPS Canva link without credentials or a custom port",
        ));
    }
    let design = match url.host_str() {
        Some("canva.link") => {
            if !Regex::new(r"^/[A-Za-z0-9_-]+/?$")?.is_match(url.path()) {
                return Err(invalid("Invalid canva.link share URL"));
            }
            None
        }
        Some("canva.com" | "www.canva.com") => {
            let expression = Regex::new(r"^/design/([A-Za-z0-9_-]+)/[A-Za-z0-9_-]+/view/?$")?;
            let id = expression
                .captures(url.path())
                .ok_or_else(|| invalid("Use Canva Share → Public view link → Copy link"))?[1]
                .to_owned();
            url.set_host(Some("www.canva.com"))?;
            Some(id)
        }
        _ => {
            return Err(invalid(
                "Use a public Canva design link or canva.link share URL",
            ));
        }
    };
    url.set_query(None);
    url.set_fragment(None);
    let mut result = json!({"url":url.as_str()});
    if let Some(id) = design {
        result["designId"] = json!(id);
    }
    Ok(result)
}
pub fn task_link(url: &Url) -> Result<Option<String>> {
    let expression = Regex::new(r"(?i)^/(?:[a-z]{2,3}(?:-[a-z0-9]{2,8})*/)?task/([a-f0-9-]+)$")?;
    expression
        .captures(url.path())
        .map(|c| uuid(&c[1]))
        .transpose()
}
pub fn task_input(task: &str) -> Result<Value> {
    Ok(json!({"taskId":uuid(task)?}))
}
pub fn field<'a>(value: &'a Value, key: &str) -> Result<&'a str> {
    value
        .get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| invalid(format!("Response is missing string field: {key}")))
}
pub fn array<'a>(value: &'a Value, key: &str) -> Result<&'a Vec<Value>> {
    value
        .get(key)
        .and_then(Value::as_array)
        .ok_or_else(|| invalid(format!("Response is missing array field: {key}")))
}
pub fn secure_url(value: &str) -> Result<Url> {
    let url = Url::parse(value).map_err(|_| invalid("Invalid endpoint URL"))?;
    let loopback = matches!(url.host_str(), Some("localhost" | "127.0.0.1" | "[::1]"));
    if !(url.scheme() == "https" || (url.scheme() == "http" && loopback))
        || !url.username().is_empty()
        || url.password().is_some()
    {
        return Err(invalid("Use HTTPS, or loopback HTTP, without credentials"));
    }
    Ok(url)
}
