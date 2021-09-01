use crate::{
    math::{margin_rectangle, relative_rectangle, Scalar},
    types::Rectangle,
};

/// Should be implemented by contexts that have rectangle information.
pub trait Rectangled: Sized {
    /// Shrinks the current rectangle equally by all sides.
    fn margin(self, m: Scalar) -> Self;

    /// Expands the current rectangle equally by all sides.
    #[inline(always)]
    fn expand(self, m: Scalar) -> Self {
        self.margin(-m)
    }

    /// Moves to a relative rectangle using the current rectangle as tile.
    fn rel(self, x: Scalar, y: Scalar) -> Self;
}

impl Rectangled for Rectangle {
    #[inline(always)]
    fn margin(self, m: Scalar) -> Self {
        margin_rectangle(self, m)
    }

    #[inline(always)]
    fn rel(self, x: Scalar, y: Scalar) -> Self {
        relative_rectangle(self, [x, y])
    }
}
