
use {
    Borrowed, 
    Color,
    Field, 
    Line,
    Matrix2d, 
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

/// A line context.
pub struct LineColorContext<'a> {
    /// Base/original transform.
    pub base: Field<'a, Matrix2d>,
    /// Current transform.
    pub transform: Field<'a, Matrix2d>,
    /// Current line.
    pub line: Field<'a, Line>,
    /// Current color.
    pub color: Field<'a, Color>,
}

impl<'a> HasTransform<'a, Matrix2d> for LineColorContext<'a> {
    #[inline(always)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a> CanTransform<'a, LineColorContext<'a>, Matrix2d> for LineColorContext<'a> {
    #[inline(always)]
    fn transform(&'a self, value: Matrix2d) -> LineColorContext<'a> {
        LineColorContext {
            base: Borrowed(self.base.get()),
            transform: Value(value),
            line: Borrowed(self.line.get()),
            color: Borrowed(self.color.get()),
        }
    }
}

impl<'a> View<'a> for LineColorContext<'a> {
    #[inline(always)]
    fn view(&'a self) -> LineColorContext<'a> {
        LineColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.base.get()),
            line: Borrowed(self.line.get()),
            color: Borrowed(self.color.get()),
        }
    }

    #[inline(always)]
    fn reset(&'a self) -> LineColorContext<'a> {
        LineColorContext {
            base: Borrowed(self.base.get()),
            transform: Value(identity()),
            line: Borrowed(self.line.get()),
            color: Borrowed(self.color.get()),
        }
    }

    #[inline(always)]
    fn store_view(&'a self) -> LineColorContext<'a> {
        LineColorContext {
            base: Borrowed(self.transform.get()),
            transform: Borrowed(self.transform.get()),
            line: Borrowed(self.line.get()),
            color: Borrowed(self.color.get()),
        }
    }
}


