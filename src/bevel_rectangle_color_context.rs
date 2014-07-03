use {
    AddBorder,
    BackEnd,
    BevelRectangleBorderColorContext,
    Draw,
    ImageSize,
};
use triangulation::{
    with_round_rectangle_tri_list_xy_f32_rgba_f32
};
use internal::{
    CanColor,
    CanRectangle,
    CanTransform,
    CanViewTransform,
    Color,
    HasColor,
    HasRectangle,
    HasTransform,
    HasViewTransform,
    Matrix2d,
    Radius,
    Rectangle,
};

/// A rectangle color context.
pub struct BevelRectangleColorContext {
    /// View transformation.
    pub view: Matrix2d,
    /// Current transformation.
    pub transform: Matrix2d,
    /// Current rectangle.
    pub rect: Rectangle,
    /// Current bevel radius.
    pub bevel_radius: Radius,
    /// Current color.
    pub color: Color,
}

impl
Clone 
for BevelRectangleColorContext {
    #[inline(always)]
    fn clone(&self) -> BevelRectangleColorContext {
        BevelRectangleColorContext {
            view: self.view,
            transform: self.transform,
            rect: self.rect,
            bevel_radius: self.bevel_radius,
            color: self.color,
        }
    }
}

impl
HasTransform<Matrix2d> 
for BevelRectangleColorContext {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform
    }
}

impl
CanTransform<BevelRectangleColorContext, Matrix2d> 
for BevelRectangleColorContext {
    #[inline(always)]
    fn transform(
        &self, 
        value: Matrix2d
    ) -> BevelRectangleColorContext {
        BevelRectangleColorContext {
            view: self.view,
            transform: value,
            rect: self.rect,
            bevel_radius: self.bevel_radius,
            color: self.color,
        }
    }
}

impl
HasViewTransform<Matrix2d> 
for BevelRectangleColorContext {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view
    }
}

impl
CanViewTransform<BevelRectangleColorContext, Matrix2d> 
for BevelRectangleColorContext {
    #[inline(always)]
    fn view_transform(
        &self, 
        value: Matrix2d
    ) -> BevelRectangleColorContext {
        BevelRectangleColorContext {
            view: value,
            transform: self.transform,
            rect: self.rect,
            bevel_radius: self.bevel_radius,
            color: self.color,
        }
    }
}

impl
HasColor<Color> 
for BevelRectangleColorContext {
    #[inline(always)]
    fn get_color(&self) -> Color {
        self.color
    }
}

impl
CanColor<BevelRectangleColorContext, Color> 
for BevelRectangleColorContext {
    #[inline(always)]
    fn color(&self, value: Color) -> BevelRectangleColorContext {
        BevelRectangleColorContext {
            view: self.view,
            transform: self.transform,
            color: value,
            rect: self.rect,
            bevel_radius: self.bevel_radius,
        }
    }
}

impl
HasRectangle<Rectangle> 
for BevelRectangleColorContext {
    #[inline(always)]
    fn get_rectangle(&self) -> Rectangle {
        self.rect
    }
}

impl
CanRectangle<BevelRectangleColorContext, Rectangle> 
for BevelRectangleColorContext {
    #[inline(always)]
    fn rectangle(
        &self, 
        rect: Rectangle
    ) -> BevelRectangleColorContext {
        BevelRectangleColorContext {
            view: self.view,
            transform: self.transform,
            rect: rect,
            bevel_radius: self.bevel_radius,
            color: self.color,
        }
    }
}

impl<B: BackEnd<I>, I: ImageSize> 
Draw<B, I> 
for BevelRectangleColorContext {
    #[inline(always)]
    fn draw(&self, back_end: &mut B) {
        if back_end.supports_tri_list_xy_f32_rgba_f32() {
            let rect = self.rect;
            let bevel_radius = self.bevel_radius;
            let color = self.color;
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely opaque.
            let needs_alpha = color[3] != 1.0;
            if needs_alpha { back_end.enable_alpha_blend(); }
            with_round_rectangle_tri_list_xy_f32_rgba_f32(
                2,
                self.transform,
                rect,
                bevel_radius,
                color,
                |vertices, colors| {
                    back_end.tri_list_xy_f32_rgba_f32(vertices, colors)
                }
            );
            if needs_alpha { back_end.disable_alpha_blend(); }
        } else {
            unimplemented!();
        }
    }
}

impl
AddBorder<BevelRectangleBorderColorContext> 
for BevelRectangleColorContext {
    #[inline(always)]
    fn border_radius(
        &self, 
        radius: f64
    ) -> BevelRectangleBorderColorContext {
        BevelRectangleBorderColorContext {
            view: self.view,
            transform: self.transform,
            rect: self.rect,
            bevel_radius: self.bevel_radius,
            color: self.color,
            border: radius,
        }
    }
}

