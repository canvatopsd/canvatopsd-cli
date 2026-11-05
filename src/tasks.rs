use crate::{
    args::Submission,
    client::Client,
    error::{invalid, page_error, task_error},
    validation::{self, array, field},
};
use anyhow::Result;
use serde_json::{Value, json};
use std::time::{Duration, Instant};

pub fn submission(input: &Submission) -> Result<Value> {
    validation::canva_link(&input.url)?;
    let mut body = json!({"canvaUrl":input.url.trim(),"layerNaming":!input.no_layer_naming});
    if let Some(key) = &input.task_id {
        body["taskId"] = json!(validation::task_id(key)?);
    }
    body["client"] = json!({"channel":"cli","version":env!("CARGO_PKG_VERSION")});
    if let Some(agent) = &input.agent {
        body["client"]["agent"] = json!(client_label(agent, 80)?);
    }
    if let Some(version) = &input.agent_version {
        body["client"]["agentVersion"] = json!(client_label(version, 64)?);
    }
    if !email_address::EmailAddress::is_valid(&input.email) {
        return Err(invalid(
            "Provide a valid --email address to receive download links for completed PSDs and other deliverables",
        ));
    }
    body["email"] = json!(input.email.to_lowercase());
    if let Some(locale) = &input.locale {
        body["locale"] = json!(locale);
    }
    body["pages"] = json!([1]);
    Ok(body)
}
fn client_label(value: &str, limit: usize) -> Result<&str> {
    let value = value.trim();
    if value.is_empty() || value.chars().count() > limit {
        return Err(invalid(format!(
            "Client label must contain 1 to {limit} characters"
        )));
    }
    Ok(value)
}
pub fn source_pages(client: &mut Client, task: &str) -> Result<Value> {
    let input = validation::task_input(task)?;
    let source = client.json("/task/source-page/list", &input)?;
    let pages = match field(&source, "status")? {
        "ready" => array(&source, "pages")?.iter().map(|page| json!({"number":page["index"],"width":page["width"],"height":page["height"],"ready":page["ready"]})).collect(),
        "pending" | "failed" => Vec::new(),
        _ => return Err(invalid("Unknown page discovery status")),
    };
    Ok(json!({"taskId":task,"status":source["status"],"message":source["message"],"pages":pages}))
}
pub fn read_status(client: &mut Client, task: &str, deadline: Option<Instant>) -> Result<Value> {
    let status = client.json_with(
        "/task/status",
        &validation::task_input(task)?,
        false,
        deadline,
    )?;
    Ok(status)
}
pub fn conversion_pages(status: &Value) -> Result<Vec<Value>> {
    let pages = array(status, "pageTasks")?
        .iter()
        .filter(|v| v["status"] != "cancelled")
        .cloned()
        .collect::<Vec<_>>();
    Ok(pages)
}
fn needs_attention(status: &Value) -> bool {
    status["status"] == "manual_required"
}
fn cancelled(status: &Value) -> bool {
    matches!(status["status"].as_str(), Some("cancelled" | "cancelling"))
}
pub fn summary(status: &Value) -> Result<Value> {
    let pages = conversion_pages(status)?;
    let ready = !pages.is_empty()
        && pages.iter().all(|v| v["status"] == "ready")
        && !cancelled(status)
        && !needs_attention(status);
    let state = if cancelled(status) || needs_attention(status) {
        field(status, "status")?
    } else if ready {
        "ready"
    } else if pages.iter().any(|v| v["status"] == "processing") {
        "processing"
    } else {
        field(status, "status")?
    };
    let message = if ready {
        json!("Selected pages are ready for preview")
    } else {
        status["message"].clone()
    };
    let pages = pages
        .iter()
        .map(|page| {
            json!({
                "number": page["pageStart"],
                "status": page["status"],
                "paymentState": page["paymentState"],
            })
        })
        .collect::<Vec<_>>();
    Ok(json!({
        "taskId": field(status, "taskId")?,
        "status": state,
        "message": message,
        "progress": status["progress"],
        "pages": pages,
    }))
}

pub fn public_result(mut value: Value, task: &str, page: u32) -> Result<Value> {
    let object = value
        .as_object_mut()
        .ok_or_else(|| invalid("Expected an object response"))?;
    object.insert("taskId".into(), json!(task));
    object.insert("page".into(), json!(page));
    Ok(value)
}

pub fn result_page(
    client: &mut Client,
    task: &str,
    page: Option<u32>,
    command: &str,
) -> Result<(Value, u32)> {
    let result = client.json("/task/result/read", &validation::task_input(task)?)?;
    let pages = array(&result, "pages")?;
    let available = pages
        .iter()
        .map(|value| validation::page_number(value, "page"))
        .collect::<Result<Vec<_>>>()?;
    let matches = available
        .iter()
        .filter(|&&number| page.is_none_or(|p| p == number))
        .copied()
        .collect::<Vec<_>>();
    let [number] = matches.as_slice() else {
        return Err(page_error(
            if page.is_none() {
                "page_required"
            } else {
                "page_not_found"
            },
            "Choose a design page and repeat the command with --page N",
            task,
            available,
            json!({"command":command,"taskId":task,"requiredOption":"--page"}),
        ));
    };
    let selected = pages
        .iter()
        .find(|value| value["page"] == *number)
        .ok_or_else(|| invalid("Page is missing from result"))?;
    if selected["status"] != "ready" {
        return Err(task_error(
            "conversion_not_ready",
            format!("Preview is not ready; run canvatopsd wait {task}"),
            task,
        ));
    }
    Ok((result, *number))
}
pub fn wait(
    client: &mut Client,
    task: &str,
    seconds: u64,
    requested: Option<&[u32]>,
) -> Result<Value> {
    validation::task_id(task)?;
    let deadline = Instant::now()
        .checked_add(Duration::from_secs(seconds))
        .ok_or_else(|| invalid("Wait duration is too large"))?;
    let result = wait_until(client, task, deadline, requested);
    if Instant::now() >= deadline {
        return Err(task_error(
            "wait_timeout",
            format!("Conversion is still pending; resume with canvatopsd wait {task}"),
            task,
        ));
    }
    result
}
fn wait_until(
    client: &mut Client,
    task: &str,
    deadline: Instant,
    requested: Option<&[u32]>,
) -> Result<Value> {
    loop {
        let status = read_status(client, task, Some(deadline))?;
        if status["submissionStatus"] == "received" {
            wait_for_next_read(deadline, task)?;
            continue;
        }
        if status["status"] == "awaiting_payment" {
            return Ok(
                json!({"taskId":task,"status":"awaiting_payment","message":"Payment is required before conversion starts. Run checkout and share checkoutUrl with the user.","nextCommand":format!("canvatopsd checkout {task}")}),
            );
        }
        if needs_attention(&status) || cancelled(&status) {
            return Err(task_error(
                "task_requires_attention",
                "Task requires attention; inspect status",
                task,
            ));
        }
        if status
            .get("selection")
            .is_some_and(|v| !v.is_null() && v["confirmed"] != true)
        {
            wait_for_next_read(deadline, task)?;
            continue;
        } else {
            let pages = conversion_pages(&status)?;
            if pages.is_empty() {
                wait_for_next_read(deadline, task)?;
                continue;
            }
            if let Some(requested) = requested {
                let mut selected = pages
                    .iter()
                    .map(|v| {
                        v["pageStart"]
                            .as_u64()
                            .ok_or_else(|| invalid("Missing conversion page number"))
                    })
                    .collect::<Result<Vec<_>>>()?;
                selected.sort_unstable();
                if selected != requested.iter().map(|v| u64::from(*v)).collect::<Vec<_>>() {
                    return Err(task_error(
                        "page_selection_conflict",
                        "Task already has a different page selection; inspect status",
                        task,
                    ));
                }
            }
            if pages.iter().any(|v| needs_attention(v) || cancelled(v)) {
                return Err(task_error(
                    "task_requires_attention",
                    "A selected page requires attention; inspect status",
                    task,
                ));
            }
            if pages.iter().all(|v| v["status"] == "ready") {
                return summary(&status);
            }
        }
        wait_for_next_read(deadline, task)?;
    }
}

fn wait_for_next_read(deadline: Instant, task: &str) -> Result<()> {
    let remaining = deadline.saturating_duration_since(Instant::now());
    if remaining.is_zero() {
        return Err(task_error(
            "wait_timeout",
            "Conversion wait deadline reached",
            task,
        ));
    }
    std::thread::sleep(remaining.min(Duration::from_secs(5)));
    Ok(())
}
