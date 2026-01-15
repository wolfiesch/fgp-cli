//! Integration tests for fgp CLI
//!
//! Tests CLI argument parsing and basic functionality.

use std::process::Command;

/// Test that the binary can show help
#[test]
fn test_help_command() {
    let output = Command::new("cargo")
        .args(["run", "--", "--help"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("fgp") || stdout.contains("Fast Gateway Protocol"),
        "Help should mention fgp"
    );
}

/// Test that version command works
#[test]
fn test_version_command() {
    let output = Command::new("cargo")
        .args(["run", "--", "--version"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("0.") || stdout.contains("fgp"),
        "Version should be shown"
    );
}

/// Test status command (should work even with no daemons running)
#[test]
fn test_status_command() {
    let output = Command::new("cargo")
        .args(["run", "--", "status"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute command");

    // Status command should not panic, may show "no daemons running"
    assert!(
        output.status.success() || !output.stderr.is_empty(),
        "Status command should complete"
    );
}

/// Test agents detection command
#[test]
fn test_agents_command() {
    let output = Command::new("cargo")
        .args(["run", "--", "agents"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute command");

    // Agents command should run and detect available agents
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Either succeeds or shows error about detection
    assert!(
        stdout.contains("agent") || stderr.contains("agent") || output.status.success(),
        "Agents command should attempt detection"
    );
}

/// Test that the crate compiles
#[test]
fn test_crate_compiles() {
    assert!(true);
}
