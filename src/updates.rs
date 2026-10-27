use crate::{
    args::Update,
    error::{CommandError, invalid, network_error},
    validation,
};
use anyhow::Result;
use reqwest::blocking::Client;
use semver::Version;
use serde::Deserialize;
use serde_json::{Value, json};
use std::time::Duration;

#[derive(Deserialize)]
struct ReleaseTags {
    latest: String,
}

pub fn run(command: &Update, timeout: u64) -> Result<Value> {
    let Update::Check { registry } = command;
    let mut url = validation::secure_url(registry)?;
    if !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
    {
        return Err(invalid(
            "Use a registry URL without credentials, query or fragment",
        ));
    }
    url.set_path(&format!(
        "{}/-/package/@canvatopsd%2fcli/dist-tags",
        url.path().trim_end_matches('/')
    ));
    let http = Client::builder()
        .timeout(Duration::from_secs(timeout))
        .redirect(reqwest::redirect::Policy::none())
        .user_agent(concat!("canvatopsd-cli/", env!("CARGO_PKG_VERSION")))
        .build()
        .map_err(network_error)?;
    let response = http.get(url).send().map_err(network_error)?;
    if !response.status().is_success() {
        let mut error = CommandError::new(
            "update_registry_failed",
            format!(
                "Update registry returned HTTP {}",
                response.status().as_u16()
            ),
        );
        error.status = Some(response.status().as_u16());
        return Err(error.into());
    }
    let release: ReleaseTags = response.json().map_err(network_error)?;
    let latest = Version::parse(&release.latest)
        .map_err(|_| invalid("Registry returned an invalid release version"))?;
    if !latest.pre.is_empty() {
        return Err(invalid(
            "Registry latest tag must identify a stable release",
        ));
    }
    let current = Version::parse(env!("CARGO_PKG_VERSION"))?;
    let update = latest > current;
    Ok(json!({
        "package":"@canvatopsd/cli", "currentVersion":current.to_string(), "latestVersion":latest.to_string(),
        "updateAvailable":update, "status":if update {"update_available"} else if current > latest {"ahead"} else {"current"},
        "installCommand":if update {json!(["npm","install","--global",format!("@canvatopsd/cli@{latest}"),"--registry",registry])} else {Value::Null}
    }))
}
