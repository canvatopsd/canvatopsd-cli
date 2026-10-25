use serde_json::{Value, json};
use std::fmt;

#[derive(Debug)]
pub struct CommandError {
    pub code: String,
    pub message: String,
    pub status: Option<u16>,
    pub task_id: Option<String>,
    pub submit_key: Option<String>,
}
impl CommandError {
    pub fn new(code: &str, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            status: None,
            task_id: None,
            submit_key: None,
        }
    }
    pub fn value(&self) -> Value {
        let mut value = json!({"code":self.code,"message":self.message});
        if let Some(status) = self.status {
            value["status"] = json!(status);
        }
        if let Some(task) = &self.task_id {
            value["taskId"] = json!(task);
        }
        if let Some(key) = &self.submit_key {
            value["submitKey"] = json!(key);
        }
        value
    }
}
impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl std::error::Error for CommandError {}
pub fn invalid(message: impl Into<String>) -> anyhow::Error {
    CommandError::new("invalid_input", message).into()
}
pub fn task_error(code: &str, message: impl Into<String>, task: &str) -> anyhow::Error {
    let mut error = CommandError::new(code, message);
    error.task_id = Some(task.into());
    error.into()
}
pub fn submission_error(error: anyhow::Error, key: &str, task: Option<&str>) -> anyhow::Error {
    let mut error = match error.downcast::<CommandError>() {
        Ok(error) => error,
        Err(error) => CommandError::new("cli_error", error.to_string()),
    };
    error.submit_key = Some(key.into());
    if let Some(task) = task {
        error.task_id = Some(task.into());
    }
    error.into()
}
pub fn network_error(error: reqwest::Error) -> anyhow::Error {
    let code = if error.is_timeout() {
        "request_timeout"
    } else {
        "request_failed"
    };
    // Preserve the underlying transport/TLS error without signed URLs or query credentials.
    let message = format!("{:#}", anyhow::Error::new(error.without_url()));
    CommandError::new(code, message).into()
}
