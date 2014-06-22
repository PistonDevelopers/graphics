
use {
    AddColor,
    Borrowed,
    Field,
    RoundBorderLineColorContext,
    Value,
};
use internal::{
    CanTransform,
    CanViewTransform,
    ColorComponent,
    HasTransform,
    HasViewTransform,
    Line,
    Matrix2d,
    Radius,
};

/// A line context with round border information.
pub struct RoundBorderLineContext<'a> {
    /// View transform.
    pub view: Field<'a, Matrix2d>,
    /// Current transform.
    pub transform: Field<'a, Matrix2d>,
    /// Current line.
    pub line: Field<'a, Line>,
    /// Current round border.
    pub round_border_radius: Field<'a, Radius>,
}

impl<'a> 
Clone 
for RoundBorderLineContext<'a> {
    #[inline(always)]   
    fn clone(&self) -> RoundBorderLineContext<'static> {
        RoundBorderLineContext {
            view: Value(*self.view.get()),
            transform: Value(*self.transform.get()),
            line: Value(*self.line.get()),
            round_border_radius: Value(*self.round_border_radius.get()),
        }
    }
}

impl<'a> 
HasTransform<'a, Matrix2d> 
for RoundBorderLineContext<'a> {
    #[inline(always)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a> 
CanTransform<'a, RoundBorderLineContext<'a>, Matrix2d> 
for RoundBorderLineContext<'a> {
    #[inline(always)]
    fn transform(
        &'a self, 
        value: Matrix2d
    ) -> RoundBorderLineContext<'a> {
        RoundBorderLineContext {
            view: Borrowed(self.view.get()),
            transform: Value(value),
            line: Borrowed(self.line.get()),
            round_border_radius: Borrowed(self.round_border_radius.get()),
        }
    }
}

impl<'a> 
HasViewTransform<'a, Matrix2d> 
for RoundBorderLineContext<'a> {
    #[inline(always)]
    fn get_view_transform(&'a self) -> &'a Matrix2d {
        self.view.get()
    }
}

impl<'a> 
CanViewTransform<'a, RoundBorderLineContext<'a>, Matrix2d> 
for RoundBorderLineContext<'a> {
    #[inline(always)]
    fn view_transform(
        &'a self, 
        value: Matrix2d
    ) -> RoundBorderLineContext<'a> {
        RoundBorderLineContext {
            view: Value(value),
            transform: Borrowed(self.transform.get()),
            line: Borrowed(self.line.get()),
            round_border_radius: Borrowed(self.round_border_radius.get()),
        }
    }
}

impl<'a> 
AddColor<'a, RoundBorderLineColorContext<'a>> 
for RoundBorderLineContext<'a> {
    #[inline(always)]
    fn rgba(
        &'a self, 
        r: ColorComponent, 
        g: ColorComponent, 
        b: ColorComponent, 
        a: ColorComponent
    ) -> RoundBorderLineColorContext<'a> {
        RoundBorderLineColorContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            line: Borrowed(self.line.get()),
            color: Value([r, g, b, a]),
            round_border_radius: Borrowed(self.round_border_radius.get()),
        }
    }
}

