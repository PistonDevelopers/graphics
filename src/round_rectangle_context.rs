use {
    AddBorder,
    AddColor,
    Field,
    RoundRectangleBorderContext,
    RoundRectangleColorContext,
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

/// A round rectangle context.
pub struct RoundRectangleContext {
    /// View transform.
    pub view: Field<Matrix2d>,
    /// Current transform.
    pub transform: Field<Matrix2d>,
    /// Current rectangle.
    pub rect: Field<Rectangle>,
    /// Current roundness radius.
    pub round_radius: Field<Radius>,
}

impl
Clone 
for RoundRectangleContext {
    #[inline(always)]
    fn clone(&self) -> RoundRectangleContext {
        RoundRectangleContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            rect: Value(self.rect.get()),
            round_radius: Value(self.round_radius.get()),
        }
    }
}

impl
HasTransform<Matrix2d> 
for RoundRectangleContext {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform.get()
    }
}

impl
CanTransform<RoundRectangleContext, Matrix2d> 
for RoundRectangleContext {
    #[inline(always)]
    fn transform(
        &self, 
        value: Matrix2d
    ) -> RoundRectangleContext {
        RoundRectangleContext {
            view: Value(self.view.get()),
            transform: Value(value),
            rect: Value(self.rect.get()),
            round_radius: Value(self.round_radius.get()),
        }
    }
}

impl
HasViewTransform<Matrix2d> 
for RoundRectangleContext {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view.get()
    }
}

impl
CanViewTransform<RoundRectangleContext, Matrix2d> 
for RoundRectangleContext {
    #[inline(always)]
    fn view_transform(
        &self, 
        value: Matrix2d
    ) -> RoundRectangleContext {
        RoundRectangleContext {
            view: Value(value),
            transform: Value(self.transform.get()),
            rect: Value(self.rect.get()),
            round_radius: Value(self.round_radius.get()),
        }
    }
}

impl
HasRectangle<Rectangle> 
for RoundRectangleContext {
    #[inline(always)]
    fn get_rectangle(&self) -> Rectangle {
        self.rect.get()
    }
}

impl
CanRectangle<RoundRectangleContext, Rectangle> 
for RoundRectangleContext {
    #[inline(always)]
    fn rectangle(
        &self, 
        rect: Rectangle
    ) -> RoundRectangleContext {
        RoundRectangleContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            rect: Value(rect),
            round_radius: Value(self.round_radius.get()),
        }
    }
}

impl
AddColor<RoundRectangleColorContext> 
for RoundRectangleContext {
    /// Creates a RectangleColorContext.
    #[inline(always)]
    fn rgba(
        &self, 
        r: ColorComponent, 
        g: ColorComponent, 
        b: ColorComponent, 
        a: ColorComponent
    ) -> RoundRectangleColorContext {
        RoundRectangleColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            color: Value([r, g, b, a]),
            rect: Value(self.rect.get()),
            round_radius: Value(self.round_radius.get()),
        }
    }
}

impl
AddBorder<RoundRectangleBorderContext> 
for RoundRectangleContext {
    #[inline(always)]
    fn border_radius(
        &self, 
        radius: f64
    ) -> RoundRectangleBorderContext {
        RoundRectangleBorderContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            rect: Value(self.rect.get()),
            round_radius: Value(self.round_radius.get()),
            border: Value(radius),
        }
    }
}

