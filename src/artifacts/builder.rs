use std::path::PathBuf;

pub async fn build_from_config(path: PathBuf) {
    if let Ok(content) = std::fs::read_to_string(&path) {
        if let Ok(config) = serde_json::from_str::<crate::artifacts::ArtifactConfig>(&content) {
            if let (Some(local_path), Some(image)) = (config.local_path, config.container_image) {
                let runner = crate::podman::runner::PodmanRunner::new();
                let _ =
                    tokio::task::spawn_blocking(move || runner.build(&image, &local_path)).await;
            }
        }
    }
}
