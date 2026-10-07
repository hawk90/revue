//! Drawing the presentation: title and content slides, transitions, footer

use super::{Presentation, Slide, SlideAlign, Transition};
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::widget::theme::{DISABLED_FG, LIGHT_GRAY, SEPARATOR_COLOR};
use crate::widget::traits::{RenderContext, View};

/// The slide fill and body text a presentation uses when neither the builder
/// nor the stylesheet names one.
const SLIDE_BG: Color = Color::rgb(20, 20, 30);
const SLIDE_FG: Color = Color::WHITE;

impl Presentation {
    /// How the current transition moves the slide, from its progress
    fn slide_fx(&self, area: crate::layout::Rect, bg: Color) -> SlideFx {
        let p = self.transition_progress.clamp(0.0, 1.0);
        let rest = 1.0 - p;
        let (w, h) = (area.width as f32, area.height as f32);
        let mut fx = SlideFx {
            dx: 0,
            dy: 0,
            alpha: 1.0,
            bg,
            zoom: None,
            max_y: area.height.saturating_sub(1),
        };
        if p >= 1.0 {
            return fx;
        }
        match self.transition {
            Transition::None => {}
            Transition::Fade => fx.alpha = p,
            Transition::SlideLeft => fx.dx = -(rest * w).round() as i32,
            Transition::SlideRight => fx.dx = (rest * w).round() as i32,
            Transition::SlideUp => fx.dy = (rest * h).round() as i32,
            Transition::ZoomIn => fx.zoom = Some((p * w / 2.0, p * h / 2.0)),
        }
        fx
    }

    /// Render the title slide (before slide 0, or for an empty presentation)
    fn render_title_slide(&self, ctx: &mut RenderContext, bg: Color) {
        let area = ctx.area;
        let fx = &self.slide_fx(area, bg);
        let center_y = area.height / 2;

        // Title
        let title_y = center_y.saturating_sub(2);
        self.render_centered_text(ctx, fx, &self.title, title_y, self.accent, Modifier::BOLD);

        // Author
        if !self.author.is_empty() {
            let author_y = center_y + 1;
            self.render_centered_text(
                ctx,
                fx,
                &self.author,
                author_y,
                LIGHT_GRAY,
                Modifier::ITALIC,
            );
        }

        // Press key hint
        let hint = "Press → or Space to start";
        let hint_y = area.height.saturating_sub(2);
        self.render_centered_text(ctx, fx, hint, hint_y, DISABLED_FG, Modifier::empty());
    }

    /// Render a content slide
    fn render_content_slide(&self, ctx: &mut RenderContext, slide: &Slide) {
        let area = ctx.area;

        // Background. A per-slide `bg` is an explicit override and stays on
        // top; otherwise the stylesheet gets the fill.
        let bg = slide
            .bg
            .unwrap_or_else(|| self.bg.unwrap_or_else(|| ctx.css_background(SLIDE_BG)));
        ctx.fill_box_background(bg);
        let fx = &self.slide_fx(area, bg);

        // Title (top center)
        let title_y = 2;
        self.render_centered_text(
            ctx,
            fx,
            &slide.title,
            title_y,
            slide.title_color,
            Modifier::BOLD,
        );

        // Separator
        let sep_y = 4;
        let sep_len =
            crate::utils::display_width(&slide.title).min((area.width as usize).saturating_sub(4));
        let sep_start = (area.width as usize - sep_len) / 2;
        for i in 0..sep_len {
            let mut cell = Cell::new('─');
            cell.fg = Some(self.accent);
            fx.put(ctx, sep_start as u16 + i as u16, sep_y, cell);
        }

        // Content. The slide title and the accent each say something one rule
        // cannot; the body is the base they are a departure from.
        let content_fg = slide
            .content_color
            .unwrap_or_else(|| ctx.css_color(SLIDE_FG));
        let content_start_y = 6;
        for (i, line) in slide.content.iter().enumerate() {
            let y = content_start_y + i as u16;
            if y >= area.height.saturating_sub(3) {
                break;
            }

            match slide.align {
                SlideAlign::Left => {
                    fx.put_str(ctx, 2, y, line, area.width, |ch| {
                        Cell::new(ch).fg(content_fg)
                    });
                }
                SlideAlign::Center => {
                    self.render_centered_text(ctx, fx, line, y, content_fg, Modifier::empty());
                }
                SlideAlign::Right => {
                    let line_width = crate::utils::display_width(line);
                    let start_x = (area.width as usize).saturating_sub(line_width + 2) as u16;
                    fx.put_str(ctx, start_x, y, line, u16::MAX, |ch| {
                        Cell::new(ch).fg(content_fg)
                    });
                }
            }
        }
    }

    /// Render centered text
    fn render_centered_text(
        &self,
        ctx: &mut RenderContext,
        fx: &SlideFx,
        text: &str,
        y: u16,
        fg: Color,
        modifier: Modifier,
    ) {
        let area = ctx.area;
        let text_width = crate::utils::display_width(text);
        let start_x = ((area.width as usize).saturating_sub(text_width) / 2) as u16;
        fx.put_str(ctx, start_x, y, text, area.width, |ch| {
            let mut cell = Cell::new(ch).fg(fg);
            cell.modifier = modifier;
            cell
        });
    }

    /// Render footer (slide numbers, progress)
    fn render_footer(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let footer_y = area.height.saturating_sub(1);
        // Slides shown so far: the title slide comes before slide 0, so it
        // counts none.
        let shown = if self.on_title_slide() {
            0
        } else {
            self.current + 1
        };

        // Time left, centered; the progress bar and slide numbers draw over
        // it when the footer is too narrow for all three
        if let Some(total) = self.timer {
            let left = (total as f32 - self.elapsed).max(0.0).ceil() as u64;
            let text = format!("{:02}:{:02}", left / 60, left % 60);
            let fg = if left == 0 { Color::RED } else { DISABLED_FG };
            let start_x = area.width.saturating_sub(text.len() as u16) / 2;
            for (i, ch) in text.chars().enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(fg);
                ctx.set(start_x + i as u16, footer_y, cell);
            }
        }

        // Slide numbers
        if self.show_numbers && !self.slides.is_empty() {
            let num_str = format!("{}/{}", shown, self.slides.len());
            let start_x = area.width.saturating_sub(num_str.len() as u16 + 1);
            for (i, ch) in num_str.chars().enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(DISABLED_FG);
                ctx.set(start_x + i as u16, footer_y, cell);
            }
        }

        // Progress bar
        if self.show_progress && !self.slides.is_empty() {
            let bar_width = (area.width / 3).max(10);
            let progress = shown as f32 / self.slides.len() as f32;
            let filled = (bar_width as f32 * progress) as u16;

            for i in 0..bar_width {
                let ch = if i < filled { '━' } else { '─' };
                let mut cell = Cell::new(ch);
                cell.fg = Some(if i < filled {
                    self.accent
                } else {
                    SEPARATOR_COLOR
                });
                ctx.set(1 + i, footer_y, cell);
            }
        }
    }
}

/// Where and how a slide's cells land while a transition runs. The footer
/// and the background are drawn outside it, so only the slide moves.
struct SlideFx {
    /// Column offset (slide transitions)
    dx: i32,
    /// Row offset (slide-up transition)
    dy: i32,
    /// How far text has faded in over the background, 0.0 to 1.0
    alpha: f32,
    /// The slide background text fades from
    bg: Color,
    /// Half width and half height of the centered window a zoom shows
    zoom: Option<(f32, f32)>,
    /// First row the slide may not draw on (the footer's)
    max_y: u16,
}

impl SlideFx {
    /// Draw `text` from `(x, y)` in terminal columns, stopping before column
    /// `max_x`, with the transition applied to every cell; see
    /// [`lay_out_str`](crate::widget::traits::render_context::overlay::lay_out_str)
    fn put_str<F>(
        &self,
        ctx: &mut RenderContext,
        x: u16,
        y: u16,
        text: &str,
        max_x: u16,
        make_cell: F,
    ) -> u16
    where
        F: FnMut(char) -> Cell,
    {
        crate::widget::traits::render_context::overlay::lay_out_str(
            x,
            text,
            max_x,
            make_cell,
            |cx, cell| self.put(ctx, cx, y, cell),
        )
    }

    /// Draw a slide cell at `(x, y)` with the transition applied
    fn put(&self, ctx: &mut RenderContext, x: u16, y: u16, mut cell: Cell) {
        if self.alpha <= 0.0 {
            return;
        }
        let x = i32::from(x) + self.dx;
        let y = i32::from(y) + self.dy;
        let area = ctx.area;
        if x < 0 || y < 0 || x >= i32::from(area.width) || y >= i32::from(self.max_y) {
            return;
        }
        if let Some((half_w, half_h)) = self.zoom {
            let cx = (x as f32 + 0.5) - area.width as f32 / 2.0;
            let cy = (y as f32 + 0.5) - area.height as f32 / 2.0;
            if cx.abs() > half_w || cy.abs() > half_h {
                return;
            }
        }
        if self.alpha < 1.0 {
            cell.fg = cell
                .fg
                .map(|fg| crate::utils::color::blend(fg, self.bg, self.alpha));
        }
        ctx.set(x as u16, y as u16, cell);
    }
}

impl View for Presentation {
    crate::impl_view_meta!("Presentation");

    fn render(&self, ctx: &mut RenderContext) {
        if ctx.area.width == 0 || ctx.area.height == 0 {
            return;
        }

        // Background
        let bg = self.bg.unwrap_or_else(|| ctx.css_background(SLIDE_BG));
        ctx.fill_box_background(bg);

        // Render current slide
        if self.on_title_slide() {
            self.render_title_slide(ctx, bg);
        } else if let Some(slide) = self.slides.get(self.current) {
            self.render_content_slide(ctx, slide);
        }

        // Footer
        self.render_footer(ctx);
    }
}

// KEEP HERE: Private tests for Presentation
// Private implementation tests
// KEEP HERE - Private rendering tests (tests smoke tests for private render methods)

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    #[test]
    fn test_render_title_slide() {
        // Test private method that handles title slide rendering
        let pres = Presentation::new()
            .title("Test Title")
            .author("Test Author");

        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Just ensure render doesn't panic
        pres.render(&mut ctx);
    }

    #[test]
    fn test_render_content_slide() {
        // Test private method that handles content slide rendering
        let slide = Slide::new("Content").line("Line 1").line("Line 2");
        let mut pres = Presentation::new().slide(slide);
        pres.goto(1); // Go to the first content slide

        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Just ensure render doesn't panic
        pres.render(&mut ctx);
    }

    #[test]
    fn test_render_centered_text() {
        // Test private method for rendering centered text
        let pres = Presentation::new().slide(Slide::new("Test"));

        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Just ensure render doesn't panic
        pres.render(&mut ctx);
    }

    #[test]
    fn test_render_footer() {
        // Test private method for rendering footer
        let pres = Presentation::new()
            .slide(Slide::new("Slide 1"))
            .slide(Slide::new("Slide 2"));

        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Just ensure render doesn't panic
        pres.render(&mut ctx);
    }

    #[test]
    fn title_slide_footer_counts_no_slide_yet() {
        let footer = |pres: &Presentation| -> String {
            let mut buffer = Buffer::new(40, 10);
            let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 40, 10));
            pres.render(&mut ctx);
            (0..40).map(|x| buffer.get(x, 9).unwrap().symbol).collect()
        };
        let mut pres = Presentation::new()
            .title("Deck")
            .slide(Slide::new("One"))
            .slide(Slide::new("Two"));

        // The title slide comes before slide 0: nothing shown yet.
        let on_title = footer(&pres);
        assert!(on_title.contains("0/2"), "{on_title:?}");
        assert!(!on_title.contains('━'), "{on_title:?}");

        pres.next_slide();
        let on_first = footer(&pres);
        assert!(on_first.contains("1/2"), "{on_first:?}");
        assert!(on_first.contains('━'), "{on_first:?}");
    }
}
