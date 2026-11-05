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
pub fn preview(
    client: &mut Client,
    task: &str,
    page: Option<u32>,
    output: Option<&str>,
) -> Result<Value> {
    let Some(output) = output else {
        let mut input = crate::validation::task_input(task)?;
        if let Some(page) = page {
            input["page"] = json!(page);
        }
        return client.json("/task/preview/link/create", &input);
    };
    ensure_new_file(output)?;
    let (_, number) = tasks::result_page(client, task, page, "preview")?;
    let saved = save(
        client.post(
            "/task/result/preview/read",
            &json!({"taskId":task,"page":number}),
            false,
            None,
        )?,
        output,
        None,
    )?;
    tasks::public_result(saved, task, number)
}

pub fn download(
    client: &mut Client,
    task: &str,
    page: Option<u32>,
    format: &str,
    output: &str,
) -> Result<Value> {
    ensure_new_file(output)?;
    let file = match format {
        "psd" => "page-1.psd",
        "fonts" => "fonts.zip",
        "font-list" => "fonts.txt",
        _ => return Err(invalid("Choose psd, fonts or font-list")),
    };
    let (_, number) = tasks::result_page(client, task, page, "download")?;
    let receipt = client.json(
        "/task/result/download-url/create",
        &json!({"taskId":task,"page":number,"name":file}),
    )?;
    let response = client.download(field(&receipt, "url")?)?;
    let mut result = save(response, output, Some(&receipt))?;
    result["taskId"] = json!(task);
    result["page"] = json!(number);
    result["format"] = json!(format);
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
    let (result, number) = tasks::result_page(client, task, page, "files")?;
    let selected = array(&result, "pages")?
        .iter()
        .find(|value| value["page"] == number)
        .ok_or_else(|| invalid("Page is missing from result"))?;
    let files = array(selected, "files")?
        .iter()
        .filter_map(|file| {
            let format = match file["name"].as_str()? {
                "page-1.psd" => "psd",
                "fonts.zip" => "fonts",
                "fonts.txt" => "font-list",
                _ => return None,
            };
            Some(json!({"format":format,"bytes":file["bytes"]}))
        })
        .collect::<Vec<_>>();
    Ok(json!({"taskId":task,"page":number,"files":files}))
}
