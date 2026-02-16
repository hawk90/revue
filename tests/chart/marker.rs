//! Marker public API tests
mod tests {
    use revue::widget::data::chart::Marker;

    #[test]
    fn test_marker_char_none() {
        assert_eq!(Marker::None.char(), ' ');
    }

    #[test]
    fn test_marker_char_dot() {
        assert_eq!(Marker::Dot.char(), '•');
    }

    #[test]
    fn test_marker_char_circle() {
        assert_eq!(Marker::Circle.char(), '○');
    }

    #[test]
    fn test_marker_char_filled_circle() {
        assert_eq!(Marker::FilledCircle.char(), '●');
    }

    #[test]
    fn test_marker_char_square() {
        assert_eq!(Marker::Square.char(), '□');
    }

    #[test]
    fn test_marker_char_filled_square() {
        assert_eq!(Marker::FilledSquare.char(), '■');
    }

    #[test]
    fn test_marker_char_diamond() {
        assert_eq!(Marker::Diamond.char(), '◇');
    }

    #[test]
    fn test_marker_char_filled_diamond() {
        assert_eq!(Marker::FilledDiamond.char(), '◆');
    }

    #[test]
    fn test_marker_char_triangle() {
        assert_eq!(Marker::Triangle.char(), '△');
    }

    #[test]
    fn test_marker_char_filled_triangle() {
        assert_eq!(Marker::FilledTriangle.char(), '▲');
    }

    #[test]
    fn test_marker_char_cross() {
        assert_eq!(Marker::Cross.char(), '+');
    }

    #[test]
    fn test_marker_char_x() {
        assert_eq!(Marker::X.char(), '×');
    }

    #[test]
    fn test_marker_char_star() {
        assert_eq!(Marker::Star.char(), '★');
    }

    #[test]
    fn test_marker_char_star_outline() {
        assert_eq!(Marker::StarOutline.char(), '☆');
    }

    #[test]
    fn test_marker_char_braille() {
        assert_eq!(Marker::Braille.char(), '⣿');
    }

    #[test]
    fn test_marker_default() {
        assert_eq!(Marker::default(), Marker::None);
    }

    #[test]
    fn test_marker_clone() {
        let marker1 = Marker::Circle;
        let marker2 = marker1.clone();
        assert_eq!(marker1, marker2);
    }

    #[test]
    fn test_marker_copy() {
        let marker1 = Marker::Triangle;
        let marker2 = marker1;
        assert_eq!(marker2, Marker::Triangle);
    }

    #[test]
    fn test_marker_partial_eq() {
        assert_eq!(Marker::Circle, Marker::Circle);
        assert_eq!(Marker::Star, Marker::Star);
        assert_ne!(Marker::Circle, Marker::FilledCircle);
    }

    #[test]
    fn test_marker_variants_unique() {
        assert_ne!(Marker::None, Marker::Dot);
        assert_ne!(Marker::Dot, Marker::Circle);
        assert_ne!(Marker::Circle, Marker::FilledCircle);
        assert_ne!(Marker::FilledCircle, Marker::Square);
        assert_ne!(Marker::Square, Marker::FilledSquare);
        assert_ne!(Marker::FilledSquare, Marker::Diamond);
        assert_ne!(Marker::Diamond, Marker::FilledDiamond);
        assert_ne!(Marker::FilledDiamond, Marker::Triangle);
        assert_ne!(Marker::Triangle, Marker::FilledTriangle);
        assert_ne!(Marker::FilledTriangle, Marker::Cross);
        assert_ne!(Marker::Cross, Marker::X);
        assert_ne!(Marker::X, Marker::Star);
        assert_ne!(Marker::Star, Marker::StarOutline);
        assert_ne!(Marker::StarOutline, Marker::Braille);
    }
}