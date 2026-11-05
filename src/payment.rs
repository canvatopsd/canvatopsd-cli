use crate::{
    args::{Payment, Selection},
    client::Client,
    error::{invalid, page_error},
    validation::{self, array},
};
use anyhow::Result;
use serde_json::{Value, json};

fn payment_pages(result: &Value, selection: &Selection, command: &str) -> Result<()> {
    let pages = array(result, "pages")?
        .iter()
        .map(|value| {
            value
                .as_u64()
                .and_then(|page| u32::try_from(page).ok())
                .filter(|page| *page > 0)
                .ok_or_else(|| invalid("Invalid payment page number"))
        })
        .collect::<Result<Vec<_>>>()?;
    if pages.is_empty() {
        return Err(invalid("Payment response has no selected pages"));
    }
    if selection.page.is_some_and(|page| !pages.contains(&page)) {
        return Err(page_error(
            "page_not_found",
            "Payment covers the task's selected pages; choose a selected page or omit --page",
            &selection.task,
            pages,
            json!({"command":command,"taskId":selection.task}),
        ));
    }
    Ok(())
}

pub fn run(client: &mut Client, command: &Payment) -> Result<Value> {
    let (selection, path) = match command {
        Payment::Status(selection) => (selection, "/task/prepayment/read"),
        Payment::Confirm(selection) => (selection, "/task/prepayment/confirm"),
    };
    let input = validation::task_input(&selection.task)?;
    // Check a requested page before a confirmation can synchronize or capture payment.
    if matches!(command, Payment::Confirm(_)) && selection.page.is_some() {
        payment_pages(
            &client.json("/task/prepayment/read", &input)?,
            selection,
            "payment",
        )?;
    }
    let mut result = client.json(path, &input)?;
    payment_pages(&result, selection, "payment")?;
    result
        .as_object_mut()
        .ok_or_else(|| invalid("Expected a payment object"))?
        .insert("taskId".into(), json!(selection.task));
    Ok(result)
}

pub fn checkout(client: &mut Client, selection: &Selection, dry_run: bool) -> Result<Value> {
    let input = validation::task_input(&selection.task)?;
    let payment = client.json("/task/prepayment/read", &input)?;
    payment_pages(&payment, selection, "checkout")?;
    let mut result = json!({
        "taskId": selection.task,
        "pages": payment["pages"],
        "amountCents": payment["amountCents"],
        "currency": "USD",
        "paymentState": payment["paymentState"],
        "activated": payment["activated"],
        "cancelled": payment["cancelled"],
        "reconciliationRequired": payment["reconciliationRequired"],
        "errorCode": payment["errorCode"],
        "dryRun": dry_run,
    });
    if !dry_run {
        let link = client.json("/task/preview/link/create", &input)?;
        result["checkoutUrl"] = json!(validation::field(&link, "previewUrl")?);
        result["expiresAt"] = json!(validation::field(&link, "expiresAt")?);
    }
    Ok(result)
}
