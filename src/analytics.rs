use crate::{
    args::{Cli, Command, Payment},
    config,
    error::CommandError,
    validation,
};
use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde_json::{Value, json};
use std::time::Duration;

// One completion event per online command, never per status poll or downloaded chunk.
fn parameters(command: &Command) -> Option<Value> {
    let mut parameters = json!({});
    let operation = match command {
        Command::Create(input) | Command::Convert { input, .. } => {
            if input.dry_run {
                return None;
            }
            let pages = if let Command::Convert {
                pages: Some(pages), ..
            } = command
            {
                validation::pages(pages).ok()
            } else {
                Some(vec![1])
            };
            if let Some(pages) = pages {
                parameters["page_count"] = json!(pages.len());
                let selection = pages
                    .iter()
                    .map(u32::to_string)
                    .collect::<Vec<_>>()
                    .join(",");
                if selection.len() <= 100 {
                    parameters["page_selection"] = json!(selection);
                }
            }
            if matches!(command, Command::Create(_)) {
                "create"
            } else {
                "convert"
            }
        }
        Command::Status { .. } => "status",
        Command::Wait { .. } => "wait",
        Command::Pages { .. } => "pages",
        Command::Preview { selection, out } => {
            parameters["page_number"] = json!(selection.page);
            parameters["preview_mode"] = json!(if out.is_some() { "image" } else { "link" });
            "preview"
        }
        Command::Download {
            selection, format, ..
        } => {
            parameters["page_number"] = json!(selection.page);
            parameters["file_type"] = json!(format);
            "download"
        }
        Command::Files(selection) => {
            parameters["page_number"] = json!(selection.page);
            "files"
        }
        Command::Checkout { dry_run, .. } => {
            if *dry_run {
                return None;
            }
            "checkout"
        }
        Command::Payment(input) => match &input.command {
            Some(Payment::Confirm(_)) => "payment_confirm",
            _ => "payment_status",
        },
        Command::Confirm(_) => "payment_confirm",
        Command::Doctor => "doctor",
        Command::Request { path, .. } => {
            if path.len() <= 100
                && regex::Regex::new(r"^/[a-z][a-z0-9/-]*$")
                    .ok()?
                    .is_match(path)
            {
                parameters["api_action"] = json!(path);
            }
            "request"
        }
        Command::Update { .. } => "update_check",
        // These commands promise offline behavior and must never initialize networking.
        Command::Discover { .. }
        | Command::Skill { .. }
        | Command::Resolve { .. }
        | Command::Init { .. }
        | Command::Auth { .. } => return None,
    };
    parameters["operation"] = json!(operation);
    parameters
        .as_object_mut()?
        .retain(|_, value| !value.is_null());
    Some(parameters)
}

pub fn record(cli: &Cli, result: &Result<Value>, elapsed: Duration) -> Result<()> {
    let Some(mut input) = parameters(&cli.command) else {
        return Ok(());
    };
    let connection = config::connection(&cli.options, &config::read()?)?;
    let mut endpoint = url::Url::parse(&connection.base_url)?;
    // Analytics belongs to the Web command API, not a separately configured API service.
    if endpoint.path().trim_end_matches('/') != "/api" {
        return Ok(());
    }
    endpoint.set_path("/api/analytics/cli/record");
    input["client_id"] = json!(hex::encode(rand::random::<[u8; 16]>()));
    input["client_version"] = json!(env!("CARGO_PKG_VERSION"));
    input["client_platform"] = json!(std::env::consts::OS);
    input["client_arch"] = json!(std::env::consts::ARCH);
    input["duration_ms"] = json!(elapsed.as_millis() as u64);
    input["command_result"] = json!(if result.is_ok() { "success" } else { "error" });
    if let Err(error) = result {
        let code = error
            .downcast_ref::<CommandError>()
            .map(|error| error.code.as_str())
            .unwrap_or("cli_error");
        let safe = code.len() <= 64
            && !code.is_empty()
            && code
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_');
        input["error_code"] = json!(if safe { code } else { "cli_error" });
    }
    let client = Client::builder()
        .timeout(Duration::from_secs(3))
        .redirect(reqwest::redirect::Policy::none())
        .build()?;
    let response = client
        .post(endpoint)
        .header("Origin", connection.origin)
        .json(&input)
        .send()
        .map_err(crate::error::network_error)?;
    let status = response.status();
    if !status.is_success() {
        anyhow::bail!("Analytics endpoint returned HTTP {status}");
    }
    let receipt: Value = response.json().context("Invalid analytics receipt")?;
    if !matches!(
        receipt["analyticsState"].as_str(),
        Some("accepted" | "local_only")
    ) {
        anyhow::bail!("Analytics endpoint did not acknowledge the event");
    }
    Ok(())
}
