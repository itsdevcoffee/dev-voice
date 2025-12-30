use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DaemonStatus {
    pub running: bool,
    pub model_loaded: bool,
    pub gpu_enabled: bool,
    pub gpu_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TranscriptionEntry {
    pub id: String,
    pub text: String,
    pub timestamp: i64,
    pub duration_ms: u64,
    pub model: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfo {
    pub cpu_cores: usize,
    pub total_ram_gb: f32,
    pub gpu_available: bool,
    pub gpu_vram_mb: Option<u32>,
    pub platform: String,
}

/// Check if the hyprvoice daemon is running
#[tauri::command]
pub async fn get_daemon_status() -> Result<DaemonStatus, String> {
    // TODO: Connect to daemon via Unix socket at ~/.local/state/hyprvoice/daemon.sock
    // For now, return mock data
    Ok(DaemonStatus {
        running: true,
        model_loaded: true,
        gpu_enabled: true,
        gpu_name: Some("NVIDIA RTX 4090".to_string()),
    })
}

/// Start recording audio
#[tauri::command]
pub async fn start_recording() -> Result<(), String> {
    // TODO: Send StartRecording command to daemon
    println!("Starting recording...");
    Ok(())
}

/// Stop recording and transcribe
#[tauri::command]
pub async fn stop_recording() -> Result<String, String> {
    // TODO: Send StopRecording command to daemon and get transcription
    println!("Stopping recording...");
    Ok("This is a test transcription".to_string())
}

/// Get transcription history
#[tauri::command]
pub async fn get_transcription_history() -> Result<Vec<TranscriptionEntry>, String> {
    // TODO: Query transcription history from daemon or local DB
    Ok(vec![
        TranscriptionEntry {
            id: "1".to_string(),
            text: "This is a test transcription from earlier".to_string(),
            timestamp: 1704067200,
            duration_ms: 1500,
            model: "whisper-large-v3-turbo".to_string(),
        },
    ])
}

/// Download a Whisper model
#[tauri::command]
pub async fn download_model(model_name: String) -> Result<(), String> {
    // TODO: Call hyprvoice download command
    println!("Downloading model: {}", model_name);
    Ok(())
}

/// Get system information
#[tauri::command]
pub async fn get_system_info() -> Result<SystemInfo, String> {
    Ok(SystemInfo {
        cpu_cores: num_cpus::get(),
        total_ram_gb: 32.0, // TODO: Get actual RAM
        gpu_available: true,
        gpu_vram_mb: Some(24576), // TODO: Query actual VRAM
        platform: std::env::consts::OS.to_string(),
    })
}
