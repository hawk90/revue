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
        // Development build: try to get git SHA
        let output = std::process::Command::new("git")
            .args(["rev-parse", "--short", "HEAD"])
            .output();

        let sha = output
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty());

        if let Some(sha) = sha {
            println!("cargo:rustc-env=REVUE_VERSION={}-{}", version, sha);
            println!("cargo:rustc-env=GIT_SHA={}", sha);
            println!("cargo:rustc-env=REVUE_IS_DEV=true");
        } else {
            // Fallback when git is not available (e.g., crates.io build)
            println!("cargo:rustc-env=REVUE_VERSION={}", version);
            println!("cargo:rustc-env=GIT_SHA=");
            println!("cargo:rustc-env=REVUE_IS_DEV=false");
        }
    }
}
