# Voice-to-Text Model Research (2024-2025)

**Status:** Research Complete | **Last Updated:** 2025-12-19

Experimental research into modern Speech-to-Text (STT) models and performance metrics.

## Model Comparison Matrix

| Model Name | Organization | Release Date | Speed Factor* | File Size (FP16 / Q4) | Parameters |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Whisper-large-v3-turbo** | OpenAI | Oct 2024 | ~4x | 1.6 GB / 850 MB | 809M |
| **Distil-large-v3** | Hugging Face | Mar 2024 | ~6.3x | 1.5 GB / 650 MB | 756M |
| **Whisper-large-v3** | OpenAI | Nov 2023 | 1x (Base) | 3.0 GB / 1.5 GB | 1.54B |
| **Faster-Whisper** | Systran | Dec 2024 | ~4x | Optimized | Optimized |
| **Canary** | NVIDIA | 2024 | <100ms Latency | ~2.5 GB | SOTA |
| **Nova-2** | Deepgram | 2024 | Ultra-Low | API / SH | - |
| **Universal-1** | AssemblyAI | 2024 | <500ms | API / SH | - |
| **SeamlessM4T v2** | Meta | Dec 2023 | - | ~4.5 GB | 2.3B |
| **Wav2Vec2** | Meta | 2021 | Ultra-Light | 380 MB / 200 MB | 94M |
| **Whisper.cpp (Turbo)** | G. Gerganov | Dec 2024 | CPU Optimized | 75 MB (tiny) | - |

*\*Speed factor relative to standard Whisper Large V3 on modern GPU hardware.*

## Top Recommendations

### 1. General Purpose Performance
**Whisper-large-v3-turbo** is the current sweet spot.
- 4 decoder layers vs 32 in standard Large-V3
- Near-identical accuracy for English
- Fully supported by latest `whisper-rs` integrations

### 2. High-Efficiency / Low-Power
**Distil-Whisper Large V3**
- 50% parameter reduction
- 6x speedup
- Designed to be a drop-in replacement for any Whisper implementation

### 3. Real-Time / Low Latency
**NVIDIA Canary (Riva)**
- Specialized for streaming
- <100ms latency from audio chunk to text
- Excellent for live captioning or voice-controlled UIs

## Key Metrics Definitions
- **WER (Word Error Rate):** Percentage of errors in transcription (lower is better)
- **Parameters:** Total number of learned variables; generally correlates with VRAM usage
- **Inference Latency:** Time taken from "audio end" to "text generation"

## Resources
- [OpenAI Whisper GitHub](https://github.com/openai/whisper)
- [Nugging Face ASR Leaderboard](https://huggingface.co/spaces/hf-audio/open_asr_leaderboard)
- [Distil-Whisper Collection](https://huggingface.co/distil-whisper)

🤖 Generated with [Claude Code](https://claude.com/claude-code)
