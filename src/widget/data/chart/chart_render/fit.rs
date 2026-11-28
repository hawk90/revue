//! Items along a category axis: how many fit, and where the `…` goes when
//! some are left out

/// How many of `count` items fit along `extent` cells, when item `i` starts
/// at `i * slot` and needs its first `item` cells inside (`1 <= item <=
/// slot`).
///
/// Returns the number of items to draw and, when items are left out, the
/// offset of the cell that shows `…` instead: the first cell of the first
/// left-out item's slot. Items are dropped from the end until that cell is
/// inside `extent`, so the marker always shows, as Pagination and Breadcrumb
/// mark the items they hide.
pub(crate) fn fit_items(count: usize, slot: u16, item: u16, extent: u16) -> (usize, Option<u16>) {
    if count == 0 || extent == 0 {
        return (0, None);
    }
    let slot = slot.max(1);
    let item = item.clamp(1, slot);
    let last_end = (count as u64 - 1) * u64::from(slot) + u64::from(item);
    if last_end <= u64::from(extent) {
        return (count, None);
    }
    // `shown * slot <= extent - 1`, and `shown < count` because the items
    // did not all fit
    let shown = (extent - 1) / slot;
    (usize::from(shown), Some(shown * slot))
}
