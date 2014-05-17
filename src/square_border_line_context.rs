use {
    AddColor,
    Borrowed,
    Color,
    Field,
    Line,
    Matrix2d,
    SquareBorderLineColorContext,
    Value,
};
use internal::{
    CanTransform,
    CanViewTransform,
    HasTransform,
    HasViewTransform,
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

impl<'a> Clone for SquareBorderLineContext<'a> {
    #[inline(always)]
    fn clone(&self) -> SquareBorderLineContext<'static> {
        SquareBorderLineContext {
            base: self.base.clone(),
            transform: self.transform.clone(),
            line: self.line.clone(),
            square_border_radius: self.square_border_radius.clone(),
        }
    }
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

impl<'a> HasViewTransform<'a, Matrix2d> for SquareBorderLineContext<'a> {
    #[inline(always)]
    fn get_view_transform(&'a self) -> &'a Matrix2d {
        self.base.get()
    }
}

impl<'a> CanViewTransform<'a, SquareBorderLineContext<'a>, Matrix2d> for SquareBorderLineContext<'a> {
    #[inline(always)]
    fn view_transform(&'a self, value: Matrix2d) -> SquareBorderLineContext<'a> {
        SquareBorderLineContext {
            base: Value(value),
            transform: Borrowed(self.transform.get()),
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
            color: Value(Color([r, g, b, a])),
            square_border_radius: Borrowed(self.square_border_radius.get()),
        }
    }
}

