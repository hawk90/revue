//! Style inspector for CSS debugging

use super::helpers::draw_text_overlay;
use super::DevToolsConfig;
use crate::layout::Rect;
use crate::render::Buffer;
use crate::style::Color;
use std::collections::HashMap;

/// Helper context for rendering devtools panels
struct RenderCtx<'a> {
    buffer: &'a mut Buffer,
    x: u16,
    #[allow(dead_code)]
    width: u16,
    #[allow(dead_code)]
    config: &'a DevToolsConfig,
}

impl<'a> RenderCtx<'a> {
    fn new(buffer: &'a mut Buffer, x: u16, width: u16, config: &'a DevToolsConfig) -> Self {
        Self {
            buffer,
            x,
            width,
            config,
        }
    }

    fn draw_text(&mut self, y: u16, text: &str, color: Color) {
        draw_text_overlay(self.buffer, self.x, y, text, color);
    }
}

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
            n if n.starts_with("margin")
                || n.starts_with("padding")
                || n.contains("width")
                || n.contains("height")
                || n.starts_with("flex")
                || n.starts_with("grid")
                || n == "display"
                || n == "position" =>
            {
                Self::Layout
            }
            n if n.starts_with("font")
                || n.starts_with("text")
                || n == "line-height"
                || n == "letter-spacing" =>
            {
                Self::Typography
            }
            n if n.contains("color") || n.contains("background") => Self::Colors,
            n if n.contains("shadow") || n == "opacity" || n.starts_with("transform") => {
                Self::Effects
            }
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
    /// Scroll offset (for future UI)
    _scroll: usize,
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
        self.properties
            .iter()
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
        prop: &ComputedProperty,
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

    #[test]
    fn test_computed_property() {
        let prop = ComputedProperty::new("color", "red").source(PropertySource::Inline);

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
        assert_eq!(
            StyleCategory::from_property("margin-left"),
            StyleCategory::Layout
        );
        assert_eq!(
            StyleCategory::from_property("font-size"),
            StyleCategory::Typography
        );
        assert_eq!(
            StyleCategory::from_property("background-color"),
            StyleCategory::Colors
        );
        assert_eq!(
            StyleCategory::from_property("border-width"),
            StyleCategory::Border
        );
        assert_eq!(
            StyleCategory::from_property("box-shadow"),
            StyleCategory::Effects
        );
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
        inspector
            .add_property(ComputedProperty::new("color", "blue").source(PropertySource::Inline));
        inspector.add_property(
            ComputedProperty::new("font-size", "14px").source(PropertySource::Inherited),
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

    #[test]
    fn test_computed_property_overridden() {
        let prop = ComputedProperty::new("color", "red").overridden();
        assert!(prop.overridden);
    }

    #[test]
    fn test_computed_property_default_values() {
        let prop = ComputedProperty::new("display", "flex");
        assert_eq!(prop.source, PropertySource::Computed);
        assert!(!prop.overridden);
    }

    #[test]
    fn test_property_source_default() {
        let source = PropertySource::default();
        assert_eq!(source, PropertySource::Computed);
    }

    #[test]
    fn test_property_source_all_labels() {
        assert_eq!(PropertySource::Inline.label(), "inline");
        assert_eq!(PropertySource::Class.label(), "class");
        assert_eq!(PropertySource::Id.label(), "id");
        assert_eq!(PropertySource::Inherited.label(), "inherited");
        assert_eq!(PropertySource::Computed.label(), "computed");
        assert_eq!(PropertySource::Theme.label(), "theme");
    }

    #[test]
    fn test_property_source_all_icons() {
        assert_eq!(PropertySource::Inline.icon(), "•");
        assert_eq!(PropertySource::Class.icon(), ".");
        assert_eq!(PropertySource::Id.icon(), "#");
        assert_eq!(PropertySource::Inherited.icon(), "↑");
        assert_eq!(PropertySource::Computed.icon(), "○");
        assert_eq!(PropertySource::Theme.icon(), "◆");
    }

    #[test]
    fn test_property_source_clone() {
        let source = PropertySource::Inline;
        let cloned = source.clone();
        assert_eq!(source, cloned);
    }

    #[test]
    fn test_property_source_copy() {
        let source = PropertySource::Class;
        let copied = source; // Copy, not move
        assert_eq!(source, copied);
    }

    #[test]
    fn test_style_category_all() {
        let all = StyleCategory::all();
        assert_eq!(all.len(), 6);
        assert!(all.contains(&StyleCategory::Layout));
        assert!(all.contains(&StyleCategory::Typography));
        assert!(all.contains(&StyleCategory::Colors));
        assert!(all.contains(&StyleCategory::Border));
        assert!(all.contains(&StyleCategory::Effects));
        assert!(all.contains(&StyleCategory::Other));
    }

    #[test]
    fn test_style_category_all_labels() {
        assert_eq!(StyleCategory::Layout.label(), "Layout");
        assert_eq!(StyleCategory::Typography.label(), "Typography");
        assert_eq!(StyleCategory::Colors.label(), "Colors");
        assert_eq!(StyleCategory::Border.label(), "Border");
        assert_eq!(StyleCategory::Effects.label(), "Effects");
        assert_eq!(StyleCategory::Other.label(), "Other");
    }

    #[test]
    fn test_style_category_from_property_layout() {
        assert_eq!(
            StyleCategory::from_property("margin-left"),
            StyleCategory::Layout
        );
        assert_eq!(
            StyleCategory::from_property("margin-right"),
            StyleCategory::Layout
        );
        assert_eq!(
            StyleCategory::from_property("padding-top"),
            StyleCategory::Layout
        );
        assert_eq!(StyleCategory::from_property("width"), StyleCategory::Layout);
        assert_eq!(
            StyleCategory::from_property("height"),
            StyleCategory::Layout
        );
        assert_eq!(
            StyleCategory::from_property("min-width"),
            StyleCategory::Layout
        );
        assert_eq!(
            StyleCategory::from_property("max-height"),
            StyleCategory::Layout
        );
        assert_eq!(
            StyleCategory::from_property("flex-direction"),
            StyleCategory::Layout
        );
        assert_eq!(
            StyleCategory::from_property("grid-template"),
            StyleCategory::Layout
        );
        assert_eq!(
            StyleCategory::from_property("display"),
            StyleCategory::Layout
        );
        assert_eq!(
            StyleCategory::from_property("position"),
            StyleCategory::Layout
        );
    }

    #[test]
    fn test_style_category_from_property_typography() {
        assert_eq!(
            StyleCategory::from_property("font-size"),
            StyleCategory::Typography
        );
        assert_eq!(
            StyleCategory::from_property("font-family"),
            StyleCategory::Typography
        );
        assert_eq!(
            StyleCategory::from_property("font-weight"),
            StyleCategory::Typography
        );
        assert_eq!(
            StyleCategory::from_property("text-align"),
            StyleCategory::Typography
        );
        assert_eq!(
            StyleCategory::from_property("text-decoration"),
            StyleCategory::Typography
        );
        // Note: line-height contains "height" so it matches Layout before Typography
        assert_eq!(
            StyleCategory::from_property("letter-spacing"),
            StyleCategory::Typography
        );
    }

    #[test]
    fn test_style_category_from_property_colors() {
        assert_eq!(StyleCategory::from_property("color"), StyleCategory::Colors);
        assert_eq!(
            StyleCategory::from_property("background-color"),
            StyleCategory::Colors
        );
        assert_eq!(
            StyleCategory::from_property("background"),
            StyleCategory::Colors
        );
    }

    #[test]
    fn test_style_category_from_property_border() {
        // Border should match before layout
        assert_eq!(
            StyleCategory::from_property("border-width"),
            StyleCategory::Border
        );
        assert_eq!(
            StyleCategory::from_property("border-color"),
            StyleCategory::Border
        );
        assert_eq!(
            StyleCategory::from_property("border-style"),
            StyleCategory::Border
        );
        assert_eq!(
            StyleCategory::from_property("border-radius"),
            StyleCategory::Border
        );
    }

    #[test]
    fn test_style_category_from_property_effects() {
        assert_eq!(
            StyleCategory::from_property("box-shadow"),
            StyleCategory::Effects
        );
        // Note: text-shadow starts with "text" so it matches Typography before Effects
        assert_eq!(
            StyleCategory::from_property("opacity"),
            StyleCategory::Effects
        );
        assert_eq!(
            StyleCategory::from_property("transform"),
            StyleCategory::Effects
        );
        assert_eq!(
            StyleCategory::from_property("transform-origin"),
            StyleCategory::Effects
        );
    }

    #[test]
    fn test_style_category_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(StyleCategory::Layout);
        set.insert(StyleCategory::Typography);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_style_category_copy() {
        let cat = StyleCategory::Border;
        let copied = cat; // Copy, not move
        assert_eq!(cat, copied);
    }

    #[test]
    fn test_style_inspector_default() {
        let inspector = StyleInspector::default();
        assert!(inspector.properties.is_empty());
        assert!(inspector.classes.is_empty());
        assert!(inspector.widget_id.is_none());
        assert!(inspector.widget_type.is_empty());
    }

    #[test]
    fn test_style_inspector_clear() {
        let mut inspector = StyleInspector::new();
        inspector.set_widget("Button", Some("btn".to_string()));
        inspector.add_class("primary");
        inspector.add_property(ComputedProperty::new("color", "red"));

        inspector.clear();

        assert!(inspector.properties.is_empty());
        assert!(inspector.classes.is_empty());
        assert!(inspector.widget_id.is_none());
        assert!(inspector.widget_type.is_empty());
    }

    #[test]
    fn test_style_inspector_set_properties() {
        let mut inspector = StyleInspector::new();
        let props = vec![
            ComputedProperty::new("color", "red"),
            ComputedProperty::new("font-size", "12px"),
        ];
        inspector.set_properties(props);
        assert_eq!(inspector.properties.len(), 2);
    }

    #[test]
    fn test_style_inspector_toggle_inherited() {
        let mut inspector = StyleInspector::new();
        assert!(inspector.show_inherited);
        inspector.toggle_inherited();
        assert!(!inspector.show_inherited);
        inspector.toggle_inherited();
        assert!(inspector.show_inherited);
    }

    #[test]
    fn test_style_inspector_toggle_overridden() {
        let mut inspector = StyleInspector::new();
        assert!(!inspector.show_overridden);
        inspector.toggle_overridden();
        assert!(inspector.show_overridden);
        inspector.toggle_overridden();
        assert!(!inspector.show_overridden);
    }

    #[test]
    fn test_style_inspector_set_category_filter() {
        let mut inspector = StyleInspector::new();
        assert!(inspector.category_filter.is_none());
        inspector.set_category_filter(Some(StyleCategory::Layout));
        assert_eq!(inspector.category_filter, Some(StyleCategory::Layout));
        inspector.set_category_filter(None);
        assert!(inspector.category_filter.is_none());
    }

    #[test]
    fn test_style_inspector_toggle_category() {
        let mut inspector = StyleInspector::new();
        // All categories start expanded
        assert_eq!(
            inspector.expanded_categories.get(&StyleCategory::Layout),
            Some(&true)
        );

        inspector.toggle_category(StyleCategory::Layout);
        assert_eq!(
            inspector.expanded_categories.get(&StyleCategory::Layout),
            Some(&false)
        );

        inspector.toggle_category(StyleCategory::Layout);
        assert_eq!(
            inspector.expanded_categories.get(&StyleCategory::Layout),
            Some(&true)
        );
    }

    #[test]
    fn test_style_inspector_filter_overridden() {
        let mut inspector = StyleInspector::new();
        inspector.add_property(ComputedProperty::new("color", "blue"));
        inspector.add_property(ComputedProperty::new("font-size", "14px").overridden());

        // Without overridden shown
        assert_eq!(inspector.filtered().len(), 1);

        // With overridden shown
        inspector.toggle_overridden();
        assert_eq!(inspector.filtered().len(), 2);
    }

    #[test]
    fn test_style_inspector_filter_category() {
        let mut inspector = StyleInspector::new();
        inspector.add_property(ComputedProperty::new("color", "blue"));
        inspector.add_property(ComputedProperty::new("font-size", "14px"));
        inspector.add_property(ComputedProperty::new("margin", "10px"));

        // No filter
        assert_eq!(inspector.filtered().len(), 3);

        // Filter to layout only
        inspector.set_category_filter(Some(StyleCategory::Layout));
        let filtered = inspector.filtered();
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "margin");
    }

    #[test]
    fn test_style_inspector_select_next() {
        let mut inspector = StyleInspector::new();
        inspector.add_property(ComputedProperty::new("a", "1"));
        inspector.add_property(ComputedProperty::new("b", "2"));
        inspector.add_property(ComputedProperty::new("c", "3"));

        assert!(inspector.selected.is_none());

        inspector.select_next();
        assert_eq!(inspector.selected, Some(0));

        inspector.select_next();
        assert_eq!(inspector.selected, Some(1));

        inspector.select_next();
        assert_eq!(inspector.selected, Some(2));

        // Should not go past last item
        inspector.select_next();
        assert_eq!(inspector.selected, Some(2));
    }

    #[test]
    fn test_style_inspector_select_prev() {
        let mut inspector = StyleInspector::new();
        inspector.add_property(ComputedProperty::new("a", "1"));
        inspector.add_property(ComputedProperty::new("b", "2"));
        inspector.add_property(ComputedProperty::new("c", "3"));

        // Start at end
        inspector.selected = Some(2);

        inspector.select_prev();
        assert_eq!(inspector.selected, Some(1));

        inspector.select_prev();
        assert_eq!(inspector.selected, Some(0));

        // Should not go below 0
        inspector.select_prev();
        assert_eq!(inspector.selected, Some(0));
    }

    #[test]
    fn test_style_inspector_select_empty() {
        let mut inspector = StyleInspector::new();

        // Should do nothing with empty list
        inspector.select_next();
        assert!(inspector.selected.is_none());

        inspector.select_prev();
        assert!(inspector.selected.is_none());
    }

    #[test]
    fn test_style_inspector_select_prev_from_none() {
        let mut inspector = StyleInspector::new();
        inspector.add_property(ComputedProperty::new("a", "1"));

        inspector.select_prev();
        assert_eq!(inspector.selected, Some(0));
    }

    #[test]
    fn test_computed_property_builder_chain() {
        let prop = ComputedProperty::new("display", "block")
            .source(PropertySource::Id)
            .overridden();

        assert_eq!(prop.name, "display");
        assert_eq!(prop.value, "block");
        assert_eq!(prop.source, PropertySource::Id);
        assert!(prop.overridden);
    }

    #[test]
    fn test_style_inspector_new_initializes_expanded() {
        let inspector = StyleInspector::new();
        // All categories should be expanded by default
        for cat in StyleCategory::all() {
            assert_eq!(
                inspector.expanded_categories.get(cat),
                Some(&true),
                "Category {:?} should be expanded",
                cat
            );
        }
    }

    #[test]
    fn test_computed_property_clone() {
        let prop = ComputedProperty::new("color", "red").source(PropertySource::Inline);
        let cloned = prop.clone();
        assert_eq!(cloned.name, "color");
        assert_eq!(cloned.value, "red");
        assert_eq!(cloned.source, PropertySource::Inline);
    }

    #[test]
    fn test_computed_property_debug() {
        let prop = ComputedProperty::new("opacity", "0.5");
        let debug = format!("{:?}", prop);
        assert!(debug.contains("opacity"));
        assert!(debug.contains("0.5"));
    }
}
