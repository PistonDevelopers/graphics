use {
    AddColor,
    BevelRectangleBorderColorContext,
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
    pub view: Matrix2d,
    /// Current transform.
    pub transform: Matrix2d,
    /// Current rectangle.
    pub rect: Rectangle,
    /// Current bevel radius.
    pub bevel_radius: Radius,
    /// Current border.
    pub border: Radius,
}

impl
Clone 
for BevelRectangleBorderContext {
    #[inline(always)]
    fn clone(&self) -> BevelRectangleBorderContext {
        BevelRectangleBorderContext {
            view: self.view,
            transform: self.transform,
            rect: self.rect,
            bevel_radius: self.bevel_radius,
            border: self.border,
        }
    }
}

impl
HasTransform<Matrix2d> 
for BevelRectangleBorderContext {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform
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
            view: self.view,
            transform: value,
            rect: self.rect,
            bevel_radius: self.bevel_radius,
            border: self.border,
        }
    }
}

impl
HasViewTransform<Matrix2d> 
for BevelRectangleBorderContext {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view
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
            view: value,
            transform: self.transform,
            rect: self.rect,
            bevel_radius: self.bevel_radius,
            border: self.border,
        }
    }
}

impl
HasRectangle<Rectangle> 
for BevelRectangleBorderContext {
    #[inline(always)]
    fn get_rectangle(&self) -> Rectangle {
        self.rect
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
            view: self.view,
            transform: self.transform,
            rect: rect,
            bevel_radius: self.bevel_radius,
            border: self.border,
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
            view: self.view,
            transform: self.transform,
            color: [r, g, b, a],
            rect: self.rect,
            bevel_radius: self.bevel_radius,
            border: self.border,
        }
    }
}

