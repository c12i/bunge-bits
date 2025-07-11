use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::YtDlpError;

/// The main struct for interacting with yt-dlp.
///
/// This struct provides methods to download subtitles and process VTT files.
/// It can be created with a custom binary path or use a vendored binary.
#[derive(Debug, Clone)]
pub struct YtDlp {
    pub(crate) binary_path: PathBuf,
    pub(crate) cookies_path: Option<PathBuf>,
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
        Self::new_with_cookies(None)
    }

    /// Creates a new `YtDlp` instance with optional cookies support, using a vendored binary.
    ///
    /// # Arguments
    ///
    /// * `cookies_path` - Optional path to a `cookies.txt` file for authenticated scraping.
    ///
    /// # Errors
    ///
    /// Returns [`YtDlpError::BinaryNotFound`] if the binary is not found.
    #[cfg(feature = "yt-dlp-vendored")]
    #[tracing::instrument]
    pub fn new_with_cookies(cookies_path: Option<PathBuf>) -> Result<Self, YtDlpError> {
        #[allow(clippy::const_is_empty)]
        let binary_path = Self::resolve_yt_dlp_binary()?;

        Ok(YtDlp {
            binary_path,
            cookies_path,
        })
    }

    /// Dynamically resolve path to yt-dlp binary
    fn resolve_yt_dlp_binary() -> Result<std::path::PathBuf, YtDlpError> {
        // use the vendored binary if it exists
        #[cfg(feature = "yt-dlp-vendored")]
        {
            let path = std::path::Path::new(env!("YTDLP_BINARY"));
            if path.exists() {
                return Ok(path.to_path_buf());
            }
        }

        // fallback to looking it up in PATH
        which::which("yt-dlp").map_err(|_| YtDlpError::BinaryNotFound("yt-dlp".to_string()))
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
        Self::new_with_cookies(binary_path, None)
    }

    /// Creates a new `YtDlp` instance with a custom binary and optional cookies path.
    ///
    /// # Arguments
    ///
    /// * `binary_path` - Path to the `yt-dlp` binary.
    /// * `cookies_path` - Optional path to a `cookies.txt` file for authenticated scraping.
    #[cfg(not(feature = "yt-dlp-vendored"))]
    pub fn new_with_cookies<P1: Into<PathBuf>, P2: Into<PathBuf>>(
        binary_path: P1,
        cookies_path: Option<P2>,
    ) -> Self {
        YtDlp {
            binary_path: binary_path.into(),
            cookies_path: cookies_path.map(Into::into),
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
    #[tracing::instrument(skip(self))]
    pub fn download_video<P: AsRef<Path> + Debug>(
        &self,
        url: &str,
        output_template: P,
    ) -> Result<(), YtDlpError> {
        let output_str = output_template.as_ref().to_str().ok_or_else(|| {
            YtDlpError::InvalidPath(output_template.as_ref().display().to_string())
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
    #[tracing::instrument(skip(self))]
    pub fn download_audio<P: AsRef<Path> + Debug>(
        &self,
        url: &str,
        output_template: P,
    ) -> Result<(), YtDlpError> {
        tracing::info!(binary_path=?self.binary_path, "yt-dlp command path");

        let output_str = output_template.as_ref().to_str().ok_or_else(|| {
            YtDlpError::InvalidPath(output_template.as_ref().display().to_string())
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
    #[tracing::instrument(skip(self))]
    pub fn download_playlist<P: AsRef<Path> + Debug>(
        &self,
        playlist_url: &str,
        output_template: P,
    ) -> Result<(), YtDlpError> {
        let output_str = output_template.as_ref().to_str().ok_or_else(|| {
            YtDlpError::InvalidPath(output_template.as_ref().display().to_string())
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
    /// ```no_run
    /// use ytdlp_bindings::YtDlp;
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let ytdlp = YtDlp::new()?;
    ///     ytdlp.download_with_options(
    ///         "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
    ///         &["--format", "bestaudio/best", "--extract-audio", "--audio-format", "mp3", "--output", "audio.%(ext)s"]
    ///     )?;
    ///     Ok(())
    ///  }
    /// ```
    #[tracing::instrument(skip(self))]
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
    #[tracing::instrument(skip(self))]
    pub fn download_auto_sub<P: AsRef<Path> + Debug>(
        &self,
        url: &str,
        output_template: P,
    ) -> Result<(), YtDlpError> {
        let output_str = output_template.as_ref().to_str().ok_or_else(|| {
            YtDlpError::InvalidPath(output_template.as_ref().display().to_string())
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
    #[tracing::instrument(skip(self))]
    pub fn download_sub<P: AsRef<Path> + Debug>(
        &self,
        url: &str,
        output_path: P,
    ) -> Result<(), YtDlpError> {
        let output_str = output_path
            .as_ref()
            .to_str()
            .ok_or_else(|| YtDlpError::InvalidPath(output_path.as_ref().display().to_string()))?;

        self.run_yt_dlp(&[
            "--write-sub",
            "--skip-download",
            "--output",
            output_str,
            url,
        ])
    }

    /// Runs the `yt-dlp` command with optional `--cookies` support.
    ///
    /// This method appends the cookies argument to the command if `cookies_path` is set.
    #[tracing::instrument(skip(self))]
    pub(crate) fn run_yt_dlp(&self, args: &[&str]) -> Result<(), YtDlpError> {
        let max_retries = 3;
        let retry_delay = std::time::Duration::from_secs(2);
        let mut attempts = 0;

        loop {
            attempts += 1;
            let result = self.run_yt_dlp_once(args);

            match result {
                Ok(()) => return Ok(()),
                Err(err) if matches!(err, YtDlpError::NonZeroExit { .. }) => {
                    tracing::warn!(
                        ?err,
                        attempts,
                        "yt-dlp failed (attempt {}/{})",
                        attempts,
                        max_retries
                    );

                    if attempts == max_retries {
                        return Err(err);
                    }

                    std::thread::sleep(retry_delay);
                }
                Err(err) => return Err(err),
            }
        }
    }

    fn run_yt_dlp_once(&self, args: &[&str]) -> Result<(), YtDlpError> {
        let mut cmd = std::process::Command::new(&self.binary_path);

        if let Some(ref cookies) = self.cookies_path {
            if !cookies.exists() {
                return Err(YtDlpError::InvalidPath(format!(
                    "Cookies file not found: {}",
                    cookies.display()
                )));
            }
            cmd.arg("--cookies").arg(cookies);
        }

        cmd.args(args);
        let output = cmd.output()?;

        if output.status.success() {
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);

            let output_msg = if !stderr.trim().is_empty() {
                stderr.into()
            } else if !stdout.trim().is_empty() {
                stdout.into()
            } else {
                "yt-dlp exited with non-zero status but produced no output.".into()
            };

            Err(YtDlpError::NonZeroExit {
                command: self.binary_path.to_string_lossy().into(),
                status: output.status.code().unwrap_or(-1),
                output: output_msg,
            })
        }
    }

    #[cfg(any(feature = "audio-processing", feature = "video-processing"))]
    #[tracing::instrument(skip(self))]
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
    use std::env;
    use std::fs;
    use std::io::Read;
    use tempfile::tempdir;

    fn set_yt_dlp_env() {
        let out_dir = env!("OUT_DIR");
        let path = format!("{}/yt-dlp", out_dir);
        unsafe {
            std::env::set_var("YTDLP_BINARY", path);
        }
    }

    #[test]
    fn test_new() {
        set_yt_dlp_env();

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
    #[ignore = "TODO: Passes locally but fails in CI"]
    fn test_download_auto_sub() {
        let ytdlp = YtDlp::new().unwrap();
        let temp_dir = env::temp_dir();
        let output_path = temp_dir.join("%(title)s.%(ext)s");
        let result =
            ytdlp.download_auto_sub("https://www.youtube.com/watch?v=p1OqRc15K3o", output_path);
        assert!(result.is_ok());
    }

    #[test]
    #[ignore = "Needs cookies.txt which is not available in CI"]
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
    #[ignore = "Only here for debugging"]
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

    #[test]
    #[ignore = "Needs cookies.txt which is not available in CI"]
    fn test_download_video() {
        let ytdlp = YtDlp::new_with_cookies(Some(PathBuf::from("")))
            .expect("Failed to create YtDlp instance");
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
    fn test_missing_cookies_file_fails_gracefully() {
        set_yt_dlp_env();

        let ytdlp =
            YtDlp::new_with_cookies(Some(PathBuf::from("/nonexistent/cookies.txt"))).unwrap();
        let output_path = std::env::temp_dir().join("dummy.%(ext)s");
        let result = ytdlp.download_auto_sub(TEST_VIDEO_URL, output_path);

        assert!(matches!(result, Err(YtDlpError::InvalidPath(_))));
    }

    #[test]
    fn test_download_invalid_url_fails() {
        set_yt_dlp_env();

        let ytdlp = YtDlp::new().unwrap();
        let output_path = std::env::temp_dir().join("invalid.%(ext)s");
        let result = ytdlp.download_video("https://www.youtube.com/watch?v=invalid", output_path);

        assert!(
            matches!(result, Err(YtDlpError::NonZeroExit { .. })),
            "Expected a failure but got: {:?}",
            result
        );
    }
}
