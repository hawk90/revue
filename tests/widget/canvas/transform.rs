//! Transform tests

use revue::widget::canvas::Transform;

#[test]
fn test_identity() {
    let t = Transform::identity();
    assert_eq!(t.sx, 1.0);
    assert_eq!(t.sy, 1.0);
    assert_eq!(t.tx, 0.0);
    assert_eq!(t.ty, 0.0);
    assert_eq!(t.shx, 0.0);
    assert_eq!(t.shy, 0.0);
}

#[test]
fn test_default() {
    let t = Transform::default();
    assert_eq!(t.sx, 1.0);
    assert_eq!(t.sy, 1.0);
}

#[test]
fn test_translate() {
    let t = Transform::translate(10.0, 20.0);
    assert_eq!(t.tx, 10.0);
    assert_eq!(t.ty, 20.0);
    assert_eq!(t.sx, 1.0);
    assert_eq!(t.sy, 1.0);
}

#[test]
fn test_scale() {
    let t = Transform::scale(2.0, 3.0);
    assert_eq!(t.sx, 2.0);
    assert_eq!(t.sy, 3.0);
    assert_eq!(t.tx, 0.0);
    assert_eq!(t.ty, 0.0);
}

#[test]
fn test_scale_uniform() {
    let t = Transform::scale_uniform(2.5);
    assert_eq!(t.sx, 2.5);
    assert_eq!(t.sy, 2.5);
}

#[test]
fn test_rotate() {
    let t = Transform::rotate(std::f64::consts::FRAC_PI_2); // 90 degrees
    let (cos, sin) = (
        std::f64::consts::FRAC_PI_2.cos(),
        std::f64::consts::FRAC_PI_2.sin(),
    );
    assert!((t.sx - cos).abs() < 1e-10);
    assert!((t.sy - cos).abs() < 1e-10);
    assert!((t.shx - (-sin)).abs() < 1e-10);
    assert!((t.shy - sin).abs() < 1e-10);
}

#[test]
fn test_rotate_degrees() {
    let t = Transform::rotate_degrees(90.0);
    let t2 = Transform::rotate(std::f64::consts::FRAC_PI_2);
    assert!((t.sx - t2.sx).abs() < 1e-10);
    assert!((t.sy - t2.sy).abs() < 1e-10);
}

#[test]
fn test_apply_identity() {
    let t = Transform::identity();
    let (x, y) = t.apply(5.0, 10.0);
    assert_eq!(x, 5.0);
    assert_eq!(y, 10.0);
}

#[test]
fn test_apply_translate() {
    let t = Transform::translate(10.0, 20.0);
    let (x, y) = t.apply(5.0, 10.0);
    assert_eq!(x, 15.0);
    assert_eq!(y, 30.0);
}

#[test]
fn test_apply_scale() {
    let t = Transform::scale(2.0, 3.0);
    let (x, y) = t.apply(5.0, 10.0);
    assert_eq!(x, 10.0);
    assert_eq!(y, 30.0);
}

#[test]
fn test_then_identity() {
    let t1 = Transform::identity();
    let t2 = Transform::identity();
    let result = t1.then(&t2);
    assert_eq!(result.sx, 1.0);
    assert_eq!(result.sy, 1.0);
}

#[test]
fn test_then_translate() {
    let t1 = Transform::translate(10.0, 20.0);
    let t2 = Transform::translate(5.0, 10.0);
    let result = t1.then(&t2);
    // Combined translation should be (15, 30)
    assert_eq!(result.tx, 15.0);
    assert_eq!(result.ty, 30.0);
}

#[test]
fn test_then_scale() {
    let t1 = Transform::scale(2.0, 3.0);
    let t2 = Transform::scale(4.0, 5.0);
    let result = t1.then(&t2);
    // Combined scale should be (8, 15)
    assert_eq!(result.sx, 8.0);
    assert_eq!(result.sy, 15.0);
}

#[test]
fn test_with_translate() {
    let t = Transform::identity().with_translate(10.0, 20.0);
    assert_eq!(t.tx, 10.0);
    assert_eq!(t.ty, 20.0);
}

#[test]
fn test_with_scale() {
    let t = Transform::identity().with_scale(2.0, 3.0);
    assert_eq!(t.sx, 2.0);
    assert_eq!(t.sy, 3.0);
}

#[test]
fn test_with_rotate() {
    let t = Transform::identity().with_rotate(std::f64::consts::FRAC_PI_2);
    let t2 = Transform::rotate(std::f64::consts::FRAC_PI_2);
    assert!((t.sx - t2.sx).abs() < 1e-10);
}

#[test]
fn test_apply_combined_transform() {
    let t = Transform::identity()
        .with_scale(2.0, 2.0)
        .with_translate(10.0, 10.0);
    let (x, y) = t.apply(5.0, 5.0);
    // translate is applied first (due to then composition), then scale
    // translate: (5, 5) -> (15, 15), then scale: (30, 30)
    assert_eq!(x, 30.0);
    assert_eq!(y, 30.0);
}

#[test]
fn test_transform_copy() {
    let t1 = Transform::translate(10.0, 20.0);
    let t2 = t1;
    assert_eq!(t2.tx, 10.0);
    assert_eq!(t2.ty, 20.0);
}

#[test]
fn test_transform_clone() {
    let t1 = Transform::scale(2.0, 3.0);
    let t2 = t1.clone();
    assert_eq!(t2.sx, 2.0);
    assert_eq!(t2.sy, 3.0);
}

#[test]
fn test_zero_scale() {
    let t = Transform::scale(0.0, 1.0);
    let (x, y) = t.apply(5.0, 10.0);
    assert_eq!(x, 0.0);
    assert_eq!(y, 10.0);
}

#[test]
fn test_negative_scale() {
    let t = Transform::scale(-1.0, 1.0);
    let (x, y) = t.apply(5.0, 10.0);
    assert_eq!(x, -5.0);
    assert_eq!(y, 10.0);
}

#[test]
fn test_rotation_180_degrees() {
    let t = Transform::rotate_degrees(180.0);
    let (x, y) = t.apply(1.0, 0.0);
    // 180 degrees rotation: (1, 0) -> (-1, 0)
    assert!((x - (-1.0)).abs() < 1e-10);
    assert!((y - 0.0).abs() < 1e-10);
}

#[test]
fn test_rotation_45_degrees() {
    let t = Transform::rotate_degrees(45.0);
    let (x, y) = t.apply(1.0, 0.0);
    // 45 degrees rotation: (1, 0) -> (cos45, sin45) = (~0.707, ~0.707)
    assert!((x - std::f64::consts::FRAC_1_SQRT_2).abs() < 1e-10);
    assert!((y - std::f64::consts::FRAC_1_SQRT_2).abs() < 1e-10);
}

#[test]
fn test_rotate_then_translate() {
    let t = Transform::rotate_degrees(90.0).with_translate(10.0, 0.0);
    let (x, y): (f64, f64) = t.apply(1.0, 0.0);
    // translate is applied first (due to then composition), then rotate
    // translate: (11, 0), rotate 90 deg: (0, 11)
    assert!((x - 0.0).abs() < 1e-10);
    assert!((y - 11.0).abs() < 1e-10);
}

#[test]
fn test_translate_then_rotate() {
    let t = Transform::translate(10.0, 0.0).with_rotate(90_f64.to_radians());
    let (x, y): (f64, f64) = t.apply(1.0, 0.0);
    // rotate is applied first (due to then composition), then translate
    // rotate 90 deg: (1, 0) -> (0, 1), translate: (10, 1)
    assert!((x - 10.0).abs() < 1e-10);
    assert!((y - 1.0).abs() < 1e-10);
}

#[test]
fn test_with_rotate_degrees() {
    let t = Transform::identity().with_rotate(45_f64.to_radians());
    let (x, y): (f64, f64) = t.apply(1.0, 0.0);
    assert!((x - std::f64::consts::FRAC_1_SQRT_2).abs() < 1e-10);
    assert!((y - std::f64::consts::FRAC_1_SQRT_2).abs() < 1e-10);
}

#[test]
fn test_rotation_order_matters() {
    let t1 = Transform::translate(10.0, 0.0).with_rotate(90_f64.to_radians());
    let t2 = Transform::rotate(90_f64.to_radians()).with_translate(10.0, 0.0);
    let (x1, y1): (f64, f64) = t1.apply(1.0, 0.0);
    let (x2, y2): (f64, f64) = t2.apply(1.0, 0.0);
    // Results should be different due to order
    assert!((x1 - x2).abs() > 1e-10 || (y1 - y2).abs() > 1e-10);
}

#[test]
fn test_scale_with_translation() {
    let t = Transform::scale(2.0, 3.0).with_translate(5.0, 10.0);
    let (x, y): (f64, f64) = t.apply(10.0, 20.0);
    // translate is applied first (due to then composition), then scale
    // translate: (15, 30), scale: (30, 90)
    assert_eq!(x, 30.0);
    assert_eq!(y, 90.0);
}

#[test]
fn test_transform_fields_public() {
    let t = Transform {
        sx: 1.0,
        shx: 0.5,
        tx: 10.0,
        shy: 0.3,
        sy: 1.0,
        ty: 20.0,
    };
    assert_eq!(t.sx, 1.0);
    assert_eq!(t.shx, 0.5);
    assert_eq!(t.tx, 10.0);
    assert_eq!(t.shy, 0.3);
    assert_eq!(t.sy, 1.0);
    assert_eq!(t.ty, 20.0);
}

#[test]
fn test_apply_negative_point() {
    let t = Transform::translate(10.0, 20.0).with_scale(2.0, 3.0);
    let (x, y) = t.apply(-5.0, -10.0);
    // Scale: (-10, -30), then translate: (0, -10)
    assert_eq!(x, 0.0);
    assert_eq!(y, -10.0);
}

#[test]
fn test_shear_in_rotation() {
    let t = Transform::rotate(std::f64::consts::FRAC_PI_4);
    // For 45 degrees, cos = sin = sqrt(2)/2, shx = -sin, shy = sin
    assert!((t.shx + t.shy).abs() < 1e-10); // shx = -shy
}

#[test]
fn test_complex_transform_chain() {
    let t = Transform::identity()
        .with_scale(2.0, 2.0)
        .with_rotate(90_f64.to_radians())
        .with_translate(10.0, 20.0);
    let (x, y): (f64, f64) = t.apply(5.0, 0.0);
    // The order matters here - this tests the full chain
    let _ = (x, y);
    // Just verify it computes without panicking
}
