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
    env,
    fs::{self, OpenOptions},
    path::PathBuf,
};
pub type Entries = BTreeMap<String, String>;
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
    Ok(match env::var_os("CANVATOPSD_CONFIG") {
        Some(path) => path.into(),
        None => home()?.join(".config/canvatopsd/config.json"),
    })
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
    let base = options
        .base_url
        .clone()
        .or_else(|| env::var("CANVATOPSD_BASE_URL").ok())
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
        .or_else(|| env::var("CANVATOPSD_ORIGIN").ok())
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
        operator_token: env::var("CANVATOPSD_OPERATOR_TOKEN").ok(),
        proxy_token: env::var("CANVATOPSD_PROXY_TOKEN").ok(),
    })
}
