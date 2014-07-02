
use {
    AddBorder,
    BackEnd,
    Field,
    Draw,
    ImageSize,
    RoundRectangleBorderColorContext,
    Value,
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
    pub view: Field<Matrix2d>,
    /// Current transformation.
    pub transform: Field<Matrix2d>,
    /// Current rectangle.
    pub rect: Field<Rectangle>,
    /// Current roundness radius.
    pub round_radius: Field<Radius>,
    /// Current color.
    pub color: Field<Color>,
}

impl
Clone 
for RoundRectangleColorContext {
    #[inline(always)]
    fn clone(&self) -> RoundRectangleColorContext {
        RoundRectangleColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            rect: Value(self.rect.get()),
            round_radius: Value(self.round_radius.get()),
            color: Value(self.color.get()),
        }
    }
}

impl
HasTransform<Matrix2d> 
for RoundRectangleColorContext {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform.get()
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
            view: Value(self.view.get()),
            transform: Value(value),
            rect: Value(self.rect.get()),
            round_radius: Value(self.round_radius.get()),
            color: Value(self.color.get()),
        }
    }
}

impl
HasViewTransform<Matrix2d> 
for RoundRectangleColorContext {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view.get()
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
            view: Value(value),
            transform: Value(self.transform.get()),
            rect: Value(self.rect.get()),
            round_radius: Value(self.round_radius.get()),
            color: Value(self.color.get()),
        }
    }
}

impl
HasColor<Color> 
for RoundRectangleColorContext {
    #[inline(always)]
    fn get_color(&self) -> Color {
        self.color.get()
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
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            color: Value(value),
            rect: Value(self.rect.get()),
            round_radius: Value(self.round_radius.get()),
        }
    }
}

impl
HasRectangle<Rectangle> 
for RoundRectangleColorContext {
    #[inline(always)]
    fn get_rectangle(&self) -> Rectangle {
        self.rect.get()
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
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            rect: Value(rect),
            round_radius: Value(self.round_radius.get()),
            color: Value(self.color.get()),
        }
    }
}

impl<B: BackEnd<I>, I: ImageSize> 
Draw<B, I> 
for RoundRectangleColorContext {
    #[inline(always)]
    fn draw(&self, back_end: &mut B) {
        if back_end.supports_tri_list_xy_f32_rgba_f32() {
            let rect = self.rect.get();
            let round_radius = self.round_radius.get();
            let color = self.color.get();
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely opaque.
            let needs_alpha = color[3] != 1.0;
            if needs_alpha { back_end.enable_alpha_blend(); }
            with_round_rectangle_tri_list_xy_f32_rgba_f32(
                32,
                self.transform.get(),
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
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            rect: Value(self.rect.get()),
            round_radius: Value(self.round_radius.get()),
            color: Value(self.color.get()),
            border: Value(radius),
        }
    }
}

