# Phase 4 Days 4-5: enigo/arboard Migration - Implementation Handoff

**Date:** 2025-12-16
**Branch:** `phase4-cpal-migration`
**Status:** Day 1 complete, ready for Days 4-5

---

## Context: What's Been Done

### ✅ Day 1: CPAL Migration (COMPLETE)
- Replaced PipeWire audio capture with CPAL (cross-platform)
- Removed `pipewire = "0.9"` and `libspa = "0.9"` dependencies
- Added `cpal = "0.16"` dependency
- Upgraded `rubato = "0.15"` → `"0.16"`
- Reduced audio code: 515 lines → 280 lines (-46%)
- All 31 tests passing
- Real audio capture verified on Fedora 42 + Hyprland
- Committed to branch: `phase4-cpal-migration` (commit 3fd0f04)

### ⏭️ Days 2-3: Skipped
- Day 2 tested on Linux (working)
- Day 3 macOS testing deferred (no Mac available)

---

## Your Task: Days 4-5

Implement cross-platform text injection using **enigo v0.6** and **arboard v3.6** to replace the current wtype/xdotool implementation.

### Day 4: Replace wtype/xdotool with enigo (6 hours estimated)

**Current state:**
- `src/output/inject.rs` uses external commands (`wtype`, `xdotool`, `wl-clipboard`)
- Works only on Linux Wayland/X11
- Requires external tools to be installed

**Target state:**
- Pure Rust implementation using enigo + arboard
- Cross-platform (Linux, macOS, Windows)
- No external dependencies

**Implementation Plan:**

#### Step 1: Read Current Implementation
```bash
# Understand what needs to be replaced
Read src/output/inject.rs
Read src/output/mod.rs
```

**Key functions to replace:**
- `inject_text(text: &str, mode: OutputMode)` - Main entry point
- Display server detection (Wayland vs X11)
- Clipboard save/restore
- Terminal detection for paste shortcuts
- Text typing via wtype/xdotool

#### Step 2: Update Cargo.toml

Add platform-specific dependencies:

```toml
# Linux: enigo with Wayland + X11 support
[target.'cfg(target_os = "linux")'.dependencies]
enigo = { version = "0.6", features = ["wayland", "x11rb"] }
arboard = { version = "3.6", features = ["wayland-data-control"] }

# macOS: no special features needed
[target.'cfg(target_os = "macos")'.dependencies]
enigo = "0.6"
arboard = "3.6"

# Windows: no special features needed
[target.'cfg(target_os = "windows")'.dependencies]
enigo = "0.6"
arboard = "3.6"
```

#### Step 3: Implement New output/mod.rs

Reference implementation from `docs/project/phase4-implementation-plan.md` lines 477-687.

**Critical API differences from plan:**
- Plan shows enigo 0.2 API, but we're using **0.6.1**
- Clipboard operations use `arboard::Clipboard`, not enigo
- Feature flags required for Linux

**Key implementation details:**

```rust
use anyhow::{Context, Result};
use enigo::{Direction, Enigo, Key, Keyboard, Settings};

// Clipboard via arboard (enigo v0.6+ removed clipboard support)
use arboard::Clipboard;

fn paste_at_cursor(text: &str) -> Result<()> {
    let mut enigo = Enigo::new(&Settings::default())?;

    // Save clipboard
    let mut clipboard = Clipboard::new()?;
    let saved = clipboard.get_text()?;

    // Copy text
    clipboard.set_text(text)?;
    std::thread::sleep(Duration::from_millis(50));

    // Detect terminal and paste
    let is_terminal = is_terminal_window()?;
    if is_terminal {
        paste_with_shift(&mut enigo)?; // Ctrl+Shift+V
    } else {
        paste_normal(&mut enigo)?;      // Ctrl+V
    }

    // Restore clipboard
    std::thread::sleep(Duration::from_millis(50));
    clipboard.set_text(&saved)?;

    Ok(())
}
```

**Terminal detection approaches:**
- Linux: Try `hyprctl activewindow -j`, `swaymsg`, `wmctrl` (fallback to conservative)
- macOS: AppleScript to get frontmost app name
- Match against known terminal list: kitty, alacritty, foot, wezterm, etc.

#### Step 4: Remove Old Implementation

```bash
# Backup original
mv src/output/inject.rs src/output/inject.rs.wtype-backup

# The new implementation goes in src/output/mod.rs
# (similar to how we did audio/mod.rs)
```

#### Step 5: Update Integration Points

Check and update imports in:
- `src/main.rs` (should use `output::inject_text`)
- Any other files that import from `src/output/`

#### Step 6: Build and Fix Compilation Errors

```bash
cargo build
# Fix any errors iteratively
```

### Day 5: Test enigo on Linux (4 hours estimated)

**Testing checklist:**

1. **Basic text injection:**
   ```bash
   cargo run -- start -d 3
   # Speak something, verify it types correctly
   ```

2. **Clipboard preservation:**
   - Copy something to clipboard
   - Run `cargo run -- start -d 3`
   - Speak, let it type
   - Verify original clipboard is restored

3. **Terminal detection:**
   - Test in kitty (should use Ctrl+Shift+V)
   - Test in browser (should use Ctrl+V)
   - Check logs: "Using Ctrl+Shift+V for terminal paste" vs "Using Ctrl+V"

4. **Clipboard mode:**
   ```bash
   cargo run -- start -d 3 -c
   # Should copy to clipboard only, not type
   ```

5. **All tests pass:**
   ```bash
   cargo test
   # All 31+ tests should pass
   ```

6. **Non-US keyboard layouts:**
   - If you have alternate layouts, test that characters work correctly
   - enigo v0.6+ should handle this automatically via compositor keymap

---

## Reference Documentation

**Primary source:** `docs/project/phase4-implementation-plan.md`
- Section 5, Days 4-5 (lines 470-720)
- Section 13: Dependencies (lines 1150+)
- Section 10: Risk Assessment (updated)

**Key points from plan:**
- enigo v0.6.1 fixes Wayland keyboard layout issues (v0.4.0+)
- arboard needs `wayland-data-control` feature on Linux
- Terminal detection is process-based, not perfect (make configurable)
- Clipboard timing: use 50ms delays

---

## Success Criteria

### Must Have:
- ✅ Compiles successfully with enigo + arboard
- ✅ All existing tests pass
- ✅ Basic text injection works on Linux
- ✅ Clipboard preservation works
- ✅ Terminal detection works for common terminals (kitty, alacritty)

### Nice to Have:
- ✅ Clipboard-only mode works
- ✅ Graceful fallback if terminal detection fails
- ✅ Configurable terminal list

### Out of Scope (for later):
- ❌ macOS testing (no Mac available)
- ❌ Windows testing (future)
- ❌ Advanced terminal detection (can improve later)

---

## File Changes Expected

```
Modified:
  Cargo.toml                  (add enigo + arboard with features)
  src/output/mod.rs           (new enigo implementation)

Removed:
  src/output/inject.rs        (backed up as inject.rs.wtype-backup)

Possibly modified:
  src/main.rs                 (if import paths change)
```

**Estimated diff:**
- ~200 lines removed (wtype/xdotool subprocess code)
- ~250 lines added (enigo + arboard implementation)
- Net: +50 lines (but simpler, cross-platform code)

---

## Important Notes

1. **enigo API version:** The plan references v0.2, but we're using **v0.6.1**
   - Key differences documented in plan lines 1224-1228
   - Clipboard removed from enigo, use arboard instead

2. **Feature flags are critical:**
   - Linux MUST have `["wayland", "x11rb"]` for enigo
   - Linux MUST have `["wayland-data-control"]` for arboard
   - Without these, Wayland support won't work

3. **Terminal detection is heuristic:**
   - Not 100% accurate
   - Conservative fallback: use normal paste (Ctrl+V)
   - Can be improved incrementally

4. **Testing priority:**
   - Get basic injection working first
   - Then clipboard preservation
   - Then terminal detection
   - Then edge cases

5. **If you get stuck:**
   - Check enigo v0.6.1 docs: https://docs.rs/enigo/0.6.1/enigo/
   - Check arboard v3.6 docs: https://docs.rs/arboard/3.6/arboard/
   - Reference plan: `docs/project/phase4-implementation-plan.md`

---

## Git Workflow

When complete:

```bash
# Stage changes
git add Cargo.toml src/output/mod.rs src/output/inject.rs.wtype-backup

# Remove old file
git rm src/output/inject.rs

# Commit
git commit -m "feat: migrate text injection from wtype/xdotool to enigo (Phase 4, Days 4-5)

Replace Linux-specific wtype/xdotool with cross-platform enigo + arboard.

Changes:
- Replace src/output/inject.rs with enigo-based mod.rs
- Add enigo 0.6 + arboard 3.6 dependencies (platform-specific features)
- Implement clipboard preservation via arboard
- Implement terminal detection for paste shortcuts
- Backup original as inject.rs.wtype-backup

Benefits:
- Cross-platform support (Linux, macOS, Windows)
- No external dependencies (wtype, xdotool, wl-clipboard)
- Native Rust implementation
- Better Wayland support (enigo v0.6 fixes keyboard layouts)

Testing:
- Basic text injection verified
- Clipboard preservation working
- Terminal detection functional
- All tests passing

See docs/project/phase4-implementation-plan.md for details."

# Push
git push origin phase4-cpal-migration
```

---

## Questions to Ask User

Before starting:
1. ✅ "I'm implementing Days 4-5 of Phase 4 (enigo migration). Ready to proceed?"

During implementation:
2. ✅ "Compilation successful with enigo + arboard. Ready to test?"

After testing:
3. ✅ "Text injection working on your system. Should I commit?"

If issues arise:
4. ⚠️ "Hit an issue with [X]. Here's what I found... How should we proceed?"

---

## Expected Timeline

- **Reading & planning:** 30 mins
- **Cargo.toml update:** 10 mins
- **Implementation:** 3-4 hours
- **Compilation fixes:** 1 hour
- **Testing:** 2 hours
- **Commit & cleanup:** 30 mins

**Total:** ~6-8 hours (can split into multiple sessions)

---

## Current System Info

**Environment:**
- OS: Fedora 42
- Desktop: Hyprland (Wayland)
- Terminal: kitty
- Rust: 1.91.1
- Branch: `phase4-cpal-migration`

**Existing daemon:**
- User may have old daemon running
- Not needed for testing basic injection
- Can test with: `cargo run -- start -d 3`

---

## Ready to Start?

Execute this plan step-by-step. Use TodoWrite to track progress. Ask for approval before major changes. Test incrementally.

Good luck! 🚀
