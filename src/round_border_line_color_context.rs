
use {
    BackEnd,
    Borrowed,
    Color,
    Clear,
    Field,
    Line,
    Matrix2d,
    Stroke,
    Value,
    View,
};
use triangulation::{
    with_round_border_line_tri_list_xy_f32_rgba_f32
};
use vecmath::{
    identity,
};
use internal::{
    CanColor,
    CanTransform,
    HasColor,
    HasTransform,
};

/// A line context with round border information.
pub struct RoundBorderLineColorContext<'a> {
    /// Base/original transform.
    pub base: Field<'a, Matrix2d>,
    /// Current transform.
    pub transform: Field<'a, Matrix2d>,
    /// Current line.
    pub line: Field<'a, Line>,
    /// Current color.
    pub color: Field<'a, Color>,
    /// Current round border.
    pub round_border_radius: Field<'a, f64>,
}

impl<'a> HasTransform<'a, Matrix2d> for RoundBorderLineColorContext<'a> {
    #[inline(always)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a> CanTransform<'a, RoundBorderLineColorContext<'a>, Matrix2d> for RoundBorderLineColorContext<'a> {
    #[inline(always)]
    fn transform(&'a self, value: Matrix2d) -> RoundBorderLineColorContext<'a> {
        RoundBorderLineColorContext {
            base: Borrowed(self.base.get()),
            transform: Value(value),
            line: Borrowed(self.line.get()),
            color: Borrowed(self.color.get()),
            round_border_radius: Borrowed(self.round_border_radius.get()),
        }
    }
}

impl<'a> HasColor<'a, Color> for RoundBorderLineColorContext<'a> {
    #[inline(always)]
    fn get_color(&'a self) -> &'a Color {
        self.color.get()
    }
}

impl<'a> CanColor<'a, RoundBorderLineColorContext<'a>, Color> for RoundBorderLineColorContext<'a> {
    #[inline(always)]
    fn color(&'a self, value: Color) -> RoundBorderLineColorContext<'a> {
        RoundBorderLineColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            line: Borrowed(self.line.get()),
            color: Value(value),
            round_border_radius: Borrowed(self.round_border_radius.get()),
        }
    }
}

impl<'a> Stroke<'a> for RoundBorderLineColorContext<'a> {
    fn stroke<B: BackEnd>(&'a self, back_end: &mut B) {
        if back_end.supports_tri_list_xy_f32_rgba_f32() {
            let line = self.line.get();
            let round_border_radius = self.round_border_radius.get();
            let color = self.color.get();
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely opaque.
            let needs_alpha = color[3] != 1.0;
            if needs_alpha { back_end.enable_alpha_blend(); }
            with_round_border_line_tri_list_xy_f32_rgba_f32(
                64,
                self.transform.get(),
                line,
                round_border_radius,
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

impl<'a> View<'a> for RoundBorderLineColorContext<'a> {
    #[inline(always)]
    fn view(&'a self) -> RoundBorderLineColorContext<'a> {
        RoundBorderLineColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.base.get()),
            line: Borrowed(self.line.get()),
            color: Borrowed(self.color.get()),
            round_border_radius: Borrowed(self.round_border_radius.get()),
        }
    }

    #[inline(always)]
    fn reset(&'a self) -> RoundBorderLineColorContext<'a> {
        RoundBorderLineColorContext {
            base: Borrowed(self.base.get()),
            transform: Value(identity()),
            line: Borrowed(self.line.get()),
            color: Borrowed(self.color.get()),
            round_border_radius: Borrowed(self.round_border_radius.get()),
        }
    }

    #[inline(always)]
    fn store_view(&'a self) -> RoundBorderLineColorContext<'a> {
        RoundBorderLineColorContext {
            base: Borrowed(self.transform.get()),
            transform: Borrowed(self.transform.get()),
            line: Borrowed(self.line.get()),
            color: Borrowed(self.color.get()),
            round_border_radius: Borrowed(self.round_border_radius.get()),
        }
    }
}

impl<'a> Clear for RoundBorderLineColorContext<'a> {
    fn clear<B: BackEnd>(&self, back_end: &mut B) {
        if back_end.supports_clear_rgba() {
            let color = self.color.get();
            back_end.clear_rgba(color[0], color[1], color[2], color[3]);
        }
    }
}

