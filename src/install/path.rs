use anyhow::Context;
use std::fs;
use tracing::info;

use uncver_artifacts::output;

pub fn register_binary_in_path() -> anyhow::Result<()> {
    let bin_dir = dirs::home_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?
        .join(".local/bin");
    fs::create_dir_all(&bin_dir)?;
    let target = bin_dir.join("uncver-artifacts");
    let exe = std::env::current_exe()?;
    if target.exists() { fs::remove_file(&target)?; }
    std::os::unix::fs::symlink(&exe, &target)
        .or_else(|_| fs::copy(&exe, &target).map(|_| ()))
        .context("Failed to install binary to ~/.local/bin")?;
    info!("Binary linked to {:?}", target);

    let path = std::env::var("PATH").unwrap_or_default();
    let bin_str = bin_dir.to_string_lossy();
    if !path.split(':').any(|p| p == bin_str.as_ref()) {
        output::info(format!("Warning: {} is not in your PATH.", bin_str));
        output::info("Add this to your shell config:".to_string());
        output::info("  export PATH=\"$HOME/.local/bin:$PATH\"".to_string());
    }
    Ok(())
}
