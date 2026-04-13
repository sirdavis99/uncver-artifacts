use anyhow::Result;
use std::path::PathBuf;
use std::process::Command;

pub fn get_data_dir() -> Result<PathBuf> {
    let path = PathBuf::from("/tmp/uncver-artifacts");
    if !path.exists() {
        std::fs::create_dir_all(&path)?;
    }
    // Ensure world-readable for container (recursive)
    let _ = Command::new("chmod")
        .args(["-R", "777", "/tmp/uncver-artifacts"])
        .output();
    Ok(path)
}

pub fn get_traefik_config_dir() -> Result<PathBuf> {
    let mut path = get_data_dir()?;
    path.push("traefik");
    if !path.exists() {
        std::fs::create_dir_all(&path)?;
    }
    let _ = Command::new("chmod")
        .args(["-R", "777", "/tmp/uncver-artifacts/traefik"])
        .output();
    Ok(path)
}

pub fn get_artifacts_dir() -> Result<PathBuf> {
    let mut path = get_data_dir()?;
    path.push("artifacts");
    if !path.exists() {
        std::fs::create_dir_all(&path)?;
    }
    Ok(path)
}
