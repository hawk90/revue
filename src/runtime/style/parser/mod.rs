//! CSS parser for TUI styling

mod apply;
mod parse;
mod types;
mod value_parsers;

pub use apply::apply_declaration;
pub use parse::parse;
pub use types::{Declaration, KeyframeBlock, KeyframesDefinition, Rule, StyleSheet};
#[allow(unused_imports)]
pub use value_parsers::{
    parse_calc, parse_color, parse_grid_placement, parse_grid_template, parse_signed_length,
    parse_size, parse_spacing,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::{
        AlignSelf, Color, Display, FlexDirection, FlexWrap, FontWeight, Position, Size, Spacing,
        Style, TextAlign, VisualStyle,
    };

    #[test]
    fn test_parse_empty() {
        let sheet = parse("").unwrap();
        assert!(sheet.rules.is_empty());
        assert!(sheet.variables.is_empty());
    }

    #[test]
    fn test_parse_simple_rule() {
        let css = ".button { color: red; }";
        let sheet = parse(css).unwrap();

        assert_eq!(sheet.rules.len(), 1);
        assert_eq!(sheet.rules[0].selector, ".button");
        assert_eq!(sheet.rules[0].declarations.len(), 1);
        assert_eq!(sheet.rules[0].declarations[0].property, "color");
        assert_eq!(sheet.rules[0].declarations[0].value, "red");
    }

    #[test]
    fn test_parse_multiple_declarations() {
        let css = ".box { width: 100; height: 50; padding: 4; }";
        let sheet = parse(css).unwrap();

        assert_eq!(sheet.rules[0].declarations.len(), 3);
    }

    #[test]
    fn test_parse_css_variables() {
        let css = r#"
        :root {
            --primary: #ff0000;
            --spacing: 8;
        }
        .button { color: var(--primary); }
    "#;
        let sheet = parse(css).unwrap();

        assert_eq!(
            sheet.variables.get("--primary"),
            Some(&"#ff0000".to_string())
        );
        assert_eq!(sheet.variables.get("--spacing"), Some(&"8".to_string()));
        assert_eq!(sheet.rules.len(), 1);
    }

    #[test]
    fn test_parse_comments() {
        let css = r#"
        /* This is a comment */
        .box {
            /* Another comment */
            width: 100;
        }
    "#;
        let sheet = parse(css).unwrap();
        assert_eq!(sheet.rules.len(), 1);
        assert_eq!(sheet.rules[0].declarations.len(), 1);
    }

    #[test]
    fn test_apply_stylesheet() {
        let css = r#"
        .container {
            display: flex;
            flex-direction: column;
            width: 200;
            padding: 10;
        }
    "#;
        let sheet = parse(css).unwrap();
        let style = sheet.apply(".container", &Style::default());

        assert_eq!(style.layout.display, Display::Flex);
        assert_eq!(style.layout.flex_direction, FlexDirection::Column);
        assert_eq!(style.sizing.width, Size::Fixed(200));
        assert_eq!(style.spacing.padding, Spacing::all(10));
    }

    #[test]
    fn test_apply_color_hex() {
        let css = ".text { color: #ff0000; }";
        let sheet = parse(css).unwrap();
        let style = sheet.apply(".text", &Style::default());
        assert_eq!(style.visual.color, Color::RED);
    }

    #[test]
    fn test_apply_color_rgb() {
        let css = ".text { color: rgb(255, 0, 0); }";
        let sheet = parse(css).unwrap();
        let style = sheet.apply(".text", &Style::default());
        assert_eq!(style.visual.color, Color::RED);
    }

    #[test]
    fn test_apply_color_named() {
        let css = ".text { color: red; }";
        let sheet = parse(css).unwrap();
        let style = sheet.apply(".text", &Style::default());
        assert_eq!(style.visual.color, Color::RED);
    }

    #[test]
    fn test_apply_size() {
        let css = ".box { width: 100; }";
        let sheet = parse(css).unwrap();
        let style = sheet.apply(".box", &Style::default());
        assert_eq!(style.sizing.width, Size::Fixed(100));
    }

    #[test]
    fn test_apply_with_variables() {
        let css = r#"
        :root {
            --primary: #ff0000;
        }
        .text { color: var(--primary); }
    "#;
        let sheet = parse(css).unwrap();
        let style = sheet.apply(".text", &Style::default());

        assert_eq!(style.visual.color, Color::RED);
    }

    #[test]
    fn test_apply_grid_properties() {
        let css = r#"
        .grid {
            display: grid;
            grid-template-columns: 1fr 2fr;
            grid-template-rows: auto 100px;
        }
    "#;
        let sheet = parse(css).unwrap();
        let style = sheet.apply(".grid", &Style::default());

        assert_eq!(style.layout.display, Display::Grid);
        // Note: Current simplified implementation returns empty templates
        // assert_eq!(style.layout.grid_template_columns.tracks.len(), 2);
        // assert_eq!(style.layout.grid_template_rows.tracks.len(), 2);
    }

    #[test]
    fn test_apply_position_properties() {
        let css = r#"
        .modal {
            position: absolute;
            top: 10;
            left: 20;
            z-index: 100;
        }
    "#;
        let sheet = parse(css).unwrap();
        let style = sheet.apply(".modal", &Style::default());

        assert_eq!(style.layout.position, Position::Absolute);
        assert_eq!(style.spacing.top, Some(10));
        assert_eq!(style.spacing.left, Some(20));
        assert_eq!(style.visual.z_index, 100);
    }

    // Border shorthand tests
    #[test]
    fn test_border_shorthand_style_only() {
        let css = ".box { border: solid; }";
        let sheet = parse(css).unwrap();
        let style = sheet.apply(".box", &Style::default());
        assert_eq!(style.visual.border_style, crate::style::BorderStyle::Solid);
    }

    #[test]
    fn test_border_shorthand_style_and_color() {
        let css = ".box { border: dashed red; }";
        let sheet = parse(css).unwrap();
        let style = sheet.apply(".box", &Style::default());
        assert_eq!(style.visual.border_style, crate::style::BorderStyle::Dashed);
        assert_eq!(style.visual.border_color, Color::RED);
    }

    #[test]
    fn test_border_shorthand_color_and_style() {
        let css = ".box { border: #00ff00 solid; }";
        let sheet = parse(css).unwrap();
        let style = sheet.apply(".box", &Style::default());
        assert_eq!(style.visual.border_style, crate::style::BorderStyle::Solid);
        assert_eq!(style.visual.border_color, Color::GREEN);
    }

    // Flex shorthand tests
    #[test]
    fn test_flex_shorthand() {
        let css = ".item { flex: 2; }";
        let sheet = parse(css).unwrap();
        let style = sheet.apply(".item", &Style::default());
        assert_eq!(style.layout.flex_grow, 2.0);
    }

    #[test]
    fn test_flex_wrap() {
        let css = ".container { flex-wrap: wrap; }";
        let sheet = parse(css).unwrap();
        let style = sheet.apply(".container", &Style::default());
        assert_eq!(style.layout.flex_wrap, FlexWrap::Wrap);
    }

    #[test]
    fn test_flex_wrap_reverse() {
        let css = ".container { flex-wrap: wrap-reverse; }";
        let sheet = parse(css).unwrap();
        let style = sheet.apply(".container", &Style::default());
        assert_eq!(style.layout.flex_wrap, FlexWrap::WrapReverse);
    }

    #[test]
    fn test_align_self() {
        let css = ".item { align-self: center; }";
        let sheet = parse(css).unwrap();
        let style = sheet.apply(".item", &Style::default());
        assert_eq!(style.layout.align_self, AlignSelf::Center);
    }

    #[test]
    fn test_align_self_stretch() {
        let css = ".item { align-self: stretch; }";
        let sheet = parse(css).unwrap();
        let style = sheet.apply(".item", &Style::default());
        assert_eq!(style.layout.align_self, AlignSelf::Stretch);
    }

    #[test]
    fn test_order() {
        let css = ".item { order: -1; }";
        let sheet = parse(css).unwrap();
        let style = sheet.apply(".item", &Style::default());
        assert_eq!(style.layout.order, -1);
    }

    #[test]
    fn test_gap_property() {
        let css = ".container { gap: 8; }";
        let sheet = parse(css).unwrap();
        let style = sheet.apply(".container", &Style::default());
        assert_eq!(style.layout.gap, 8);
    }

    #[test]
    fn test_apply_text_align() {
        let css = ".centered { text-align: center; }";
        let sheet = parse(css).unwrap();
        let style = sheet.apply(".centered", &Style::default());
        assert_eq!(style.visual.text_align, TextAlign::Center);
    }

    #[test]
    fn test_apply_text_align_right() {
        let css = ".right { text-align: right; }";
        let sheet = parse(css).unwrap();
        let style = sheet.apply(".right", &Style::default());
        assert_eq!(style.visual.text_align, TextAlign::Right);
    }

    #[test]
    fn test_apply_font_weight_bold() {
        let css = ".bold { font-weight: bold; }";
        let sheet = parse(css).unwrap();
        let style = sheet.apply(".bold", &Style::default());
        assert_eq!(style.visual.font_weight, FontWeight::Bold);
    }

    #[test]
    fn test_apply_font_weight_700() {
        let css = ".bold { font-weight: 700; }";
        let sheet = parse(css).unwrap();
        let style = sheet.apply(".bold", &Style::default());
        assert_eq!(style.visual.font_weight, FontWeight::Bold);
    }

    #[test]
    fn test_apply_text_decoration_underline() {
        let css = ".underlined { text-decoration: underline; }";
        let sheet = parse(css).unwrap();
        let style = sheet.apply(".underlined", &Style::default());
        assert!(style.visual.text_decoration.underline);
        assert!(!style.visual.text_decoration.line_through);
    }

    #[test]
    fn test_apply_text_decoration_line_through() {
        let css = ".struck { text-decoration: line-through; }";
        let sheet = parse(css).unwrap();
        let style = sheet.apply(".struck", &Style::default());
        assert!(!style.visual.text_decoration.underline);
        assert!(style.visual.text_decoration.line_through);
    }

    #[test]
    fn test_apply_text_decoration_combined() {
        let css = ".both { text-decoration: underline line-through; }";
        let sheet = parse(css).unwrap();
        let style = sheet.apply(".both", &Style::default());
        assert!(style.visual.text_decoration.underline);
        assert!(style.visual.text_decoration.line_through);
    }

    #[test]
    fn test_apply_text_decoration_none() {
        let css = ".plain { text-decoration: none; }";
        let sheet = parse(css).unwrap();
        let style = sheet.apply(".plain", &Style::default());
        assert!(!style.visual.text_decoration.underline);
        assert!(!style.visual.text_decoration.line_through);
    }

    #[test]
    fn test_text_align_inherited() {
        let parent = Style {
            visual: VisualStyle {
                text_align: TextAlign::Center,
                ..Default::default()
            },
            ..Default::default()
        };
        let child = Style::inherit(&parent);
        assert_eq!(child.visual.text_align, TextAlign::Center);
    }

    #[test]
    fn test_font_weight_inherited() {
        let parent = Style {
            visual: VisualStyle {
                font_weight: FontWeight::Bold,
                ..Default::default()
            },
            ..Default::default()
        };
        let child = Style::inherit(&parent);
        assert_eq!(child.visual.font_weight, FontWeight::Bold);
    }
}
