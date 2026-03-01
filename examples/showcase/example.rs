//! Example struct for individual widget demonstrations

use revue::prelude::*;

pub struct Example {
    pub title: &'static str,
    pub description: &'static str,
    pub widget: Box<dyn View>,
}

impl Example {
    pub fn new(
        title: &'static str,
        description: &'static str,
        widget: impl View + 'static,
    ) -> Self {
        Self {
            title,
            description,
            widget: Box::new(widget),
        }
    }
}
