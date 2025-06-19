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

Some YouTube videos (e.g. livestreams, age-restricted, or member-only) require authentication. To download them using yt-dlp, you need to provide a valid cookies.txt file.

> [!WARNING]
> Downloading videos while authenticated carries risk. Use a throwaway account and moderate your request rate.

### Recommended Approach: Use a Throwaway Account + Browser Extension

This method gives you the most stable cookies and avoids common rotation issues.

1. Install trusted extension

- Chrome/ Chromium based browsers: [Get cookies.txt LOCALLY](https://chrome.google.com/webstore/detail/get-cookiestxt-locally/jgbbilmfbammlbbhmmgaagdkbkepnijn)
- Firefox: [cookies.txt](https://addons.mozilla.org/en-US/firefox/addon/cookies-txt/)

> [!WARNING]
> Avoid the extension called “Get cookies.txt” (without “LOCALLY”) — it was flagged as malware and removed.

2. Create a throwaway Google account - This avoids risking your main account.

3. Log in via a clean incognito/private session - Open a private/incognito window, log in to your throwaway YouTube account and only open

```
https://www.youtube.com/robots.txt
```

This helps lock your session cookies without triggering rotation by YouTube’s frontend.

4. Export the cookies

While still on the `robots.txt` page, click the extension icon and export cookies for `youtube.com`.
Save the file as cookies.txt.

Your file should look something like this

```
# Netscape HTTP Cookie File
.youtube.com	TRUE	/	FALSE	2147385600	SID	...
.youtube.com	TRUE	/	FALSE	2147385600	HSID	...
...
```

5. Close the incognito window - To prevent further cookie rotation

you’ll need to pass a valid `cookies.txt` file to `yt-dlp`.

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

6. Save this `cookies.txt` file at the root of the repo (It's gitignored, don't worry) for use with `ytdlp-bindings` when initializing with `YtDlp::new_with_cookies`

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
I am especially open to extending the video/ audio / vtt processing capabilities which at the moment only contain methods that are required by the bunge-bits project project.

## License

This project is licensed under the MIT License - see the [LICENSE](../../LICENSE) file for details.
