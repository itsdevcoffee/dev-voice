# dev-voice

Voice dictation for Linux developers. Capture speech, transcribe with Whisper, inject text at cursor.

## Requirements

### System Dependencies (Fedora)

```bash
sudo dnf install -y pipewire-devel clang cmake wtype
```

### Runtime Dependencies

- **PipeWire** (audio server) — pre-installed on modern Fedora/Ubuntu
- **wtype** (Wayland) or **xdotool** (X11) — for text injection

## Build

```bash
# Clone
git clone https://github.com/yourusername/dev-voice.git
cd dev-voice

# Build release
cargo build --release

# Binary at ./target/release/dev-voice
```

### GPU Acceleration (optional)

```bash
# NVIDIA CUDA
cargo build --release --features cuda

# AMD ROCm
cargo build --release --features rocm

# Vulkan (cross-platform)
cargo build --release --features vulkan
```

## Usage

```bash
# Download a whisper model
dev-voice download base.en

# Check system readiness
dev-voice doctor

# Start recording (5 seconds)
dev-voice start --duration 5

# View config
dev-voice config
```

## Hyprland Integration

Add to `~/.config/hypr/hyprland.conf`:

```ini
bind = SUPER, V, exec, dev-voice start --duration 5
```

## Configuration

Config file: `~/.config/dev-voice/config.toml`

```toml
[model]
path = "~/.local/share/dev-voice/models/ggml-base.en.bin"
language = "en"

[audio]
sample_rate = 16000
timeout_secs = 30

[output]
append_space = true
```

## License

MIT
