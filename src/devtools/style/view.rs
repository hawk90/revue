//! View/render implementations for StyleInspector

use super::core::StyleInspector;
use super::helper::RenderCtx;
use super::types::StyleCategory;
use crate::devtools::DevToolsConfig;
use crate::layout::Rect;
use crate::render::Buffer;
use crate::style::Color;
use std::collections::HashMap;

impl StyleInspector {
    /// Render style inspector content
    pub fn render_content(&self, buffer: &mut Buffer, area: Rect, config: &DevToolsConfig) {
        let mut ctx = RenderCtx::new(buffer, area.x, area.width, config);
        let mut y = area.y;
        let max_y = area.y + area.height;

        // Widget info header
        if !self.widget_type.is_empty() {
            let mut header = self.widget_type.clone();
            if let Some(ref id) = self.widget_id {
                header.push_str(&format!("#{}", id));
            }
            ctx.draw_text(y, &header, config.accent_color);
            y += 1;

            // Classes
            if !self.classes.is_empty() {
                let classes_str = self
                    .classes
                    .iter()
                    .map(|c| format!(".{}", c))
                    .collect::<Vec<_>>()
                    .join(" ");
                ctx.draw_text(y, &classes_str, config.fg_color);
                y += 1;
            }
            y += 1;
        }

        // Properties by category
        let filtered = self.filtered();
        if filtered.is_empty() {
            ctx.draw_text(y, "No styles to display", config.fg_color);
            return;
        }

        // Group by category
        let mut by_category: HashMap<StyleCategory, Vec<&super::types::ComputedProperty>> =
            HashMap::new();
        for prop in &filtered {
            let cat = StyleCategory::from_property(&prop.name);
            by_category.entry(cat).or_default().push(prop);
        }

        let mut prop_idx = 0;
        for category in StyleCategory::all() {
            if y >= max_y {
                break;
            }

            if let Some(props) = by_category.get(category) {
                let expanded = self
                    .expanded_categories
                    .get(category)
                    .copied()
                    .unwrap_or(true);
                let indicator = if expanded { "▼" } else { "▶" };
                let header = format!("{} {} ({})", indicator, category.label(), props.len());
                ctx.draw_text(y, &header, config.accent_color);
                y += 1;

                if expanded {
                    for prop in props {
                        if y >= max_y {
                            break;
                        }

                        let is_selected = self.selected == Some(prop_idx);
                        Self::render_property(&mut ctx, 2, y, prop, is_selected);
                        y += 1;
                        prop_idx += 1;
                    }
                } else {
                    prop_idx += props.len();
                }

                y += 1; // Gap between categories
            }
        }
    }

    fn render_property(
        ctx: &mut RenderCtx<'_>,
        indent: u16,
        y: u16,
        prop: &super::types::ComputedProperty,
        selected: bool,
    ) {
        let source_icon = prop.source.icon();
        let strike = if prop.overridden { "̶" } else { "" };
        let line = format!("{} {}{}: {}", source_icon, prop.name, strike, prop.value);

        let fg = if selected {
            ctx.config.bg_color
        } else if prop.overridden {
            Color::rgb(128, 128, 128)
        } else {
            ctx.config.fg_color
        };
        let bg = if selected {
            Some(ctx.config.accent_color)
        } else {
            None
        };

        let x = ctx.x + indent;
        let width = ctx.width.saturating_sub(indent);
        for (i, ch) in line.chars().enumerate() {
            if (i as u16) < width {
                if let Some(cell) = ctx.buffer.get_mut(x + i as u16, y) {
                    cell.symbol = ch;
                    cell.fg = Some(fg);
                    if let Some(b) = bg {
                        cell.bg = Some(b);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::devtools::DevToolsConfig;
    use crate::layout::Rect;

    #[test]
    fn test_render_content_empty_inspector() {
        let inspector = StyleInspector::default();
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let config = DevToolsConfig::default();

        // Should not panic
        inspector.render_content(&mut buffer, area, &config);
    }

    #[test]
    fn test_render_content_with_widget_type() {
        let mut inspector = StyleInspector::default();
        inspector.widget_type = "Button".to_string();
        inspector.widget_id = Some("my-button".to_string());

        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let config = DevToolsConfig::default();

        // Should not panic
        inspector.render_content(&mut buffer, area, &config);
    }

    #[test]
    fn test_render_content_with_classes() {
        let mut inspector = StyleInspector::default();
        inspector.widget_type = "Button".to_string();
        inspector.classes = vec!["btn".to_string(), "primary".to_string()];

        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let config = DevToolsConfig::default();

        // Should not panic
        inspector.render_content(&mut buffer, area, &config);
    }

    #[test]
    fn test_render_content_with_properties() {
        use crate::devtools::style::types::{ComputedProperty, PropertySource};

        let mut inspector = StyleInspector::default();
        inspector.widget_type = "Button".to_string();
        inspector.properties = vec![
            ComputedProperty {
                name: "color".to_string(),
                value: "red".to_string(),
                source: PropertySource::Class,
                overridden: false,
            },
            ComputedProperty {
                name: "background".to_string(),
                value: "blue".to_string(),
                source: PropertySource::Inline,
                overridden: false,
            },
        ];

        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let config = DevToolsConfig::default();

        // Should not panic
        inspector.render_content(&mut buffer, area, &config);
    }

    #[test]
    fn test_render_content_no_styles_message() {
        let mut inspector = StyleInspector::default();
        inspector.widget_type = "Button".to_string();
        // Empty properties should show "No styles to display"

        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let config = DevToolsConfig::default();

        inspector.render_content(&mut buffer, area, &config);
    }

    #[test]
    fn test_render_content_with_selected_property() {
        use crate::devtools::style::types::{ComputedProperty, PropertySource};

        let mut inspector = StyleInspector::default();
        inspector.widget_type = "Button".to_string();
        inspector.properties = vec![ComputedProperty {
            name: "color".to_string(),
            value: "red".to_string(),
            source: PropertySource::Class,
            overridden: false,
        }];
        inspector.selected = Some(0);

        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let config = DevToolsConfig::default();

        // Should not panic
        inspector.render_content(&mut buffer, area, &config);
    }

    #[test]
    fn test_render_content_with_overridden_property() {
        use crate::devtools::style::types::{ComputedProperty, PropertySource};

        let mut inspector = StyleInspector::default();
        inspector.widget_type = "Button".to_string();
        inspector.properties = vec![ComputedProperty {
            name: "color".to_string(),
            value: "red".to_string(),
            source: PropertySource::Class,
            overridden: true,
        }];

        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let config = DevToolsConfig::default();

        // Should not panic
        inspector.render_content(&mut buffer, area, &config);
    }
}
