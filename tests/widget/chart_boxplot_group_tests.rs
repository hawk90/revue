//! Box plot group tests extracted from source files
//! Tests only use public APIs

mod box_group {
    use revue::widget::data::chart::boxplot::{types::BoxStats, group::BoxGroup};
    use revue::style::Color;

    #[test]
    fn test_box_group_new() {
        let group = BoxGroup::new("test", &[1.0, 2.0, 3.0, 4.0, 5.0]);
        assert_eq!(group.label, "test");
        assert_eq!(group.data, vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        assert!(group.stats.is_none());
        assert!(group.color.is_none());
    }

    #[test]
    fn test_box_group_from_stats() {
        let stats = BoxStats {
            min: 1.0,
            q1: 2.0,
            median: 3.0,
            q3: 4.0,
            max: 5.0,
            outliers: vec![],
            whisker_low: 1.0,
            whisker_high: 5.0,
        };
        let group = BoxGroup::from_stats("test", stats.clone());
        assert_eq!(group.label, "test");
        assert!(group.data.is_empty());
        assert!(group.stats.is_some());
        assert!(group.color.is_none());
        // Can't compare stats directly due to no PartialEq, but we verified Some(stats)
    }

    #[test]
    fn test_box_group_color() {
        let group = BoxGroup::new("test", &[1.0, 2.0]).color(Color::RED);
        assert_eq!(group.color, Some(Color::RED));
    }

    #[test]
    fn test_box_group_get_stats_with_precomputed() {
        let stats = BoxStats {
            min: 1.0,
            q1: 2.0,
            median: 3.0,
            q3: 4.0,
            max: 5.0,
            outliers: vec![],
            whisker_low: 1.0,
            whisker_high: 5.0,
        };
        let group = BoxGroup::from_stats("test", stats.clone());
        let result = group.get_stats(super::types::WhiskerStyle::MinMax);
        assert!(result.is_some());
        // Can't compare BoxStats directly as it doesn't derive PartialEq
        let result_stats = result.unwrap();
        assert_eq!(result_stats.min, stats.min);
        assert_eq!(result_stats.median, stats.median);
    }

    #[test]
    fn test_box_group_get_stats_compute() {
        let group = BoxGroup::new("test", &[1.0, 2.0, 3.0, 4.0, 5.0]);
        let result = group.get_stats(super::types::WhiskerStyle::MinMax);
        assert!(result.is_some());
        let stats = result.unwrap();
        assert_eq!(stats.min, 1.0);
        assert_eq!(stats.max, 5.0);
        assert_eq!(stats.median, 3.0);
    }

    #[test]
    fn test_box_group_public_fields() {
        let mut group = BoxGroup::new("test", &[1.0, 2.0]);
        group.label = "modified".to_string();
        group.data.push(3.0);
        group.stats = None;
        group.color = Some(Color::BLUE);

        assert_eq!(group.label, "modified");
        assert_eq!(group.data, vec![1.0, 2.0, 3.0]);
        assert!(group.stats.is_none());
        assert_eq!(group.color, Some(Color::BLUE));
    }

    #[test]
    fn test_box_group_empty_data() {
        let group = BoxGroup::new("test", &[]);
        assert_eq!(group.data, Vec::<f64>::new());
        assert!(group.stats.is_none());
    }

    #[test]
    fn test_box_group_new_from_slice() {
        let data = vec![10.0, 20.0, 30.0];
        let group = BoxGroup::new("slice", &data);
        assert_eq!(group.data, data);
    }
}

mod box_stats {
    use revue::widget::data::chart::boxplot::types::{BoxStats, WhiskerStyle};

    #[test]
    fn test_whisker_style_default() {
        let style = WhiskerStyle::default();
        assert_eq!(style, WhiskerStyle::IQR);
    }

    #[test]
    fn test_whisker_style_variants() {
        assert_eq!(WhiskerStyle::IQR, WhiskerStyle::IQR);
        assert_eq!(WhiskerStyle::MinMax, WhiskerStyle::MinMax);
        assert_eq!(WhiskerStyle::Percentile, WhiskerStyle::Percentile);
    }

    #[test]
    fn test_box_stats_from_data_empty() {
        let result = BoxStats::from_data(&[], WhiskerStyle::IQR);
        assert!(result.is_none());
    }

    #[test]
    fn test_box_stats_from_data_single_value() {
        let data = vec![5.0];
        let result = BoxStats::from_data(&data, WhiskerStyle::IQR);
        assert!(result.is_some());
        let stats = result.unwrap();
        assert_eq!(stats.min, 5.0);
        assert_eq!(stats.max, 5.0);
        assert_eq!(stats.median, 5.0);
    }

    #[test]
    fn test_box_stats_from_data_multiple_values() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = BoxStats::from_data(&data, WhiskerStyle::IQR);
        assert!(result.is_some());
        let stats = result.unwrap();
        assert_eq!(stats.min, 1.0);
        assert_eq!(stats.max, 5.0);
        assert_eq!(stats.median, 3.0);
    }

    #[test]
    fn test_box_stats_from_data_with_nan() {
        let data = vec![1.0, f64::NAN, 3.0, f64::INFINITY, 5.0];
        let result = BoxStats::from_data(&data, WhiskerStyle::IQR);
        assert!(result.is_some());
        let stats = result.unwrap();
        assert_eq!(stats.min, 1.0);
        assert_eq!(stats.max, 5.0);
        assert_eq!(stats.median, 3.0);
    }

    #[test]
    fn test_box_stats_public_fields() {
        let stats = BoxStats {
            min: 1.0,
            q1: 2.0,
            median: 3.0,
            q3: 4.0,
            max: 5.0,
            outliers: vec![],
            whisker_low: 1.0,
            whisker_high: 5.0,
        };
        assert_eq!(stats.min, 1.0);
        assert_eq!(stats.q1, 2.0);
        assert_eq!(stats.median, 3.0);
        assert_eq!(stats.q3, 4.0);
        assert_eq!(stats.max, 5.0);
        assert!(stats.outliers.is_empty());
        assert_eq!(stats.whisker_low, 1.0);
        assert_eq!(stats.whisker_high, 5.0);
    }

    #[test]
    fn test_box_stats_from_data_whisker_style_iqr() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = BoxStats::from_data(&data, WhiskerStyle::IQR);
        assert!(result.is_some());
        let stats = result.unwrap();
        assert!(stats.outliers.is_empty());
    }

    #[test]
    fn test_box_stats_from_data_whisker_style_minmax() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = BoxStats::from_data(&data, WhiskerStyle::MinMax);
        assert!(result.is_some());
        let stats = result.unwrap();
        assert_eq!(stats.whisker_low, 1.0);
        assert_eq!(stats.whisker_high, 5.0);
        assert!(stats.outliers.is_empty());
    }

    #[test]
    fn test_box_stats_from_data_whisker_style_percentile() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = BoxStats::from_data(&data, WhiskerStyle::Percentile);
        assert!(result.is_some());
        let stats = result.unwrap();
        // Percentile style may or may not have outliers depending on the data
        // Just verify we got some values
        assert!(stats.min <= stats.whisker_low);
        assert!(stats.max >= stats.whisker_high);
    }

    #[test]
    fn test_box_stats_from_data_with_outliers() {
        // Data with potential outliers
        let data = vec![1.0, 2.0, 3.0, 4.0, 100.0];
        let result = BoxStats::from_data(&data, WhiskerStyle::IQR);
        assert!(result.is_some());
        let stats = result.unwrap();
        assert_eq!(stats.min, 1.0);
        assert_eq!(stats.max, 100.0);
        assert_eq!(stats.median, 3.0);
    }
}