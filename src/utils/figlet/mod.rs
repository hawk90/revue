//! Figlet-style ASCII art text rendering

mod api;
mod banner;
mod block;
mod mini;
mod slant;
mod small;
#[cfg(test)]
mod tests;
mod types;

pub use api::{figlet, figlet_lines, figlet_with_font, font_height};
pub use types::FigletFont;
