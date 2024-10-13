//! # audio
//!
//! Enrich `YtDlp` by adding audio processing capabilities

use std::path::Path;

use crate::{YtDlp, YtDlpError};

/// A trait for processing audio files.
/// Requires `ffmpeg` v7* available in the evironment
pub trait AudioProcessor {
    /// Split an given audi into chunks based on a segment time in seconds
    ///
    /// # Arguments
    ///
    /// * `file_input_path` - The path to the downloaded audio file.
    /// * `segment_time` - The duration of segments to split the audio file by in seconds
    /// * `out_template` - Path/ template string of the split audio files
    ///
    /// # Errors
    ///
    /// Returns `YtDlpError` if the file cannot be read.
    fn split_audio_to_chunks(
        &self,
        file_input_path: impl AsRef<Path>,
        segment_time: u16,
        out_template: impl AsRef<Path>,
    ) -> Result<(), YtDlpError>;
}

impl AudioProcessor for YtDlp {
    fn split_audio_to_chunks(
        &self,
        file_input_path: impl AsRef<Path>,
        segment_time: u16,
        output_template: impl AsRef<Path>,
    ) -> Result<(), YtDlpError> {
        let input_str = file_input_path.as_ref().to_str().ok_or_else(|| {
            YtDlpError::InvalidOutputPath(file_input_path.as_ref().display().to_string())
        })?;
        let output_str = output_template.as_ref().to_str().ok_or_else(|| {
            YtDlpError::InvalidOutputPath(output_template.as_ref().display().to_string())
        })?;

        self.run_ffmpeg(&[
            "-i",
            input_str,
            "-f",
            "segment",
            "-segment_time",
            &segment_time.to_string(),
            "-c",
            "copy",
            output_str,
        ])
    }
}
