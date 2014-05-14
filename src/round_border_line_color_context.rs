use {
    BackEnd,
    Color,
    Field,
    Line,
    Matrix2d,
    Stroke,
};
use triangulation::{
    with_round_border_line_tri_list_xy_f32_rgba_f32
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
                *color,
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
