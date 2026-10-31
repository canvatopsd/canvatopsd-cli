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
    let key = match input.get("submissionId").and_then(Value::as_str) {
        Some(id) => id.to_owned(),
        None => field(
            &client.json("/submission/prepare", &json!({}))?,
            "submissionId",
        )?
        .to_owned(),
    };
    let id = uuid::Uuid::parse_str(&key)?;
    if id.get_version_num() != 7 {
        return Err(invalid("Inbox returned an invalid submission UUIDv7"));
    }
    input["submissionId"] = json!(key);
    input["taskToken"] = json!(auth::submission_token(&client.config, &key)?);
    eprintln!("{}", json!({"submitKey":key,"taskId":key}));
    let mut result = client
        .json("/submission/create", &input)
        .map_err(|error| submission_error(error, &key, Some(&key)))?;
    if result["taskId"] != key || result["accepted"] != true || result["status"] != "received" {
        return Err(submission_error(
            invalid("Unexpected R2 inbox acknowledgement"),
            &key,
            Some(&key),
        ));
    }
    result["submitKey"] = json!(key);
    Ok(result)
}
