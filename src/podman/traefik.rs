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
        let output = Command::new("podman").args(["ps", "-a", "--format", "{{.Image}} {{.Ports}} {{.Names}}"]).output()?;
        let containers = String::from_utf8_lossy(&output.stdout);
        
        // Ensure config directory exists
        let config_dir = std::env::current_dir()?.join(".uncver").join("traefik");
        std::fs::create_dir_all(&config_dir)?;

        let needs_recreate = !containers.contains("uncver-traefik");

        if needs_recreate {
            tracing::info!("Initializing generic Traefik bridge on safe port 42080...");
            let _ = Command::new("podman").args(["rm", "-f", "uncver-traefik"]).output();
            
            let output = Command::new("podman").args([
                "run", "-d",
                "--name", "uncver-traefik",
                "--network", "uncver-network",
                "-p", "42080:80",
                "-v", &format!("{}:/etc/traefik/dynamic:ro,z", config_dir.display()),
                "docker.io/library/traefik:v3.0",
                "--api.insecure=true",
                "--log.level=DEBUG",
                "--providers.file.directory=/etc/traefik/dynamic",
                "--providers.file.watch=true",
                "--entrypoints.web.address=:80",
            ]).output()?;

            if !output.status.success() {
                let err = String::from_utf8_lossy(&output.stderr);
                tracing::error!("Failed to start Traefik: {}", err);
                return Err(anyhow::anyhow!("Traefik failed to start: {}", err));
            }
        } else {
            let _ = Command::new("podman").args(["start", "uncver-traefik"]).output();
        }
        Ok(())
    }

    pub fn register_artifact_route(name: &str, port: u16) -> anyhow::Result<()> {
        let config_dir = std::env::current_dir()?.join(".uncver").join("traefik");
        let safe_name = name.replace("uncver-", "");
        
        // Use HostRegexp to match the domain regardless of the port (e.g. :42080)
        let config = format!(r#"
http:
  routers:
    {0}-router:
      rule: "HostRegexp(`{0}.localhost(:[0-9]+)?`)"
      service: {0}-service
  services:
    {0}-service:
      loadBalancer:
        servers:
          - url: "http://{1}:{2}"
"#, safe_name, name, port);

        let file_path = config_dir.join(format!("{}.yml", safe_name));
        std::fs::write(file_path, config)?;
        tracing::info!("Route registered for artifact: {}.localhost -> {}:{}", safe_name, name, port);
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
