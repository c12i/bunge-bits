use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::YtDlpError;

/// The main struct for interacting with yt-dlp.
///
/// This struct provides methods to download subtitles and process VTT files.
/// It can be created with a custom binary path or use a vendored binary.
pub struct YtDlp {
    pub(crate) binary_path: PathBuf,
}

impl YtDlp {
    /// Creates a new `YtDlp` instance using the vendored binary.
    ///
    /// This method is only available when the `yt-dlp-vendored` feature is enabled.
    ///
    /// # Errors
    ///
    /// Returns `YtDlpError::BinaryNotFound` if the binary cannot be located.
    #[cfg(feature = "yt-dlp-vendored")]
    pub fn new() -> Result<Self, YtDlpError> {
        let binary_path = env::var("YTDLP_BINARY")
            .map(PathBuf::from)
            .or_else(|_| which::which("yt-dlp"))
            .map_err(|_| YtDlpError::BinaryNotFound("yt-dlp".to_string()))?;

        Ok(YtDlp { binary_path })
    }

    /// Creates a new `YtDlp` instance with a custom binary path.
    ///
    /// This method is only available when the `yt-dlp-vendored` feature is disabled.
    ///
    /// # Arguments
    ///
    /// * `binary_path` - The path to the yt-dlp binary.
    #[cfg(not(feature = "yt-dlp-vendored"))]
    pub fn new<P: Into<PathBuf>>(binary_path: P) -> Self {
        YtDlp {
            binary_path: binary_path.into(),
        }
    }

    /// Downloads a single video from the given URL.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL of the video to download.
    /// * `output_template` - A template string for the output filename.
    ///   See yt-dlp documentation for available template options.
    ///
    /// # Errors
    ///
    /// Returns `YtDlpError` if the download fails or if the output template is invalid.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use ytdlp_bindings::YtDlp;
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let ytdlp = YtDlp::new()?;
    ///
    ///     ytdlp.download_video("https://www.youtube.com/watch?v=dQw4w9WgXcQ", "%(title)s.%(ext)s")?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn download_video<P: AsRef<Path>>(
        &self,
        url: &str,
        output_template: P,
    ) -> Result<(), YtDlpError> {
        let output_str = output_template.as_ref().to_str().ok_or_else(|| {
            YtDlpError::InvalidOutputPath(output_template.as_ref().display().to_string())
        })?;

        self.run_yt_dlp(&["--output", output_str, url])
    }

    /// Downloads a single audio from the given URL in mp3 format.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL of the video whose audio to download.
    /// * `output_template` - A template string for the output filename.
    ///   See yt-dlp documentation for available template options.
    ///
    /// # Errors
    ///
    /// Returns `YtDlpError` if the download fails or if the output template is invalid.
    pub fn download_audio<P: AsRef<Path>>(
        &self,
        url: &str,
        output_template: P,
    ) -> Result<(), YtDlpError> {
        let output_str = output_template.as_ref().to_str().ok_or_else(|| {
            YtDlpError::InvalidOutputPath(output_template.as_ref().display().to_string())
        })?;

        self.run_yt_dlp(&[
            "-f",
            "bestaudio",
            "-x",
            "--audio-format",
            "mp3",
            "--output",
            output_str,
            url,
        ])
    }

    /// Downloads all videos from a playlist URL.
    ///
    /// # Arguments
    ///
    /// * `playlist_url` - The URL of the playlist to download.
    /// * `output_template` - A template string for the output filenames.
    ///   See yt-dlp documentation for available template options.
    ///
    /// # Errors
    ///
    /// Returns `YtDlpError` if the download fails or if the output template is invalid.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use ytdlp_bindings::YtDlp;
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let ytdlp = YtDlp::new()?;
    ///
    ///     ytdlp.download_playlist(
    ///         "https://www.youtube.com/playlist?list=PLv3TTBr1W_9tppikBxAE_G6qjWdBljBHJ",
    ///         "playlist_videos/%(playlist_index)s-%(title)s.%(ext)s"
    ///     )?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn download_playlist<P: AsRef<Path>>(
        &self,
        playlist_url: &str,
        output_template: P,
    ) -> Result<(), YtDlpError> {
        let output_str = output_template.as_ref().to_str().ok_or_else(|| {
            YtDlpError::InvalidOutputPath(output_template.as_ref().display().to_string())
        })?;

        self.run_yt_dlp(&["--output", output_str, "--yes-playlist", playlist_url])
    }

    /// Downloads video or audio aud with custom options.
    ///
    /// This method allows you to pass custom yt-dlp options for more advanced use cases.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL of the video to download.
    /// * `options` - A slice of strings representing yt-dlp options.
    ///
    /// # Errors
    ///
    /// Returns `YtDlpError` if the download fails.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use ytdlp_bindings::YtDlp;
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let ytdlp = YtDlp::new()?;
    ///     ytdlp.download_with_options(
    ///         "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
    ///         &["--format", "bestaudio/best", "--extract-audio", "--audio-format", "mp3", "--output", "audio.%(ext)s"]
    ///     )?;
    ///     Ok(())
    /// }
    /// ```
    pub fn download_with_options(&self, url: &str, options: &[&str]) -> Result<(), YtDlpError> {
        let mut args = options.to_vec();
        args.push(url);
        self.run_yt_dlp(&args)
    }

    /// Downloads auto-generated subtitles for a given URL in VTT format.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL of the video to download subtitles for.
    /// * `output_path` - A template string of the output file
    ///
    /// # Errors
    ///
    /// Returns `YtDlpError` if the download fails or if the output path is invalid.
    pub fn download_auto_sub<P: AsRef<Path>>(
        &self,
        url: &str,
        output_template: P,
    ) -> Result<(), YtDlpError> {
        let output_str = output_template.as_ref().to_str().ok_or_else(|| {
            YtDlpError::InvalidOutputPath(output_template.as_ref().display().to_string())
        })?;

        self.run_yt_dlp(&[
            "--write-auto-sub",
            "--skip-download",
            "--output",
            output_str,
            url,
        ])
    }

    /// Downloads available subtitles for a given URL.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL of the video to download subtitles for.
    /// * `output_path` - The path where the subtitle file will be saved.
    ///
    /// # Errors
    ///
    /// Returns `YtDlpError` if the download fails or if the output path is invalid.
    pub fn download_sub<P: AsRef<Path>>(
        &self,
        url: &str,
        output_path: P,
    ) -> Result<(), YtDlpError> {
        let output_str = output_path.as_ref().to_str().ok_or_else(|| {
            YtDlpError::InvalidOutputPath(output_path.as_ref().display().to_string())
        })?;

        self.run_yt_dlp(&[
            "--write-sub",
            "--skip-download",
            "--output",
            output_str,
            url,
        ])
    }

    pub(crate) fn run_yt_dlp(&self, args: &[&str]) -> Result<(), YtDlpError> {
        let output = Command::new(&self.binary_path).args(args).output()?;

        if output.status.success() {
            Ok(())
        } else {
            Err(YtDlpError::NonZeroExit {
                command: self.binary_path.to_string_lossy().into(),
                status: output.status.code().unwrap_or(-1),
                output: String::from_utf8_lossy(&output.stdout.to_vec()).into(),
            })
        }
    }

    #[cfg(any(feature = "audio-processing", feature = "video-processing"))]
    pub(crate) fn run_ffmpeg(&self, args: &[&str]) -> Result<(), YtDlpError> {
        if which::which("ffmpeg").is_err() {
            return Err(YtDlpError::BinaryNotFound("ffmpeg".to_string()));
        }
        let output = Command::new("ffmpeg").args(args).output()?;

        if output.status.success() {
            Ok(())
        } else {
            Err(YtDlpError::NonZeroExit {
                command: "ffmpeg".to_string(),
                status: output.status.code().unwrap_or(-1),
                output: String::from_utf8_lossy(&output.stdout.to_vec()).into(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glob::glob;
    use std::fs;
    use std::io::Read;
    use tempfile::tempdir;

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
    #[ignore = "Flaky"]
    fn test_download_sub() {
        let ytdlp = YtDlp::new().unwrap();
        let temp_dir = env::temp_dir();
        let output_path = temp_dir.join("%(title)s.%(ext)s");
        let result = ytdlp.download_sub("https://www.youtube.com/watch?v=NJMW2app0VI", output_path);
        assert!(result.is_ok());

        // The sample video explicitly has no non-auto subs, so we expect nothing to have been downloaded
        let pattern = temp_dir.join("*.vtt").to_str().unwrap().to_string();
        let paths: Vec<_> = glob(&pattern).unwrap().collect();
        assert!(paths.is_empty());
    }

    #[test]
    fn debug_download_auto_sub_part() -> Result<(), Box<dyn std::error::Error>> {
        let ytdlp = YtDlp::new()?;
        let temp_dir = env::temp_dir();
        let output_pattern = temp_dir.join("%(title)s.%(ext)s");

        println!("Downloading subtitles to: {}", temp_dir.display());

        ytdlp.download_auto_sub(
            "https://www.youtube.com/watch?v=p1OqRc15K3o",
            output_pattern,
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

            // Delete the file after the test
            fs::remove_file(path)?;
        }

        Ok(())
    }

    const TEST_VIDEO_URL: &str = "https://www.youtube.com/watch?v=jNQXAC9IVRw";
    const TEST_PLAYLIST_URL: &str =
        "https://www.youtube.com/playlist?list=PLzH6n4zXuckpKAj1_88VS-8Z6yn9zX_P6";

    #[test]
    fn test_download_video() {
        let ytdlp = YtDlp::new().expect("Failed to create YtDlp instance");
        let temp_dir = tempdir().expect("Failed to create temp directory");
        let output_template = temp_dir.path().join("%(title)s.%(ext)s");

        let result = ytdlp.download_video(TEST_VIDEO_URL, output_template);

        assert!(
            result.is_ok(),
            "Failed to download video: {:?}",
            result.err()
        );

        let files: Vec<_> = fs::read_dir(temp_dir.path())
            .expect("Failed to read temp directory")
            .collect();

        assert!(!files.is_empty(), "No files were downloaded");

        let downloaded_file = &files[0].as_ref().expect("Failed to get file info").path();
        assert!(downloaded_file.exists(), "Downloaded file does not exist");

        let file_size = fs::metadata(downloaded_file)
            .expect("Failed to get file metadata")
            .len();
        assert!(
            file_size > 500_000,
            "File size is too small, expected > 1MB, got {} bytes",
            file_size
        );
    }

    #[test]
    fn test_download_video_with_format() {
        let ytdlp = YtDlp::new().expect("Failed to create YtDlp instance");
        let temp_dir = tempdir().expect("Failed to create temp directory");
        let output_template = temp_dir.path().join("%(title)s.%(ext)s");

        let result = ytdlp.download_with_options(
            TEST_VIDEO_URL,
            &[
                "--format",
                "bestaudio[ext=m4a]",
                "--output",
                output_template.to_str().unwrap(),
            ],
        );

        assert!(
            result.is_ok(),
            "Failed to download video: {:?}",
            result.err()
        );

        let files: Vec<_> = fs::read_dir(temp_dir.path())
            .expect("Failed to read temp directory")
            .collect();

        assert!(!files.is_empty(), "No files were downloaded");

        let downloaded_file = &files[0].as_ref().expect("Failed to get file info").path();
        assert!(downloaded_file.exists(), "Downloaded file does not exist");
        assert!(
            downloaded_file.extension().unwrap() == "m4a",
            "File is not in m4a format"
        );
    }

    #[test]
    fn test_download_playlist() {
        let ytdlp = YtDlp::new().expect("Failed to create YtDlp instance");
        let temp_dir = tempdir().expect("Failed to create temp directory");
        let output_template = temp_dir.path().join("%(playlist_index)s-%(title)s.%(ext)s");

        let result = ytdlp.download_playlist(TEST_PLAYLIST_URL, output_template);

        assert!(
            result.is_ok(),
            "Failed to download playlist: {:?}",
            result.err()
        );

        let files: Vec<_> = fs::read_dir(temp_dir.path())
            .expect("Failed to read temp directory")
            .collect();

        assert!(
            files.len() > 1,
            "Not enough files were downloaded. Expected multiple, got {}",
            files.len()
        );

        for file in files {
            let file_path = file.expect("Failed to get file info").path();
            assert!(
                file_path.exists(),
                "Downloaded file does not exist: {:?}",
                file_path
            );

            let file_size = fs::metadata(&file_path)
                .expect("Failed to get file metadata")
                .len();
            assert!(
                file_size > 1000000,
                "File size is too small, expected > 1MB, got {} bytes: {:?}",
                file_size,
                file_path
            );
        }
    }
}
