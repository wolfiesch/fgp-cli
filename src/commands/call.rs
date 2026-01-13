//! Call a method on a daemon.

use anyhow::{bail, Context, Result};
use colored::Colorize;

use super::service_socket_path;

pub fn run(method: &str, params: &str, service_override: Option<&str>) -> Result<()> {
    // Infer service from method name (e.g., "gmail.list" -> "gmail")
    let service = service_override.unwrap_or_else(|| {
        method.split('.').next().unwrap_or(method)
    });

    let socket_path = service_socket_path(service);

    if !socket_path.exists() {
        bail!(
            "Service '{}' is not running. Run 'fgp start {}' first.",
            service,
            service
        );
    }

    // Parse params as JSON
    let params_value: serde_json::Value = serde_json::from_str(params)
        .context("Invalid JSON in params. Use format: '{\"key\": \"value\"}'")?;

    // Connect and call
    let client = fgp_daemon::FgpClient::new(&socket_path)
        .context("Failed to connect to daemon")?;

    let start = std::time::Instant::now();
    let response = client.call(method, params_value)?;
    let elapsed = start.elapsed();

    // Print response
    if response.ok {
        if let Some(result) = response.result {
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
    } else {
        let error = response.error.unwrap_or_default();
        eprintln!(
            "{} Error ({}): {}",
            "✗".red().bold(),
            error.code,
            error.message
        );
        std::process::exit(1);
    }

    // Print timing in stderr so it doesn't interfere with JSON output
    eprintln!(
        "{}",
        format!(
            "({:.1}ms client, {:.1}ms server)",
            elapsed.as_secs_f64() * 1000.0,
            response.meta.server_ms
        )
        .dimmed()
    );

    Ok(())
}
