pub mod container;
pub mod image;
pub mod path;
pub mod repo;

use std::sync::Arc;
use tracing::warn;

use uncver_artifacts::output;
use uncver_artifacts::ArtifactManager;

pub fn ensure_required_artifacts(
    lib_dir: &std::path::Path,
    artifacts: &Arc<ArtifactManager>,
    verbose: bool,
) -> anyhow::Result<()> {
    let root: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(lib_dir.join("artifact.json"))?
    )?;

    let required = root["required_artifacts"].as_array().cloned().unwrap_or_default();
    let all_artifacts = root["artifacts"].as_array().cloned().unwrap_or_default();

    for req in required {
        if !req["auto_start"].as_bool().unwrap_or(false) { continue; }
        let req_name = match req["name"].as_str() {
            Some(n) if !n.is_empty() => n,
            _ => continue,
        };

        let entry = all_artifacts.iter().find(|a| a["name"].as_str() == Some(req_name));
        let artifact_dir = lib_dir.join(entry.and_then(|a| a["path"].as_str()).unwrap_or(""));
        let json_path = artifact_dir.join("artifact.json");

        if !json_path.exists() {
            warn!("Artifact '{}' has no artifact.json", req_name);
            continue;
        }

        let art_val: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&json_path)?)?;
        let art_name = art_val["name"].as_str().unwrap_or(req_name);

        output::separator();
        output::action(format!("Setting up: {} ({})", art_name, artifact_dir.file_name().unwrap_or_default().to_string_lossy()));

        let image_tag = container::ensure_image(&art_val, &artifact_dir, art_name, verbose)?;
        container::run_and_register(artifacts, &art_val, art_name, &artifact_dir, &image_tag)?;
    }
    Ok(())
}
