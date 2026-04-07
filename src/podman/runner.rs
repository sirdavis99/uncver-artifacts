use anyhow::Context;
use std::process::Command;

pub struct PodmanRunner;

impl PodmanRunner {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&self, image: &str) -> anyhow::Result<String> {
        tracing::info!("Running podman container: {}", image);

        let output = Command::new("podman")
            .args(&["run", "--rm", image])
            .output()
            .context("Failed to run podman container")?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Podman run failed: {}", stderr)
        }
    }

    pub fn build(&self, tag: &str, path: &str) -> anyhow::Result<()> {
        tracing::info!("Building podman image: {} from path: {}", tag, path);

        let status = Command::new("podman")
            .args(&["build", "-t", tag, path])
            .status()
            .context("Failed to build podman image")?;

        if !status.success() {
            anyhow::bail!("Podman build failed for image: {}", tag);
        }

        Ok(())
    }

    pub fn run_detached(&self, image: &str, name: Option<&str>) -> anyhow::Result<String> {
        tracing::info!("Running podman container detached: {}", image);

        let mut args = vec!["run", "-d"];

        if let Some(n) = name {
            args.push("--name");
            args.push(n);
        }

        args.push(image);

        let output = Command::new("podman")
            .args(&args)
            .output()
            .context("Failed to run podman container detached")?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Podman run detached failed: {}", stderr)
        }
    }

    pub fn list_containers(&self) -> anyhow::Result<Vec<ContainerInfo>> {
        let output = Command::new("podman")
            .args(&["ps", "--format", "json"])
            .output()
            .context("Failed to list podman containers")?;

        if !output.status.success() {
            return Ok(vec![]);
        }

        #[derive(serde::Deserialize)]
        struct PodmanPs {
            id: String,
            names: Vec<String>,
            image: String,
            state: String,
        }

        match serde_json::from_slice::<Vec<PodmanPs>>(&output.stdout) {
            Ok(containers) => Ok(containers
                .into_iter()
                .map(|c| ContainerInfo {
                    id: c.id,
                    name: c.names.first().cloned().unwrap_or_default(),
                    image: c.image,
                    state: c.state,
                })
                .collect()),
            Err(_) => Ok(vec![]),
        }
    }

    pub fn pull(&self, image: &str) -> anyhow::Result<()> {
        tracing::info!("Pulling podman image: {}", image);

        let status = Command::new("podman")
            .args(&["pull", image])
            .status()
            .context("Failed to pull podman image")?;

        if !status.success() {
            anyhow::bail!("Podman pull failed for image: {}", image);
        }

        Ok(())
    }
}

impl Default for PodmanRunner {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct ContainerInfo {
    pub id: String,
    pub name: String,
    pub image: String,
    pub state: String,
}
