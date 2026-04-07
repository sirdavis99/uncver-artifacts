use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    match detect_podman() {
        Some(version) => {
            println!("cargo:rustc-env=PODMAN_VERSION={}", version);
            println!("cargo:warning=Podman detected: {}", version);
        }
        None => {
            println!("cargo:warning=Podman not detected. It will be installed on first run.");
        }
    }
}

fn detect_podman() -> Option<String> {
    let output = Command::new("podman").arg("--version").output();

    match output {
        Ok(out) if out.status.success() => String::from_utf8(out.stdout)
            .ok()
            .map(|s| s.trim().to_string()),
        _ => None,
    }
}
