//! Install a package from local path.

use anyhow::{bail, Context, Result};
use colored::Colorize;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use super::fgp_services_dir;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Manifest {
    name: String,
    version: String,
    #[serde(default)]
    protocol: String,
    daemon: DaemonConfig,
    #[serde(default)]
    skills: HashMap<String, SkillConfig>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct DaemonConfig {
    entrypoint: String,
    #[serde(default)]
    socket: String,
}

#[derive(Debug, Deserialize)]
struct SkillConfig {
    source: String,
    target: String,
}

/// Known AI agent skill directories.
const AGENT_SKILL_DIRS: &[(&str, &str)] = &[
    ("claude-code", "~/.claude/skills"),
    ("cursor", "~/.cursor/rules"),
    ("windsurf", "~/.windsurf/workflows"),
    ("continue", "~/.continue/rules"),
];

pub fn run(path: &str) -> Result<()> {
    let package_path = Path::new(path);

    // Support both directory and manifest.json path
    let (package_dir, manifest_path) = if package_path.is_dir() {
        (
            package_path.to_path_buf(),
            package_path.join("manifest.json"),
        )
    } else if package_path.file_name().map(|f| f == "manifest.json").unwrap_or(false) {
        (
            package_path.parent().unwrap_or(Path::new(".")).to_path_buf(),
            package_path.to_path_buf(),
        )
    } else {
        bail!("Expected a directory or manifest.json path");
    };

    if !manifest_path.exists() {
        bail!("manifest.json not found at {}", manifest_path.display());
    }

    // Parse manifest
    let manifest_content = fs::read_to_string(&manifest_path)
        .context("Failed to read manifest.json")?;
    let manifest: Manifest = serde_json::from_str(&manifest_content)
        .context("Failed to parse manifest.json")?;

    println!(
        "{} Installing {} v{}...",
        "→".blue().bold(),
        manifest.name.bold(),
        manifest.version
    );

    // Create service directory
    let service_dir = fgp_services_dir().join(&manifest.name);
    fs::create_dir_all(&service_dir)
        .context("Failed to create service directory")?;

    // Copy daemon files
    println!("  {} Copying daemon files...", "→".dimmed());
    copy_dir_contents(&package_dir, &service_dir)
        .context("Failed to copy daemon files")?;

    // Install skill files for detected agents
    let mut installed_skills = 0;
    for (agent_id, target_base) in AGENT_SKILL_DIRS {
        let target_expanded = shellexpand::tilde(target_base);
        let target_base_path = Path::new(target_expanded.as_ref());

        // Check if agent is installed
        if !target_base_path.exists() {
            continue;
        }

        // Check if we have skills for this agent
        if let Some(skill_config) = manifest.skills.get(*agent_id) {
            let source_path = package_dir.join(&skill_config.source);
            if !source_path.exists() {
                println!(
                    "  {} Skill source not found for {}: {}",
                    "!".yellow(),
                    agent_id,
                    source_path.display()
                );
                continue;
            }

            // Expand target path
            let target_expanded = shellexpand::tilde(&skill_config.target);
            let target_path = Path::new(target_expanded.as_ref());

            // Create target directory
            if let Some(parent) = target_path.parent() {
                fs::create_dir_all(parent)?;
            }

            // Copy skill files
            copy_dir_contents(&source_path, target_path)
                .with_context(|| format!("Failed to install {} skill", agent_id))?;

            println!(
                "  {} {} skill installed",
                "✓".green(),
                agent_id
            );
            installed_skills += 1;
        }
    }

    // Summary
    println!();
    println!(
        "{} {} v{} installed successfully!",
        "✓".green().bold(),
        manifest.name.bold(),
        manifest.version
    );

    if installed_skills > 0 {
        println!(
            "  {} skill files installed for {} agent(s)",
            installed_skills,
            installed_skills
        );
    }

    println!();
    println!(
        "  Start the daemon: {}",
        format!("fgp start {}", manifest.name).cyan()
    );

    Ok(())
}

/// Copy directory contents recursively.
fn copy_dir_contents(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            copy_dir_contents(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}
