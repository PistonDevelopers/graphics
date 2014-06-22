
use {
    AddBevelBorder,
    AddColor,
    AddRoundBorder,
    AddSquareBorder,
    BevelBorderLineContext,
    Borrowed,
    Field,
    LineColorContext,
    RoundBorderLineContext,
    SquareBorderLineContext,
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

/// A line context.
pub struct LineContext<'a> {
    /// View transform.
    pub view: Field<'a, Matrix2d>,
    /// Current transform.
    pub transform: Field<'a, Matrix2d>,
    /// Current line.
    pub line: Field<'a, Line>,
}

impl<'a> 
Clone for 
LineContext<'a> {
    #[inline(always)]
    fn clone(&self) -> LineContext<'static> {
        LineContext {
            view: Value(*self.view.get()),
            transform: Value(*self.transform.get()),
            line: Value(*self.line.get()),
        }
    }
}

impl<'a> 
HasTransform<'a, Matrix2d> 
for LineContext<'a> {
    #[inline(always)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a> 
CanTransform<'a, LineContext<'a>, Matrix2d> 
for LineContext<'a> {
    #[inline(always)]
    fn transform(&'a self, value: Matrix2d) -> LineContext<'a> {
        LineContext {
            view: Borrowed(self.view.get()),
            transform: Value(value),
            line: Borrowed(self.line.get()),
        }
    }
}

impl<'a> 
HasViewTransform<'a, Matrix2d> 
for LineContext<'a> {
    #[inline(always)]
    fn get_view_transform(&'a self) -> &'a Matrix2d {
        self.view.get()
    }
}

impl<'a> 
CanViewTransform<'a, LineContext<'a>, Matrix2d> 
for LineContext<'a> {
    #[inline(always)]
    fn view_transform(
        &'a self, 
        value: Matrix2d
    ) -> LineContext<'a> {
        LineContext {
            view: Value(value),
            transform: Borrowed(self.transform.get()),
            line: Borrowed(self.line.get()),
        }
    }
}

impl<'a> 
AddColor<'a, LineColorContext<'a>> 
for LineContext<'a> {
    #[inline(always)]
    fn rgba(
        &'a self, 
        r: ColorComponent, 
        g: ColorComponent, 
        b: ColorComponent, 
        a: ColorComponent
    ) -> LineColorContext<'a> {
        LineColorContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            line: Borrowed(self.line.get()),
            color: Value([r, g, b, a]),
        }
    }
}

impl<'a> 
AddRoundBorder<'a, RoundBorderLineContext<'a>> 
for LineContext<'a> {
    #[inline(always)]
    fn round_border_radius(
        &'a self, 
        radius: Radius
    ) -> RoundBorderLineContext<'a> {
        RoundBorderLineContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            line: Borrowed(self.line.get()),
            round_border_radius: Value(radius),
        }
    }
}

impl<'a> 
AddBevelBorder<'a, BevelBorderLineContext<'a>> 
for LineContext<'a> {
    #[inline(always)]
    fn bevel_border_radius(
        &'a self, 
        radius: Radius
    ) -> BevelBorderLineContext<'a> {
        BevelBorderLineContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            line: Borrowed(self.line.get()),
            bevel_border_radius: Value(radius),
        }
    }
}

impl<'a> 
AddSquareBorder<'a, SquareBorderLineContext<'a>> 
for LineContext<'a> {
    #[inline(always)]
    fn square_border_radius(
        &'a self, 
        radius: Radius
    ) -> SquareBorderLineContext<'a> {
        SquareBorderLineContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            line: Borrowed(self.line.get()),
            square_border_radius: Value(radius),
        }
    }
}


