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
        let input_path = file_input_path.as_ref();
        let output_path = output_template.as_ref();

        let input_str = input_path
            .to_str()
            .ok_or_else(|| YtDlpError::InvalidPath(input_path.display().to_string()))?;
        let output_str = output_path
            .to_str()
            .ok_or_else(|| YtDlpError::InvalidPath(output_path.display().to_string()))?;

        let ext = output_path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();

        let codec = match ext.as_str() {
            "wav" => "pcm_s16le",
            "mp3" => "libmp3lame",
            "flac" => "flac",
            "aac" => "aac",
            _ => return Err(YtDlpError::UnsupportedFormat(ext)),
        };

        self.run_ffmpeg(&[
            "-i",
            input_str,
            "-f",
            "segment",
            "-segment_time",
            &segment_time_s.to_string(),
            "-ac",
            "1",
            "-ar",
            "16000",
            "-c:a",
            codec,
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

        let codec = infer_codec(output_path.as_ref())?;

        self.run_ffmpeg(&[
            "-i", input_str, "-af", "loudnorm", "-ar", "16000", "-ac", "1", "-c:a", codec,
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

        let codec = infer_codec(output_path.as_ref())?;

        self.run_ffmpeg(&[
            "-i", input_str, "-af", "afftdn", "-ar", "16000", "-ac", "1", "-c:a", codec, output_str,
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

        let codec = infer_codec(output_path.as_ref())?;

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
            codec,
            output_str,
        ])
    }
}

fn infer_codec(path: &Path) -> Result<&'static str, YtDlpError> {
    match path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase()
        .as_str()
    {
        "wav" => Ok("pcm_s16le"),
        "mp3" => Ok("libmp3lame"),
        "flac" => Ok("flac"),
        "aac" => Ok("aac"),
        ext => Err(YtDlpError::UnsupportedFormat(ext.to_string())),
    }
}
