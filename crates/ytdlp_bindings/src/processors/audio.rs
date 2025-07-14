//! # audio
//!
//! Enrich `YtDlp` by adding audio processing capabilities such as
//! denoising, volume normalization, silence trimming, and chunking.

use std::path::Path;

use crate::{YtDlp, YtDlpError};

/// A trait for processing audio files using `ffmpeg`.
/// Requires `ffmpeg` v7+ available in the environment.
pub trait AudioProcessor {
    /// Split an audio file into fixed-length chunks (in seconds).
    fn split_audio_to_chunks(
        &self,
        file_input_path: impl AsRef<Path>,
        segment_time_s: u16,
        out_template: impl AsRef<Path>,
    ) -> Result<(), YtDlpError>;

    /// Normalize volume using EBU R128 loudness standard.
    fn normalize_volume(
        &self,
        input_path: impl AsRef<Path>,
        output_path: impl AsRef<Path>,
    ) -> Result<(), YtDlpError>;

    /// Apply basic denoising filter (FFT-based).
    fn denoise_audio(
        &self,
        input_path: impl AsRef<Path>,
        output_path: impl AsRef<Path>,
    ) -> Result<(), YtDlpError>;

    /// Trim leading and trailing silence.
    fn trim_silence(
        &self,
        input_path: impl AsRef<Path>,
        output_path: impl AsRef<Path>,
    ) -> Result<(), YtDlpError>;
}

impl AudioProcessor for YtDlp {
    fn split_audio_to_chunks(
        &self,
        file_input_path: impl AsRef<Path>,
        segment_time_s: u16,
        output_template: impl AsRef<Path>,
    ) -> Result<(), YtDlpError> {
        let input_str = file_input_path.as_ref().to_str().ok_or_else(|| {
            YtDlpError::InvalidPath(file_input_path.as_ref().display().to_string())
        })?;
        let output_str = output_template.as_ref().to_str().ok_or_else(|| {
            YtDlpError::InvalidPath(output_template.as_ref().display().to_string())
        })?;

        self.run_ffmpeg(&[
            "-i",
            input_str,
            "-f",
            "segment",
            "-segment_time",
            &segment_time_s.to_string(),
            "-ac",
            "1",
            "-b:a",
            "64k",
            "-ar",
            "16000",
            "-c:a",
            "libmp3lame",
            output_str,
        ])
    }

    fn normalize_volume(
        &self,
        input_path: impl AsRef<Path>,
        output_path: impl AsRef<Path>,
    ) -> Result<(), YtDlpError> {
        let input_str = input_path
            .as_ref()
            .to_str()
            .ok_or_else(|| YtDlpError::InvalidPath(input_path.as_ref().display().to_string()))?;
        let output_str = output_path
            .as_ref()
            .to_str()
            .ok_or_else(|| YtDlpError::InvalidPath(output_path.as_ref().display().to_string()))?;

        self.run_ffmpeg(&[
            "-i",
            input_str,
            "-af",
            "loudnorm",
            "-ar",
            "16000",
            "-ac",
            "1",
            "-c:a",
            "libmp3lame",
            output_str,
        ])
    }

    fn denoise_audio(
        &self,
        input_path: impl AsRef<Path>,
        output_path: impl AsRef<Path>,
    ) -> Result<(), YtDlpError> {
        let input_str = input_path
            .as_ref()
            .to_str()
            .ok_or_else(|| YtDlpError::InvalidPath(input_path.as_ref().display().to_string()))?;
        let output_str = output_path
            .as_ref()
            .to_str()
            .ok_or_else(|| YtDlpError::InvalidPath(output_path.as_ref().display().to_string()))?;

        self.run_ffmpeg(&[
            "-i",
            input_str,
            "-af",
            "afftdn",
            "-ar",
            "16000",
            "-ac",
            "1",
            "-c:a",
            "libmp3lame",
            output_str,
        ])
    }

    fn trim_silence(
        &self,
        input_path: impl AsRef<Path>,
        output_path: impl AsRef<Path>,
    ) -> Result<(), YtDlpError> {
        let input_str = input_path
            .as_ref()
            .to_str()
            .ok_or_else(|| YtDlpError::InvalidPath(input_path.as_ref().display().to_string()))?;
        let output_str = output_path
            .as_ref()
            .to_str()
            .ok_or_else(|| YtDlpError::InvalidPath(output_path.as_ref().display().to_string()))?;

        self.run_ffmpeg(&[
            "-i",
            input_str,
            "-af",
            "silenceremove=start_periods=1:start_threshold=-50dB:start_silence=0.1",
            "-ar",
            "16000",
            "-ac",
            "1",
            "-c:a",
            "libmp3lame",
            output_str,
        ])
    }
}
