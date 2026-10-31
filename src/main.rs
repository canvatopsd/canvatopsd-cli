mod args;
mod artifacts;
mod auth;
mod client;
mod config;
mod discovery;
mod error;
mod payment;
mod skill;
mod submission;
mod tasks;
mod updates;
mod validation;
#[cfg(windows)]
mod windows_permissions;

use anyhow::Result;
use args::{Cli, Command, Operator, Payment, Selection};
use clap::Parser;
use client::Client;
use error::{CommandError, submission_error};
use serde_json::{Value, json};
use validation::field;

fn context(options: &args::Options) -> Result<Client> {
    Client::new(config::connection(options, &config::read()?)?)
}
fn run(cli: Cli) -> Result<Value> {
    let options = &cli.options;
    // Offline commands deliberately do not read credentials or initialize networking.
    match &cli.command {
        Command::Update { command } => return updates::run(command, options.timeout),
        Command::Discover { command, advanced } => {
            return discovery::discover(&command.join(" "), *advanced);
        }
        Command::Skill { command } => return skill::run(command),
        Command::Resolve { url } => {
            let parsed = url::Url::parse(url).map_err(|_| error::invalid("Invalid URL"))?;
            return if let Some(task) = validation::task_link(&parsed)? {
                Ok(
                    json!({"taskId":task,"hasAccess":url::form_urlencoded::parse(parsed.fragment().unwrap_or("").as_bytes()).any(|(k,_)| k == "access" || k == "preview")}),
                )
            } else {
                validation::canva_link(url)
            };
        }
        Command::Init { url } => {
            let init = args::Options {
                base_url: Some(
                    url.clone()
                        .unwrap_or_else(|| "https://canvatopsd.com/api".into()),
                ),
                origin: options.origin.clone(),
                timeout: 30,
            };
            let connection = config::connection(&init, &config::Settings::default())?;
            config::update(|settings| {
                settings.base_url = Some(connection.base_url.clone());
                settings.origin = Some(connection.origin.clone());
                Ok(())
            })?;
            return Ok(json!({"baseUrl":connection.base_url,"origin":connection.origin}));
        }
        Command::Create(input) | Command::Convert { input, .. } => {
            let mut body = tasks::submission(input)?;
            let selection = if let Command::Convert { pages, .. } = &cli.command {
                pages.as_deref().map(validation::pages).transpose()?
            } else {
                None
            };
            if let Some(pages) = &selection {
                body["pages"] = json!(pages);
            }
            if input.dry_run {
                return if let Command::Convert { seconds, .. } = &cli.command {
                    let mut result = json!({"dryRun":true,"input":body,"seconds":seconds});
                    if let Some(pages) = selection {
                        result["pages"] = json!(pages);
                    }
                    Ok(result)
                } else {
                    Ok(
                        json!({"path":"/submission/create","preparePath":"/submission/prepare","input":body,"dryRun":true}),
                    )
                };
            }
            let mut client = context(options)?;
            let created = submission::receive(&mut client, body)?;
            let key = field(&created, "submitKey")?.to_owned();
            if let Command::Convert { seconds, .. } = &cli.command {
                let task = field(&created, "taskId")?;
                eprintln!("{}", json!({"taskId":task}));
                let mut result = tasks::wait(&mut client, task, *seconds, selection.as_deref())
                    .map_err(|e| submission_error(e, &key, Some(task)))?;
                result["submitKey"] = json!(key);
                return Ok(result);
            }
            return Ok(created);
        }
        Command::Select {
            task,
            pages,
            key,
            dry_run,
        } => {
            let body = tasks::selection(task, pages, key.as_deref())?;
            if *dry_run {
                return Ok(
                    json!({"taskId":task,"pages":body["pages"],"submitKey":body["submitKey"],"dryRun":true}),
                );
            }
            client::submit(
                &mut context(options)?,
                "/task/page-request/create",
                &body,
                None,
            )?;
            return Ok(json!({"taskId":task,"pages":body["pages"],"submitKey":body["submitKey"]}));
        }
        _ => (),
    }
    let mut client = context(options)?;
    match &cli.command {
        Command::Doctor => {
            let protocol = client.json("/task/protocol/read", &json!({}))?;
            if protocol["protocol"] != "canvatopsd-cli-v1"
                || protocol["taskAccess"] != "task-token-v1"
                || protocol["optionalEmail"] != true
            {
                return Err(CommandError::new(
                    "api_protocol_unsupported",
                    "API does not support this CLI's task credential protocol",
                )
                .into());
            }
            let config = &client.config;
            let web = url::Url::parse(&config.base_url)?
                .path()
                .starts_with("/api");
            Ok(
                json!({"baseUrl":config.base_url,"origin":config.origin,"version":env!("CARGO_PKG_VERSION"),"runtime":"rust","endpoint":if web {"web"} else {"api"},"protocol":protocol,"taskSessions":config.cookies.len(),"operatorAuth":if config.operator_token.is_some() {"env"} else {"missing"},"proxyAuth":if web {"web-managed"} else if config.proxy_token.is_some() {"env"} else {"missing"},"note":"API protocol verified; task authorization, Runner readiness, payment and PSD delivery require separate checks."}),
            )
        }
        Command::Status { task } => tasks::summary(&tasks::read_status(&mut client, task, None)?),
        Command::Wait { task, seconds } => tasks::wait(&mut client, task, *seconds, None),
        Command::Pages { task, refresh } => tasks::source_pages(&mut client, task, *refresh),
        Command::Preview { selection, out } => {
            artifacts::preview(&mut client, &selection.task, selection.page, out.as_deref())
        }
        Command::Download {
            selection,
            out,
            format,
        } => artifacts::download(&mut client, &selection.task, selection.page, format, out),
        Command::Files(selection) => artifacts::files(&mut client, &selection.task, selection.page),
        Command::Payment(input) => {
            if let Some(command) = &input.command {
                payment::run(&mut client, command)
            } else {
                let selection = Selection {
                    task: input.task.clone().ok_or_else(|| {
                        error::invalid("Use payment status TASK or payment confirm TASK")
                    })?,
                    page: input.page,
                };
                payment::run(&mut client, &Payment::Status(selection))
            }
        }
        Command::Confirm(selection) => {
            payment::run(&mut client, &Payment::Confirm(selection.clone()))
        }
        Command::Checkout { selection, dry_run } => {
            payment::checkout(&mut client, selection, *dry_run)
        }
        Command::Auth { command } => auth::run(&mut client, command),
        Command::Operator { command } => match command {
            Operator::TaskList { offset } => {
                client.json_with("/task/list", &json!({"offset":offset}), true, None)
            }
            Operator::NotificationStatus { task } => client.json_with(
                "/notification/status",
                &validation::task_input(task)?,
                true,
                None,
            ),
        },
        Command::Request {
            path,
            body,
            operator,
            out,
        } => {
            let input: Value = serde_json::from_slice(&std::fs::read(body)?)?;
            if let Some(out) = out {
                artifacts::ensure_new_file(out)?;
                artifacts::save(client.post(path, &input, *operator, None)?, out, None)
            } else {
                client.json_with(path, &input, *operator, None)
            }
        }
        _ => unreachable!("Offline commands handled before initializing client"),
    }
}
fn main() {
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(error) if error.kind() == clap::error::ErrorKind::DisplayVersion => {
            println!("{}", env!("CARGO_PKG_VERSION"));
            return;
        }
        Err(error) if error.kind() == clap::error::ErrorKind::DisplayHelp => {
            print!("{error}");
            return;
        }
        Err(error) => {
            emit_error(error::argument_error(&error));
        }
    };
    match run(cli) {
        Ok(data) => println!(
            "{}",
            serde_json::to_string_pretty(&json!({"ok":true,"data":data}))
                .expect("JSON serialization")
        ),
        Err(error) => {
            let value = error
                .downcast_ref::<CommandError>()
                .map(CommandError::value)
                .unwrap_or_else(|| json!({"code":"cli_error","message":format!("{error:#}")}));
            emit_error(value);
        }
    }
}
fn emit_error(error: Value) -> ! {
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({"ok":false,"error":error}))
            .expect("JSON serialization")
    );
    std::process::exit(1)
}
