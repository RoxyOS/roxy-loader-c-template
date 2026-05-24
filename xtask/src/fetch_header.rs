use std::path::PathBuf;

use anyhow::{Context, Result};

use crate::utils::downloaded_header_path;

const ROXY_LOADER_REF: &str = "v0.2.3";
const HEADER_PATH: &str = "c_api/roxy_loader.h";

pub fn fetch_header() -> Result<PathBuf> {
    let output = downloaded_header_path()?;
    let parent = output
        .parent()
        .ok_or_else(|| anyhow::anyhow!("header output path has no parent"))?;

    std::fs::create_dir_all(parent)?;

    let header_url =
        format!("https://raw.githubusercontent.com/RoxyOS/roxy-loader/{ROXY_LOADER_REF}/{HEADER_PATH}");

    let response = ureq::get(&header_url)
        .call()
        .with_context(|| format!("failed to download header from {header_url}"))?;
    let mut body = response.into_body();
    let header = body
        .read_to_string()
        .context("failed to read downloaded roxy_loader.h")?;

    std::fs::write(&output, header)
        .with_context(|| format!("failed to write header to {}", output.display()))?;

    Ok(output)
}
