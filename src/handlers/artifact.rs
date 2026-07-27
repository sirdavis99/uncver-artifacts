use std::process::Command;
use std::sync::Arc;
use tracing::{error, info};

use uncver_artifacts::podman::{Podman, TraefikOrchestrator};
use uncver_artifacts::{open_gui_window, ArtifactConfig, ArtifactManager};

pub async fn list(artifacts: &Arc<ArtifactManager>) -> anyhow::Result<()> {
    let list = artifacts.list_artifacts().await?;
    if list.is_empty() {
        println!("No artifacts found.");
    } else {
        println!("Artifacts:");
        for artifact in list {
            println!("  - {}: {}", artifact.name, artifact.description.as_deref().unwrap_or("No description"));
        }
    }
    Ok(())
}

pub fn create(artifacts: &Arc<ArtifactManager>, name: &str, description: Option<&String>, url: Option<&String>, local_path: Option<&String>, container_image: Option<&String>) -> anyhow::Result<()> {
    let config = ArtifactConfig {
        name: name.to_string(),
        description: description.cloned(),
        url: url.cloned(),
        local_path: local_path.cloned(),
        container_image: container_image.cloned(),
        gui_window: None, ports: None, environment: None,
        run_type: None, run_command: None, working_dir: None,
    };
    let path = artifacts.create_artifact(&config)?;
    info!("Created artifact at: {:?}", path);
    Ok(())
}

pub fn delete(artifacts: &Arc<ArtifactManager>, name: &str) -> anyhow::Result<()> {
    artifacts.delete_artifact(name)?;
    info!("Deleted artifact: {}", name);
    Ok(())
}

pub fn watch() -> anyhow::Result<()> {
    info!("Watching artifacts directory for changes...");
    info!("Press Ctrl+C to stop");
    use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
    let (tx, rx) = std::sync::mpsc::channel();
    let path = uncver_artifacts::paths::get_artifacts_dir()?;
    let mut watcher = RecommendedWatcher::new(
        move |res: notify::Result<Event>| {
            if let Ok(event) = res {
                for path in event.paths {
                    if path.extension().is_some_and(|ext| ext == "json") {
                        let _ = tx.send(path);
                    }
                }
            }
        }, notify::Config::default(),
    )?;
    watcher.watch(&path, RecursiveMode::Recursive)?;
    loop {
        match rx.recv() {
            Ok(path) => info!("Artifact updated: {:?}", path),
            Err(e) => { error!("Watch error: {}", e); break; }
        }
    }
    Ok(())
}

pub async fn load(podman: &Podman, artifacts: &Arc<ArtifactManager>, path: &str) -> anyhow::Result<()> {
    let absolute_path = std::fs::canonicalize(path).map_err(|e| anyhow::anyhow!("Invalid path: {}", e))?;
    TraefikOrchestrator::ensure_traefik()?;
    let json_path = absolute_path.join("artifact.json");
    if !json_path.exists() { anyhow::bail!("No artifact.json found in {:?}", absolute_path); }
    let content = std::fs::read_to_string(&json_path)?;
    let mut config: ArtifactConfig = serde_json::from_str(&content).map_err(|e| anyhow::anyhow!("Invalid artifact.json: {}", e))?;
    if absolute_path.join("Dockerfile").exists() {
        podman.ensure_installed()?;
        podman.ensure_machine_running()?;
        podman.build(&config.name, &absolute_path.to_string_lossy())?;
        config.container_image = Some(config.name.clone());
    }
    config.url = Some(absolute_path.to_string_lossy().into_owned());
    artifacts.create_artifact(&config)?;
    info!("Artifact '{}' loaded and registered", config.name);
    match config.run_type.as_deref().unwrap_or("container") {
        "host" => {
            let cmd = config.run_command.as_deref().unwrap_or("");
            if cmd.is_empty() { error!("Host artifact '{}' has no run_command", config.name); }
            else {
                let child = Command::new("sh").args(["-c", cmd])
                    .current_dir(config.working_dir.as_deref().unwrap_or(""))
                    .envs(config.environment.as_ref().cloned().unwrap_or_default())
                    .spawn().map_err(|e| anyhow::anyhow!("Failed to start host process: {}", e))?;
                println!("Host artifact '{}' started with PID: {}", config.name, child.id());
            }
        }
        _ => {
            if let Some(ref image) = config.container_image {
                let _ = Command::new("podman").args(["rm", "-f", &config.name]).output();
                podman.ensure_installed()?;
                podman.ensure_machine_running()?;
                match podman.run_detached(image, Some(&config.name), config.ports.as_ref(), config.environment.as_ref()) {
                    Ok(output) => {
                        println!("{} started in background with ID:\n{}", config.name, output);
                        if let Err(e) = TraefikOrchestrator::register_artifact_route(&config.name, config.gui_window.as_ref().and_then(|g| g.port).unwrap_or(8080)) {
                            error!("Failed to register Traefik route: {}", e);
                        }
                        open_gui_window(&config);
                    }
                    Err(e) => error!("Failed to start {}: {}", config.name, e),
                }
            } else { error!("Artifact '{}' has no container image", config.name); }
        }
    }
    Ok(())
}
