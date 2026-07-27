use std::path::Path;
use std::process::Command;

use uncver_artifacts::output;

pub fn get_local_digest(image: &str) -> Option<String> {
    let output = Command::new("podman")
        .args(["image", "inspect", image, "--format", "{{.Digest}}"])
        .output()
        .ok()?;
    if !output.status.success() { return None; }
    let digest = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if digest.is_empty() { None } else { Some(digest) }
}

pub fn get_remote_digest(image: &str) -> Option<String> {
    let (user, repo, tag) = parse_image(image);
    let url = format!(
        "https://hub.docker.com/v2/repositories/{}/{}/tags/{}/",
        user, repo, tag
    );
    let resp = reqwest::blocking::get(&url).ok()?;
    if !resp.status().is_success() { return None; }
    let json: serde_json::Value = resp.json().ok()?;
    json["images"][0]["digest"].as_str().map(|s| s.to_string())
}

pub fn parse_image(image: &str) -> (&str, &str, &str) {
    let no_registry = image.trim_start_matches("docker.io/");
    let parts: Vec<&str> = no_registry.splitn(2, '/').collect();
    match parts.len() {
        1 => {
            let (repo, tag) = parts[0].split_once(':').unwrap_or((parts[0], "latest"));
            ("library", repo, tag)
        }
        2 => {
            let (user, rest) = (parts[0], parts[1]);
            let (repo, tag) = rest.split_once(':').unwrap_or((rest, "latest"));
            (user, repo, tag)
        }
        _ => ("library", image, "latest"),
    }
}

pub fn pull_image(tag: &str, verbose: bool) -> anyhow::Result<bool> {
    let out = Command::new("podman").args(["pull", tag]).output()?;
    if out.status.success() {
        if verbose {
            let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if !s.is_empty() { println!("{}", s); }
        }
        Ok(true)
    } else {
        let err = String::from_utf8_lossy(&out.stderr).trim().to_string();
        output::error(format!("Pull failed: {}", err));
        Ok(false)
    }
}

pub fn build_image(tag: &str, dir: &Path, name: &str, verbose: bool) -> anyhow::Result<()> {
    let out = Command::new("podman")
        .args(["build", "-t", tag, "."])
        .current_dir(dir)
        .output()?;
    let out_str = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if !out.status.success() {
        println!("{}", out_str);
        let err = String::from_utf8_lossy(&out.stderr).trim().to_string();
        if !err.is_empty() { eprintln!("{}", err); }
        return Err(anyhow::anyhow!("Build failed for '{}'", name));
    }
    if verbose && !out_str.is_empty() {
        println!("{}", out_str);
    }
    Ok(())
}
