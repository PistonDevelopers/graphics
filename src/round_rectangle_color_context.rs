
use {
    BackEnd,
    Borrowed, 
    CanColor,
    CanTransform,
    Clear, 
    Color,
    Field, 
    Fill, 
    HasColor,
    HasTransform,
    Matrix2d, 
    RelativeRectangle, 
    RoundRectangle, 
    Value,
    View,
};
use vecmath::{
    identity,
    margin_round_rectangle, 
    relative_round_rectangle, 
};
use triangulation::{
    with_round_rectangle_tri_list_xy_f32_rgba_f32
};

/// A rectangle color context.
pub struct RoundRectangleColorContext<'a> {
    /// Base/original transformation.
    pub base: Field<'a, Matrix2d>,
    /// Current transformation.
    pub transform: Field<'a, Matrix2d>,
    /// Current rectangle.
    pub round_rect: Field<'a, RoundRectangle>,
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
            round_rect: Borrowed(self.round_rect.get()),
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
            round_rect: Borrowed(self.round_rect.get()),
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

impl<'a> RelativeRectangle<'a> for RoundRectangleColorContext<'a> {
    #[inline(always)]
    fn margin(&'a self, m: f64) -> RoundRectangleColorContext<'a> {
        RoundRectangleColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            color: Borrowed(self.color.get()),
            round_rect: Value(margin_round_rectangle(self.round_rect.get(), m)),
        }
    }

    #[inline(always)]
    fn rel(&'a self, x: f64, y: f64) -> RoundRectangleColorContext<'a> {
        RoundRectangleColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            color: Borrowed(self.color.get()),
            round_rect: Value(relative_round_rectangle(self.round_rect.get(), x, y)),
        }
    }
}

impl<'a> Fill<'a> for RoundRectangleColorContext<'a> {
    fn fill<B: BackEnd>(&'a self, back_end: &mut B) {
        if back_end.supports_tri_list_xy_f32_rgba_f32() {
            let round_rect = self.round_rect.get();
            let color = self.color.get();
            let color: [f32, ..4] = [color[0], color[1], color[2], color[3]];
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely opaque.
            let needs_alpha = color[3] != 1.0;
            if needs_alpha { back_end.enable_alpha_blend(); }
            with_round_rectangle_tri_list_xy_f32_rgba_f32(
                32,
                self.transform.get(),
                round_rect,
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
            round_rect: Borrowed(self.round_rect.get()),
            color: Borrowed(self.color.get()),
        }
    }

    #[inline(always)]
    fn reset(&'a self) -> RoundRectangleColorContext<'a> {
        RoundRectangleColorContext {
            base: Borrowed(self.base.get()),
            transform: Value(identity()),
            round_rect: Borrowed(self.round_rect.get()),
            color: Borrowed(self.color.get()),
        }
    }

    #[inline(always)]
    fn store_view(&'a self) -> RoundRectangleColorContext<'a> {
        RoundRectangleColorContext {
            base: Borrowed(self.transform.get()),
            transform: Borrowed(self.transform.get()),
            round_rect: Borrowed(self.round_rect.get()),
            color: Borrowed(self.color.get()),
        }
    }
}

