
use {
    BackEnd,
    Draw,
    ImageSize,
};
use triangulation::{
    with_round_rectangle_border_tri_list_xy_f32_rgba_f32
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

/// A round rectangle border color context.
pub struct RoundRectangleBorderColorContext {
    /// View transformation.
    pub view: Matrix2d,
    /// Current transformation.
    pub transform: Matrix2d,
    /// Current rectangle.
    pub rect: Rectangle,
    /// Current roundness radius.
    pub round_radius: Radius,
    /// Current color.
    pub color: Color,
    /// Current border.
    pub border: Radius,
}

impl Clone for RoundRectangleBorderColorContext {
    #[inline(always)]
    fn clone(&self) -> RoundRectangleBorderColorContext {
        RoundRectangleBorderColorContext {
            view: self.view,
            transform: self.transform,
            rect: self.rect,
            round_radius: self.round_radius,
            color: self.color,
            border: self.border,
        }
    }
}

impl HasTransform<Matrix2d> for RoundRectangleBorderColorContext {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform
    }
}

impl CanTransform<RoundRectangleBorderColorContext, Matrix2d> 
for RoundRectangleBorderColorContext {
    #[inline(always)]
    fn transform(&self, value: Matrix2d) -> RoundRectangleBorderColorContext {
        RoundRectangleBorderColorContext {
            view: self.view,
            transform: value,
            rect: self.rect,
            round_radius: self.round_radius,
            color: self.color,
            border: self.border,
        }
    }
}

impl HasViewTransform<Matrix2d> 
for RoundRectangleBorderColorContext {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view
    }
}

impl CanViewTransform<RoundRectangleBorderColorContext, Matrix2d> 
for RoundRectangleBorderColorContext {
    #[inline(always)]
    fn view_transform(&self, value: Matrix2d) -> RoundRectangleBorderColorContext {
        RoundRectangleBorderColorContext {
            view: value,
            transform: self.transform,
            rect: self.rect,
            round_radius: self.round_radius,
            color: self.color,
            border: self.border,
        }
    }
}

impl HasColor<Color> 
for RoundRectangleBorderColorContext {
    #[inline(always)]
    fn get_color(&self) -> Color {
        self.color
    }
}

impl CanColor<RoundRectangleBorderColorContext, Color> 
for RoundRectangleBorderColorContext {
    #[inline(always)]
    fn color(&self, value: Color) -> RoundRectangleBorderColorContext {
        RoundRectangleBorderColorContext {
            view: self.view,
            transform: self.transform,
            color: value,
            rect: self.rect,
            round_radius: self.round_radius,
            border: self.border,
        }
    }
}

impl HasRectangle<Rectangle> 
for RoundRectangleBorderColorContext {
    #[inline(always)]
    fn get_rectangle(&self) -> Rectangle {
        self.rect
    }
}

impl CanRectangle<RoundRectangleBorderColorContext, Rectangle> 
for RoundRectangleBorderColorContext {
    #[inline(always)]
    fn rectangle(&self, rect: Rectangle) -> RoundRectangleBorderColorContext {
        RoundRectangleBorderColorContext {
            view: self.view,
            transform: self.transform,
            rect: rect,
            round_radius: self.round_radius,
            color: self.color,
            border: self.border,
        }
    }
}

impl<B: BackEnd<I>, I: ImageSize>
Draw<B, I>
for RoundRectangleBorderColorContext {
    #[inline(always)]
    fn draw( &self, back_end: &mut B) {
        if back_end.supports_tri_list_xy_f32_rgba_f32() {
            let rect = self.rect;
            let color = self.color;
            let border_radius = self.border;
            let round_radius = self.round_radius;
            // Complete transparency does  not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not complete opaque.
            let needs_alpha = color[3] != 1.0;
            if needs_alpha { back_end.enable_alpha_blend(); }
            with_round_rectangle_border_tri_list_xy_f32_rgba_f32(
                128,
                self.transform,
                rect,
                round_radius,
                border_radius,
                color,
                |vertices, colors| {
                    back_end.tri_list_xy_f32_rgba_f32(vertices, colors)
                }
            );
            if needs_alpha { back_end.disable_alpha_blend(); }
        }
    }
}


