use crate::{
    client::Client,
    error::invalid,
    tasks,
    validation::{array, field},
};
use anyhow::{Context, Result};
use reqwest::blocking::Response;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    io::{Read, Write},
    path::Path,
};

pub fn save(mut response: Response, output: &str, receipt: Option<&Value>) -> Result<Value> {
    let path = std::path::absolute(output)?;
    if path.exists() {
        anyhow::bail!("Output already exists: {}", path.display());
    }
    let parent = path
        .parent()
        .context("Output requires a parent directory")?;
    std::fs::create_dir_all(parent)?;
    let mut temporary = tempfile::NamedTempFile::new_in(parent)?;
    let mut bytes = 0u64;
    let mut hash = receipt.map(|_| Sha256::new());
    let mut buffer = [0u8; 65536];
    loop {
        let length = response
            .read(&mut buffer)
            .context("Failed while reading download stream")?;
        if length == 0 {
            break;
        }
        temporary.write_all(&buffer[..length])?;
        bytes += length as u64;
        if let Some(hash) = hash.as_mut() {
            hash.update(&buffer[..length]);
        }
    }
    if let Some(receipt) = receipt {
        let expected_bytes = receipt["bytes"]
            .as_u64()
            .ok_or_else(|| invalid("Invalid delivery receipt bytes"))?;
        let expected_hash = field(receipt, "sha256")?;
        let actual_hash = hex::encode(hash.context("Missing delivery digest")?.finalize());
        if bytes != expected_bytes || actual_hash != expected_hash {
            anyhow::bail!("Downloaded size or SHA-256 does not match delivery receipt");
        }
    }
    temporary.as_file().sync_all()?;
    temporary.persist_noclobber(&path)?;
    let mut result = json!({"path":path,"bytes":bytes});
    if let Some(receipt) = receipt {
        result["sha256"] = receipt["sha256"].clone();
        result["verified"] = json!(true);
    }
    Ok(result)
}
fn select_page(pages: &[Value], artifact: Option<&str>, number: Option<u32>) -> Result<String> {
    if let Some(id) = artifact {
        return pages
            .iter()
            .find(|v| v["pageId"] == id)
            .map(|_| id.to_owned())
            .ok_or_else(|| invalid("Page ID does not belong to this task"));
    }
    if let Some(number) = number {
        return pages
            .iter()
            .find(|v| v["index"] == number)
            .map(|v| field(v, "pageId").map(str::to_owned))
            .transpose()?
            .ok_or_else(|| invalid("Requested page is not in the available files"));
    }
    if pages.len() != 1 {
        return Err(invalid("Expected one file page; specify --artifact-page"));
    }
    Ok(field(&pages[0], "pageId")?.into())
}
pub fn preview(
    client: &mut Client,
    task: &str,
    page: Option<u32>,
    artifact: Option<&str>,
    file: &str,
    output: Option<&str>,
) -> Result<Value> {
    if output.is_none() && (artifact.is_some() || file != "preview.webp") {
        return Err(invalid("Use --out to save the selected preview"));
    }
    if let Some(output) = output {
        ensure_new_file(output)?;
    }
    let quote = tasks::quote(client, task, page, "preview")?;
    let mut input = quote.input;
    input["name"] = json!(file);
    let listing = client.json("/task/preview/list", &input)?;
    let Some(output) = output else {
        return tasks::public_result(listing, task, quote.page);
    };
    input["pageId"] = json!(select_page(array(&listing, "pages")?, artifact, page)?);
    let saved = save(
        client.post("/task/preview/read", &input, false, None)?,
        output,
        None,
    )?;
    tasks::public_result(saved, task, quote.page)
}
pub fn download(
    client: &mut Client,
    task: &str,
    page: Option<u32>,
    artifact: Option<&str>,
    file: &str,
    output: &str,
) -> Result<Value> {
    ensure_new_file(output)?;
    let quote = tasks::quote(client, task, page, "download")?;
    let mut input = quote.input;
    client.json("/task/checkout/read", &input)?;
    let listing = client.json("/task/delivery/read", &input)?;
    input["pageId"] = json!(select_page(array(&listing, "pages")?, artifact, page)?);
    input["name"] = json!(file);
    let receipt = client.json("/task/delivery/download-url/create", &input)?;
    let response = client.download(field(&receipt, "url")?)?;
    let mut result = save(response, output, Some(&receipt))?;
    result["taskId"] = json!(task);
    result["page"] = json!(quote.page);
    result["pageId"] = input["pageId"].clone();
    result["name"] = json!(file);
    Ok(result)
}
pub fn ensure_new_file(output: &str) -> Result<()> {
    match std::fs::symlink_metadata(Path::new(output)) {
        Ok(_) => anyhow::bail!("Output already exists: {output}"),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(e.into()),
    }
}

pub fn files(client: &mut Client, task: &str, page: Option<u32>) -> Result<Value> {
    let quote = tasks::quote(client, task, page, "files")?;
    let listing = client.json("/task/delivery/read", &quote.input)?;
    tasks::public_result(listing, task, quote.page)
}
