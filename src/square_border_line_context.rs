use {
    AddColor,
    Borrowed,
    Field,
    Line,
    Matrix2d,
    SquareBorderLineColorContext,
    Value,
    View,
};
use vecmath::{
    identity,
};
use internal::{
    CanTransform,
    HasTransform,
};

/// A line context with square border information.
pub struct SquareBorderLineContext<'a> {
    /// Base/original transform.
    pub base: Field<'a, Matrix2d>,
    /// Current transform.
    pub transform: Field<'a, Matrix2d>,
    /// Current line.
    pub line: Field<'a, Line>,
    /// Current square border.
    pub square_border_radius: Field<'a, f64>,
}

impl<'a> HasTransform<'a, Matrix2d> for SquareBorderLineContext<'a> {
    #[inline(always)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a> CanTransform<'a, SquareBorderLineContext<'a>, Matrix2d> for SquareBorderLineContext<'a> {
    #[inline(always)]
    fn transform(&'a self, value: Matrix2d) -> SquareBorderLineContext<'a> {
        SquareBorderLineContext {
            base: Borrowed(self.base.get()),
            transform: Value(value),
            line: Borrowed(self.line.get()),
            square_border_radius: Borrowed(self.square_border_radius.get()),
        }
    }
}

impl<'a> AddColor<'a, SquareBorderLineColorContext<'a>> for SquareBorderLineContext<'a> {
    #[inline(always)]
    fn rgba(&'a self, r: f32, g: f32, b: f32, a: f32) -> SquareBorderLineColorContext<'a> {
        SquareBorderLineColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            line: Borrowed(self.line.get()),
            color: Value([r, g, b, a]),
            square_border_radius: Borrowed(self.square_border_radius.get()),
        }
    }
}

impl<'a> View<'a> for SquareBorderLineContext<'a> {
    #[inline(always)]
    fn view(&'a self) -> SquareBorderLineContext<'a> {
        SquareBorderLineContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.base.get()),
            line: Borrowed(self.line.get()),
            square_border_radius: Borrowed(self.square_border_radius.get()),
        }
    }

    #[inline(always)]
    fn reset(&'a self) -> SquareBorderLineContext<'a> {
        SquareBorderLineContext {
            base: Borrowed(self.base.get()),
            transform: Value(identity()),
            line: Borrowed(self.line.get()),
            square_border_radius: Borrowed(self.square_border_radius.get()),
        }
    }

    #[inline(always)]
    fn store_view(&'a self) -> SquareBorderLineContext<'a> {
        SquareBorderLineContext {
            base: Borrowed(self.transform.get()),
            transform: Borrowed(self.transform.get()),
            line: Borrowed(self.line.get()),
            square_border_radius: Borrowed(self.square_border_radius.get()),
        }
    }
}

