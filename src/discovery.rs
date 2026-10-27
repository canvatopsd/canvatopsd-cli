use crate::{
    args::{Cli, MAX_REQUEST_SECONDS},
    error::CommandError,
};
use clap::CommandFactory;
use serde_json::{Value, json};

fn argument_schema(argument: &clap::Arg) -> Value {
    let mut schema = if !argument.get_action().takes_values() {
        json!({"type":"boolean"})
    } else {
        match argument.get_id().as_str() {
            "timeout" => json!({"type":"integer","minimum":1,"maximum":MAX_REQUEST_SECONDS}),
            "seconds" => json!({"type":"integer","minimum":1}),
            "page" => json!({"type":"integer","minimum":1,"maximum":u32::MAX}),
            "offset" => json!({"type":"integer","minimum":0}),
            "task" | "key" => json!({"type":"string","format":"uuid"}),
            "url" => json!({"type":"string","format":"uri"}),
            _ => json!({"type":"string"}),
        }
    };
    let choices = argument
        .get_possible_values()
        .iter()
        .map(|v| v.get_name().to_owned())
        .collect::<Vec<_>>();
    if !choices.is_empty() {
        schema["enum"] = json!(choices);
    }
    schema
}

fn effects(path: &[&str]) -> Value {
    let root = path.first().copied().unwrap_or("");
    let child = path.get(1).copied().unwrap_or("");
    let (network, local_write, server_action) = match (root, child) {
        ("", _) | ("help" | "discover" | "resolve", _) => (false, false, "none"),
        ("skill", "install") | ("init", _) | ("auth", "clear" | "export") => (false, true, "none"),
        ("skill", _) | ("auth", "status") => (false, false, "none"),
        ("auth", "import") => (true, true, "access-exchange"),
        ("convert" | "create" | "select", _) => (true, true, "conversion-submit"),
        ("pages" | "wait", _) => (true, true, "page-discovery-may-start-conversion"),
        ("checkout", _) => (true, true, "checkout-create"),
        ("payment", "confirm") | ("confirm", _) => (true, true, "payment-capture"),
        ("payment", _) | ("download", _) => (true, true, "payment-sync"),
        ("request", _) => (true, true, "depends-on-request-path"),
        _ => (true, true, "read"),
    };
    let mut result = json!({"network":network,"mayWriteLocalState":local_write,"serverAction":server_action,
        "mayCapturePayment": if root == "request" { Value::Null } else { json!(root == "confirm" || (root == "payment" && child == "confirm")) },
        "rawApi":root == "request" || root == "operator"});
    if ["convert", "create", "select", "checkout"].contains(&root) {
        result["dryRun"] = json!({"network":root == "checkout","serverAction":if root == "checkout" {"read"} else {"none"},"writesLocalCredentials":root == "checkout"});
    }
    result
}

fn describe(command: &clap::Command, parents: &[&str], advanced: bool) -> Value {
    let mut path = parents.to_vec();
    if !parents.is_empty() || command.get_name() != "canvatopsd" {
        path.push(command.get_name());
    }
    let arguments = command.get_positionals().map(|argument| {
        json!({"name":argument.get_id().as_str(),"required":argument.is_required_set(),
            "variadic":argument.get_num_args().is_some_and(|range| range.max_values() > 1),
            "description":argument.get_help().map(ToString::to_string),"schema":argument_schema(argument)})
    }).collect::<Vec<_>>();
    let options = command.get_arguments().filter(|argument| argument.get_long().is_some() && (advanced || !argument.is_hide_set())).map(|argument| {
        let mut flags = format!("--{}", argument.get_long().unwrap());
        if argument.get_action().takes_values() {
            let name = argument.get_value_names().and_then(|values| values.first())
                .map(|name| name.as_str()).unwrap_or(argument.get_id().as_str());
            flags.push_str(&format!(" <{name}>"));
        }
        json!({"flags":flags,"description":argument.get_help().map(ToString::to_string),
            "required":argument.is_required_set(),"default":argument.get_default_values().first().map(|v|v.to_string_lossy()),
            "choices":argument.get_possible_values().iter().map(|v|v.get_name()).collect::<Vec<_>>(),
            "advanced":argument.is_hide_set(),"schema":argument_schema(argument)})
    }).collect::<Vec<_>>();
    json!({"name":command.get_name(),"description":command.get_about().map(ToString::to_string),
        "advanced":command.is_hide_set(),"effects":effects(&path),"arguments":arguments,"options":options,
        "commands":command.get_subcommands().filter(|child| advanced || !child.is_hide_set()).map(|child|describe(child, &path, advanced)).collect::<Vec<_>>()})
}
pub fn discover(path: &str, advanced: bool) -> anyhow::Result<Value> {
    let mut root = Cli::command();
    root.build();
    let mut command = &root;
    let names = path.split_whitespace().collect::<Vec<_>>();
    for name in &names {
        command = command
            .get_subcommands()
            .find(|c| c.get_name() == *name)
            .ok_or_else(|| {
                CommandError::new(
                    "unknown_command",
                    "Unknown command path; run canvatopsd discover",
                )
            })?;
    }
    let product: Value =
        serde_json::from_str(include_str!(concat!(env!("OUT_DIR"), "/product.json")))?;
    Ok(
        json!({"version":env!("CARGO_PKG_VERSION"),"product":product,"output":"JSON envelope on stdout; diagnostics on stderr; help/version are text","exitCodes":{"success":0,"failure":1},"command":describe(command, &names[..names.len().saturating_sub(1)], advanced)}),
    )
}
