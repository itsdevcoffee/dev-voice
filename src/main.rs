use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing::info;

mod audio;
mod config;
mod output;
mod transcribe;

#[derive(Parser)]
#[command(name = "dev-voice")]
#[command(about = "Voice dictation for Linux developers")]
#[command(version)]
struct Cli {
    /// Enable verbose logging
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start listening for voice input
    Start {
        /// Override model path
        #[arg(short, long)]
        model: Option<String>,

        /// Recording duration in seconds (0 = until silence)
        #[arg(short, long, default_value = "0")]
        duration: u32,
    },

    /// Download a whisper model
    Download {
        /// Model size: tiny.en, base.en, small.en, medium.en, large
        #[arg(default_value = "base.en")]
        model: String,
    },

    /// Show or edit configuration
    Config {
        /// Print config file path
        #[arg(long)]
        path: bool,

        /// Reset to default configuration
        #[arg(long)]
        reset: bool,
    },

    /// Check system dependencies
    Doctor,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    let filter = if cli.verbose { "debug" } else { "info" };
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .init();

    match cli.command {
        Commands::Start { model, duration } => {
            cmd_start(model, duration)?;
        }
        Commands::Download { model } => {
            cmd_download(&model)?;
        }
        Commands::Config { path, reset } => {
            cmd_config(path, reset)?;
        }
        Commands::Doctor => {
            cmd_doctor()?;
        }
    }

    Ok(())
}

fn cmd_start(model_override: Option<String>, duration: u32) -> Result<()> {
    info!("Loading configuration...");
    let mut cfg = config::load()?;

    if let Some(model_path) = model_override {
        cfg.model.path = model_path.into();
    }

    info!("Model: {}", cfg.model.path.display());

    // Check model exists
    if !cfg.model.path.exists() {
        anyhow::bail!(
            "Model not found: {}\nRun: dev-voice download {}",
            cfg.model.path.display(),
            cfg.model.path.file_stem().unwrap_or_default().to_string_lossy()
        );
    }

    // Detect display server
    let display_server = output::DisplayServer::detect();
    info!("Display server: {:?}", display_server);

    // Initialize transcriber
    info!("Loading whisper model...");
    let transcriber = transcribe::Transcriber::new(&cfg.model.path)?;
    info!("Model loaded successfully");

    // Initialize audio capture
    info!("Initializing audio capture...");
    let audio_data = audio::capture(duration, cfg.audio.sample_rate)?;
    info!("Captured {} samples", audio_data.len());

    // Transcribe
    info!("Transcribing...");
    let text = transcriber.transcribe(&audio_data)?;

    if text.is_empty() {
        info!("No speech detected");
        return Ok(());
    }

    info!("Transcribed: {}", text);

    // Inject text
    output::inject_text(&text, &display_server)?;
    info!("Text injected");

    Ok(())
}

fn cmd_download(model: &str) -> Result<()> {
    let cfg = config::load()?;
    let models_dir = cfg.model.path.parent().unwrap_or(std::path::Path::new("."));

    // Ensure models directory exists
    std::fs::create_dir_all(models_dir)?;

    let model_name = if model.starts_with("ggml-") {
        format!("{}.bin", model)
    } else {
        format!("ggml-{}.bin", model)
    };

    let url = format!(
        "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/{}",
        model_name
    );

    let dest = models_dir.join(&model_name);

    if dest.exists() {
        info!("Model already exists: {}", dest.display());
        return Ok(());
    }

    info!("Downloading {} to {}", model_name, dest.display());
    info!("URL: {}", url);

    // Use curl for download with progress
    let status = std::process::Command::new("curl")
        .args(["-L", "-o"])
        .arg(&dest)
        .arg("--progress-bar")
        .arg(&url)
        .status()?;

    if !status.success() {
        anyhow::bail!("Download failed");
    }

    info!("Download complete: {}", dest.display());
    Ok(())
}

fn cmd_config(show_path: bool, reset: bool) -> Result<()> {
    if reset {
        let cfg = config::Config::default();
        config::save(&cfg)?;
        info!("Configuration reset to defaults");
        return Ok(());
    }

    if show_path {
        let path = config::config_path()?;
        println!("{}", path.display());
        return Ok(());
    }

    // Print current config
    let cfg = config::load()?;
    let toml = toml::to_string_pretty(&cfg)?;
    println!("{}", toml);

    Ok(())
}

fn cmd_doctor() -> Result<()> {
    println!("Checking system dependencies...\n");

    // Check wtype
    let wtype_ok = std::process::Command::new("which")
        .arg("wtype")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);
    println!(
        "[{}] wtype (Wayland text injection)",
        if wtype_ok { "OK" } else { "MISSING" }
    );

    // Check xdotool
    let xdotool_ok = std::process::Command::new("which")
        .arg("xdotool")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);
    println!(
        "[{}] xdotool (X11 text injection)",
        if xdotool_ok { "OK" } else { "MISSING" }
    );

    // Check display server
    let display = output::DisplayServer::detect();
    println!("\nDisplay server: {:?}", display);

    match display {
        output::DisplayServer::Wayland if !wtype_ok => {
            println!("\nWARNING: You're on Wayland but wtype is not installed.");
            println!("Install with: sudo dnf install wtype");
        }
        output::DisplayServer::X11 if !xdotool_ok => {
            println!("\nWARNING: You're on X11 but xdotool is not installed.");
            println!("Install with: sudo dnf install xdotool");
        }
        _ => {}
    }

    // Check model
    let cfg = config::load()?;
    let model_ok = cfg.model.path.exists();
    println!(
        "\n[{}] Whisper model: {}",
        if model_ok { "OK" } else { "MISSING" },
        cfg.model.path.display()
    );

    if !model_ok {
        println!("\nDownload a model with: dev-voice download base.en");
    }

    // Check PipeWire
    let pw_ok = std::process::Command::new("pw-cli")
        .arg("info")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);
    println!(
        "\n[{}] PipeWire",
        if pw_ok { "OK" } else { "MISSING" }
    );

    println!();
    Ok(())
}
