//! Style inspector for CSS debugging

use crate::layout::Rect;
use crate::render::Buffer;
use crate::style::Color;
use super::DevToolsConfig;
use std::collections::HashMap;

/// Computed CSS property
#[derive(Debug, Clone)]
pub struct ComputedProperty {
    /// Property name
    pub name: String,
    /// Property value
    pub value: String,
    /// Source (inline, class, inherited)
    pub source: PropertySource,
    /// Is overridden by higher specificity
    pub overridden: bool,
}

impl ComputedProperty {
    /// Create a new computed property
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            source: PropertySource::Computed,
            overridden: false,
        }
    }

    /// Set source
    pub fn source(mut self, source: PropertySource) -> Self {
        self.source = source;
        self
    }

    /// Mark as overridden
    pub fn overridden(mut self) -> Self {
        self.overridden = true;
        self
    }
}

/// Source of a CSS property
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PropertySource {
    /// Inline style
    Inline,
    /// From a CSS class
    Class,
    /// From widget ID selector
    Id,
    /// Inherited from parent
    Inherited,
    /// Default/computed value
    #[default]
    Computed,
    /// From theme
    Theme,
}

impl PropertySource {
    /// Get display label
    pub fn label(&self) -> &'static str {
        match self {
            Self::Inline => "inline",
            Self::Class => "class",
            Self::Id => "id",
            Self::Inherited => "inherited",
            Self::Computed => "computed",
            Self::Theme => "theme",
        }
    }

    /// Get icon
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Inline => "•",
            Self::Class => ".",
            Self::Id => "#",
            Self::Inherited => "↑",
            Self::Computed => "○",
            Self::Theme => "◆",
        }
    }
}

/// Style category for grouping properties
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StyleCategory {
    /// Layout properties (width, height, margin, padding)
    Layout,
    /// Typography (font, text)
    Typography,
    /// Colors and backgrounds
    Colors,
    /// Borders
    Border,
    /// Effects (shadows, opacity)
    Effects,
    /// Other
    Other,
}

impl StyleCategory {
    /// Get category label
    pub fn label(&self) -> &'static str {
        match self {
            Self::Layout => "Layout",
            Self::Typography => "Typography",
            Self::Colors => "Colors",
            Self::Border => "Border",
            Self::Effects => "Effects",
            Self::Other => "Other",
        }
    }

    /// All categories
    pub fn all() -> &'static [StyleCategory] {
        &[
            Self::Layout,
            Self::Typography,
            Self::Colors,
            Self::Border,
            Self::Effects,
            Self::Other,
        ]
    }

    /// Categorize a property name
    pub fn from_property(name: &str) -> Self {
        match name {
            // Border must come before layout (border-width vs width)
            n if n.starts_with("border") => Self::Border,
            n if n.starts_with("margin") || n.starts_with("padding")
                || n.contains("width") || n.contains("height")
                || n.starts_with("flex") || n.starts_with("grid")
                || n == "display" || n == "position" => Self::Layout,
            n if n.starts_with("font") || n.starts_with("text")
                || n == "line-height" || n == "letter-spacing" => Self::Typography,
            n if n.contains("color") || n.contains("background") => Self::Colors,
            n if n.contains("shadow") || n == "opacity"
                || n.starts_with("transform") => Self::Effects,
            _ => Self::Other,
        }
    }
}

/// Style inspector for viewing computed styles
#[derive(Debug, Default)]
pub struct StyleInspector {
    /// Properties for current selection
    properties: Vec<ComputedProperty>,
    /// Applied classes
    classes: Vec<String>,
    /// Widget ID
    widget_id: Option<String>,
    /// Widget type
    widget_type: String,
    /// Selected property index
    selected: Option<usize>,
    /// Scroll offset
    scroll: usize,
    /// Show inherited properties
    show_inherited: bool,
    /// Show overridden properties
    show_overridden: bool,
    /// Filter by category
    category_filter: Option<StyleCategory>,
    /// Expanded categories
    expanded_categories: HashMap<StyleCategory, bool>,
}

impl StyleInspector {
    /// Create new style inspector
    pub fn new() -> Self {
        let mut expanded = HashMap::new();
        for cat in StyleCategory::all() {
            expanded.insert(*cat, true);
        }

        Self {
            show_inherited: true,
            show_overridden: false,
            expanded_categories: expanded,
            ..Default::default()
        }
    }

    /// Clear current selection
    pub fn clear(&mut self) {
        self.properties.clear();
        self.classes.clear();
        self.widget_id = None;
        self.widget_type.clear();
        self.selected = None;
    }

    /// Set widget info
    pub fn set_widget(&mut self, type_name: impl Into<String>, id: Option<String>) {
        self.widget_type = type_name.into();
        self.widget_id = id;
    }

    /// Add a class
    pub fn add_class(&mut self, class: impl Into<String>) {
        self.classes.push(class.into());
    }

    /// Add a property
    pub fn add_property(&mut self, prop: ComputedProperty) {
        self.properties.push(prop);
    }

    /// Set properties
    pub fn set_properties(&mut self, props: Vec<ComputedProperty>) {
        self.properties = props;
    }

    /// Toggle show inherited
    pub fn toggle_inherited(&mut self) {
        self.show_inherited = !self.show_inherited;
    }

    /// Toggle show overridden
    pub fn toggle_overridden(&mut self) {
        self.show_overridden = !self.show_overridden;
    }

    /// Set category filter
    pub fn set_category_filter(&mut self, category: Option<StyleCategory>) {
        self.category_filter = category;
    }

    /// Toggle category expansion
    pub fn toggle_category(&mut self, category: StyleCategory) {
        let expanded = self.expanded_categories.entry(category).or_insert(true);
        *expanded = !*expanded;
    }

    /// Get filtered properties
    fn filtered(&self) -> Vec<&ComputedProperty> {
        self.properties.iter()
            .filter(|p| {
                if !self.show_inherited && p.source == PropertySource::Inherited {
                    return false;
                }
                if !self.show_overridden && p.overridden {
                    return false;
                }
                if let Some(cat) = self.category_filter {
                    if StyleCategory::from_property(&p.name) != cat {
                        return false;
                    }
                }
                true
            })
            .collect()
    }

    /// Select next property
    pub fn select_next(&mut self) {
        let count = self.filtered().len();
        if count == 0 {
            return;
        }

        self.selected = Some(match self.selected {
            Some(i) => (i + 1).min(count - 1),
            None => 0,
        });
    }

    /// Select previous property
    pub fn select_prev(&mut self) {
        let count = self.filtered().len();
        if count == 0 {
            return;
        }

        self.selected = Some(match self.selected {
            Some(i) => i.saturating_sub(1),
            None => 0,
        });
    }

    /// Render style inspector content
    pub fn render_content(&self, buffer: &mut Buffer, area: Rect, config: &DevToolsConfig) {
        let mut y = area.y;
        let max_y = area.y + area.height;

        // Widget info header
        if !self.widget_type.is_empty() {
            let mut header = self.widget_type.clone();
            if let Some(ref id) = self.widget_id {
                header.push_str(&format!("#{}", id));
            }
            self.draw_text(buffer, area.x, y, &header, config.accent_color);
            y += 1;

            // Classes
            if !self.classes.is_empty() {
                let classes_str = self.classes.iter()
                    .map(|c| format!(".{}", c))
                    .collect::<Vec<_>>()
                    .join(" ");
                self.draw_text(buffer, area.x, y, &classes_str, config.fg_color);
                y += 1;
            }
            y += 1;
        }

        // Properties by category
        let filtered = self.filtered();
        if filtered.is_empty() {
            self.draw_text(buffer, area.x, y, "No styles to display", config.fg_color);
            return;
        }

        // Group by category
        let mut by_category: HashMap<StyleCategory, Vec<&ComputedProperty>> = HashMap::new();
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
                let expanded = self.expanded_categories.get(category).copied().unwrap_or(true);
                let indicator = if expanded { "▼" } else { "▶" };
                let header = format!("{} {} ({})", indicator, category.label(), props.len());
                self.draw_text(buffer, area.x, y, &header, config.accent_color);
                y += 1;

                if expanded {
                    for prop in props {
                        if y >= max_y {
                            break;
                        }

                        let is_selected = self.selected == Some(prop_idx);
                        self.render_property(buffer, area.x + 2, y, area.width - 2, prop, is_selected, config);
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
        &self,
        buffer: &mut Buffer,
        x: u16,
        y: u16,
        width: u16,
        prop: &ComputedProperty,
        selected: bool,
        config: &DevToolsConfig,
    ) {
        let source_icon = prop.source.icon();
        let strike = if prop.overridden { "̶" } else { "" };
        let line = format!("{} {}{}: {}", source_icon, prop.name, strike, prop.value);

        let fg = if selected {
            config.bg_color
        } else if prop.overridden {
            Color::rgb(128, 128, 128)
        } else {
            config.fg_color
        };
        let bg = if selected { Some(config.accent_color) } else { None };

        for (i, ch) in line.chars().enumerate() {
            if (i as u16) < width {
                if let Some(cell) = buffer.get_mut(x + i as u16, y) {
                    cell.symbol = ch;
                    cell.fg = Some(fg);
                    if let Some(b) = bg {
                        cell.bg = Some(b);
                    }
                }
            }
        }
    }

    fn draw_text(&self, buffer: &mut Buffer, x: u16, y: u16, text: &str, color: Color) {
        for (i, ch) in text.chars().enumerate() {
            if let Some(cell) = buffer.get_mut(x + i as u16, y) {
                cell.symbol = ch;
                cell.fg = Some(color);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_computed_property() {
        let prop = ComputedProperty::new("color", "red")
            .source(PropertySource::Inline);

        assert_eq!(prop.name, "color");
        assert_eq!(prop.value, "red");
        assert_eq!(prop.source, PropertySource::Inline);
    }

    #[test]
    fn test_property_source_label() {
        assert_eq!(PropertySource::Inline.label(), "inline");
        assert_eq!(PropertySource::Class.label(), "class");
        assert_eq!(PropertySource::Inherited.label(), "inherited");
    }

    #[test]
    fn test_style_category_from_property() {
        assert_eq!(StyleCategory::from_property("margin-left"), StyleCategory::Layout);
        assert_eq!(StyleCategory::from_property("font-size"), StyleCategory::Typography);
        assert_eq!(StyleCategory::from_property("background-color"), StyleCategory::Colors);
        assert_eq!(StyleCategory::from_property("border-width"), StyleCategory::Border);
        assert_eq!(StyleCategory::from_property("box-shadow"), StyleCategory::Effects);
        assert_eq!(StyleCategory::from_property("custom"), StyleCategory::Other);
    }

    #[test]
    fn test_style_inspector_add_property() {
        let mut inspector = StyleInspector::new();
        inspector.add_property(ComputedProperty::new("color", "blue"));
        inspector.add_property(ComputedProperty::new("font-size", "14px"));

        assert_eq!(inspector.properties.len(), 2);
    }

    #[test]
    fn test_style_inspector_filter() {
        let mut inspector = StyleInspector::new();
        inspector.add_property(
            ComputedProperty::new("color", "blue").source(PropertySource::Inline)
        );
        inspector.add_property(
            ComputedProperty::new("font-size", "14px").source(PropertySource::Inherited)
        );

        // With inherited shown
        assert_eq!(inspector.filtered().len(), 2);

        // Without inherited
        inspector.toggle_inherited();
        assert_eq!(inspector.filtered().len(), 1);
    }

    #[test]
    fn test_style_inspector_widget_info() {
        let mut inspector = StyleInspector::new();
        inspector.set_widget("Button", Some("submit".to_string()));
        inspector.add_class("primary");
        inspector.add_class("large");

        assert_eq!(inspector.widget_type, "Button");
        assert_eq!(inspector.widget_id, Some("submit".to_string()));
        assert_eq!(inspector.classes.len(), 2);
    }
}
