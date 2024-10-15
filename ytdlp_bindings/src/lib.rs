//! ytdlp-bindings
//!
//! This crate provides a Rust interface to the yt-dlp command-line program,
//! which is used for downloading videos and subtitles from YouTube and other platforms.
//!
//! The main struct `YtDlp` offers methods to download subtitles and process VTT files.
//!
//! # Features
//!
//! - `yt-dlp-vendored`: When enabled, the crate will use a vendored version of yt-dlp.
//!   When disabled, you need to provide the path to the yt-dlp binary.
//! - `audio-processing`: Adds downloaded audio processing capabilities to YtDlp via vendored ffmpeg (v7*)
//! - `video-processing`: Adds downloaded video processing capabilities to YtDlp also via vendored ffmpeg (v7*)
//! - `vtt-processing`: Adds downloaded VTT file processing capabilities to YtDlp
//!
//! # Examples
//!
//! ```rust,no_run
//! use ytdlp_bindings::YtDlp;
//! use std::path::Path;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let ytdlp = YtDlp::new()?;
//!     let output_path = Path::new("output.vtt");
//!
//!     ytdlp.download_auto_sub("https://www.youtube.com/watch?v=dQw4w9WgXcQ", output_path)?;
//!
//!     Ok(())
//! }
//! ```

mod error;
#[cfg(any(
    feature = "audio-processing",
    feature = "video-processing",
    feature = "vtt-processing"
))]
mod processors;
mod ytldp;

pub use error::YtDlpError;
#[cfg(feature = "audio-processing")]
pub use processors::audio::AudioProcessor;
#[cfg(feature = "video-processing")]
pub use processors::video::VideoProcessor;
#[cfg(feature = "vtt-processing")]
pub use processors::vtt::VttProcessor;
#[cfg(feature = "vtt-processing")]
pub use webvtt_parser::{OwnedVtt, OwnedVttCue};
pub use ytldp::YtDlp;
