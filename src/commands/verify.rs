use crate::models::PluginManifest;
use anyhow::{bail, Context, Result};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub fn verify_plugin(path: PathBuf) -> Result<()> {
    if !path.exists() {
        bail!("File not found: {:?}", path);
    }

    let file = File::open(&path).context("Failed to open file")?;
    let mut archive = zip::ZipArchive::new(file).context("Failed to read as ZIP archive")?;

    let mut has_manifest = false;
    let mut has_wasm = false;

    for i in 0..archive.len() {
        let file = archive.by_index(i).context("Failed to access file in archive")?;
        match file.name() {
            "manifest.json" => has_manifest = true,
            "main.wasm" => has_wasm = true,
            _ => {}
        }
    }

    if !has_manifest || !has_wasm {
        bail!("Verification failed: Missing manifest.json or main.wasm in the archive.");
    }

    // Read manifest
    let mut manifest_file = archive.by_name("manifest.json").context("Failed to open manifest.json inside archive")?;
    let mut manifest_str = String::new();
    manifest_file.read_to_string(&mut manifest_str).context("Failed to read manifest.json")?;

    let manifest: PluginManifest = serde_json::from_str(&manifest_str)
        .context("Verification failed: Invalid manifest JSON")?;

    println!("Plugin {} v{} verified successfully.", manifest.name, manifest.version);
    Ok(())
}
