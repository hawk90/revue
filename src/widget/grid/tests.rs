#[cfg(test)]
mod tests {
    use super::super::super::Text;
    use super::*;
    use crate::render::Buffer;
    use crate::widget::Text;

    #[test]
    fn test_grid_new() {
        let g = Grid::new();
        assert!(g.items.is_empty());
        assert!(g.columns.is_empty());
    }

    #[test]
    fn test_grid_columns() {
        let g = Grid::new().columns(vec![
            TrackSize::Fixed(10),
            TrackSize::Fr(1.0),
            TrackSize::Fr(2.0),
        ]);
        assert_eq!(g.columns.len(), 3);
    }

    #[test]
    fn test_grid_cols() {
        let g = Grid::new().cols(3);
        assert_eq!(g.columns.len(), 3);
        assert!(g.columns.iter().all(|t| matches!(t, TrackSize::Fr(1.0))));
    }

    #[test]
    fn test_grid_gap() {
        let g = Grid::new().gap(2);
        assert_eq!(g.col_gap, 2);
        assert_eq!(g.row_gap, 2);
    }

    #[test]
    fn test_grid_placement_cell() {
        let p = GridPlacement::cell(2, 3);
        assert_eq!(p.col_start, 2);
        assert_eq!(p.col_end, 3);
        assert_eq!(p.row_start, 3);
        assert_eq!(p.row_end, 4);
    }

    #[test]
    fn test_grid_placement_area() {
        let p = GridPlacement::area(1, 1, 2, 3);
        assert_eq!(p.col_start, 1);
        assert_eq!(p.col_end, 3);
        assert_eq!(p.row_start, 1);
        assert_eq!(p.row_end, 4);
    }

    #[test]
    fn test_grid_item_at() {
        let item = GridItem::new(Text::new("Test")).at(2, 3);
        assert_eq!(item.placement.col_start, 2);
        assert_eq!(item.placement.row_start, 3);
    }

    #[test]
    fn test_grid_item_span() {
        let item = GridItem::new(Text::new("Test"))
            .at(1, 1)
            .col_span(2)
            .row_span(3);
        assert_eq!(item.placement.col_end, 3);
        assert_eq!(item.placement.row_end, 4);
    }

    #[test]
    fn test_track_size_default() {
        let t = TrackSize::default();
        assert!(matches!(t, TrackSize::Fr(1.0)));
    }

    #[test]
    fn test_calculate_tracks_fixed() {
        let g = Grid::new();
        let tracks = vec![TrackSize::Fixed(10), TrackSize::Fixed(20)];
        let sizes = g.calculate_tracks(100, &tracks, TrackSize::Fr(1.0), 0);
        assert_eq!(sizes, vec![10, 20]);
    }

    #[test]
    fn test_calculate_tracks_fr() {
        let g = Grid::new();
        let tracks = vec![TrackSize::Fr(1.0), TrackSize::Fr(1.0)];
        let sizes = g.calculate_tracks(100, &tracks, TrackSize::Fr(1.0), 0);
        assert_eq!(sizes, vec![50, 50]);
    }

    #[test]
    fn test_calculate_tracks_mixed() {
        let g = Grid::new();
        let tracks = vec![TrackSize::Fixed(20), TrackSize::Fr(1.0), TrackSize::Fr(2.0)];
        let sizes = g.calculate_tracks(100, &tracks, TrackSize::Fr(1.0), 0);
        // 20 fixed, remaining 80 split 1:2 = ~26, ~53
        assert_eq!(sizes[0], 20);
        assert!(sizes[1] > 0);
        assert!(sizes[2] > sizes[1]);
    }

    #[test]
    fn test_calculate_tracks_with_gap() {
        let g = Grid::new();
        let tracks = vec![TrackSize::Fr(1.0), TrackSize::Fr(1.0)];
        let sizes = g.calculate_tracks(100, &tracks, TrackSize::Fr(1.0), 10);
        // 100 - 10 gap = 90, split evenly = 45, 45
        assert_eq!(sizes, vec![45, 45]);
    }

    #[test]
    fn test_track_positions() {
        let g = Grid::new();
        let sizes = vec![10, 20, 30];
        let positions = g.track_positions(&sizes, 0);
        assert_eq!(positions, vec![0, 10, 30, 60]);
    }

    #[test]
    fn test_track_positions_with_gap() {
        let g = Grid::new();
        let sizes = vec![10, 20];
        let positions = g.track_positions(&sizes, 5);
        assert_eq!(positions, vec![0, 15, 35]);
    }

    #[test]
    fn test_grid_render() {
        let mut buffer = Buffer::new(40, 20);
        let area = crate::layout::Rect::new(0, 0, 40, 20);
        let mut ctx = crate::widget::traits::RenderContext::new(&mut buffer, area);

        let g = Grid::new()
            .cols(2)
            .rows_count(2)
            .child(Text::new("A"))
            .child(Text::new("B"))
            .child(Text::new("C"))
            .child(Text::new("D"));

        g.render(&mut ctx);
        // Smoke test
    }

    #[test]
    fn test_grid_with_explicit_placement() {
        let mut buffer = Buffer::new(40, 20);
        let area = crate::layout::Rect::new(0, 0, 40, 20);
        let mut ctx = crate::widget::traits::RenderContext::new(&mut buffer, area);

        let g = Grid::new()
            .cols(3)
            .rows_count(3)
            .item(GridItem::new(Text::new("Header")).at(1, 1).col_span(3))
            .item(GridItem::new(Text::new("Sidebar")).at(1, 2).row_span(2))
            .item(GridItem::new(Text::new("Content")).at(2, 2).col_span(2));

        g.render(&mut ctx);
    }

    #[test]
    fn test_grid_helper() {
        let g = grid().cols(3);
        assert_eq!(g.columns.len(), 3);
    }

    #[test]
    fn test_grid_template() {
        let g = grid_template(3, 2);
        assert_eq!(g.columns.len(), 3);
        assert_eq!(g.rows.len(), 2);
    }

    #[test]
    fn test_grid_align() {
        let g = Grid::new()
            .justify_items(GridAlign::Center)
            .align_items(GridAlign::End);

        assert!(matches!(g.justify_items, GridAlign::Center));
        assert!(matches!(g.align_items, GridAlign::End));
    }

    #[test]
    fn test_grid_auto_flow() {
        let g1 = Grid::new().auto_flow_row();
        assert!(g1.auto_flow_row);

        let g2 = Grid::new().auto_flow_col();
        assert!(!g2.auto_flow_row);
    }

    #[test]
    fn test_grid_percent_tracks() {
        let g = Grid::new();
        let tracks = vec![TrackSize::Percent(25.0), TrackSize::Percent(75.0)];
        let sizes = g.calculate_tracks(100, &tracks, TrackSize::Fr(1.0), 0);
        assert_eq!(sizes, vec![25, 75]);
    }
}
