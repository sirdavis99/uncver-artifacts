use std::path::Path;
use std::process::Command;
use std::sync::Arc;

use uncver_artifacts::output;
use uncver_artifacts::output::Spinner;
use uncver_artifacts::{ArtifactConfig, ArtifactManager};

use super::image;

pub fn ensure_image(
    art_val: &serde_json::Value,
    artifact_dir: &Path,
    art_name: &str,
    verbose: bool,
) -> anyhow::Result<String> {
    let tag = art_val["container_image"].as_str().unwrap_or("");
    let has_df = artifact_dir.join("Dockerfile").exists();

    if tag.is_empty() && !has_df {
        anyhow::bail!("Artifact '{}' has no container_image and no Dockerfile", art_name);
    }
    if tag.is_empty() {
        let local_tag = format!("local/{}:latest", art_name);
        let spin = Spinner::start("Building image...");
        image::build_image(&local_tag, artifact_dir, art_name, verbose)
            .map_err(|e| { spin.fail("Build failed"); e })?;
        spin.done("Image built");
        return Ok(local_tag);
    }

    let exists = Command::new("podman")
        .args(["image", "exists", tag])
        .output().map(|o| o.status.success()).unwrap_or(false);

    let needs = if exists {
        match (image::get_local_digest(tag), image::get_remote_digest(tag)) {
            (Some(l), Some(r)) if l == r => { output::dim(format!("  Image up to date (digest {})", &l[..19])); false }
            (Some(_), Some(_)) => { output::info("  Remote digest changed"); true }
            _ => false,
        }
    } else { true };

    if needs {
        let spin = Spinner::start("Pulling image...");
        if image::pull_image(tag, verbose)? {
            spin.done("Image pulled");
        } else {
            spin.fail("Pull failed");
            if !has_df {
                anyhow::bail!("Cannot obtain image for '{}' — pull failed and no Dockerfile", art_name);
            }
            output::info("  Falling back to local build...");
            let spin = Spinner::start("Building image...");
            image::build_image(tag, artifact_dir, art_name, verbose)
                .map_err(|e| { spin.fail("Build failed"); e })?;
            spin.done("Image built");
        }
    }
    Ok(tag.to_string())
}

pub fn run_and_register(
    artifacts: &Arc<ArtifactManager>,
    art_val: &serde_json::Value,
    art_name: &str,
    artifact_dir: &Path,
    image_tag: &str,
) -> anyhow::Result<()> {
    let _ = Command::new("podman").args(["rm", "-f", art_name]).output();

    let network = art_val["network"].as_str().unwrap_or("uncver-network");
    let env_map = art_val["environment"].as_object();
    let ports = art_val["ports"].as_array();
    let volumes = art_val["volumes"].as_array();

    let args = vec![
        "run", "-d", "--name", art_name,
        "--network", network,
    ];
    let mut arg_owned: Vec<String> = Vec::new();

    if let Some(env) = env_map {
        for (k, v) in env {
            let val = format!("{}={}", k, v.as_str().unwrap_or(""));
            arg_owned.push("-e".into());
            arg_owned.push(val);
        }
    }
    if let Some(pts) = ports {
        for p in pts {
            if let Some(s) = p.as_str() { if !s.is_empty() { arg_owned.push("-p".into()); arg_owned.push(s.into()); } }
        }
    }
    if let Some(vls) = volumes {
        for v in vls {
            if let Some(s) = v.as_str() { if !s.is_empty() { arg_owned.push("-v".into()); arg_owned.push(s.into()); } }
        }
    }
    arg_owned.push(image_tag.into());

    let all_args: Vec<&str> = args.into_iter().chain(arg_owned.iter().map(|s| s.as_str())).collect();

    let spin = Spinner::start("Starting container...");
    let res = Command::new("podman").args(&all_args).output()
        .map_err(|e| anyhow::anyhow!("Failed to run '{}': {}", art_name, e))?;
    if !res.status.success() {
        spin.fail("Failed to start container");
        anyhow::bail!("Failed to start '{}': {}", art_name, String::from_utf8_lossy(&res.stderr));
    }
    spin.done("Container started");

    artifacts.create_artifact(&ArtifactConfig {
        name: art_name.to_string(),
        description: art_val["description"].as_str().map(|s| s.to_string()),
        url: art_val["repository"].as_str().map(|s| s.to_string()),
        local_path: Some(artifact_dir.to_string_lossy().to_string()),
        container_image: Some(image_tag.to_string()),
        gui_window: None,
        ports: ports.map(|p| p.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect()),
        environment: env_map.map(|e| e.iter().map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_string())).collect()),
        run_type: None,
        run_command: None,
        working_dir: None,
    })?;
    Ok(())
}
