use {
    AddColor,
    Field,
    BevelRectangleBorderColorContext,
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

/// A bevel rectangle border context.
pub struct BevelRectangleBorderContext {
    /// View transform.
    pub view: Field<Matrix2d>,
    /// Current transform.
    pub transform: Field<Matrix2d>,
    /// Current rectangle.
    pub rect: Field<Rectangle>,
    /// Current bevel radius.
    pub bevel_radius: Field<Radius>,
    /// Current border.
    pub border: Field<Radius>,
}

impl
Clone 
for BevelRectangleBorderContext {
    #[inline(always)]
    fn clone(&self) -> BevelRectangleBorderContext {
        BevelRectangleBorderContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            rect: Value(self.rect.get()),
            bevel_radius: Value(self.bevel_radius.get()),
            border: Value(self.border.get()),
        }
    }
}

impl
HasTransform<Matrix2d> 
for BevelRectangleBorderContext {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform.get()
    }
}

impl
CanTransform<BevelRectangleBorderContext, Matrix2d> 
for BevelRectangleBorderContext {
    #[inline(always)]
    fn transform(
        &self, 
        value: Matrix2d
    ) -> BevelRectangleBorderContext {
        BevelRectangleBorderContext {
            view: Value(self.view.get()),
            transform: Value(value),
            rect: Value(self.rect.get()),
            bevel_radius: Value(self.bevel_radius.get()),
            border: Value(self.border.get()),
        }
    }
}

impl
HasViewTransform<Matrix2d> 
for BevelRectangleBorderContext {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view.get()
    }
}

impl
CanViewTransform<BevelRectangleBorderContext, Matrix2d> 
for BevelRectangleBorderContext {
    #[inline(always)]
    fn view_transform(
        &self, 
        value: Matrix2d
    ) -> BevelRectangleBorderContext {
        BevelRectangleBorderContext {
            view: Value(value),
            transform: Value(self.transform.get()),
            rect: Value(self.rect.get()),
            bevel_radius: Value(self.bevel_radius.get()),
            border: Value(self.border.get()),
        }
    }
}

impl
HasRectangle<Rectangle> 
for BevelRectangleBorderContext {
    #[inline(always)]
    fn get_rectangle(&self) -> Rectangle {
        self.rect.get()
    }
}

impl
CanRectangle<BevelRectangleBorderContext, Rectangle> 
for BevelRectangleBorderContext {
    #[inline(always)]
    fn rectangle(
        &self, 
        rect: Rectangle
    ) -> BevelRectangleBorderContext {
        BevelRectangleBorderContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            rect: Value(rect),
            bevel_radius: Value(self.bevel_radius.get()),
            border: Value(self.border.get()),
        }
    }
}

impl
AddColor<BevelRectangleBorderColorContext> 
for BevelRectangleBorderContext {
    #[inline(always)]
    fn rgba(
        &self, 
        r: ColorComponent, 
        g: ColorComponent, 
        b: ColorComponent, 
        a: ColorComponent
    ) -> BevelRectangleBorderColorContext {
        BevelRectangleBorderColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            color: Value([r, g, b, a]),
            rect: Value(self.rect.get()),
            bevel_radius: Value(self.bevel_radius.get()),
            border: Value(self.border.get()),
        }
    }
}

