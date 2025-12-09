use anyhow::{Context, Result};
use std::path::Path;
use tracing::{debug, info};
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

pub struct Transcriber {
    ctx: WhisperContext,
    language: String,
}

impl Transcriber {
    /// Create a new transcriber with the given model path
    pub fn new(model_path: &Path) -> Result<Self> {
        Self::with_language(model_path, "en")
    }

    /// Create a new transcriber with a specific language
    pub fn with_language(model_path: &Path, language: &str) -> Result<Self> {
        let params = WhisperContextParameters::default();

        let ctx = WhisperContext::new_with_params(
            model_path
                .to_str()
                .context("Invalid model path encoding")?,
            params,
        )
        .context("Failed to load whisper model")?;

        Ok(Self {
            ctx,
            language: language.to_string(),
        })
    }

    /// Transcribe audio data to text
    ///
    /// Audio must be:
    /// - 16kHz sample rate
    /// - Mono channel
    /// - f32 PCM format
    pub fn transcribe(&self, audio: &[f32]) -> Result<String> {
        if audio.is_empty() {
            return Ok(String::new());
        }

        debug!(
            "Transcribing {} samples ({:.2}s)",
            audio.len(),
            audio.len() as f32 / 16000.0
        );

        let mut state = self
            .ctx
            .create_state()
            .context("Failed to create whisper state")?;

        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });

        // Configure for dictation use case
        params.set_language(Some(&self.language));
        params.set_print_special(false);
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);
        params.set_no_context(true);
        params.set_single_segment(false);

        // Run inference
        state
            .full(params, audio)
            .context("Whisper inference failed")?;

        // Get segment count - returns i32 directly
        let num_segments = state.full_n_segments();
        debug!("Got {} segments", num_segments);

        // Collect all segments using get_segment() and to_str()
        let mut result = String::new();
        for i in 0..num_segments {
            if let Some(segment) = state.get_segment(i) {
                if let Ok(text) = segment.to_str() {
                    result.push_str(text);
                }
            }
        }

        let text = result.trim().to_string();
        info!("Transcribed: \"{}\"", text);

        Ok(text)
    }
}

/// Convert i16 audio samples to f32 (normalized to -1.0 to 1.0)
pub fn convert_i16_to_f32(samples: &[i16]) -> Vec<f32> {
    samples
        .iter()
        .map(|&s| s as f32 / i16::MAX as f32)
        .collect()
}

/// Convert stereo audio to mono by averaging channels
pub fn convert_stereo_to_mono(stereo: &[f32]) -> Vec<f32> {
    stereo
        .chunks(2)
        .map(|chunk| {
            if chunk.len() == 2 {
                (chunk[0] + chunk[1]) / 2.0
            } else {
                chunk[0]
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i16_to_f32_conversion() {
        let samples: Vec<i16> = vec![0, i16::MAX, i16::MIN];
        let converted = convert_i16_to_f32(&samples);

        assert!((converted[0] - 0.0).abs() < 0.001);
        assert!((converted[1] - 1.0).abs() < 0.001);
        assert!((converted[2] - (-1.0)).abs() < 0.01);
    }

    #[test]
    fn test_stereo_to_mono() {
        let stereo = vec![0.5, 0.3, 0.8, 0.2];
        let mono = convert_stereo_to_mono(&stereo);

        assert_eq!(mono.len(), 2);
        assert!((mono[0] - 0.4).abs() < 0.001);
        assert!((mono[1] - 0.5).abs() < 0.001);
    }
}
