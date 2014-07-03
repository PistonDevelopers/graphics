use {
    BackEnd,
    ImageSize,
    Draw,
};
use triangulation::{
    with_round_rectangle_border_tri_list_xy_f32_rgba_f32,
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

/// A bevel rectangle border color context.
pub struct BevelRectangleBorderColorContext {
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
    /// Current border.
    pub border: Radius,
}

impl
Clone 
for BevelRectangleBorderColorContext {
    #[inline(always)]
    fn clone(&self) -> BevelRectangleBorderColorContext {
        BevelRectangleBorderColorContext {
            view: self.view,
            transform: self.transform,
            rect: self.rect,
            bevel_radius: self.bevel_radius,
            color: self.color,
            border: self.border,
        }
    }
}

impl
HasTransform<Matrix2d> 
for BevelRectangleBorderColorContext {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform
    }
}

impl
CanTransform<BevelRectangleBorderColorContext, Matrix2d> 
for BevelRectangleBorderColorContext {
    #[inline(always)]
    fn transform(
        &self, 
        value: Matrix2d
    ) -> BevelRectangleBorderColorContext {
        BevelRectangleBorderColorContext {
            view: self.view,
            transform: value,
            rect: self.rect,
            bevel_radius: self.bevel_radius,
            color: self.color,
            border: self.border,
        }
    }
}

impl
HasViewTransform<Matrix2d> 
for BevelRectangleBorderColorContext {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view
    }
}

impl
CanViewTransform<BevelRectangleBorderColorContext, Matrix2d> 
for BevelRectangleBorderColorContext {
    #[inline(always)]
    fn view_transform(
        &self, 
        value: Matrix2d
    ) -> BevelRectangleBorderColorContext {
        BevelRectangleBorderColorContext {
            view: value,
            transform: self.transform,
            rect: self.rect,
            bevel_radius: self.bevel_radius,
            color: self.color,
            border: self.border,
        }
    }
}

impl
HasColor<Color> 
for BevelRectangleBorderColorContext {
    #[inline(always)]
    fn get_color(&self) -> Color {
        self.color
    }
}

impl
CanColor<BevelRectangleBorderColorContext, Color> 
for BevelRectangleBorderColorContext {
    #[inline(always)]
    fn color(
        &self, 
        value: Color
    ) -> BevelRectangleBorderColorContext {
        BevelRectangleBorderColorContext {
            view: self.view,
            transform: self.transform,
            color: value,
            rect: self.rect,
            bevel_radius: self.bevel_radius,
            border: self.border,
        }
    }
}

impl
HasRectangle<Rectangle> 
for BevelRectangleBorderColorContext {
    #[inline(always)]
    fn get_rectangle(&self) -> Rectangle {
        self.rect
    }
}

impl
CanRectangle<BevelRectangleBorderColorContext, Rectangle> 
for BevelRectangleBorderColorContext {
    #[inline(always)]
    fn rectangle(
        &self, 
        rect: Rectangle
    ) -> BevelRectangleBorderColorContext {
        BevelRectangleBorderColorContext {
            view: self.view,
            transform: self.transform,
            rect: rect,
            bevel_radius: self.bevel_radius,
            color: self.color,
            border: self.border,
        }
    }
}

impl<B: BackEnd<I>, I: ImageSize> 
Draw<B, I> 
for BevelRectangleBorderColorContext {
    #[inline(always)]
    fn draw(&self, back_end: &mut B) {
        if back_end.supports_tri_list_xy_f32_rgba_f32() {
            let rect = self.rect;
            let bevel_radius = self.bevel_radius;
            let border = self.border;
            let color = self.color;
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely opaque.
            let needs_alpha = color[3] != 1.0;
            if needs_alpha { back_end.enable_alpha_blend(); }
            with_round_rectangle_border_tri_list_xy_f32_rgba_f32(
                2,
                self.transform,
                rect,
                bevel_radius,
                border,
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
