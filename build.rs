use std::env;

fn main() {
    // Read version from Cargo.toml
    let version = env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "unknown".to_string());

    // Check if this is a release build (for crates.io publishing)
    let is_release_build = env::var("REVUE_RELEASE").is_ok();

    if is_release_build {
        // Release build: use version from Cargo.toml as-is
        println!("cargo:rustc-env=REVUE_VERSION={}", version);
        println!("cargo:rustc-env=GIT_SHA=");
        println!("cargo:rustc-env=REVUE_IS_DEV=false");
    } else {
        // Development build: try to get both short and full git SHA
        let short = std::process::Command::new("git")
            .args(["rev-parse", "--short", "HEAD"])
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty());

        let full = std::process::Command::new("git")
            .args(["rev-parse", "HEAD"])
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty());

        match (short, full) {
            (Some(short_sha), Some(full_sha)) => {
                // VERSION includes short SHA, GIT_SHA keeps full 40-char hash
                println!("cargo:rustc-env=REVUE_VERSION={}-{}", version, short_sha);
                println!("cargo:rustc-env=GIT_SHA={}", full_sha);
                println!("cargo:rustc-env=REVUE_IS_DEV=true");
            }
            _ => {
                // Fallback when git is not available (e.g., crates.io build)
                println!("cargo:rustc-env=REVUE_VERSION={}", version);
                println!("cargo:rustc-env=GIT_SHA=");
                println!("cargo:rustc-env=REVUE_IS_DEV=false");
            }
        }
    }
}
