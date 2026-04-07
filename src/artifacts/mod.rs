pub mod watcher;

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::fs;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactConfig {
    pub name: String,
    pub description: Option<String>,
    pub url: Option<String>,
    pub local_path: Option<String>,
    pub container_image: Option<String>,
}

#[derive(Clone)]
pub struct ArtifactManager {
    base_dir: PathBuf,
}

impl ArtifactManager {
    pub fn new() -> Result<Self> {
        let mut path = dirs::data_dir().ok_or_else(|| anyhow::anyhow!("No data dir found"))?;
        path.push("uncver-artifacts");
        path.push("artifacts");
        
        if !path.exists() {
            fs::create_dir_all(&path)?;
        }
        
        Ok(Self { base_dir: path.to_path_buf() })
    }

    pub fn get_artifacts_dir(&self) -> &Path {
        &self.base_dir
    }

    pub fn create_artifact(&self, config: &ArtifactConfig) -> Result<PathBuf> {
        let folder_name = config.name.to_lowercase().replace(' ', "-");
        let mut artifact_path = self.base_dir.clone();
        artifact_path.push(&folder_name);
        
        if !artifact_path.exists() {
            fs::create_dir_all(&artifact_path)?;
        }
        
        let mut config_path = artifact_path.clone();
        config_path.push("artifact.json");
        
        let json = serde_json::to_string_pretty(config)?;
        fs::write(config_path, json)?;
        
        Ok(artifact_path)
    }

    pub async fn list_artifacts(&self) -> Result<Vec<ArtifactConfig>> {
        let mut artifacts = Vec::new();
        if self.base_dir.exists() {
            for entry in fs::read_dir(&self.base_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    let config_path = path.join("artifact.json");
                    if config_path.exists() {
                        let content = fs::read_to_string(config_path)?;
                        if let Ok(config) = serde_json::from_str::<ArtifactConfig>(&content) {
                            artifacts.push(config);
                        }
                    }
                }
            }
        }
        Ok(artifacts)
    }
}
