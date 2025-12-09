use anyhow::{Context, Result};
use std::process::Command;

#[derive(Debug, Clone, Copy)]
pub enum DisplayServer {
    Wayland,
    X11,
}

impl DisplayServer {
    /// Auto-detect the current display server
    pub fn detect() -> Self {
        if std::env::var("WAYLAND_DISPLAY").is_ok() {
            Self::Wayland
        } else {
            Self::X11
        }
    }
}

/// Inject text at the current cursor position
pub fn inject_text(text: &str, display: &DisplayServer) -> Result<()> {
    if text.is_empty() {
        return Ok(());
    }

    match display {
        DisplayServer::Wayland => inject_wayland(text),
        DisplayServer::X11 => inject_x11(text),
    }
}

fn inject_wayland(text: &str) -> Result<()> {
    let status = Command::new("wtype")
        .arg("--")
        .arg(text)
        .status()
        .context("Failed to execute wtype. Is it installed? (sudo dnf install wtype)")?;

    if !status.success() {
        anyhow::bail!("wtype exited with status: {}", status);
    }

    Ok(())
}

fn inject_x11(text: &str) -> Result<()> {
    let status = Command::new("xdotool")
        .args(["type", "--clearmodifiers", "--", text])
        .status()
        .context("Failed to execute xdotool. Is it installed? (sudo dnf install xdotool)")?;

    if !status.success() {
        anyhow::bail!("xdotool exited with status: {}", status);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_detection() {
        // This test just ensures the function doesn't panic
        let _display = DisplayServer::detect();
    }
}
