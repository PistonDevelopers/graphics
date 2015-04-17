use std::ops::{ Mul };

use math::Scalar;

/// The size of a shape.
#[derive(Clone, Copy, Debug)]
pub struct Size<S = Scalar> {
    /// The horizontal length of the shape (width).
    pub w: S,
    /// The vertical length of the shape (height).
    pub h: S,
}

impl From<Size<f32>> for Size {
    fn from(size: Size<f32>) -> Size {
        Size { w: size.w as Scalar, h: size.h as Scalar }
    }
}

impl<S: Copy> From<[S; 2]> for Size<S> {
    fn from(v: [S; 2]) -> Size<S> {
        Size { w: v[0], h: v[1] }
    }
}

impl From<[f32; 2]> for Size {
    fn from(v: [f32; 2]) -> Size {
        Size { w: v[0] as Scalar, h: v[1] as Scalar }
    }
}

impl<S> From<(S, S)> for Size<S> {
    fn from((w, h): (S, S)) -> Size<S> {
        Size { w: w, h: h }
    }
}

impl<S> Size<S> {
    /// Convert size to an array.
    pub fn to_array(self) -> [S; 2] {
        [self.w, self.h]
    }
}

impl<S: Mul<S, Output = S>, T: Into<Size<S>>> Mul<T> for Size<S> {
    type Output = Size<S>;

    fn mul(self, v: T) -> Size<S> {
        let v: Size<S> = v.into();
        Size { w: self.w * v.w, h: self.h * v.h }
    }
}

impl Mul<Scalar> for Size {
    type Output = Size;

    fn mul(self, s: Scalar) -> Size {
        Size { w: self.w * s, h: self.h * s }
    }
}

impl Mul<f32> for Size<f32> {
    type Output = Size<f32>;

    fn mul(self, s: f32) -> Size<f32> {
        Size { w: self.w * s, h: self.h * s }
    }
}
