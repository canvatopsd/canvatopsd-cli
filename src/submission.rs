use crate::{
    auth,
    client::Client,
    error::{invalid, submission_error},
    validation::field,
};
use anyhow::Result;
use serde_json::{Value, json};

pub fn receive(client: &mut Client, mut input: Value) -> Result<Value> {
    if url::Url::parse(&client.config.base_url)?
        .path()
        .trim_end_matches('/')
        != "/api"
    {
        return Err(invalid(
            "New submissions require the Web R2 inbox endpoint ending /api",
        ));
    }
    let key = match input.get("taskId").and_then(Value::as_str) {
        Some(id) => id.to_owned(),
        None => field(&client.json("/task/prepare", &json!({}))?, "taskId")?.to_owned(),
    };
    crate::validation::task_id(&key)?;
    if client.config.internal_submissions {
        input["client"]["internal"] = json!(true);
    }
    input["taskId"] = json!(key);
    input["taskToken"] = json!(auth::submission_token(&client.config, &key)?);
    eprintln!("{}", json!({"taskId":key}));
    let result = client
        .json("/task/create", &input)
        .map_err(|error| submission_error(error, &key))?;
    if result["taskId"] != key || result["accepted"] != true || result["status"] != "received" {
        return Err(submission_error(
            invalid("Unexpected R2 inbox acknowledgement"),
            &key,
        ));
    }
    Ok(result)
}
