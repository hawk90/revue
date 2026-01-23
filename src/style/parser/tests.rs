#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_parse_color_hex() {
        assert_eq!(parse_color("#ff0000"), Some(Color::RED));
        assert_eq!(parse_color("#00ff00"), Some(Color::GREEN));
        assert_eq!(parse_color("#f00"), Some(Color::RED));
    }

    #[test]
    fn test_parse_color_rgb() {
        assert_eq!(parse_color("rgb(255, 0, 0)"), Some(Color::RED));
        assert_eq!(parse_color("rgb(0, 255, 0)"), Some(Color::GREEN));
    }

    #[test]
    fn test_parse_color_named() {
        assert_eq!(parse_color("red"), Some(Color::RED));
        assert_eq!(parse_color("WHITE"), Some(Color::WHITE));
    }

    #[test]
    fn test_parse_size() {
        assert_eq!(parse_size("auto"), Size::Auto);
        assert_eq!(parse_size("100"), Size::Fixed(100));
        assert_eq!(parse_size("100px"), Size::Fixed(100));
        assert_eq!(parse_size("50%"), Size::Percent(50.0));
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
    fn test_parse_grid_template() {
        let template = parse_grid_template("1fr 2fr 1fr");
        assert_eq!(template.tracks.len(), 3);
        assert!(matches!(template.tracks[0], GridTrack::Fr(v) if (v - 1.0).abs() < 0.01));
        assert!(matches!(template.tracks[1], GridTrack::Fr(v) if (v - 2.0).abs() < 0.01));
    }

    #[test]
    fn test_parse_grid_template_mixed() {
        let template = parse_grid_template("100px auto 1fr");
        assert_eq!(template.tracks.len(), 3);
        assert!(matches!(template.tracks[0], GridTrack::Fixed(100)));
        assert!(matches!(template.tracks[1], GridTrack::Auto));
        assert!(matches!(template.tracks[2], GridTrack::Fr(_)));
    }

    #[test]
    fn test_parse_grid_placement_line() {
        let placement = parse_grid_placement("2");
        assert_eq!(placement.start, 2);
        assert_eq!(placement.end, 0);
    }

    #[test]
    fn test_parse_grid_placement_span() {
        let placement = parse_grid_placement("span 3");
        assert_eq!(placement.start, 0);
        assert_eq!(placement.end, -3);
    }

    #[test]
    fn test_parse_grid_placement_range() {
        let placement = parse_grid_placement("1 / 4");
        assert_eq!(placement.start, 1);
        assert_eq!(placement.end, 4);
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
        assert_eq!(style.layout.grid_template_columns.tracks.len(), 2);
        assert_eq!(style.layout.grid_template_rows.tracks.len(), 2);
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

    // =========================================================================
    // Issue #181: Byte-slicing grid parsing tests
    // =========================================================================

    #[test]
    fn test_parse_grid_template_repeat() {
        // Test repeat() function parsing with byte-slicing
        let template = parse_grid_template("repeat(3, 1fr)");
        assert_eq!(template.tracks.len(), 3);
        assert!(matches!(template.tracks[0], GridTrack::Fr(v) if (v - 1.0).abs() < 0.01));
        assert!(matches!(template.tracks[1], GridTrack::Fr(v) if (v - 1.0).abs() < 0.01));
        assert!(matches!(template.tracks[2], GridTrack::Fr(v) if (v - 1.0).abs() < 0.01));
    }

    #[test]
    fn test_parse_grid_template_repeat_fixed() {
        // Test repeat() with fixed values
        let template = parse_grid_template("repeat(2, 100px)");
        assert_eq!(template.tracks.len(), 2);
        assert!(matches!(template.tracks[0], GridTrack::Fixed(100)));
        assert!(matches!(template.tracks[1], GridTrack::Fixed(100)));
    }

    #[test]
    fn test_parse_grid_template_minmax() {
        // Test minmax() function parsing with byte-slicing
        let template = parse_grid_template("minmax(100px, 1fr)");
        assert_eq!(template.tracks.len(), 1);
        // Currently uses max value (simplified implementation)
        assert!(matches!(template.tracks[0], GridTrack::Fr(_)));
    }

    #[test]
    fn test_parse_grid_template_repeat_with_minmax() {
        // Test nested repeat() with minmax()
        let template = parse_grid_template("repeat(2, minmax(50px, 1fr))");
        assert_eq!(template.tracks.len(), 2);
    }

    #[test]
    fn test_parse_grid_template_complex() {
        // Test complex grid template with mixed values
        let template = parse_grid_template("100px repeat(2, 1fr) auto");
        assert_eq!(template.tracks.len(), 4);
        assert!(matches!(template.tracks[0], GridTrack::Fixed(100)));
        assert!(matches!(template.tracks[1], GridTrack::Fr(_)));
        assert!(matches!(template.tracks[2], GridTrack::Fr(_)));
        assert!(matches!(template.tracks[3], GridTrack::Auto));
    }

    #[test]
    fn test_parse_grid_template_whitespace() {
        // Test that whitespace is handled correctly
        let template = parse_grid_template("  1fr   2fr   ");
        assert_eq!(template.tracks.len(), 2);
    }

    #[test]
    fn test_parse_repeat_function_invalid() {
        // Test that invalid repeat() returns None through parse_grid_template
        let template = parse_grid_template("repeat(abc, 1fr)");
        assert!(template.tracks.is_empty());
    }

    #[test]
    fn test_parse_minmax_function_invalid() {
        // Test that invalid minmax() is handled
        let template = parse_grid_template("minmax(100px)"); // Missing second arg
        assert!(template.tracks.is_empty());
    }
}
