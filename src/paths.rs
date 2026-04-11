use std::path::PathBuf;
use anyhow::Result;

pub fn get_data_dir() -> Result<PathBuf> {
    let mut path = dirs::data_dir().ok_or_else(|| anyhow::anyhow!("No data dir found"))?;
    path.push("uncver-artifacts");
    if !path.exists() {
        std::fs::create_dir_all(&path)?;
    }
    Ok(path)
}

pub fn get_traefik_config_dir() -> Result<PathBuf> {
    let mut path = get_data_dir()?;
    path.push("traefik");
    if !path.exists() {
        std::fs::create_dir_all(&path)?;
    }
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
