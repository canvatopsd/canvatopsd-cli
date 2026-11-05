// Shared build contract for independently built native deployment units.
pub fn embed(default_environment: &str) {
    use std::{env, fs, path::PathBuf};
    for key in [
        "CANVATOPSD_BUILD_CONFIGURATION",
        "CANVATOPSD_BUILD_ENVIRONMENT",
        "CANVATOPSD_BUILD_REVISION",
    ] {
        println!("cargo:rerun-if-env-changed={key}");
    }
    let environment =
        env::var("CANVATOPSD_BUILD_ENVIRONMENT").unwrap_or_else(|_| default_environment.into());
    assert!(
        matches!(environment.as_str(), "local" | "prod"),
        "Invalid build environment"
    );
    let contents = match env::var_os("CANVATOPSD_BUILD_CONFIGURATION") {
        Some(path) => {
            println!("cargo:rerun-if-changed={}", PathBuf::from(&path).display());
            fs::read(path).expect("Read private build configuration")
        }
        None => b"{}".to_vec(),
    };
    let output = PathBuf::from(env::var_os("OUT_DIR").expect("Cargo output directory"));
    fs::write(output.join("runtime.json"), &contents).expect("Embed private build configuration");
    let version = fs::read_to_string("VERSION")
        .unwrap_or_else(|_| env::var("CARGO_PKG_VERSION").expect("Package version"));
    let revision = env::var("CANVATOPSD_BUILD_REVISION").unwrap_or_else(|_| "development".into());
    println!("cargo:rustc-env=CP_INSTALL_ENVIRONMENT={environment}");
    println!("cargo:rustc-env=CP_INSTALL_VERSION={}", version.trim());
    println!("cargo:rustc-env=CP_INSTALL_REVISION={revision}");
    let os = env::var("CARGO_CFG_TARGET_OS").expect("Target OS");
    let arch = env::var("CARGO_CFG_TARGET_ARCH").expect("Target architecture");
    let identity = format!(
        r#"{{"version":"{}","environment":"{environment}","revision":"{revision}","operatingSystem":"{os}","architecture":"{arch}","embeddedConfiguration":{}}}"#,
        version.trim(),
        contents != b"{}"
    );
    println!("cargo:rustc-env=CP_INSTALL_IDENTITY={identity}");
}
