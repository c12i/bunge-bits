//! ytdlp-bindings
//!
//! This crate provides a Rust interface to the yt-dlp command-line program,
//! which is used for downloading videos and subtitles from YouTube and other platforms.
//!
//! The main struct `YtDlp` offers methods to download subtitles and process VTT files.
//! It also implements the `VttProcessor` trait for handling VTT subtitle files.
//!
//! This crate was developed to initially serve a single purpose of downloading closed
//! captions from YouTube videos, hence the limited features. PRs are open to add more
//! features and add support for video processing.
//!
//! # Features
//!
//! - `yt-dlp-vendored`: When enabled, the crate will use a vendored version of yt-dlp.
//!   When disabled, you need to provide the path to the yt-dlp binary.
//!
//! # Examples
//!
//! ```rust,no_run
//! use ytdlp_bindings::YtDlp;
//! use std::path::Path;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let ytdlp = YtDlp::new()?;
//! let output_path = Path::new("output.vtt");
//! ytdlp.download_auto_sub("https://www.youtube.com/watch?v=dQw4w9WgXcQ", output_path)?;
//! # Ok(())
//! # }
//! ```

mod error;
mod file_types;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::{Path, PathBuf};
use std::process::Command;

use error::YtDlpError;
use file_types::{parse_vtt_content, SubtitleEntry, VttProcessor};

pub struct YtDlp {
    binary_path: PathBuf,
}

impl YtDlp {
    #[cfg(feature = "yt-dlp-vendored")]
    pub fn new() -> Result<Self, YtDlpError> {
        let binary_path = env::var("YTDLP_BINARY")
            .map(PathBuf::from)
            .or_else(|_| which::which("yt-dlp"))
            .map_err(|_| YtDlpError::BinaryNotFound)?;

        Ok(YtDlp { binary_path })
    }

    #[cfg(not(feature = "yt-dlp-vendored"))]
    pub fn new<P: Into<PathBuf>>(binary_path: P) -> Self {
        YtDlp {
            binary_path: binary_path.into(),
        }
    }

    pub fn download_auto_sub<P: AsRef<Path>>(
        &self,
        url: &str,
        output_path: P,
    ) -> Result<(), YtDlpError> {
        let output_str = output_path.as_ref().to_str().ok_or_else(|| {
            YtDlpError::InvalidOutputPath(output_path.as_ref().display().to_string())
        })?;

        self.run_command(&[
            "--write-auto-sub",
            "--skip-download",
            "--output",
            output_str,
            url,
        ])
    }

    pub fn download_sub<P: AsRef<Path>>(
        &self,
        url: &str,
        output_path: P,
    ) -> Result<(), YtDlpError> {
        let output_str = output_path.as_ref().to_str().ok_or_else(|| {
            YtDlpError::InvalidOutputPath(output_path.as_ref().display().to_string())
        })?;

        self.run_command(&[
            "--write-sub",
            "--skip-download",
            "--output",
            output_str,
            url,
        ])
    }

    fn run_command(&self, args: &[&str]) -> Result<(), YtDlpError> {
        let output = Command::new(&self.binary_path).args(args).output()?;

        if output.status.success() {
            Ok(())
        } else {
            Err(YtDlpError::NonZeroExit(output.status.code().unwrap_or(-1)))
        }
    }
}

impl VttProcessor for YtDlp {
    fn read_vtt_file<P: AsRef<Path>>(&self, vtt_path: P) -> Result<String, YtDlpError> {
        let mut file = File::open(vtt_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }

    fn stream_vtt_file<P: AsRef<Path>>(
        &self,
        vtt_path: P,
    ) -> Box<dyn Iterator<Item = Result<String, YtDlpError>>> {
        let file = match File::open(vtt_path) {
            Ok(file) => file,
            Err(e) => return Box::new(std::iter::once(Err(YtDlpError::VttReadError(e)))),
        };

        let reader = BufReader::new(file);
        Box::new(
            reader
                .lines()
                .map(|line| line.map_err(YtDlpError::VttReadError)),
        )
    }

    fn process_vtt_file<P: AsRef<Path>>(
        &self,
        vtt_path: P,
    ) -> Result<Vec<SubtitleEntry>, YtDlpError> {
        let content = self.read_vtt_file(vtt_path)?;
        Ok(parse_vtt_content(&content))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glob::glob;
    use std::fs;
    use std::io::Read;

    #[test]
    fn test_new() {
        let result = YtDlp::new();
        assert!(result.is_ok());
    }

    #[cfg(not(feature = "yt-dlp-vendored"))]
    #[test]
    #[ignore = "This test depends on an existing installation of yt-dlp"]
    fn test_with_binary() {
        let ytdlp = YtDlp::new("yt-dlp").unwrap();
        assert_eq!(ytdlp.binary_path, PathBuf::from("yt-dlp"));
    }

    #[test]
    fn test_download_auto_sub() {
        let ytdlp = YtDlp::new().unwrap();
        let temp_dir = env::temp_dir();
        let output_path = temp_dir.join("%(title)s.%(ext)s");
        let result =
            ytdlp.download_auto_sub("https://www.youtube.com/watch?v=p1OqRc15K3o", output_path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_download_sub() {
        let ytdlp = YtDlp::new().unwrap();
        let temp_dir = env::temp_dir();
        let output_path = temp_dir.join("%(title)s.%(ext)s");
        let result = ytdlp.download_sub("https://www.youtube.com/watch?v=p1OqRc15K3o", output_path);
        assert!(result.is_ok());

        // The sample video explicitly has no non-auto subs, so we expect nothing to have been downloaded
        let pattern = temp_dir.join("*.vtt").to_str().unwrap().to_string();
        let paths: Vec<_> = glob(&pattern).unwrap().collect();
        assert!(paths.is_empty());
    }

    #[test]
    #[ignore = "This test is only for debugging purposes"]
    fn test_download_auto_sub_part2() -> Result<(), Box<dyn std::error::Error>> {
        let ytdlp = YtDlp::new()?;
        let temp_dir = env::temp_dir();
        let output_pattern = temp_dir.join("%(title)s.%(ext)s");

        println!("Downloading subtitles to: {}", temp_dir.display());

        ytdlp.download_auto_sub(
            "https://www.youtube.com/watch?v=p1OqRc15K3o",
            &output_pattern,
        )?;

        // Use glob to find the downloaded file
        let pattern = temp_dir.join("*.vtt").to_str().unwrap().to_string();
        let paths: Vec<_> = glob(&pattern)?.collect();

        assert!(!paths.is_empty(), "No subtitle file found");

        for path in paths {
            let path = path?;
            println!("Found subtitle file: {}", path.display());

            // Read and print file contents
            let mut file = fs::File::open(&path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            println!("File contents:\n{}", contents);

            // Optional: Delete the file after the test
            fs::remove_file(path)?;
        }

        Ok(())
    }

    struct MockVttProcessor;

    impl VttProcessor for MockVttProcessor {
        fn read_vtt_file<P: AsRef<Path>>(&self, _vtt_path: P) -> Result<String, YtDlpError> {
            Ok("WEBVTT\n\n00:00:01.000 --> 00:00:04.000\nHello, world!\n\n".to_string())
        }

        fn stream_vtt_file<P: AsRef<Path>>(
            &self,
            _vtt_path: P,
        ) -> Box<dyn Iterator<Item = Result<String, YtDlpError>>> {
            Box::new(
                vec![
                    Ok("WEBVTT".to_string()),
                    Ok("".to_string()),
                    Ok("00:00:01.000 --> 00:00:04.000".to_string()),
                    Ok("Hello, world!".to_string()),
                    Ok("".to_string()),
                ]
                .into_iter(),
            )
        }

        fn process_vtt_file<P: AsRef<Path>>(
            &self,
            _vtt_path: P,
        ) -> Result<Vec<SubtitleEntry>, YtDlpError> {
            Ok(vec![SubtitleEntry {
                start_time: "00:00:01.000".to_string(),
                end_time: "00:00:04.000".to_string(),
                text: "Hello, world!".to_string(),
            }])
        }
    }

    #[test]
    fn test_read_vtt_file() {
        let processor = MockVttProcessor;
        let content = processor.read_vtt_file("dummy.vtt").unwrap();
        assert!(content.contains("WEBVTT"));
        assert!(content.contains("Hello, world!"));
    }

    #[test]
    fn test_stream_vtt_file() {
        let processor = MockVttProcessor;
        let lines: Vec<String> = processor
            .stream_vtt_file("dummy.vtt")
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert_eq!(lines[0], "WEBVTT");
        assert!(lines.contains(&"Hello, world!".to_string()));
    }

    #[test]
    fn test_process_vtt_file() {
        let processor = MockVttProcessor;
        let entries = processor.process_vtt_file("dummy.vtt").unwrap();

        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].start_time, "00:00:01.000");
        assert_eq!(entries[0].end_time, "00:00:04.000");
        assert_eq!(entries[0].text, "Hello, world!");
    }
}
