use {
    AddBorder,
    AddColor,
    Field,
    BevelRectangleBorderContext,
    BevelRectangleColorContext,
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

/// A bevel rectangle context.
pub struct BevelRectangleContext {
    /// View transform.
    pub view: Field<Matrix2d>,
    /// Current transform.
    pub transform: Field<Matrix2d>,
    /// Current rectangle.
    pub rect: Field<Rectangle>,
    /// Current bevel radius.
    pub bevel_radius: Field<Radius>,
}

impl
Clone 
for BevelRectangleContext {
    #[inline(always)]
    fn clone(&self) -> BevelRectangleContext {
        BevelRectangleContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            rect: Value(self.rect.get()),
            bevel_radius: Value(self.bevel_radius.get()),
        }
    }
}

impl
HasTransform<Matrix2d> 
for BevelRectangleContext {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform.get()
    }
}

impl
CanTransform<BevelRectangleContext, Matrix2d> 
for BevelRectangleContext {
    #[inline(always)]
    fn transform(
        &self, 
        value: Matrix2d
    ) -> BevelRectangleContext {
        BevelRectangleContext {
            view: Value(self.view.get()),
            transform: Value(value),
            rect: Value(self.rect.get()),
            bevel_radius: Value(self.bevel_radius.get()),
        }
    }
}

impl
HasViewTransform<Matrix2d> 
for BevelRectangleContext {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view.get()
    }
}

impl
CanViewTransform<BevelRectangleContext, Matrix2d> 
for BevelRectangleContext {
    #[inline(always)]
    fn view_transform(
        &self, 
        value: Matrix2d
    ) -> BevelRectangleContext {
        BevelRectangleContext {
            view: Value(value),
            transform: Value(self.transform.get()),
            rect: Value(self.rect.get()),
            bevel_radius: Value(self.bevel_radius.get()),
        }
    }
}

impl
HasRectangle<Rectangle> 
for BevelRectangleContext {
    #[inline(always)]
    fn get_rectangle(&self) -> Rectangle {
        self.rect.get()
    }
}

impl
CanRectangle<BevelRectangleContext, Rectangle> 
for BevelRectangleContext {
    #[inline(always)]
    fn rectangle(
        &self, 
        rect: Rectangle
    ) -> BevelRectangleContext {
        BevelRectangleContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            rect: Value(rect),
            bevel_radius: Value(self.bevel_radius.get()),
        }
    }
}

impl
AddColor<BevelRectangleColorContext> 
for BevelRectangleContext {
    /// Creates a RectangleColorContext.
    #[inline(always)]
    fn rgba(
        &self, 
        r: ColorComponent, 
        g: ColorComponent, 
        b: ColorComponent, 
        a: ColorComponent
    ) -> BevelRectangleColorContext {
        BevelRectangleColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            color: Value([r, g, b, a]),
            rect: Value(self.rect.get()),
            bevel_radius: Value(self.bevel_radius.get()),
        }
    }
}

impl
AddBorder<BevelRectangleBorderContext> 
for BevelRectangleContext {
    #[inline(always)]
    fn border_radius(
        &self, 
        radius: f64
    ) -> BevelRectangleBorderContext {
        BevelRectangleBorderContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            rect: Value(self.rect.get()),
            bevel_radius: Value(self.bevel_radius.get()),
            border: Value(radius),
        }
    }
}

