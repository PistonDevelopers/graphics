use std::ops::{ Mul };

use math::{ Scalar, Vec2d };

/// The size of a shape.
#[derive(Clone, Copy, Debug)]
pub struct Size {
    /// The horizontal length of the shape (width).
    pub w: Scalar,
    /// The vertical length of the shape (height).
    pub h: Scalar,
}

impl From<Vec2d> for Size {
    fn from(v: Vec2d) -> Size {
        Size { w: v[0], h: v[1] }
    }
}

impl From<(Scalar, Scalar)> for Size {
    fn from((w, h): (Scalar, Scalar)) -> Size {
        Size { w: w, h: h }
    }
}

impl Size {
    /// Convert size to a vector.
    pub fn to_array(self) -> Vec2d {
        [self.w, self.h]
    }
}

impl<T: Into<Size>> Mul<T> for Size {
    type Output = Size;

    fn mul(self, v: T) -> Size {
        let v: Size = v.into();
        Size { w: self.w * v.w, h: self.h * v.h }
    }
}

impl Mul<Scalar> for Size {
    type Output = Size;

    fn mul(self, s: Scalar) -> Size {
        Size { w: self.w * s, h: self.h * s }
    }
}
