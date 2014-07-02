use {
    AddColor,
    Field,
    RoundRectangleBorderColorContext,
    Value,
};
use internal::{
    CanRectangle,
    CanTransform,
    CanViewTransform,
    ColorComponent,
    HasRectangle,
    HasTransform,
    HasViewTransform,
    Matrix2d,
    Radius,
    Rectangle,
};

/// A round rectangle border context.
pub struct RoundRectangleBorderContext {
    /// View transform.
    pub view: Field<Matrix2d>,
    /// Current transform.
    pub transform: Field<Matrix2d>,
    /// Current rectangle.
    pub rect: Field<Rectangle>,
    /// Current roundness radius.
    pub round_radius: Field<Radius>,
    /// Curren tobrder.
    pub border: Field<Radius>,
}

impl Clone for RoundRectangleBorderContext {
    #[inline(always)]
    fn clone(&self) -> RoundRectangleBorderContext {
        RoundRectangleBorderContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            rect: Value(self.rect.get()),
            round_radius: Value(self.round_radius.get()),
            border: Value(self.border.get()),
        }
    }
}

impl HasTransform<Matrix2d> 
for RoundRectangleBorderContext {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform.get()
    }
}

impl CanTransform<RoundRectangleBorderContext, Matrix2d> 
for RoundRectangleBorderContext {
    #[inline(always)]
    fn transform(&self, value: Matrix2d) -> RoundRectangleBorderContext {
        RoundRectangleBorderContext {
            view: Value(self.view.get()),
            transform: Value(value),
            rect: Value(self.rect.get()),
            round_radius: Value(self.round_radius.get()),
            border: Value(self.border.get()),
        }
    }
}

impl HasViewTransform<Matrix2d> 
for RoundRectangleBorderContext {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view.get()
    }
}

impl CanViewTransform<RoundRectangleBorderContext, Matrix2d> 
for RoundRectangleBorderContext {
    #[inline(always)]
    fn view_transform(&self, value: Matrix2d) -> RoundRectangleBorderContext {
        RoundRectangleBorderContext {
            view: Value(value),
            transform: Value(self.transform.get()),
            rect: Value(self.rect.get()),
            round_radius: Value(self.round_radius.get()),
            border: Value(self.border.get()),
        }
    }
}

impl HasRectangle<Rectangle> 
for RoundRectangleBorderContext {
    #[inline(always)]
    fn get_rectangle(&self) -> Rectangle {
        self.rect.get()
    }
}

impl CanRectangle<RoundRectangleBorderContext, Rectangle> 
for RoundRectangleBorderContext {
    #[inline(always)]
    fn rectangle(&self, rect: Rectangle) -> RoundRectangleBorderContext {
        RoundRectangleBorderContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            rect: Value(rect),
            round_radius: Value(self.round_radius.get()),
            border: Value(self.border.get()),
        }
    }
}

impl AddColor<RoundRectangleBorderColorContext> for RoundRectangleBorderContext {
    #[inline(always)]
    fn rgba(
        &self, 
        r: ColorComponent, 
        g: ColorComponent, 
        b: ColorComponent, 
        a: ColorComponent
    ) -> RoundRectangleBorderColorContext {
        RoundRectangleBorderColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            color: Value([r, g, b, a]),
            rect: Value(self.rect.get()),
            round_radius: Value(self.round_radius.get()),
            border: Value(self.border.get()),
        }
    }
}

