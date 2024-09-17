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
mod ytldp;

pub use error::YtDlpError;
pub use file_types::vtt::{parse_vtt_content, SubtitleEntry, VttProcessor};
pub use ytldp::YtDlp;
