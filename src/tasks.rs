use crate::{
    args::Submission,
    client::{Client, submit},
    error::{invalid, page_error, task_error},
    validation::{self, array, field},
};
use anyhow::Result;
use serde_json::{Value, json};
use std::time::{Duration, Instant};

pub fn submission(input: &Submission) -> Result<Value> {
    let key = input
        .key
        .clone()
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    let mut body = json!({"submitKey":validation::uuid(&key)?,"canvaUrl":validation::canva_link(&input.url)?["url"],"layerNaming":!input.no_layer_naming});
    if let Some(email) = &input.email {
        if !email_address::EmailAddress::is_valid(email) {
            return Err(invalid("Invalid notification email"));
        }
        body["email"] = json!(email.to_lowercase());
    }
    Ok(body)
}
pub fn selection(task: &str, pages: &str, key: Option<&str>) -> Result<Value> {
    Ok(
        json!({"taskId":validation::uuid(task)?,"submitKey":validation::uuid(&key.map(str::to_owned).unwrap_or_else(|| uuid::Uuid::new_v4().to_string()))?,"pages":validation::pages(pages)?}),
    )
}
pub fn read_status(client: &mut Client, task: &str, deadline: Option<Instant>) -> Result<Value> {
    client.json_with(
        "/task/status",
        &validation::task_input(task)?,
        false,
        deadline,
    )
}
pub fn conversion_pages(status: &Value) -> Result<Vec<Value>> {
    let mut pages = array(status, "pageTasks")?
        .iter()
        .filter(|v| v["status"] != "cancelled")
        .cloned()
        .collect::<Vec<_>>();
    if status.get("selection").is_none_or(Value::is_null) {
        let page = status
            .get("sourcePage")
            .and_then(Value::as_u64)
            .unwrap_or(1);
        pages.insert(0, json!({"taskId":field(status,"taskId")?,"pageStart":page,"pageEnd":page,"status":field(status,"status")?,"inputRequired":status["inputRequired"],"paymentState":status["quote"]["payment_state"]}));
    }
    Ok(pages)
}
fn cancelled(status: &Value) -> bool {
    matches!(status["status"].as_str(), Some("cancelled" | "cancelling"))
}
fn requires_input(status: &Value) -> bool {
    status["inputRequired"].as_bool().unwrap_or(false)
}
pub fn summary(status: &Value) -> Result<Value> {
    let pages = conversion_pages(status)?;
    let ready =
        !pages.is_empty() && pages.iter().all(|v| v["status"] == "ready") && !cancelled(status);
    let state = if ready {
        "ready"
    } else if pages.iter().any(|v| v["status"] == "processing") {
        "processing"
    } else {
        field(status, "status")?
    };
    Ok(
        json!({"taskId":field(status,"taskId")?,"status":state,"inputRequired":requires_input(status) || pages.iter().any(requires_input),"message":if ready { json!("Selected pages are ready for preview") } else { status["message"].clone() },"pageSelectionRequired":status.get("selection").is_some_and(|v| !v.is_null() && v["confirmed"] != true),"pages":pages.iter().map(|v| json!({"number":v["pageStart"],"status":v["status"],"paymentState":v["paymentState"]})).collect::<Vec<_>>()}),
    )
}
pub struct TaskQuote {
    pub input: Value,
    pub page: u32,
}

pub fn public_result(mut value: Value, task: &str, page: u32) -> Result<Value> {
    let object = value
        .as_object_mut()
        .ok_or_else(|| invalid("Expected an object response"))?;
    object.insert("taskId".into(), json!(task));
    object.insert("page".into(), json!(page));
    Ok(value)
}

pub fn quote(
    client: &mut Client,
    task: &str,
    page: Option<u32>,
    command: &str,
) -> Result<TaskQuote> {
    let mut status = read_status(client, task, None)?;
    if cancelled(&status) {
        return Err(task_error(
            "task_requires_attention",
            "Task is cancelled or cancelling; inspect status",
            task,
        ));
    }
    let pages = conversion_pages(&status)?;
    let candidates = pages
        .iter()
        .filter(|v| page.is_none_or(|p| v["pageStart"] == p && v["pageEnd"] == p))
        .collect::<Vec<_>>();
    if candidates.len() != 1 {
        let available = pages
            .iter()
            .map(|v| validation::page_number(v, "pageStart"))
            .collect::<Result<Vec<_>>>()?;
        return Err(page_error(
            if page.is_none() {
                "page_required"
            } else {
                "page_not_found"
            },
            "Choose a design page from availablePages and repeat the command with --page N",
            task,
            available,
            json!({"command":command,"taskId":task,"requiredOption":"--page"}),
        ));
    }
    let number = validation::page_number(candidates[0], "pageStart")?;
    let id = field(candidates[0], "taskId")?.to_owned();
    if id != task {
        status = read_status(client, &id, None)?;
    }
    if status["quote"].is_null() {
        return Err(task_error(
            "conversion_not_ready",
            format!("Preview is not ready; run canvatopsd wait {task}"),
            task,
        ));
    }
    let quote = validation::uuid(field(&status["quote"], "quote_id")?)?;
    Ok(TaskQuote {
        input: json!({"taskId":validation::uuid(field(&status,"taskId")?)?,"quoteId":quote}),
        page: number,
    })
}
pub fn wait(
    client: &mut Client,
    task: &str,
    seconds: u64,
    requested: Option<&[u32]>,
) -> Result<Value> {
    validation::uuid(task)?;
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
        if requires_input(&status) || cancelled(&status) {
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
            let source = client.json_with(
                "/task/source-page/list",
                &json!({"taskId":task,"refresh":false}),
                false,
                Some(deadline),
            )?;
            match field(&source, "status")? {
                "failed" => {
                    return Err(task_error(
                        "page_discovery_failed",
                        field(&source, "message")?,
                        task,
                    ));
                }
                "ready" => {
                    let pages = array(&source, "pages")?;
                    let available = pages
                        .iter()
                        .map(|v| validation::page_number(v, "index"))
                        .collect::<Result<Vec<_>>>()?;
                    if requested.is_some_and(|selection| {
                        selection
                            .iter()
                            .any(|p| !pages.iter().any(|v| v["index"] == *p))
                    }) {
                        return Err(page_error(
                            "page_out_of_bounds",
                            "Choose design page numbers from availablePages",
                            task,
                            available.clone(),
                            json!({"command":"select","taskId":task,"requiredArgument":"pages"}),
                        ));
                    }
                    if source.get("autoTaskId").is_none_or(Value::is_null) {
                        let selection = requested.ok_or_else(|| page_error("page_selection_required", format!("This design has {} pages. Choose from availablePages, run canvatopsd select {task} PAGE_NUMBERS, then canvatopsd wait {task}", pages.len()), task, available, json!({"command":"select","taskId":task,"requiredArgument":"pages"})))?;
                        submit(
                            client,
                            "/task/page-request/create",
                            &json!({"taskId":task,"submitKey":task,"pages":selection}),
                            Some(deadline),
                        )?;
                    }
                    continue;
                }
                "pending" => (),
                _ => return Err(invalid("Unknown page discovery status")),
            }
        } else {
            let pages = conversion_pages(&status)?;
            if pages.is_empty() {
                return Err(task_error(
                    "conversion_pages_missing",
                    "No active conversion pages remain; inspect status",
                    task,
                ));
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
            if pages.iter().any(|v| requires_input(v) || cancelled(v)) {
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
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            return Err(task_error(
                "wait_timeout",
                "Conversion wait deadline reached",
                task,
            ));
        }
        std::thread::sleep(remaining.min(Duration::from_secs(5)));
    }
}
