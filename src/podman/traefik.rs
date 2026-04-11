use std::process::Command;

pub struct TraefikOrchestrator;

impl TraefikOrchestrator {
    pub fn ensure_network() -> anyhow::Result<()> {
        let output = Command::new("podman").args(["network", "ls", "--format", "{{.Name}}"]).output()?;
        let networks = String::from_utf8_lossy(&output.stdout);
        if !networks.lines().any(|n| n.trim() == "uncver-network") {
            tracing::info!("Creating podman network: uncver-network");
            let _ = Command::new("podman").args(["network", "create", "uncver-network"]).output();
        }
        Ok(())
    }

    pub fn ensure_traefik() -> anyhow::Result<()> {
        let output = Command::new("podman").args(["ps", "-a", "--format", "{{.Names}}"]).output()?;
        let containers = String::from_utf8_lossy(&output.stdout);
        if !containers.lines().any(|c| c.trim() == "uncver-traefik") {
            tracing::info!("Starting Traefik reverse proxy");
            let _ = Command::new("podman").args(["rm", "-f", "uncver-traefik"]).output();
            let _ = Command::new("podman").args([
                "run", "-d",
                "--name", "uncver-traefik",
                "--network", "uncver-network",
                "-p", "42080:80",  // Safe isolated entrypoint mapping!
                "-v", "/var/run/docker.sock:/var/run/docker.sock:ro",
                "docker.io/library/traefik:v3.0",
                "--api.insecure=true",
                "--providers.docker=true",
                "--providers.docker.exposedbydefault=false",
            ]).output()?;
        } else {
            let _ = Command::new("podman").args(["start", "uncver-traefik"]).output();
        }
        Ok(())
    }

    pub fn inject_labels_and_env(string_args: &mut Vec<String>, name: &str, ports: Option<&Vec<String>>) {
        let domain = format!("{}.localhost", name.replace("uncver-", ""));
        
        // 1. Setup traefik routing rules
        string_args.push("-l".to_string());
        string_args.push("traefik.enable=true".to_string());
        string_args.push("-l".to_string());
        string_args.push(format!("traefik.http.routers.{}.rule=Host(`{}`)", name, domain));

        // 2. Set proxy target port dynamically (Defaults to 8080 now)
        let mut target_port = "8080".to_string();
        if let Some(p_list) = ports {
            if let Some(p) = p_list.first() {
                if let Some(internal) = p.split(':').last() {
                    target_port = internal.to_string();
                }
            }
        }
        string_args.push("-l".to_string());
        string_args.push(format!("traefik.http.services.{}.loadbalancer.server.port={}", name, target_port));

        // 3. Inject dynamic assignment globally
        string_args.push("-e".to_string());
        string_args.push(format!("UNCVER_DOMAIN={}", domain));
    }
}
