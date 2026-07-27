use std::fs;
use std::process::Command;
use tracing::info;

pub fn sync_artifact_lib() -> anyhow::Result<std::path::PathBuf> {
    let data_dir = uncver_artifacts::paths::get_data_dir()?;
    let lib_dir = data_dir.join("artifact-lib");
    if lib_dir.join("artifact.json").exists() {
        info!("Updating artifact-lib...");
        let _ = Command::new("git")
            .args(["-C", lib_dir.to_str().unwrap(), "stash"])
            .output();
        let pull = Command::new("git")
            .args(["-C", lib_dir.to_str().unwrap(), "pull"])
            .output()
            .map_err(|e| anyhow::anyhow!("Failed to git pull artifact-lib: {}", e))?;
        if !pull.status.success() {
            anyhow::bail!("Failed to update artifact-lib: {}", String::from_utf8_lossy(&pull.stderr));
        }
    } else {
        info!("Cloning artifact-lib...");
        if let Some(parent) = lib_dir.parent() {
            fs::create_dir_all(parent)?;
        }
        let clone = Command::new("git")
            .args(["clone", "https://github.com/uncoverthefuture-org/uncver-artifact-lib.git",
                lib_dir.to_str().unwrap()])
            .output()
            .map_err(|e| anyhow::anyhow!("Failed to clone artifact-lib: {}", e))?;
        if !clone.status.success() {
            anyhow::bail!("Failed to clone artifact-lib: {}", String::from_utf8_lossy(&clone.stderr));
        }
    }
    Ok(lib_dir)
}
