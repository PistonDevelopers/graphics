
use {
    AddBorder,
    BackEnd,
    Draw,
    ImageSize,
    RoundRectangleBorderColorContext,
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

/// A round rectangle color context.
pub struct RoundRectangleColorContext {
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
}

impl
Clone 
for RoundRectangleColorContext {
    #[inline(always)]
    fn clone(&self) -> RoundRectangleColorContext {
        RoundRectangleColorContext {
            view: self.view,
            transform: self.transform,
            rect: self.rect,
            round_radius: self.round_radius,
            color: self.color,
        }
    }
}

impl
HasTransform<Matrix2d> 
for RoundRectangleColorContext {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform
    }
}

impl
CanTransform<RoundRectangleColorContext, Matrix2d> 
for RoundRectangleColorContext {
    #[inline(always)]
    fn transform(
        &self, 
        value: Matrix2d
    ) -> RoundRectangleColorContext {
        RoundRectangleColorContext {
            view: self.view,
            transform: value,
            rect: self.rect,
            round_radius: self.round_radius,
            color: self.color,
        }
    }
}

impl
HasViewTransform<Matrix2d> 
for RoundRectangleColorContext {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view
    }
}

impl
CanViewTransform<RoundRectangleColorContext, Matrix2d> 
for RoundRectangleColorContext {
    #[inline(always)]
    fn view_transform(
        &self, 
        value: Matrix2d
    ) -> RoundRectangleColorContext {
        RoundRectangleColorContext {
            view: value,
            transform: self.transform,
            rect: self.rect,
            round_radius: self.round_radius,
            color: self.color,
        }
    }
}

impl
HasColor<Color> 
for RoundRectangleColorContext {
    #[inline(always)]
    fn get_color(&self) -> Color {
        self.color
    }
}

impl
CanColor<RoundRectangleColorContext, Color> 
for RoundRectangleColorContext {
    #[inline(always)]
    fn color(
        &self, 
        value: Color
    ) -> RoundRectangleColorContext {
        RoundRectangleColorContext {
            view: self.view,
            transform: self.transform,
            color: value,
            rect: self.rect,
            round_radius: self.round_radius,
        }
    }
}

impl
HasRectangle<Rectangle> 
for RoundRectangleColorContext {
    #[inline(always)]
    fn get_rectangle(&self) -> Rectangle {
        self.rect
    }
}

impl
CanRectangle<RoundRectangleColorContext, Rectangle> 
for RoundRectangleColorContext {
    #[inline(always)]
    fn rectangle(
        &self, 
        rect: Rectangle
    ) -> RoundRectangleColorContext {
        RoundRectangleColorContext {
            view: self.view,
            transform: self.transform,
            rect: rect,
            round_radius: self.round_radius,
            color: self.color,
        }
    }
}

impl<B: BackEnd<I>, I: ImageSize> 
Draw<B, I> 
for RoundRectangleColorContext {
    #[inline(always)]
    fn draw(&self, back_end: &mut B) {
        if back_end.supports_tri_list_xy_f32_rgba_f32() {
            let rect = self.rect;
            let round_radius = self.round_radius;
            let color = self.color;
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely opaque.
            let needs_alpha = color[3] != 1.0;
            if needs_alpha { back_end.enable_alpha_blend(); }
            with_round_rectangle_tri_list_xy_f32_rgba_f32(
                32,
                self.transform,
                rect,
                round_radius,
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
AddBorder<RoundRectangleBorderColorContext> 
for RoundRectangleColorContext {
    #[inline(always)]
    fn border_radius(
        &self, 
        radius: f64
    ) -> RoundRectangleBorderColorContext {
        RoundRectangleBorderColorContext {
            view: self.view,
            transform: self.transform,
            rect: self.rect,
            round_radius: self.round_radius,
            color: self.color,
            border: radius,
        }
    }
}

