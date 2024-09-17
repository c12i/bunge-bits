use std::env;
use std::fs::File;
use std::io::copy;
use std::path::Path;

/// The yt-dlp version based off their github releases
/// <https://github.com/yt-dlp/yt-dlp/releases>
const YTDLP_RELEASE: &str = "2024.08.06";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Determine the target OS and architecture
    let target_os = env::var("CARGO_CFG_TARGET_OS")?;
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH")?;

    // Determine the appropriate filename based on the target platform
    let filename = match (target_os.as_str(), target_arch.as_str()) {
        ("windows", _) => "yt-dlp.exe",
        ("macos", "x86_64") => "yt-dlp_macos_legacy",
        ("macos", "aarch64") => "yt-dlp_macos",
        ("linux", "x86_64") => "yt-dlp_linux",
        ("linux", "aarch64") => "yt-dlp_linux_aarch64",
        ("linux", "armv7I") => "yt-dlp_linux_armv7I",
        _ => return Err(format!("Unsupported platform: {} {}", target_os, target_arch).into()),
    };

    // Construct the download URL
    let url =
        format!("https://github.com/yt-dlp/yt-dlp/releases/download/{YTDLP_RELEASE}/{filename}");
    println!("Download URL: {}", url);

    // Create an output directory for the binary
    let out_dir = env::var("OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join("yt-dlp");

    // Download the file
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=TARGET");

    let mut response = reqwest::blocking::get(&url)?;
    let mut dest = File::create(&dest_path)?;
    copy(&mut response, &mut dest)?;

    // Make the file executable on Unix-like systems
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = dest_path.metadata()?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&dest_path, perms)?;
        println!("Set execute permissions on the binary");
    }

    // Tell Cargo where to find the downloaded binary
    println!("cargo:rustc-env=YTDLP_BINARY={}", dest_path.display());

    Ok(())
}
