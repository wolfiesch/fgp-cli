//! Call a method on a daemon.

use anyhow::{bail, Context, Result};
use colored::Colorize;

use super::service_socket_path;

pub fn run(method: &str, params: &str, service_override: Option<&str>) -> Result<()> {
    // Resolve service/socket and normalize the method we send over the wire.
    //
    // Preferred:
    // - Fully-qualified method names: "gmail.search"
    //
    // Also supported:
    // - Built-in methods with explicit service: `fgp call methods --service gmail`
    // - Action-only with explicit service: `fgp call search --service gmail`
    let (service, wire_method) = if let Some(service) = service_override {
        if method.contains('.') {
            // If the user provided --service, ensure it matches the namespace.
            let namespace = method.split('.').next().unwrap_or("");
            if namespace != service {
                bail!(
                    "Method namespace '{}' does not match --service '{}'",
                    namespace,
                    service
                );
            }
            (service, method.to_string())
        } else {
            // Built-ins are un-namespaced; service methods get namespaced here.
            let wire_method = match method {
                "health" | "methods" | "stop" | "bundle" => method.to_string(),
                _ => format!("{}.{}", service, method),
            };
            (service, wire_method)
        }
    } else {
        // Infer service from method name (e.g., "gmail.search" -> "gmail").
        // If method is not namespaced, we keep the legacy behavior of treating it as both
        // service and method (e.g., "echo" for the echo service).
        let service = method.split('.').next().unwrap_or(method);
        (service, method.to_string())
    };

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
    let response = client.call(&wire_method, params_value)?;
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
