//! Presentation Mode widget for terminal slideshows
//!
//! Create beautiful terminal-based presentations with slides,
//! transitions, and speaker notes.

mod navigation;
mod render;
mod slide;
mod types;

pub use slide::Slide;
pub use types::{SlideAlign, Transition};

use crate::style::Color;
use crate::widget::traits::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};

/// Presentation widget
///
/// # Example
///
/// ```rust,ignore
/// use revue::prelude::*;
///
/// let pres = Presentation::new()
///     .title("My Presentation")
///     .slide(Slide::new("Introduction")
///         .bullet("First point")
///         .bullet("Second point"))
///     .slide(Slide::new("Code Example")
///         .code("fn main() {\n    println!(\"Hello!\");\n}"));
///
/// // Navigate
/// pres.next_slide();
/// pres.prev();
/// ```
#[derive(Clone)]
pub struct Presentation {
    /// Presentation title
    title: String,
    /// Author name
    author: String,
    /// All slides
    slides: Vec<Slide>,
    /// Current slide index
    current: usize,
    /// Whether the deck has moved past the title slide. A presentation with
    /// a title opens on its title slide, which comes before slide 0.
    started: bool,
    /// Transition effect
    transition: Transition,
    /// Transition progress (0.0 to 1.0)
    transition_progress: f32,
    /// Show slide numbers
    show_numbers: bool,
    /// Show progress bar
    show_progress: bool,
    /// Timer (seconds)
    timer: Option<u64>,
    /// Seconds counted by `tick`, for the timer
    elapsed: f32,
    /// Background color
    /// The color the builder named, if it named one - see #656.
    bg: Option<Color>,
    /// Accent color
    accent: Color,
    /// Widget properties
    props: WidgetProps,
}

impl Presentation {
    /// Create a new presentation
    pub fn new() -> Self {
        Self {
            title: String::new(),
            author: String::new(),
            slides: Vec::new(),
            current: 0,
            started: false,
            transition: Transition::None,
            transition_progress: 1.0,
            show_numbers: true,
            show_progress: true,
            timer: None,
            elapsed: 0.0,
            bg: None,
            accent: Color::CYAN,
            props: WidgetProps::new(),
        }
    }

    /// Set presentation title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Set author
    pub fn author(mut self, author: impl Into<String>) -> Self {
        self.author = author.into();
        self
    }

    /// Add a slide
    pub fn slide(mut self, slide: Slide) -> Self {
        self.slides.push(slide);
        self
    }

    /// Add multiple slides
    pub fn slides(mut self, slides: Vec<Slide>) -> Self {
        self.slides.extend(slides);
        self
    }

    /// Set transition effect
    ///
    /// It plays each time the slide changes, advanced by
    /// [`tick`](Self::tick); the footer stays still.
    pub fn transition(mut self, transition: Transition) -> Self {
        self.transition = transition;
        self
    }

    /// Show/hide slide numbers
    pub fn numbers(mut self, show: bool) -> Self {
        self.show_numbers = show;
        self
    }

    /// Show/hide progress bar
    pub fn progress(mut self, show: bool) -> Self {
        self.show_progress = show;
        self
    }

    /// Set background color
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    /// Set accent color
    pub fn accent(mut self, color: Color) -> Self {
        self.accent = color;
        self
    }

    /// Set timer (in seconds)
    ///
    /// The footer counts the time left down as `MM:SS`, advanced by
    /// [`tick`](Self::tick).
    pub fn timer(mut self, seconds: u64) -> Self {
        self.timer = Some(seconds);
        self
    }

    /// Whether the title slide is showing: always for an empty deck, and
    /// before the first move when the presentation has a title
    fn on_title_slide(&self) -> bool {
        self.slides.is_empty() || !self.title.is_empty() && !self.started
    }
}

impl Default for Presentation {
    fn default() -> Self {
        Self::new()
    }
}

impl_styled_view!(Presentation);
impl_props_builders!(Presentation);

/// Create a new presentation
pub fn presentation() -> Presentation {
    Presentation::new()
}

/// Create a slide
pub fn slide(title: impl Into<String>) -> Slide {
    Slide::new(title)
}
