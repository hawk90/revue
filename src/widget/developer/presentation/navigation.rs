//! Moving between slides, querying the current one, and the transition/timer clock

use super::{Presentation, Slide};

impl Presentation {
    /// Go to next slide
    ///
    /// From the title slide this moves to slide 0.
    pub fn next_slide(&mut self) -> bool {
        if self.on_title_slide() && !self.slides.is_empty() {
            self.started = true;
            self.transition_progress = 0.0;
            true
        } else if self.current < self.slides.len().saturating_sub(1) {
            self.current += 1;
            self.transition_progress = 0.0;
            true
        } else {
            false
        }
    }

    /// Go to previous slide
    ///
    /// From slide 0 of a presentation with a title this returns to the
    /// title slide.
    pub fn prev(&mut self) -> bool {
        if self.current == 0 && !self.on_title_slide() && !self.title.is_empty() {
            self.started = false;
            self.transition_progress = 0.0;
            true
        } else if self.current > 0 {
            self.current -= 1;
            self.transition_progress = 0.0;
            true
        } else {
            false
        }
    }

    /// Go to specific slide
    pub fn goto(&mut self, index: usize) {
        if index < self.slides.len() {
            self.current = index;
            self.started = true;
            self.transition_progress = 0.0;
        }
    }

    /// Go to first slide
    pub fn first(&mut self) {
        self.goto(0);
    }

    /// Go to last slide
    pub fn last(&mut self) {
        self.goto(self.slides.len().saturating_sub(1));
    }

    /// Get current slide index
    pub fn current_index(&self) -> usize {
        self.current
    }

    /// Get total slides
    pub fn slide_count(&self) -> usize {
        self.slides.len()
    }

    /// Get current slide
    pub fn current_slide(&self) -> Option<&Slide> {
        self.slides.get(self.current)
    }

    /// Get speaker notes for current slide
    pub fn current_notes(&self) -> Option<&str> {
        self.current_slide().map(|s| s.notes.as_str())
    }

    /// Advance the transition animation and the timer by `dt` seconds
    pub fn tick(&mut self, dt: f32) {
        self.elapsed += dt.max(0.0);
        if self.transition_progress < 1.0 {
            self.transition_progress = (self.transition_progress + dt * 3.0).min(1.0);
        }
    }
}
