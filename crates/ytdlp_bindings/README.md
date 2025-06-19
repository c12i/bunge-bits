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

## Using `cookies.txt` for Authenticated YouTube Downloads

Some YouTube videos require authentication to access. If you encounter errors like:

```txt
Sign in to confirm you're not a bot. Use --cookies-from-browser or --cookies...
```

you’ll need to pass a valid `cookies.txt` file to `yt-dlp`.

### Exporting cookies.txt using browser-cookie3

[`browser-cookie3`](https://github.com/borisbabic/browser_cookie3) is a Python tool that extracts and decrypts browser cookies directly from Chrome, Firefox or any other browser on your machine.

- Install it locally (requires Python and pip installation):

```bash
pip install browser-cookie3
```

- Export cookies for YouTube:

```bash
echo -e ".youtube.com\tTRUE\t/\tFALSE\t2147385600\tSID\t$(browser-cookie --chrome youtube.com SID)" > cookies.txt
```

Replace chrome with your browser of choice if needed.

This outputs a `cookies.txt` file in the Netscape format, compatible with `yt-dlp`.

> [!TIP]
> Use a separate Google account for scraping if you're concerned about exposing personal cookies.

### Using the cookies in your application (yt-dlp-vendored mode)

```rust
use ytdlp_bindings::YtDlp;
use std::path::PathBuf;

let ytdlp = YtDlp::new_with_cookies(Some(PathBuf::from("/app/cookies.txt")))?;
```

This will automatically inject the --cookies /app/cookies.txt flag when running yt-dlp.

Then call any of the usual methods:

```rust
ytdlp.download_audio(
    "https://www.youtube.com/watch?v=CEsTRpeOGkg",
    "/var/tmp/bunge-bits/audio/%(title)s.%(ext)s"
)?;
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
I am especially open to extending the video/ audio / vtt processing capabilities which at the moment only contain methods that are required by the bunge-bits project project.

## License

This project is licensed under the MIT License - see the [LICENSE](../../LICENSE) file for details.
