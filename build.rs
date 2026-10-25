use serde_json::{Value, json};
use std::{env, fs, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=package.json");
    let manifest_path =
        PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("Cargo manifest directory"));
    let manifest: Value = serde_json::from_slice(
        &fs::read(manifest_path.join("package.json")).expect("Read CLI manifest"),
    )
    .expect("Parse CLI manifest");
    let mut product = json!({
        "name": "CanvaToPSD",
        "package": manifest["name"],
        "description": manifest["description"],
        "homepage": manifest["homepage"]
    });
    for field in [
        "capabilities",
        "inputFormats",
        "outputFormats",
        "documentation",
        "service",
    ] {
        product[field] = manifest["canvatopsd"]
            .get(field)
            .unwrap_or_else(|| panic!("Missing public product metadata: {field}"))
            .clone();
    }
    let output = PathBuf::from(env::var_os("OUT_DIR").expect("Cargo output directory"));
    fs::write(
        output.join("product.json"),
        serde_json::to_vec(&product).expect("Serialize public metadata"),
    )
    .expect("Write public metadata");
}
