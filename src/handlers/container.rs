use std::process::Command;
use std::sync::Arc;
use tracing::{error, info};

use uncver_artifacts::podman::{Podman, TraefikOrchestrator};
use uncver_artifacts::{open_gui_window, ArtifactManager};

pub async fn start(podman: &Podman, artifacts: &Arc<ArtifactManager>, name: &str) -> anyhow::Result<()> {
    info!("Starting artifact: {}", name);
    let list = artifacts.list_artifacts().await?;
    if let Some(artifact) = list.iter().find(|a| a.name == name) {
        match artifact.run_type.as_deref().unwrap_or("container") {
            "host" => {
                let cmd = artifact.run_command.as_deref().unwrap_or("");
                if cmd.is_empty() {
                    error!("Host artifact '{}' has no run_command", name);
                } else {
                    info!("Starting host process: {}", cmd);
                    let child = Command::new("sh").args(["-c", cmd])
                        .current_dir(artifact.working_dir.as_deref().unwrap_or(""))
                        .envs(artifact.environment.as_ref().cloned().unwrap_or_default())
                        .spawn().map_err(|e| anyhow::anyhow!("Failed to start host process: {}", e))?;
                    println!("Host artifact '{}' started with PID: {}", name, child.id());
                }
            }
            _ => {
                if let Some(ref image) = artifact.container_image {
                    podman.ensure_installed()?;
                    podman.ensure_machine_running()?;
                    match podman.run_detached(image, Some(name), artifact.ports.as_ref(), artifact.environment.as_ref()) {
                        Ok(output) => {
                            println!("Artifact started in background with ID:\n{}", output);
                            let port = artifact.gui_window.as_ref().and_then(|g| g.port).unwrap_or(8080);
                            let _ = TraefikOrchestrator::register_artifact_route(&artifact.name, port);
                            open_gui_window(artifact);
                        }
                        Err(e) => error!("Failed to start artifact: {}", e),
                    }
                } else {
                    error!("Artifact '{}' has no container image and no run_type", name);
                }
            }
        }
    } else {
        error!("Artifact '{}' not found", name);
    }
    Ok(())
}

pub async fn run(podman: &Podman, artifacts: &Arc<ArtifactManager>) -> anyhow::Result<()> {
    info!("Running default artifacts...");
    podman.ensure_installed()?;
    podman.ensure_machine_running()?;
    for artifact in artifacts.list_artifacts().await? {
        if let Some(ref image) = artifact.container_image {
            info!("Starting artifact: {}", artifact.name);
            match podman.run_detached(image, Some(&artifact.name), artifact.ports.as_ref(), artifact.environment.as_ref()) {
                Ok(output) => { println!("{} started in background with ID:\n{}", artifact.name, output); open_gui_window(&artifact); }
                Err(e) => error!("Failed to start {}: {}", artifact.name, e),
            }
        }
    }
    Ok(())
}

pub fn ps(podman: &Podman) -> anyhow::Result<()> {
    let containers = podman.list_containers()?;
    println!("{:<30} {:<30} {:<20} {:<20}", "ID", "NAME", "IMAGE", "STATE");
    for c in containers { println!("{:<30} {:<30} {:<20} {:<20}", c.id, c.name, c.image, c.state); }
    Ok(())
}

pub fn logs(podman: &Podman, name: &str) -> anyhow::Result<()> {
    println!("{}", podman.get_logs(name)?);
    Ok(())
}

pub fn reset() -> anyhow::Result<()> {
    info!("Resetting uncver environment...");
    let output = Command::new("podman").args(["ps", "-a", "--format", "{{.Names}}"]).output()?;
    for name in String::from_utf8_lossy(&output.stdout).lines() {
        if name.contains("uncver-") || name.contains("redis-stream-push-gui") {
            let _ = Command::new("podman").args(["rm", "-f", name]).output();
        }
    }
    if let Ok(config_dir) = uncver_artifacts::paths::get_traefik_config_dir() {
        let _ = std::fs::remove_dir_all(&config_dir);
        let _ = std::fs::create_dir_all(&config_dir);
    }
    TraefikOrchestrator::ensure_network()?;
    TraefikOrchestrator::ensure_traefik()?;
    let _ = Command::new("podman").args(["run", "-d", "--name", "uncver-redis-stream", "--network", "uncver-network", "docker.io/library/redis:7-alpine"]).output()?;
    info!("Reset complete!");
    Ok(())
}
