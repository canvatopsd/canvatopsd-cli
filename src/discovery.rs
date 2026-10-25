use crate::{args::Cli, error::CommandError};
use clap::CommandFactory;
use serde_json::{Value, json};

fn describe(command: &clap::Command) -> Value {
    let arguments = command
        .get_positionals()
        .map(|argument| {
            json!({
                "name": argument.get_id().as_str(),
                "required": argument.is_required_set(),
                "variadic": false,
                "description": argument.get_help().map(ToString::to_string)
            })
        })
        .collect::<Vec<_>>();
    let options = command.get_arguments().filter(|argument| argument.get_long().is_some()).map(|argument| {
        let mut flags = format!("--{}", argument.get_long().unwrap());
        if argument.get_action().takes_values() {
            let name = argument.get_value_names().and_then(|values| values.first())
                .map(|name| name.as_str()).unwrap_or(argument.get_id().as_str());
            flags.push_str(&format!(" <{name}>"));
        }
        json!({
            "flags": flags,
            "description": argument.get_help().map(ToString::to_string),
            "required": argument.is_required_set(),
            "default": argument.get_default_values().first().map(|value| value.to_string_lossy()),
            "choices": argument.get_possible_values().iter().map(|value| value.get_name()).collect::<Vec<_>>()
        })
    }).collect::<Vec<_>>();
    json!({
        "name": command.get_name(),
        "description": command.get_about().map(ToString::to_string),
        "arguments": arguments,
        "options": options,
        "commands": command.get_subcommands().map(describe).collect::<Vec<_>>()
    })
}
pub fn discover(path: Option<&str>) -> anyhow::Result<Value> {
    let mut root = Cli::command();
    root.build();
    let mut command = &root;
    for name in path.unwrap_or("").split_whitespace() {
        command = command
            .get_subcommands()
            .find(|c| c.get_name() == name)
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
        json!({"version":env!("CARGO_PKG_VERSION"),"product":product,"output":"JSON envelope on stdout; diagnostics on stderr; help/version are text","exitCodes":{"success":0,"failure":1},"command":describe(command)}),
    )
}
