//! # video
//!
//! Enrich `YtDlp` by adding video processing capabilities

use crate::{error::YtDlpError, YtDlp};
use std::path::Path;

/// A trait for processing video files.
/// Requires `ffmpeg` v7* available in the evironment
pub trait VideoProcessor {
    /// Converts a video to a different format.
    ///
    /// # Arguments
    /// * `input_path` - Path to video
    /// * `output_path` - Path to converted video
    ///
    /// # Errors
    ///
    /// Returns `YtDlpError` if the file cannot be read.
    fn convert_video<P: AsRef<Path>>(
        &self,
        input_path: P,
        output_path: P,
    ) -> Result<(), YtDlpError>;

    /// Extracts frames from a video
    ///
    /// # Arguments
    /// * `video_input_path` - Path to video
    /// * `fps`: Set frames per second options e.g "1", "1/10" or "fps=1,scale=320:24"
    /// * `out_template` - Path/ template string of the frame PNG image files
    /// * `extra_args` - Optional extra args to pass to the underlying ffmpeg command
    ///
    /// # Errors
    ///
    /// Returns `YtDlpError` if the file cannot be read.
    fn extract_frames<P: AsRef<Path>>(
        &self,
        input_path: P,
        fps: &str,
        output_template: P,
        extra_args: Option<&[&str]>,
    ) -> Result<(), YtDlpError>;
}

impl VideoProcessor for YtDlp {
    fn convert_video<P: AsRef<Path>>(
        &self,
        input_path: P,
        output_path: P,
    ) -> Result<(), YtDlpError> {
        let input_str = input_path.as_ref().to_str().ok_or_else(|| {
            YtDlpError::InvalidOutputPath(input_path.as_ref().display().to_string())
        })?;
        let output_str = output_path.as_ref().to_str().ok_or_else(|| {
            YtDlpError::InvalidOutputPath(output_path.as_ref().display().to_string())
        })?;

        self.run_ffmpeg(&["-i", input_str, output_str])
    }

    fn extract_frames<P: AsRef<Path>>(
        &self,
        input_path: P,
        fps: &str,
        output_template: P,
        extra_args: Option<&[&str]>,
    ) -> Result<(), YtDlpError> {
        let input_str = input_path.as_ref().to_str().ok_or_else(|| {
            YtDlpError::InvalidOutputPath(input_path.as_ref().display().to_string())
        })?;
        let output_str = output_template.as_ref().to_str().ok_or_else(|| {
            YtDlpError::InvalidOutputPath(output_template.as_ref().display().to_string())
        })?;
        let fps = format!("fps={}", fps);
        let extra_args = extra_args.map(|args| args.join(" ")).unwrap_or_default();

        // ffmpeg -i input_video.mp4 -vf fps=1/10 output_%04d.png
        self.run_ffmpeg(&["-i", input_str, "-vf", &fps, &extra_args, output_str])
    }
}
