use revue::widget::data::chart::ChartOrientation;
mod tests {

#[test]
fn test_orientation_default() {
  let orientation = ChartOrientation::default();
  assert_eq!(orientation, ChartOrientation::Vertical);
}

#[test]
fn test_orientation_clone() {
  let orientation1 = ChartOrientation::Horizontal;
  let orientation2 = orientation1.clone();
  assert_eq!(orientation1, orientation2);
}

#[test]
fn test_orientation_copy() {
  let orientation1 = ChartOrientation::Vertical;
  let orientation2 = orientation1;
  assert_eq!(orientation2, ChartOrientation::Vertical);
}

#[test]
fn test_orientation_partial_eq() {
  assert_eq!(ChartOrientation::Vertical, ChartOrientation::Vertical);
  assert_eq!(ChartOrientation::Horizontal, ChartOrientation::Horizontal);
  assert_ne!(ChartOrientation::Vertical, ChartOrientation::Horizontal);
}

#[test]
fn test_orientation_variants_unique() {
  assert_ne!(ChartOrientation::Vertical, ChartOrientation::Horizontal);
}}
