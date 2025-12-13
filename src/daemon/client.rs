use anyhow::{Context, Result};
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::process::Command;
use std::thread;
use std::time::Duration;
use tracing::{debug, info};

use super::protocol::{DaemonRequest, DaemonResponse};
use super::server::{get_socket_path, is_daemon_running};

/// Ensure daemon is running, spawn if needed
pub fn ensure_daemon_running(model_path: &str) -> Result<()> {
    if is_daemon_running() {
        debug!("Daemon already running");
        return Ok(());
    }

    info!("Daemon not running, spawning...");

    // Spawn daemon in background
    Command::new(std::env::current_exe()?)
        .arg("daemon")
        .arg("--model")
        .arg(model_path)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .context("Failed to spawn daemon")?;

    // Wait for daemon to be ready
    for i in 0..50 {
        thread::sleep(Duration::from_millis(100));
        if is_daemon_running() {
            info!("Daemon ready after {}ms", i * 100);
            return Ok(());
        }
    }

    anyhow::bail!("Daemon failed to start within 5 seconds")
}

/// Send request to daemon and get response
pub fn send_request(request: &DaemonRequest) -> Result<DaemonResponse> {
    let socket_path = get_socket_path()?;

    let mut stream = UnixStream::connect(&socket_path)
        .context("Failed to connect to daemon. Is it running?")?;

    // Send request
    let request_json = serde_json::to_string(request)?;
    info!("Sending to daemon: {}", request_json);
    stream.write_all(request_json.as_bytes())?;
    stream.write_all(b"\n")?;
    stream.flush()?;

    // Read response
    let mut reader = BufReader::new(stream);
    let mut line = String::new();
    reader.read_line(&mut line)?;

    let response: DaemonResponse = serde_json::from_str(&line.trim())
        .context("Failed to parse daemon response")?;

    Ok(response)
}

/// Start recording via daemon
pub fn daemon_start_recording(model_path: &str, max_duration: u32) -> Result<String> {
    ensure_daemon_running(model_path)?;

    let request = DaemonRequest::StartRecording { max_duration };
    let response = send_request(&request)?;

    match response {
        DaemonResponse::Success { text } => Ok(text),
        DaemonResponse::Error { message } => {
            anyhow::bail!("Recording failed: {}", message)
        }
        _ => anyhow::bail!("Unexpected response: {:?}", response),
    }
}

/// Stop recording via daemon
pub fn daemon_stop_recording() -> Result<()> {
    if !is_daemon_running() {
        anyhow::bail!("Daemon is not running");
    }

    let request = DaemonRequest::StopRecording;
    let response = send_request(&request)?;

    match response {
        DaemonResponse::Ok { .. } => Ok(()),
        DaemonResponse::Error { message } => {
            anyhow::bail!("Stop failed: {}", message)
        }
        _ => anyhow::bail!("Unexpected response: {:?}", response),
    }
}

/// Shutdown daemon
pub fn daemon_shutdown() -> Result<()> {
    if !is_daemon_running() {
        info!("Daemon is not running");
        return Ok(());
    }

    let request = DaemonRequest::Shutdown;
    send_request(&request)?;

    Ok(())
}
