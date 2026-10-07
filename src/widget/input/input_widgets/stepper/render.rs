//! Drawing horizontal and vertical steppers, with the status icons and colors

use super::{Step, StepStatus, Stepper, StepperOrientation, StepperStyle};
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::utils::unicode::truncate_with_ellipsis;
use crate::widget::theme::{DISABLED_FG, SUBTLE_GRAY};
use crate::widget::traits::{RenderContext, View};

impl StepStatus {
    fn icon(&self) -> char {
        match self {
            StepStatus::Pending => '○',
            StepStatus::Active => '●',
            StepStatus::Completed => '✓',
            StepStatus::Error => '✗',
            StepStatus::Skipped => '⊘',
        }
    }
}

impl Step {
    /// Get display icon
    fn display_icon(&self) -> char {
        self.icon.unwrap_or_else(|| self.status.icon())
    }
}

impl Stepper {
    /// Get color for step.
    ///
    /// Steps not yet reached take `color`; active, completed and error keep
    /// theirs - those are status, and a rule cannot address them separately.
    fn step_color(&self, ctx: &RenderContext, step: &Step) -> Color {
        match step.status {
            StepStatus::Active => self.active_color,
            StepStatus::Completed => self.completed_color,
            StepStatus::Error => self.error_color,
            StepStatus::Pending | StepStatus::Skipped => self
                .pending_color
                .unwrap_or_else(|| ctx.css_color(DISABLED_FG)),
        }
    }
}

impl View for Stepper {
    crate::impl_view_meta!("Stepper", focusable);

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 3 || area.height < 1 || self.steps.is_empty() {
            return;
        }

        match self.orientation {
            StepperOrientation::Horizontal => self.render_horizontal(ctx),
            StepperOrientation::Vertical => self.render_vertical(ctx),
        }
    }
}

impl Stepper {
    fn render_horizontal(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let step_count = self.steps.len();
        let available_width = area.width as usize;

        // Calculate spacing
        let step_width = available_width / step_count.max(1);

        let y: u16 = 0;

        for (i, step) in self.steps.iter().enumerate() {
            let x = (i * step_width) as u16;
            let color = self.step_color(ctx, step);

            // Step indicator
            match self.style {
                StepperStyle::Numbered => {
                    let num = format!("{}", i + 1);
                    for (j, ch) in num.chars().enumerate() {
                        let mut cell = Cell::new(ch);
                        cell.fg = Some(color);
                        if step.status == StepStatus::Active {
                            cell.modifier |= Modifier::BOLD;
                        }
                        ctx.set(x + j as u16, y, cell);
                    }
                }
                _ => {
                    let mut cell = Cell::new(step.display_icon());
                    cell.fg = Some(color);
                    if step.status == StepStatus::Active {
                        cell.modifier |= Modifier::BOLD;
                    }
                    ctx.set(x, y, cell);
                }
            }

            // Connector (except last)
            if matches!(self.style, StepperStyle::Connected | StepperStyle::Progress)
                && i < step_count - 1
            {
                let connector_start = x + 2;
                let connector_end = ((i + 1) * step_width) as u16;

                for cx in connector_start..connector_end {
                    let ch = if matches!(self.style, StepperStyle::Progress)
                        && step.status == StepStatus::Completed
                    {
                        '━'
                    } else {
                        '─'
                    };
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(if step.status == StepStatus::Completed {
                        self.completed_color
                    } else {
                        self.connector_color
                    });
                    ctx.set(cx, y, cell);
                }
            }

            // Title (below indicator)
            // Measured in columns: slicing the title at a byte count would
            // split (and panic on) a multibyte glyph.
            if y + 1 < area.height {
                let title = truncate_with_ellipsis(&step.title, step_width.saturating_sub(1));
                ctx.put_str_with(x, y + 1, &title, area.width, |ch| {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(color);
                    if step.status == StepStatus::Active {
                        cell.modifier |= Modifier::BOLD;
                    }
                    cell
                });
            }

            // Description (if enabled and space available)
            if self.show_descriptions && y + 2 < area.height {
                if let Some(ref desc) = step.description {
                    let desc_str = truncate_with_ellipsis(desc, step_width.saturating_sub(1));
                    ctx.put_str_with(x, y + 2, &desc_str, area.width, |ch| {
                        Cell::new(ch).fg(SUBTLE_GRAY)
                    });
                }
            }
        }
    }

    fn render_vertical(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let mut y: u16 = 0;

        for (i, step) in self.steps.iter().enumerate() {
            if y >= area.height {
                break;
            }

            let color = self.step_color(ctx, step);
            let x: u16 = 0;

            // Step indicator
            let indicator = if self.show_numbers {
                format!("{}", i + 1)
            } else {
                step.display_icon().to_string()
            };

            for (j, ch) in indicator.chars().enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(color);
                if step.status == StepStatus::Active {
                    cell.modifier |= Modifier::BOLD;
                }
                ctx.set(x + j as u16, y, cell);
            }

            // Title
            let title_x = x + 3;
            ctx.put_str_with(title_x, y, &step.title, area.width, |ch| {
                let mut cell = Cell::new(ch);
                cell.fg = Some(color);
                if step.status == StepStatus::Active {
                    cell.modifier |= Modifier::BOLD;
                }
                cell
            });

            y += 1;

            // Description
            if self.show_descriptions {
                if let Some(ref desc) = step.description {
                    if y < area.height {
                        let desc_x = x + 3;
                        ctx.put_str_with(desc_x, y, desc, area.width, |ch| {
                            Cell::new(ch).fg(SUBTLE_GRAY)
                        });
                        y += 1;
                    }
                }
            }

            // Connector (except last)
            if matches!(self.style, StepperStyle::Connected)
                && i < self.steps.len() - 1
                && y < area.height
            {
                let mut cell = Cell::new('│');
                cell.fg = Some(if step.status == StepStatus::Completed {
                    self.completed_color
                } else {
                    self.connector_color
                });
                ctx.set(x, y, cell);
                y += 1;
            }
        }
    }
}
