use std::env;
use std::process::Command;

fn main() {
    // Read version from Cargo.toml
    let version = env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "unknown".to_string());

    // Always append SHA for development builds
    let is_release_build = env::var("REVUE_RELEASE").is_ok();

    if !is_release_build {
        let output = Command::new("git")
            .args(["rev-parse", "--short", "HEAD"])
            .output();

        if let Ok(sha) = output {
            if let Ok(sha_str) = String::from_utf8(sha.stdout) {
                let sha = sha_str.trim();
                if !sha.is_empty() {
                    println!("cargo:rustc-env=REVUE_VERSION={}-{}", version, sha);
                    println!("cargo:rustc-env=GIT_SHA={}", sha);
                    println!("cargo:rustc-env=REVUE_IS_DEV=true");
                }
            }
        }
    } else {
        // Release build: use version from Cargo.toml as-is
        println!("cargo:rustc-env=REVUE_VERSION={}", version);
        println!("cargo:rustc-env=GIT_SHA=");
        println!("cargo:rustc-env=REVUE_IS_DEV=false");
    }
}
