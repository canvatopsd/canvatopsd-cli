// Runtime filesystem and embedded release identity; contains no unit-specific behavior.
use std::path::PathBuf;
pub const CONFIGURATION: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/runtime.json"));
#[used]
static IDENTITY: &str = concat!(
    "CANVATOPSD_INSTALLATION_V1:",
    env!("CP_INSTALL_IDENTITY"),
    ":END_INSTALLATION"
);
pub const ENVIRONMENT: &str = env!("CP_INSTALL_ENVIRONMENT");
pub fn root() -> anyhow::Result<PathBuf> {
    let home = std::env::home_dir()
        .ok_or_else(|| anyhow::anyhow!("Cannot locate current user home directory"))?;
    Ok(home.join(".local/share/canvatopsd").join(ENVIRONMENT))
}
/// Inspects the final executable without reading credentials, connecting or creating state.
pub fn inspect() -> bool {
    if std::env::args().skip(1).collect::<Vec<_>>() != ["installation-info"] {
        return false;
    }
    std::hint::black_box(IDENTITY);
    let directory = match root() {
        Ok(path) => path,
        Err(error) => {
            eprintln!("{error:#}");
            std::process::exit(1);
        }
    };
    println!(
        "{}",
        serde_json::json!({
            "version":env!("CP_INSTALL_VERSION"), "environment":ENVIRONMENT,
            "revision":env!("CP_INSTALL_REVISION"), "operatingSystem":std::env::consts::OS,
            "architecture":std::env::consts::ARCH, "root":directory,"rootConvention":format!("~/.local/share/canvatopsd/{ENVIRONMENT}"),
            "embeddedConfiguration":CONFIGURATION != b"{}",
        })
    );
    true
}
