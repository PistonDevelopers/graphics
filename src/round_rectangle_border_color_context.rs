
use {
    BackEnd,
    Draw,
    Field,
    ImageSize,
    Value,
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
    pub view: Field<Matrix2d>,
    /// Current transformation.
    pub transform: Field<Matrix2d>,
    /// Current rectangle.
    pub rect: Field<Rectangle>,
    /// Current roundness radius.
    pub round_radius: Field<Radius>,
    /// Current color.
    pub color: Field<Color>,
    /// Current border.
    pub border: Field<Radius>,
}

impl Clone for RoundRectangleBorderColorContext {
    #[inline(always)]
    fn clone(&self) -> RoundRectangleBorderColorContext {
        RoundRectangleBorderColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            rect: Value(self.rect.get()),
            round_radius: Value(self.round_radius.get()),
            color: Value(self.color.get()),
            border: Value(self.border.get()),
        }
    }
}

impl HasTransform<Matrix2d> for RoundRectangleBorderColorContext {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform.get()
    }
}

impl CanTransform<RoundRectangleBorderColorContext, Matrix2d> 
for RoundRectangleBorderColorContext {
    #[inline(always)]
    fn transform(&self, value: Matrix2d) -> RoundRectangleBorderColorContext {
        RoundRectangleBorderColorContext {
            view: Value(self.view.get()),
            transform: Value(value),
            rect: Value(self.rect.get()),
            round_radius: Value(self.round_radius.get()),
            color: Value(self.color.get()),
            border: Value(self.border.get()),
        }
    }
}

impl HasViewTransform<Matrix2d> 
for RoundRectangleBorderColorContext {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view.get()
    }
}

impl CanViewTransform<RoundRectangleBorderColorContext, Matrix2d> 
for RoundRectangleBorderColorContext {
    #[inline(always)]
    fn view_transform(&self, value: Matrix2d) -> RoundRectangleBorderColorContext {
        RoundRectangleBorderColorContext {
            view: Value(value),
            transform: Value(self.transform.get()),
            rect: Value(self.rect.get()),
            round_radius: Value(self.round_radius.get()),
            color: Value(self.color.get()),
            border: Value(self.border.get()),
        }
    }
}

impl HasColor<Color> 
for RoundRectangleBorderColorContext {
    #[inline(always)]
    fn get_color(&self) -> Color {
        self.color.get()
    }
}

impl CanColor<RoundRectangleBorderColorContext, Color> 
for RoundRectangleBorderColorContext {
    #[inline(always)]
    fn color(&self, value: Color) -> RoundRectangleBorderColorContext {
        RoundRectangleBorderColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            color: Value(value),
            rect: Value(self.rect.get()),
            round_radius: Value(self.round_radius.get()),
            border: Value(self.border.get()),
        }
    }
}

impl HasRectangle<Rectangle> 
for RoundRectangleBorderColorContext {
    #[inline(always)]
    fn get_rectangle(&self) -> Rectangle {
        self.rect.get()
    }
}

impl CanRectangle<RoundRectangleBorderColorContext, Rectangle> 
for RoundRectangleBorderColorContext {
    #[inline(always)]
    fn rectangle(&self, rect: Rectangle) -> RoundRectangleBorderColorContext {
        RoundRectangleBorderColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            rect: Value(rect),
            round_radius: Value(self.round_radius.get()),
            color: Value(self.color.get()),
            border: Value(self.border.get()),
        }
    }
}

impl<B: BackEnd<I>, I: ImageSize>
Draw<B, I>
for RoundRectangleBorderColorContext {
    #[inline(always)]
    fn draw( &self, back_end: &mut B) {
        if back_end.supports_tri_list_xy_f32_rgba_f32() {
            let rect = self.rect.get();
            let color = self.color.get();
            let border_radius = self.border.get();
            let round_radius = self.round_radius.get();
            // Complete transparency does  not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not complete opaque.
            let needs_alpha = color[3] != 1.0;
            if needs_alpha { back_end.enable_alpha_blend(); }
            with_round_rectangle_border_tri_list_xy_f32_rgba_f32(
                128,
                self.transform.get(),
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


