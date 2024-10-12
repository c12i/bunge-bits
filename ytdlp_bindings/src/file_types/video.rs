//! # video
//!
//! Note that this module is still a WIP

use crate::{error::YtDlpError, YtDlp};
use std::path::Path;

/// Represents metadata for a video file.
#[derive(Debug, Clone)]
pub struct VideoMetadata {
    pub title: String,
    pub duration: f64,
    pub format: String,
    pub resolution: Option<(u32, u32)>,
    pub fps: Option<f64>,
}

/// A trait for processing video files.
pub trait VideoProcessor {
    /// Extracts audio from a video file.
    fn extract_audio<P: AsRef<Path>>(
        &self,
        vide_url: &str,
        output_path: P,
    ) -> Result<(), YtDlpError>;

    /// Converts a video to a different format.
    fn convert_video<P: AsRef<Path>>(
        &self,
        video_url: &str,
        output_path: P,
        format: &str,
    ) -> Result<(), YtDlpError>;
}

impl VideoProcessor for YtDlp {
    fn extract_audio<P: AsRef<Path>>(
        &self,
        _video_url: &str,
        _output_path: P,
    ) -> Result<(), YtDlpError> {
        // TODO: Update to make use of ffmpeg
        todo!();
    }

    fn convert_video<P: AsRef<Path>>(
        &self,
        video_url: &str,
        output_path: P,
        format: &str,
    ) -> Result<(), YtDlpError> {
        self.run_command(&[
            "--recode-video",
            format,
            "-o",
            output_path.as_ref().to_str().ok_or_else(|| {
                YtDlpError::InvalidOutputPath(output_path.as_ref().display().to_string())
            })?,
            video_url,
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    struct MockYtDlp {
        #[allow(unused)]
        pub binary_path: PathBuf,
    }

    impl MockYtDlp {
        fn new() -> Self {
            MockYtDlp {
                binary_path: PathBuf::from("mock_yt-dlp"),
            }
        }
    }

    impl VideoProcessor for MockYtDlp {
        fn extract_audio<P: AsRef<Path>>(
            &self,
            _video_url: &str,
            _output_path: P,
        ) -> Result<(), YtDlpError> {
            Ok(())
        }

        fn convert_video<P: AsRef<Path>>(
            &self,
            _video_url: &str,
            _output_path: P,
            _format: &str,
        ) -> Result<(), YtDlpError> {
            Ok(())
        }
    }

    #[test]
    fn test_extract_audio() {
        let mock_ytdlp = MockYtDlp::new();
        assert!(mock_ytdlp.extract_audio("dummy.mp4", "audio.mp3").is_ok());
    }

    #[test]
    fn test_convert_video() {
        let mock_ytdlp = MockYtDlp::new();
        assert!(mock_ytdlp
            .convert_video("input.mp4", "output.webm", "webm")
            .is_ok());
    }
}
