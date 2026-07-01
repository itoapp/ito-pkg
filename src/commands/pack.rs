use crate::models::{CargoToml, PluginManifest};
use anyhow::{Context, Result, bail};
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::Command;
use zip::write::SimpleFileOptions;

pub fn pack_plugin(plugin_dir: PathBuf) -> Result<()> {
    let plugin_dir = plugin_dir
        .canonicalize()
        .context("Failed to canonicalize plugin directory path")?;

    let cargo_toml_path = plugin_dir.join("Cargo.toml");
    if !cargo_toml_path.exists() {
        bail!("Cargo.toml not found in directory {:?}", plugin_dir);
    }

    let cargo_toml_content =
        std::fs::read_to_string(&cargo_toml_path).context("Failed to read Cargo.toml")?;
    let cargo_toml: CargoToml =
        toml::from_str(&cargo_toml_content).context("Failed to parse Cargo.toml")?;

    let plugin_name = cargo_toml.package.name;
    tracing::info!("Found plugin: {}", plugin_name);

    tracing::info!("Building plugin...");
    let status = Command::new("cargo")
        .arg("build")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .arg("--release")
        .current_dir(&plugin_dir)
        .status()
        .context("Failed to run cargo build")?;

    if !status.success() {
        bail!("Cargo build failed with status {:?}", status);
    }

    let wasm_file = plugin_dir
        .join("target")
        .join("wasm32-unknown-unknown")
        .join("release")
        .join(format!("{}.wasm", plugin_name.replace("-", "_")));

    if !wasm_file.exists() {
        bail!("Compiled Wasm file not found at {:?}", wasm_file);
    }

    // Look for manifest.json
    let mut manifest_file = plugin_dir.join("manifest.json");
    if !manifest_file.exists() {
        // Fallback to old name
        manifest_file = plugin_dir.join(format!("{}.json", plugin_name));
        if !manifest_file.exists() {
            bail!("manifest.json not found in plugin root.");
        }
    }

    // Validate manifest
    let manifest_content =
        std::fs::read_to_string(&manifest_file).context("Failed to read manifest.json")?;
    let mut manifest = serde_json::from_str::<PluginManifest>(&manifest_content)
        .context("Invalid manifest.json format")?;

    if manifest.archived.unwrap_or(false) && manifest.archived_date.is_none() {
        manifest.archived_date = Some(chrono::Utc::now().format("%Y-%m-%d").to_string());
        let new_manifest_content = serde_json::to_string_pretty(&manifest)
            .context("Failed to serialize updated manifest")?;
        std::fs::write(&manifest_file, &new_manifest_content)
            .context("Failed to write updated manifest.json back")?;
    }

    let final_manifest_content =
        serde_json::to_string(&manifest).context("Failed to serialize manifest")?;

    let output_file = plugin_dir.join(format!("{}.ito", plugin_name));
    tracing::info!("Creating package: {:?}", output_file);

    let file = File::create(&output_file).context("Failed to create output file")?;
    let mut zip = zip::ZipWriter::new(file);
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    zip.start_file("main.wasm", options)
        .context("Failed to start main.wasm in zip")?;
    let mut wasm_content = Vec::new();
    File::open(&wasm_file)
        .context("Failed to open Wasm file")?
        .read_to_end(&mut wasm_content)
        .context("Failed to read wasm content")?;
    zip.write_all(&wasm_content)
        .context("Failed to write wasm content to zip")?;

    zip.start_file("manifest.json", options)
        .context("Failed to start manifest.json in zip")?;
    zip.write_all(final_manifest_content.as_bytes())
        .context("Failed to write manifest content to zip")?;

    let icon_file = plugin_dir.join("icon.png");
    if icon_file.exists() {
        zip.start_file("icon.png", options)
            .context("Failed to start icon.png in zip")?;
        let mut icon_content = Vec::new();
        File::open(&icon_file)
            .context("Failed to open icon.png")?
            .read_to_end(&mut icon_content)
            .context("Failed to read icon content")?;
        zip.write_all(&icon_content)
            .context("Failed to write icon content to zip")?;
    }

    zip.finish().context("Failed to finish zip file")?;

    tracing::info!(
        "Successfully packaged plugin into {}!",
        output_file.display()
    );
    Ok(())
}
