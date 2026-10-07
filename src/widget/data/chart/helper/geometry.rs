//! Plot-grid geometry for the chart: segment clipping and line rasterizing

/// A point on the plot grid: fractional columns from the left edge and rows up
/// from the bottom edge.
pub(super) type GridPoint = (f64, f64);

/// A line segment between two points in data coordinates.
pub(super) type DataSegment = ((f64, f64), (f64, f64));

/// Whether a grid point lies inside a `gw` x `gh` grid.
pub(super) fn grid_contains((x, y): GridPoint, gw: u16, gh: u16) -> bool {
    const EPS: f64 = 1e-9;
    x >= -EPS && x <= gw as f64 - 1.0 + EPS && y >= -EPS && y <= gh as f64 - 1.0 + EPS
}

/// Clip a segment to a `gw` x `gh` grid (Liang-Barsky).
///
/// Returns `None` when the segment misses the grid. Endpoints that are already
/// inside are returned unchanged.
pub(super) fn clip_segment(
    p0: GridPoint,
    p1: GridPoint,
    gw: u16,
    gh: u16,
) -> Option<(GridPoint, GridPoint)> {
    let (x_max, y_max) = (gw as f64 - 1.0, gh as f64 - 1.0);
    let (dx, dy) = (p1.0 - p0.0, p1.1 - p0.1);
    let (mut t0, mut t1) = (0.0f64, 1.0f64);
    for (p, q) in [
        (-dx, p0.0),
        (dx, x_max - p0.0),
        (-dy, p0.1),
        (dy, y_max - p0.1),
    ] {
        if p == 0.0 {
            if q < 0.0 {
                return None;
            }
        } else {
            let r = q / p;
            if p < 0.0 {
                t0 = t0.max(r);
            } else {
                t1 = t1.min(r);
            }
        }
    }
    if t0 > t1 {
        return None;
    }
    let at = |t: f64| (p0.0 + t * dx, p0.1 + t * dy);
    let a = if t0 > 0.0 { at(t0) } else { p0 };
    let b = if t1 < 1.0 { at(t1) } else { p1 };
    Some((a, b))
}

/// Walk the cells of a line from `start` to `end` (Bresenham), calling
/// `visit(x, y, step)` for each one.
pub(super) fn bresenham(
    start: (u16, u16),
    end: (u16, u16),
    mut visit: impl FnMut(u16, u16, usize),
) {
    let (x0, y0) = (start.0 as i32, start.1 as i32);
    let (x1, y1) = (end.0 as i32, end.1 as i32);
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;

    let (mut x, mut y) = (x0, y0);
    let mut step = 0;
    loop {
        visit(x as u16, y as u16, step);
        if x == x1 && y == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }
        step += 1;
    }
}

/// The grid cell (column, row from the top) holding a point inside a
/// `gw` x `gh` grid.
pub(super) fn grid_cell((x, y): GridPoint, gw: u16, gh: u16) -> (u16, u16) {
    let col = x.floor().clamp(0.0, gw as f64 - 1.0) as u16;
    let up = y.floor().clamp(0.0, gh as f64 - 1.0) as u16;
    (col, gh - 1 - up)
}
