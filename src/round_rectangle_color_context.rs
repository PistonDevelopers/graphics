
use {
    BackEnd,
    Borrowed,
    Clear,
    Color,
    Field,
    Fill,
    Matrix2d,
    Rectangle,
    Value,
    View,
};
use vecmath::{
    identity,
};
use triangulation::{
    with_round_rectangle_tri_list_xy_f32_rgba_f32
};
use internal::{
    CanColor,
    CanRectangle,
    CanTransform,
    HasColor,
    HasRectangle,
    HasTransform,
};

/// A rectangle color context.
pub struct RoundRectangleColorContext<'a> {
    /// Base/original transformation.
    pub base: Field<'a, Matrix2d>,
    /// Current transformation.
    pub transform: Field<'a, Matrix2d>,
    /// Current rectangle.
    pub rect: Field<'a, Rectangle>,
    /// Current roundness radius.
    pub round_radius: Field<'a, f64>,
    /// Current color.
    pub color: Field<'a, Color>,
}

impl<'a> HasTransform<'a, Matrix2d> for RoundRectangleColorContext<'a> {
    #[inline(always)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a> CanTransform<'a, RoundRectangleColorContext<'a>, Matrix2d> for RoundRectangleColorContext<'a> {
    #[inline(always)]
    fn transform(&'a self, value: Matrix2d) -> RoundRectangleColorContext<'a> {
        RoundRectangleColorContext {
            base: Borrowed(self.base.get()),
            transform: Value(value),
            rect: Borrowed(self.rect.get()),
            round_radius: Borrowed(self.round_radius.get()),
            color: Borrowed(self.color.get()),
        }
    }
}

impl<'a> HasColor<'a, Color> for RoundRectangleColorContext<'a> {
    #[inline(always)]
    fn get_color(&'a self) -> &'a Color {
        self.color.get()
    }
}

impl<'a> CanColor<'a, RoundRectangleColorContext<'a>, Color> for RoundRectangleColorContext<'a> {
    #[inline(always)]
    fn color(&'a self, value: Color) -> RoundRectangleColorContext<'a> {
        RoundRectangleColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            color: Value(value),
            rect: Borrowed(self.rect.get()),
            round_radius: Borrowed(self.round_radius.get()),
        }
    }
}

impl<'a> HasRectangle<'a, Rectangle> for RoundRectangleColorContext<'a> {
    #[inline(always)]
    fn get_rectangle(&'a self) -> &'a Rectangle {
        self.rect.get()
    }
}

impl<'a> CanRectangle<'a, RoundRectangleColorContext<'a>, Rectangle> for RoundRectangleColorContext<'a> {
    #[inline(always)]
    fn rectangle(&'a self, rect: Rectangle) -> RoundRectangleColorContext<'a> {
        RoundRectangleColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            rect: Value(rect),
            round_radius: Borrowed(self.round_radius.get()),
            color: Borrowed(self.color.get()),
        }
    }
}

impl<'a> Clear for RoundRectangleColorContext<'a> {
    fn clear<B: BackEnd>(&self, back_end: &mut B) {
        if back_end.supports_clear_rgba() {
            let color = self.color.get();
            back_end.clear_rgba(color[0], color[1], color[2], color[3]);
        } else {
            unimplemented!();
        }
    }
}

impl<'a> Fill<'a> for RoundRectangleColorContext<'a> {
    fn fill<B: BackEnd>(&'a self, back_end: &mut B) {
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

impl<'a> View<'a> for RoundRectangleColorContext<'a> {
    #[inline(always)]
    fn view(&'a self) -> RoundRectangleColorContext<'a> {
        RoundRectangleColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.base.get()),
            rect: Borrowed(self.rect.get()),
            round_radius: Borrowed(self.round_radius.get()),
            color: Borrowed(self.color.get()),
        }
    }

    #[inline(always)]
    fn reset(&'a self) -> RoundRectangleColorContext<'a> {
        RoundRectangleColorContext {
            base: Borrowed(self.base.get()),
            transform: Value(identity()),
            rect: Borrowed(self.rect.get()),
            round_radius: Borrowed(self.round_radius.get()),
            color: Borrowed(self.color.get()),
        }
    }

    #[inline(always)]
    fn store_view(&'a self) -> RoundRectangleColorContext<'a> {
        RoundRectangleColorContext {
            base: Borrowed(self.transform.get()),
            transform: Borrowed(self.transform.get()),
            rect: Borrowed(self.rect.get()),
            round_radius: Borrowed(self.round_radius.get()),
            color: Borrowed(self.color.get()),
        }
    }
}

