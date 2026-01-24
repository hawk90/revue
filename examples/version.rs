use revue::VERSION;

fn main() {
    println!("Revue version: {}", VERSION);
    println!("Git SHA: {}", revue::GIT_SHA);
    println!("Is dev build: {}", revue::is_dev_build());
}
