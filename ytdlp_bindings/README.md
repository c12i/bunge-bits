# ytdlp_bindings

Rust bindings for [`yt-dlp`](https://github.com/yt-dlp/yt-dlp), a YouTube video and audio downloader.

## Features

- Download videos and playlists from YouTube and other supported platforms
- Download audio from YouTube or other supported platforms
- Download and process subtitles
- Vendored yt-dlp binary (optional)
- Video processing capabilities (Work In Progress)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ytdlp-bindings = { git="https://gitub.com/c12i/bunge-bits", package = "ytdlp_bindings" }
```

By default, this crate uses a vendored `yt-dlp binary`. If you want to use your system's yt-dlp installation, disable the default features:

```toml
[dependencies]
ytdlp-bindings = { git="https://gitub.com/c12i/bunge-bits", package = "ytdlp_bindings", default-features = false }
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
        println!("{} -> {}: {}", entry.start_time, entry.end_time, entry.text);
    }

    Ok(())
}
```

## Note on Video Processing

The video processing capabilities (such as metadata extraction and format conversion) are currently a Work In Progress. These features may be incomplete or subject to significant changes in future versions.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.