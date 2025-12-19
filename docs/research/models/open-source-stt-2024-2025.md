# Open Source Speech-to-Text Models (2024-2025)

**Status:** Research Complete | **Last Updated:** 2025-12-19

This document tracks strictly Open Source (MIT, Apache 2.0, GPL) or Open Weight models for Speech-to-Text (STT) and Voice-to-Text applications.

## Key Open Source Models

| Model Name | License | Release Date | File Size | Best Use Case |
| :--- | :--- | :--- | :--- | :--- |
| **Foxhound** | MIT | Late 2023 | ~600 MB | Privacy-focused mobile/edge ASR |
| **NeMo Conquest** | Apache 2.0 | 2024 | ~2.5 GB | Enterprise/Conversational AI |
| **Vosk** | Apache 2.0 | 2024 (Active) | 50-200 MB | Resource-constrained / Embedded |
| **Silero Models** | MIT | 2024 (Update) | ~100 MB | Multilingual Fast / Lightweight |
| **WhisperX Sierra** | MIT | 2024 | ~500 MB | Diarization / Meeting Transcripts |
| **EdgeSpeechNets** | Apache 2.0 | 2024 | ~100 MB | Ultra-low latency IoT |
| **Mondia ASR** | GPL 3.0 | 2024 | ~1.5 GB | Low-resource / Niche languages |
| **Coqui STT** | Apache 2.0 | 2024 (Niche) | ~200 MB | Customizable privacy-first STT |
| **NeMo Whisper-v3** | Apache 2.0 | 2024 | ~3.5 GB | Research / SOTA Evaluation |
| **Suntime Whisper** | MIT | 2024 | ~1.5 GB | Hybrid ASR + Contextual LLM |
| **Silero VAD** | MIT | 2024 | ~2 MB | Voice Activity Detection (Smallest) |

## Comparison of Open Licenses

### Permissive (MIT / Apache 2.0)
Most models in the `whisper` and `NeMo` ecosystem use these. They allow for commercial use, modification, and private use with minimal restrictions.
- **Examples:** Foxhound, Silero, NVIDIA NeMo variations.

### Copyleft (GPL)
Requires that any derivative works also be open-sourced under the same license.
- **Example:** Mondia ASR.

## Emerging Trends in Open STT
1. **Diarization Integration:** Models like `WhisperX` are moving beyond simple text to identifying *who* is speaking using strictly open weights.
2. **LLM Hybrids:** Projects like `Suntime` are feeding Whisper outputs directly into small LLMs (like TinyLlama) for auto-correction of transcription errors.
3. **Edge Optimization:** `EdgeSpeechNets` and `Vosk` continue to dominate the sub-200MB space for offline, private hardware integration.

## Resources
- [NVIDIA NeMo Hub](https://huggingface.co/nvidia)
- [Alphacep (Vosk) Models](https://alphacephei.com/vosk/models)
- [Hugging Face Open ASR Collection](https://huggingface.co/collections/hf-audio)

🤖 Generated with [Claude Code](https://claude.com/claude-code)
