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
    /// * `segment_time_s` - The duration of segments to split the audio file by in seconds
    /// * `out_template` - Path/ template string of the split audio files
    /// * `extra_args` - Additional optional args
    ///
    /// # Errors
    ///
    /// Returns `YtDlpError` if the file cannot be read.
    fn split_audio_to_chunks(
        &self,
        file_input_path: impl AsRef<Path>,
        segment_time_s: u16,
        out_template: impl AsRef<Path>,
        extra_args: Option<&[&str]>,
    ) -> Result<(), YtDlpError>;
}

impl AudioProcessor for YtDlp {
    fn split_audio_to_chunks(
        &self,
        file_input_path: impl AsRef<Path>,
        segment_time_s: u16,
        out_template: impl AsRef<Path>,
        extra_args: Option<&[&str]>,
    ) -> Result<(), YtDlpError> {
        let input_str = file_input_path.as_ref().to_str().ok_or_else(|| {
            YtDlpError::InvalidPath(file_input_path.as_ref().display().to_string())
        })?;
        let output_str = out_template
            .as_ref()
            .to_str()
            .ok_or_else(|| YtDlpError::InvalidPath(out_template.as_ref().display().to_string()))?;
        let segment_time_s = segment_time_s.to_string();

        let mut args = vec![
            "-i",
            input_str,
            "-f",
            "segment",
            "-segment_time",
            &segment_time_s,
            "-ac",
            "1",
            "-b:a",
            "64k",
            "-ar",
            "16000",
            "-c:a",
            "libmp3lame",
        ];

        if let Some(extra) = extra_args {
            args.extend_from_slice(extra);
        }

        args.push(output_str);

        self.run_ffmpeg(&args)
    }
}
