use crate::{
    args::{Payment, Selection},
    client::Client,
    tasks,
};
use anyhow::Result;
use serde_json::{Value, json};

pub fn run(client: &mut Client, command: &Payment) -> Result<Value> {
    let (selection, path, name) = match command {
        Payment::Status(selection) => (selection, "/task/checkout/read", "payment status"),
        Payment::Confirm(selection) => (selection, "/task/checkout/confirm", "payment confirm"),
    };
    let quote = tasks::quote(client, &selection.task, selection.page, name)?;
    let result = client.json(path, &quote.input)?;
    payment_result(result, &selection.task, quote.page)
}

fn payment_result(result: Value, task: &str, page: u32) -> Result<Value> {
    tasks::public_result(
        json!({"paymentState":result["paymentState"],"checkoutUrl":result["checkoutUrl"]}),
        task,
        page,
    )
}

pub fn checkout(client: &mut Client, selection: &Selection, dry_run: bool) -> Result<Value> {
    let quote = tasks::quote(client, &selection.task, selection.page, "checkout")?;
    let mut input = quote.input;
    input["returnTaskId"] = json!(selection.task);
    if dry_run {
        return Ok(json!({"taskId":selection.task,"page":quote.page,"dryRun":true}));
    }
    let result = client.json("/task/checkout/create", &input)?;
    payment_result(result, &selection.task, quote.page)
}
