# ytdlp_bindings

Rust bindings for [`yt-dlp`](https://github.com/yt-dlp/yt-dlp), a YouTube video and audio downloader.

# Features

The following features are enabled by default:

- `yt-dlp-vendored`: When enabled, the crate will use a vendored version of yt-dlp.
  When disabled, you need to provide the path to the yt-dlp binary when creating an instance of YtDlp.
- `audio-processing`: Adds downloaded audio processing capabilities to YtDlp via vendored ffmpeg (v7\*)
- `video-processing`: Adds downloaded video processing capabilities to YtDlp also via vendored ffmpeg (v7\*)
- `vtt-processing`: Adds downloaded web VTT (Video Text Tracks) file processing capabilities to YtDlp

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ytdlp-bindings = { git="https://github.com/c12i/bunge-bits", package = "ytdlp_bindings" }
```

By default, this crate uses a vendored `yt-dlp` binary. If you want to use your system's yt-dlp installation, disable the default features:

```toml
[dependencies]
ytdlp-bindings = { git="https://github.com/c12i/bunge-bits", package = "ytdlp_bindings", default-features = false }
```

## Usage

### Downloading a Video

```rust
use ytdlp_bindings::YtDlp;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ytdlp = YtDlp::new()?;
    ytdlp.download_video(
        "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
        Path::new("video.%(ext)s")
    )?;
    Ok(())
}
```

### Downloading a Playlist

```rust
use ytdlp_bindings::YtDlp;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ytdlp = YtDlp::new()?;
    ytdlp.download_playlist(
        "https://www.youtube.com/playlist?list=PLv3TTBr1W_9tppikBxAE_G6qjWdBljBHJ",
        Path::new("playlist/%(playlist_index)s-%(title)s.%(ext)s")
    )?;
    Ok(())
}
```

### Processing Subtitles

```rust
use ytdlp_bindings::{YtDlp, VttProcessor};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ytdlp = YtDlp::new()?;

    // Download subtitles
    ytdlp.download_auto_sub(
        "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
        Path::new("subtitles.vtt")
    )?;

    // Process subtitles
    let subtitles = ytdlp.process_vtt_file("subtitles.vtt")?;
    for entry in subtitles {
        println!("{:?}", entry);
    }

    Ok(())
}
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
I am especially open to extending the video/ audio / vtt processing capabilities which at the moment only contain methods that are required by the bunge-bits project project.

## License

This project is licensed under the MIT License - see the [LICENSE](../../LICENSE) file for details.
