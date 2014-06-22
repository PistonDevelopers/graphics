
use {
    AddColor,
    Borrowed,
    Field,
    BevelBorderLineColorContext,
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

/// A line context with bevel border information.
pub struct BevelBorderLineContext<'a> {
    /// View transform.
    pub view: Field<'a, Matrix2d>,
    /// Current transform.
    pub transform: Field<'a, Matrix2d>,
    /// Current line.
    pub line: Field<'a, Line>,
    /// Current bevel border.
    pub bevel_border_radius: Field<'a, Radius>,
}

impl<'a> 
Clone 
for BevelBorderLineContext<'a> {
    #[inline(always)]
    fn clone(&self) -> BevelBorderLineContext<'static> {
        BevelBorderLineContext {
            view: Value(*self.view.get()),
            transform: Value(*self.transform.get()),
            line: Value(*self.line.get()),
            bevel_border_radius: Value(*self.bevel_border_radius.get()),
        }
    }
}

impl<'a> 
HasTransform<'a, Matrix2d> 
for BevelBorderLineContext<'a> {
    #[inline(always)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a> 
CanTransform<'a, BevelBorderLineContext<'a>, Matrix2d> 
for BevelBorderLineContext<'a> {
    #[inline(always)]
    fn transform(&'a self, value: Matrix2d) -> BevelBorderLineContext<'a> {
        BevelBorderLineContext {
            view: Borrowed(self.view.get()),
            transform: Value(value),
            line: Borrowed(self.line.get()),
            bevel_border_radius: Borrowed(self.bevel_border_radius.get()),
        }
    }
}

impl<'a> 
HasViewTransform<'a, Matrix2d> 
for BevelBorderLineContext<'a> {
    #[inline(always)]
    fn get_view_transform(&'a self) -> &'a Matrix2d {
        self.view.get()
    }
}

impl<'a> 
CanViewTransform<'a, BevelBorderLineContext<'a>, Matrix2d> 
for BevelBorderLineContext<'a> {
    #[inline(always)]
    fn view_transform(
        &'a self, 
        value: Matrix2d
    ) -> BevelBorderLineContext<'a> {
        BevelBorderLineContext {
            view: Value(value),
            transform: Borrowed(self.transform.get()),
            line: Borrowed(self.line.get()),
            bevel_border_radius: Borrowed(self.bevel_border_radius.get()),
        }
    }
}

impl<'a> 
AddColor<'a, BevelBorderLineColorContext<'a>> 
for BevelBorderLineContext<'a> {
    #[inline(always)]
    fn rgba(
        &'a self, 
        r: ColorComponent, 
        g: ColorComponent, 
        b: ColorComponent, 
        a: ColorComponent
    ) -> BevelBorderLineColorContext<'a> {
        BevelBorderLineColorContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            line: Borrowed(self.line.get()),
            color: Value([r, g, b, a]),
            bevel_border_radius: Borrowed(self.bevel_border_radius.get()),
        }
    }
}

