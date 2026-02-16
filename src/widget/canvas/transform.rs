//! 2D transformation matrix

/// 2D transformation matrix for translate, scale, and rotate operations
#[derive(Clone, Copy, Debug)]
pub struct Transform {
    /// Scale X
    pub sx: f64,
    /// Shear X (for rotation)
    pub shx: f64,
    /// Translate X
    pub tx: f64,
    /// Shear Y (for rotation)
    pub shy: f64,
    /// Scale Y
    pub sy: f64,
    /// Translate Y
    pub ty: f64,
}

impl Default for Transform {
    fn default() -> Self {
        Self::identity()
    }
}

impl Transform {
    /// Create an identity transform (no transformation)
    pub fn identity() -> Self {
        Self {
            sx: 1.0,
            shx: 0.0,
            tx: 0.0,
            shy: 0.0,
            sy: 1.0,
            ty: 0.0,
        }
    }

    /// Create a translation transform
    pub fn translate(x: f64, y: f64) -> Self {
        Self {
            sx: 1.0,
            shx: 0.0,
            tx: x,
            shy: 0.0,
            sy: 1.0,
            ty: y,
        }
    }

    /// Create a scale transform
    pub fn scale(sx: f64, sy: f64) -> Self {
        Self {
            sx,
            shx: 0.0,
            tx: 0.0,
            shy: 0.0,
            sy,
            ty: 0.0,
        }
    }

    /// Create a uniform scale transform
    pub fn scale_uniform(s: f64) -> Self {
        Self::scale(s, s)
    }

    /// Create a rotation transform (angle in radians)
    pub fn rotate(angle: f64) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Self {
            sx: cos,
            shx: -sin,
            tx: 0.0,
            shy: sin,
            sy: cos,
            ty: 0.0,
        }
    }

    /// Create a rotation transform from degrees
    pub fn rotate_degrees(degrees: f64) -> Self {
        Self::rotate(degrees.to_radians())
    }

    /// Apply this transform to a point
    pub fn apply(&self, x: f64, y: f64) -> (f64, f64) {
        (
            self.sx * x + self.shx * y + self.tx,
            self.shy * x + self.sy * y + self.ty,
        )
    }

    /// Combine with another transform (self * other)
    pub fn then(&self, other: &Transform) -> Self {
        Self {
            sx: self.sx * other.sx + self.shx * other.shy,
            shx: self.sx * other.shx + self.shx * other.sy,
            tx: self.sx * other.tx + self.shx * other.ty + self.tx,
            shy: self.shy * other.sx + self.sy * other.shy,
            sy: self.shy * other.shx + self.sy * other.sy,
            ty: self.shy * other.tx + self.sy * other.ty + self.ty,
        }
    }

    /// Add a translation to this transform
    pub fn with_translate(self, x: f64, y: f64) -> Self {
        self.then(&Transform::translate(x, y))
    }

    /// Add a scale to this transform
    pub fn with_scale(self, sx: f64, sy: f64) -> Self {
        self.then(&Transform::scale(sx, sy))
    }

    /// Add a rotation to this transform
    pub fn with_rotate(self, angle: f64) -> Self {
        self.then(&Transform::rotate(angle))
    }
}
