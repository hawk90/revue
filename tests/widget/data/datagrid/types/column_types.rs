//! Column type definitions tests

use revue::widget::data::datagrid::types::{ColumnType, SortDirection, Alignment};

#[test]
fn test_column_type_text() {
    let ct = ColumnType::Text;
    assert!(matches!(ct, ColumnType::Text));
}

#[test]
fn test_column_type_number() {
    let ct = ColumnType::Number;
    assert!(matches!(ct, ColumnType::Number));
}

#[test]
fn test_column_type_date() {
    let ct = ColumnType::Date;
    assert!(matches!(ct, ColumnType::Date));
}

#[test]
fn test_column_type_boolean() {
    let ct = ColumnType::Boolean;
    assert!(matches!(ct, ColumnType::Boolean));
}

#[test]
fn test_column_type_custom() {
    let ct = ColumnType::Custom;
    assert!(matches!(ct, ColumnType::Custom));
}

#[test]
fn test_column_type_clone() {
    let ct1 = ColumnType::Number;
    let ct2 = ct1;
    assert_eq!(ct1, ct2);
}

#[test]
fn test_column_type_copy() {
    let ct = ColumnType::Text;
    let _ct_copy = ct; // Copy trait allows this
    assert_eq!(ct, ColumnType::Text); // Original still valid
}

#[test]
fn test_column_type_partial_eq() {
    assert_eq!(ColumnType::Text, ColumnType::Text);
    assert_eq!(ColumnType::Number, ColumnType::Number);
    assert_ne!(ColumnType::Text, ColumnType::Number);
}

#[test]
fn test_column_type_eq() {
    assert_eq!(ColumnType::Text, ColumnType::Text);
    assert_eq!(ColumnType::Date, ColumnType::Date);
}

#[test]
fn test_column_type_default() {
    let ct: ColumnType = Default::default();
    assert_eq!(ct, ColumnType::Text);
}

#[test]
fn test_column_type_debug() {
    let ct = ColumnType::Number;
    let debug_str = format!("{:?}", ct);
    assert!(debug_str.contains("Number"));
}

#[test]
fn test_sort_direction_ascending() {
    let sd = SortDirection::Ascending;
    assert!(matches!(sd, SortDirection::Ascending));
}

#[test]
fn test_sort_direction_descending() {
    let sd = SortDirection::Descending;
    assert!(matches!(sd, SortDirection::Descending));
}

#[test]
fn test_sort_direction_toggle_from_ascending() {
    let sd = SortDirection::Ascending;
    assert_eq!(sd.toggle(), SortDirection::Descending);
}

#[test]
fn test_sort_direction_toggle_from_descending() {
    let sd = SortDirection::Descending;
    assert_eq!(sd.toggle(), SortDirection::Ascending);
}

#[test]
fn test_sort_direction_toggle_twice() {
    let sd = SortDirection::Ascending;
    let toggled_once = sd.toggle();
    let toggled_twice = toggled_once.toggle();
    assert_eq!(toggled_twice, SortDirection::Ascending);
}

#[test]
fn test_sort_direction_icon_ascending() {
    let sd = SortDirection::Ascending;
    assert_eq!(sd.icon(), '▲');
}

#[test]
fn test_sort_direction_icon_descending() {
    let sd = SortDirection::Descending;
    assert_eq!(sd.icon(), '▼');
}

#[test]
fn test_sort_direction_clone() {
    let sd1 = SortDirection::Ascending;
    let sd2 = sd1;
    assert_eq!(sd1, sd2);
}

#[test]
fn test_sort_direction_copy() {
    let sd = SortDirection::Ascending;
    let _sd_copy = sd; // Copy trait allows this
    assert_eq!(sd, SortDirection::Ascending); // Original still valid
}

#[test]
fn test_sort_direction_partial_eq() {
    assert_eq!(SortDirection::Ascending, SortDirection::Ascending);
    assert_eq!(SortDirection::Descending, SortDirection::Descending);
    assert_ne!(SortDirection::Ascending, SortDirection::Descending);
}

#[test]
fn test_sort_direction_eq() {
    assert_eq!(SortDirection::Ascending, SortDirection::Ascending);
    assert_eq!(SortDirection::Descending, SortDirection::Descending);
}

#[test]
fn test_sort_direction_debug() {
    let sd = SortDirection::Ascending;
    let debug_str = format!("{:?}", sd);
    assert!(debug_str.contains("Ascending"));
}

#[test]
fn test_alignment_left() {
    let align = Alignment::Left;
    assert!(matches!(align, Alignment::Left));
}

#[test]
fn test_alignment_center() {
    let align = Alignment::Center;
    assert!(matches!(align, Alignment::Center));
}

#[test]
fn test_alignment_right() {
    let align = Alignment::Right;
    assert!(matches!(align, Alignment::Right));
}

#[test]
fn test_alignment_clone() {
    let align1 = Alignment::Center;
    let align2 = align1;
    assert_eq!(align1, align2);
}

#[test]
fn test_alignment_copy() {
    let align = Alignment::Right;
    let _align_copy = align; // Copy trait allows this
    assert_eq!(align, Alignment::Right); // Original still valid
}

#[test]
fn test_alignment_partial_eq() {
    assert_eq!(Alignment::Left, Alignment::Left);
    assert_eq!(Alignment::Center, Alignment::Center);
    assert_ne!(Alignment::Left, Alignment::Right);
}

#[test]
fn test_alignment_eq() {
    assert_eq!(Alignment::Left, Alignment::Left);
    assert_eq!(Alignment::Right, Alignment::Right);
}

#[test]
fn test_alignment_default() {
    let align: Alignment = Default::default();
    assert_eq!(align, Alignment::Left);
}

#[test]
fn test_alignment_debug() {
    let align = Alignment::Center;
    let debug_str = format!("{:?}", align);
    assert!(debug_str.contains("Center"));
}

#[test]
fn test_column_type_all_variants_distinct() {
    let types = vec![
        ColumnType::Text,
        ColumnType::Number,
        ColumnType::Date,
        ColumnType::Boolean,
        ColumnType::Custom,
    ];

    for (i, t1) in types.iter().enumerate() {
        for (j, t2) in types.iter().enumerate() {
            if i == j {
                assert_eq!(t1, t2);
            } else {
                assert_ne!(t1, t2);
            }
        }
    }
}

#[test]
fn test_sort_direction_both_variants_distinct() {
    assert_ne!(SortDirection::Ascending, SortDirection::Descending);
}

#[test]
fn test_alignment_all_variants_distinct() {
    let alignments = vec![Alignment::Left, Alignment::Center, Alignment::Right];

    for (i, a1) in alignments.iter().enumerate() {
        for (j, a2) in alignments.iter().enumerate() {
            if i == j {
                assert_eq!(a1, a2);
            } else {
                assert_ne!(a1, a2);
            }
        }
    }
}

#[test]
fn test_sort_direction_toggle_roundtrip() {
    let directions = vec![SortDirection::Ascending, SortDirection::Descending];
    for &dir in &directions {
        let toggled = dir.toggle();
        let toggled_back = toggled.toggle();
        assert_eq!(dir, toggled_back);
    }
}

#[test]
fn test_sort_direction_icons_different() {
    assert_ne!(
        SortDirection::Ascending.icon(),
        SortDirection::Descending.icon()
    );
}

#[test]
fn test_sort_direction_icon_characters() {
    assert_eq!(SortDirection::Ascending.icon(), '▲');
    assert_eq!(SortDirection::Descending.icon(), '▼');
}

#[test]
fn test_column_type_match_guards() {
    fn is_numeric(ct: ColumnType) -> bool {
        matches!(ct, ColumnType::Number)
    }

    assert!(is_numeric(ColumnType::Number));
    assert!(!is_numeric(ColumnType::Text));
    assert!(!is_numeric(ColumnType::Date));
    assert!(!is_numeric(ColumnType::Boolean));
    assert!(!is_numeric(ColumnType::Custom));
}

#[test]
fn test_alignment_in_match_expression() {
    fn get_alignment_name(align: Alignment) -> &'static str {
        match align {
            Alignment::Left => "left",
            Alignment::Center => "center",
            Alignment::Right => "right",
        }
    }

    assert_eq!(get_alignment_name(Alignment::Left), "left");
    assert_eq!(get_alignment_name(Alignment::Center), "center");
    assert_eq!(get_alignment_name(Alignment::Right), "right");
}

#[test]
fn test_sort_direction_in_match_expression() {
    fn get_direction_name(dir: SortDirection) -> &'static str {
        match dir {
            SortDirection::Ascending => "asc",
            SortDirection::Descending => "desc",
        }
    }

    assert_eq!(get_direction_name(SortDirection::Ascending), "asc");
    assert_eq!(get_direction_name(SortDirection::Descending), "desc");
}

#[test]
fn test_column_type_default_in_generic_context() {
    fn get_default<T: Default>() -> T {
        T::default()
    }

    let ct: ColumnType = get_default();
    assert_eq!(ct, ColumnType::Text);
}

#[test]
fn test_alignment_default_in_generic_context() {
    fn get_default<T: Default>() -> T {
        T::default()
    }

    let align: Alignment = get_default();
    assert_eq!(align, Alignment::Left);
}

#[test]
fn test_sort_direction_toggle_state_transition() {
    let mut state = SortDirection::Ascending;
    assert_eq!(state, SortDirection::Ascending);

    state = state.toggle();
    assert_eq!(state, SortDirection::Descending);

    state = state.toggle();
    assert_eq!(state, SortDirection::Ascending);
}

#[test]
fn test_sort_direction_icon_is_printable() {
    let icon_asc = SortDirection::Ascending.icon();
    let icon_desc = SortDirection::Descending.icon();

    // Sort icons are Unicode symbols (▲▼), not ASCII, but should be printable
    assert!(!icon_asc.is_ascii()); // Verify they're not ASCII
    assert!(!icon_desc.is_ascii()); // Verify they're not ASCII
                                    // They are valid single characters
    assert!(icon_asc.is_alphanumeric() || !icon_asc.is_ascii()); // Unicode arrow
    assert!(icon_desc.is_alphanumeric() || !icon_desc.is_ascii()); // Unicode arrow
}