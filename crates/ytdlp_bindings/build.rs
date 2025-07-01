use std::env;
use std::fs::File;
use std::io::copy;
use std::path::Path;

/// The yt-dlp version based off their github releases
/// <https://github.com/yt-dlp/yt-dlp/releases>
const YTDLP_RELEASE: &str = "2025.03.31";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Determine the target OS and architecture
    let target_os = env::var("CARGO_CFG_TARGET_OS")?;
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH")?;

    let filename = match (target_os.as_str(), target_arch.as_str()) {
        ("windows", _) => "yt-dlp.exe",
        ("macos", "x86_64") => "yt-dlp_macos_legacy",
        ("macos", "aarch64") => "yt-dlp_macos",
        ("linux", "x86_64" | "x86") => "yt-dlp_linux",
        ("linux", "aarch64") => "yt-dlp_linux_aarch64",
        ("linux", "arm") => "yt-dlp_linux_armv7I",
        _ => return Err(format!("Unsupported platform: {target_os} {target_arch}").into()),
    };

    // Create an output directory for the binary
    let out_dir = env::var("OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join("yt-dlp");

    // Download the file
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=TARGET");
    let mut response = reqwest::blocking::get(format!(
        "https://github.com/yt-dlp/yt-dlp/releases/download/{YTDLP_RELEASE}/{filename}"
    ))?;
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
