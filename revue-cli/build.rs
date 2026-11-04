//! Rebuilds the CLI when a file is added to or removed from a project
//! template. `include_dir!` embeds `templates/` through `include_bytes!`, so
//! cargo sees edits to the files it found, but on stable Rust nothing tells
//! cargo to look for new ones.

fn main() {
    println!("cargo:rerun-if-changed=templates");
}
