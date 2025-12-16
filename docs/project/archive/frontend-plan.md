# dev-voice Frontend Implementation Plan

**Created:** 2025-12-09
**Status:** Planning

---

## Context

dev-voice is a Rust CLI voice dictation tool that captures speech, transcribes it locally using Whisper, and injects text at the cursor. The CLI is feature-complete but lacks a visual frontend.

### Current CLI Capabilities

```bash
dev-voice start           # Toggle mode: start recording
dev-voice start           # Toggle mode: stop and transcribe
dev-voice stop            # Explicit stop command
dev-voice start -d 5      # Fixed duration (5 seconds)
dev-voice start -c        # Output to clipboard instead of typing
dev-voice download base.en # Download whisper model
dev-voice doctor          # Check system dependencies
dev-voice config          # View/edit configuration
```

### Key Files

| File | Purpose |
|------|---------|
| `src/main.rs` | CLI entry, commands |
| `src/state/toggle.rs` | PID file at `~/.local/state/dev-voice/recording.pid` |
| `src/state/paths.rs` | XDG directory helpers |
| `~/.local/state/dev-voice/logs/` | Log files |

### How Toggle Mode Works

1. First `dev-voice start` creates PID file, starts recording
2. Second `dev-voice start` sends SIGUSR1 to stop, transcribes, outputs text
3. PID file: `~/.local/state/dev-voice/recording.pid` (contains PID and start timestamp)
4. 5 minute timeout if never stopped

---

## Target System

| Component | Details |
|-----------|---------|
| OS | Fedora 42 |
| Compositor | Hyprland 0.51.1 |
| Status Bar | Waybar |
| Widgets | AGS (Aylur's GTK Shell) |
| Launcher | Rofi |
| Notifications | Swaync |
| Dotfiles | JaKooLit's Hyprland-Dots |

---

## Implementation Plan

### Phase 1: Waybar Custom Module

**Goal:** Show recording status in Waybar, click to toggle

#### 1.1 Create Status Script

Create `~/.config/waybar/scripts/dev-voice-status.sh`:

```bash
#!/bin/bash
# Outputs JSON for Waybar custom module

PID_FILE="$HOME/.local/state/dev-voice/recording.pid"

if [[ -f "$PID_FILE" ]]; then
    PID=$(head -1 "$PID_FILE")
    if kill -0 "$PID" 2>/dev/null; then
        # Recording in progress
        echo '{"text": "󰍬", "tooltip": "Recording... Click to stop", "class": "recording"}'
    else
        # Stale PID file
        rm -f "$PID_FILE"
        echo '{"text": "󰍮", "tooltip": "Click to start recording", "class": "idle"}'
    fi
else
    echo '{"text": "󰍮", "tooltip": "Click to start recording", "class": "idle"}'
fi
```

#### 1.2 Waybar Module Config

Add to `~/.config/waybar/config.jsonc`:

```jsonc
"custom/dev-voice": {
    "format": "{}",
    "return-type": "json",
    "exec": "~/.config/waybar/scripts/dev-voice-status.sh",
    "on-click": "dev-voice start",
    "on-click-right": "dev-voice stop",
    "interval": 1,
    "tooltip": true
}
```

Add `"custom/dev-voice"` to your modules list.

#### 1.3 Waybar Styling

Add to `~/.config/waybar/style.css`:

```css
#custom-dev-voice {
    padding: 0 10px;
    margin: 0 4px;
}

#custom-dev-voice.recording {
    color: #f38ba8;  /* Red/pink when recording */
    animation: pulse 1s ease-in-out infinite;
}

#custom-dev-voice.idle {
    color: #a6adc8;  /* Muted when idle */
}

@keyframes pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.5; }
}
```

### Phase 2: Swaync Integration (Optional)

**Goal:** Show notification when transcription completes

#### 2.1 Modify dev-voice to Send Notifications

Update `src/main.rs` to call `notify-send` after transcription:

```rust
// After successful transcription
std::process::Command::new("notify-send")
    .args([
        "-a", "dev-voice",
        "-i", "audio-input-microphone",
        "Transcription Complete",
        &format!("\"{}\"", text.chars().take(100).collect::<String>())
    ])
    .spawn()
    .ok();
```

### Phase 3: AGS Widget (Optional Enhancement)

**Goal:** Floating control panel with more features

#### 3.1 AGS Widget Features

- Recording timer (elapsed time)
- Waveform visualization (if feasible)
- Model selector dropdown
- Recent transcriptions list
- Settings quick access

#### 3.2 AGS Widget Location

Create `~/.config/ags/widgets/dev-voice/`:
- `main.js` - Widget definition
- `style.css` - Styling

#### 3.3 AGS Widget Skeleton

```javascript
// ~/.config/ags/widgets/dev-voice/main.js
const { Widget, Utils } = ags;

const PID_FILE = `${Utils.HOME}/.local/state/dev-voice/recording.pid`;

const isRecording = Variable(false, {
    poll: [1000, () => Utils.exec('test -f ' + PID_FILE + ' && echo true || echo false').trim() === 'true']
});

export const DevVoiceWidget = () => Widget.Box({
    className: 'dev-voice-widget',
    vertical: true,
    children: [
        Widget.Button({
            className: isRecording.bind().transform(r => r ? 'recording' : 'idle'),
            onClicked: () => Utils.exec('dev-voice start'),
            child: Widget.Label({
                label: isRecording.bind().transform(r => r ? '⏹ Stop Recording' : '🎤 Start Recording')
            })
        }),
        // Add more controls as needed
    ]
});
```

### Phase 4: Rofi Integration (Optional)

**Goal:** Quick access via Rofi menu

Create `~/.config/rofi/scripts/dev-voice.sh`:

```bash
#!/bin/bash
# Rofi script for dev-voice control

OPTIONS="🎤 Start/Stop Recording\n📋 Clipboard Mode\n⚙️ Settings\n📊 Doctor"

CHOICE=$(echo -e "$OPTIONS" | rofi -dmenu -p "dev-voice")

case "$CHOICE" in
    "🎤 Start/Stop Recording") dev-voice start ;;
    "📋 Clipboard Mode") dev-voice start -c ;;
    "⚙️ Settings") $TERMINAL -e "dev-voice config" ;;
    "📊 Doctor") $TERMINAL -e "dev-voice doctor" ;;
esac
```

---

## File Structure After Implementation

```
~/.config/
├── waybar/
│   ├── config.jsonc          # Add custom/dev-voice module
│   ├── style.css             # Add dev-voice styling
│   └── scripts/
│       └── dev-voice-status.sh
├── ags/
│   └── widgets/
│       └── dev-voice/        # Optional AGS widget
│           ├── main.js
│           └── style.css
├── rofi/
│   └── scripts/
│       └── dev-voice.sh      # Optional Rofi integration
└── hypr/
    └── keybindings.conf      # Optional: bind Super+V to dev-voice start
```

---

## Keybinding Suggestion

Add to Hyprland config (`~/.config/hypr/keybindings.conf` or similar):

```conf
# Voice dictation toggle
bind = SUPER, V, exec, dev-voice start
bind = SUPER SHIFT, V, exec, dev-voice start -c  # Clipboard mode
```

---

## Implementation Priority

1. **Waybar module** - Essential, do first
2. **Hyprland keybinding** - Quick win, do alongside Waybar
3. **Swaync notification** - Small code change, nice UX
4. **Rofi script** - Optional, easy to add
5. **AGS widget** - Optional, for power users

---

## Testing Checklist

- [ ] Waybar shows correct icon when idle
- [ ] Waybar shows recording icon when `dev-voice start` is running
- [ ] Click on Waybar module toggles recording
- [ ] Right-click stops recording
- [ ] Icon pulses/animates during recording
- [ ] Tooltip shows correct status
- [ ] Keybinding works (Super+V)
- [ ] Notification appears after transcription (if implemented)

---

## Notes for Implementation

1. **Waybar refresh:** The module uses `interval: 1` for 1-second polling. This is simple but you could use `exec-on-event` with inotify for instant updates.

2. **Icons:** Using Nerd Font icons (󰍬 󰍮). Ensure your Waybar font supports them.

3. **JaKooLit dots:** Check existing Waybar config structure - may need to add module to correct file if config is split.

4. **Binary location:** Assumes `dev-voice` is in PATH (symlinked to `~/.local/bin/dev-voice` pointing to `~/dev-coffee/repos/dev-voice/target/release/dev-voice`).
