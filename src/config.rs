use crate::{
    args::Options,
    error::{CommandError, invalid},
    validation::secure_url,
};
use anyhow::{Context, Result};
use fs2::FileExt;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fs::{self, OpenOptions},
    path::PathBuf,
};
pub type Entries = BTreeMap<String, String>;
#[derive(Default, Deserialize)]
#[serde(default, rename_all = "camelCase", deny_unknown_fields)]
struct BuiltinSettings {
    base_url: Option<String>,
    origin: Option<String>,
    operator_token: Option<String>,
    proxy_token: Option<String>,
}
#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(default)]
    pub sessions: BTreeMap<String, Entries>,
    #[serde(default)]
    pub submission_tokens: BTreeMap<String, Entries>,
    #[serde(default)]
    pub task_parents: BTreeMap<String, Entries>,
    #[serde(default)]
    pub internal_submissions: bool,
    #[serde(default)]
    pub local_credentials_migrated: bool,
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}
pub struct Connection {
    pub base_url: String,
    pub origin: String,
    pub timeout: u64,
    pub internal_submissions: bool,
    pub cookies: Entries,
    pub parents: Entries,
    pub operator_token: Option<String>,
    pub proxy_token: Option<String>,
}
pub fn home() -> Result<PathBuf> {
    dirs::home_dir().context("Cannot locate home directory")
}
pub fn state_path() -> Result<PathBuf> {
    Ok(crate::installation::root()?.join("data/cli/session.json"))
}
pub fn read() -> Result<Settings> {
    match fs::File::open(state_path()?) {
        Ok(file) => {
            private_permissions_file(&file, 0o600)?;
            Ok(serde_json::from_reader(file).context("Invalid configuration JSON")?)
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(Settings::default()),
        Err(error) => Err(error.into()),
    }
}
pub fn update<T>(change: impl FnOnce(&mut Settings) -> Result<T>) -> Result<T> {
    let path = state_path()?;
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(std::path::Path::new("."));
    if !parent.exists() {
        fs::create_dir_all(parent)?;
        private_permissions(parent, 0o700)?;
    }
    let lock_path = path.with_file_name(format!(
        "{}.lock",
        path.file_name()
            .context("Config path requires a filename")?
            .to_string_lossy()
    ));
    if lock_path.is_dir() {
        return Err(CommandError::new(
            "ELOCKED",
            "Configuration is locked by an older CLI process",
        )
        .into());
    }
    let lock = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(lock_path)?;
    private_permissions_file(&lock, 0o600)?;
    lock.try_lock_exclusive().map_err(|_| {
        CommandError::new(
            "ELOCKED",
            "Configuration is in use; retry after the other command completes",
        )
    })?;
    let mut settings = read()?;
    let result = change(&mut settings)?;
    let mut temporary = tempfile::NamedTempFile::new_in(parent)?;
    private_permissions_file(temporary.as_file(), 0o600)?;
    use std::io::Write;
    writeln!(temporary, "{}", serde_json::to_string_pretty(&settings)?)?;
    temporary.as_file().sync_all()?;
    temporary.persist(&path)?;
    private_permissions(&path, 0o600)?;
    Ok(result)
}
// Older local installations used the prod state root. Copy only this loopback
// endpoint's credentials once; the old file remains intact and auth clear stays final.
pub fn migrate_local_credentials(settings: &mut Settings, base_url: &str) -> Result<()> {
    if crate::installation::ENVIRONMENT != "local" || settings.local_credentials_migrated {
        return Ok(());
    }
    let url = secure_url(base_url)?;
    if !matches!(url.host_str(), Some("localhost" | "127.0.0.1" | "[::1]")) {
        return Ok(());
    }
    let source = crate::installation::root()?
        .with_file_name("prod")
        .join("data/cli/session.json");
    let file = match fs::File::open(source) {
        Ok(file) => file,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            settings.local_credentials_migrated = true;
            return Ok(());
        }
        Err(error) => return Err(error.into()),
    };
    private_permissions_file(&file, 0o600)?;
    let previous: Settings =
        serde_json::from_reader(file).context("Invalid previous CLI configuration")?;
    for (current, previous) in [
        (&mut settings.sessions, &previous.sessions),
        (&mut settings.submission_tokens, &previous.submission_tokens),
        (&mut settings.task_parents, &previous.task_parents),
    ] {
        if let Some(entries) = previous.get(base_url) {
            let target = current.entry(base_url.to_owned()).or_default();
            for (key, value) in entries {
                if target.get(key).is_some_and(|saved| saved != value) {
                    return Err(invalid(
                        "Local credential migration conflict; restore matching task access before initializing",
                    ));
                }
                target.insert(key.clone(), value.clone());
            }
        }
    }
    settings.local_credentials_migrated = true;
    Ok(())
}

pub fn private_permissions(path: &std::path::Path, mode: u32) -> Result<()> {
    #[cfg(windows)]
    {
        let _ = mode;
        crate::windows_permissions::protect_path(path)
    }
    #[cfg(unix)]
    {
        private_permissions_file(&std::fs::File::open(path)?, mode)
    }
}
pub fn private_permissions_file(file: &std::fs::File, mode: u32) -> Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        file.set_permissions(fs::Permissions::from_mode(mode))?;
    }
    #[cfg(windows)]
    {
        let _ = mode;
        crate::windows_permissions::protect_file(file)?;
    }
    Ok(())
}
pub fn connection(options: &Options, settings: &Settings) -> Result<Connection> {
    let builtin: BuiltinSettings = serde_json::from_slice(crate::installation::CONFIGURATION)?;
    let base = options
        .base_url
        .clone()
        .or(builtin.base_url)
        .or_else(|| settings.base_url.clone())
        .unwrap_or_else(|| "https://canvatopsd.com/api".into());
    let url = secure_url(&base)?;
    if url.query().is_some()
        || url.fragment().is_some()
        || !matches!(url.path(), "/" | "/api" | "/api/")
    {
        return Err(invalid(
            "Base URL path must be / or /api, without query or fragment",
        ));
    }
    let base_url = url.as_str().trim_end_matches('/').to_owned();
    let origin = options
        .origin
        .clone()
        .or(builtin.origin)
        .or_else(|| {
            if settings.base_url.as_ref() == Some(&base_url) {
                settings.origin.clone()
            } else {
                None
            }
        })
        .unwrap_or_else(|| url.origin().ascii_serialization());
    secure_url(&origin)?;
    Ok(Connection {
        cookies: settings
            .sessions
            .get(&base_url)
            .cloned()
            .unwrap_or_default(),
        parents: settings
            .task_parents
            .get(&base_url)
            .cloned()
            .unwrap_or_default(),
        base_url,
        origin,
        timeout: options.timeout,
        internal_submissions: settings.internal_submissions,
        operator_token: builtin.operator_token,
        proxy_token: builtin.proxy_token,
    })
}
