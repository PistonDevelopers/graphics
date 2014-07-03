use {
    AddBorder,
    AddColor,
    BevelRectangleBorderContext,
    BevelRectangleColorContext,
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
    pub view: Matrix2d,
    /// Current transform.
    pub transform: Matrix2d,
    /// Current rectangle.
    pub rect: Rectangle,
    /// Current bevel radius.
    pub bevel_radius: Radius,
}

impl
Clone 
for BevelRectangleContext {
    #[inline(always)]
    fn clone(&self) -> BevelRectangleContext {
        BevelRectangleContext {
            view: self.view,
            transform: self.transform,
            rect: self.rect,
            bevel_radius: self.bevel_radius,
        }
    }
}

impl
HasTransform<Matrix2d> 
for BevelRectangleContext {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform
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
            view: self.view,
            transform: value,
            rect: self.rect,
            bevel_radius: self.bevel_radius,
        }
    }
}

impl
HasViewTransform<Matrix2d> 
for BevelRectangleContext {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view
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
            view: value,
            transform: self.transform,
            rect: self.rect,
            bevel_radius: self.bevel_radius,
        }
    }
}

impl
HasRectangle<Rectangle> 
for BevelRectangleContext {
    #[inline(always)]
    fn get_rectangle(&self) -> Rectangle {
        self.rect
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
            view: self.view,
            transform: self.transform,
            rect: rect,
            bevel_radius: self.bevel_radius,
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
            view: self.view,
            transform: self.transform,
            color: [r, g, b, a],
            rect: self.rect,
            bevel_radius: self.bevel_radius,
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
            view: self.view,
            transform: self.transform,
            rect: self.rect,
            bevel_radius: self.bevel_radius,
            border: radius,
        }
    }
}

