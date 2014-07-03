
use {
    BackEnd,
    ImageSize,
    Draw,
};
use triangulation::{
    with_round_border_line_tri_list_xy_f32_rgba_f32
};
use internal::{
    CanColor,
    CanTransform,
    CanViewTransform,
    Color,
    HasColor,
    HasTransform,
    HasViewTransform,
    Line,
    Matrix2d,
    Radius,
};

/// A line context with round border information.
pub struct RoundBorderLineColorContext {
    /// View transform.
    pub view: Matrix2d,
    /// Current transform.
    pub transform: Matrix2d,
    /// Current line.
    pub line: Line,
    /// Current color.
    pub color: Color,
    /// Current round border.
    pub round_border_radius: Radius,
}

impl
Clone 
for RoundBorderLineColorContext {
    #[inline(always)]
    fn clone(&self) -> RoundBorderLineColorContext {
        RoundBorderLineColorContext {
            view: self.view,
            transform: self.transform,
            line: self.line,
            color: self.color,
            round_border_radius: self.round_border_radius,
        }
    }
}

impl
HasTransform<Matrix2d> 
for RoundBorderLineColorContext {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform
    }
}

impl
CanTransform<RoundBorderLineColorContext, Matrix2d> 
for RoundBorderLineColorContext {
    #[inline(always)]
    fn transform(
        &self, 
        value: Matrix2d
    ) -> RoundBorderLineColorContext {
        RoundBorderLineColorContext {
            view: self.view,
            transform: value,
            line: self.line,
            color: self.color,
            round_border_radius: self.round_border_radius,
        }
    }
}

impl
HasViewTransform<Matrix2d> 
for RoundBorderLineColorContext {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view
    }
}

impl
CanViewTransform<RoundBorderLineColorContext, Matrix2d> 
for RoundBorderLineColorContext {
    #[inline(always)]
    fn view_transform(
        &self, 
        value: Matrix2d
    ) -> RoundBorderLineColorContext {
        RoundBorderLineColorContext {
            view: value,
            transform: self.transform,
            line: self.line,
            round_border_radius: self.round_border_radius,
            color: self.color,
        }
    }
}

impl
HasColor<Color> 
for RoundBorderLineColorContext {
    #[inline(always)]
    fn get_color(&self) -> Color {
        self.color
    }
}

impl
CanColor<RoundBorderLineColorContext, Color> 
for RoundBorderLineColorContext {
    #[inline(always)]
    fn color(
        &self, 
        value: Color
    ) -> RoundBorderLineColorContext {
        RoundBorderLineColorContext {
            view: self.view,
            transform: self.transform,
            line: self.line,
            color: value,
            round_border_radius: self.round_border_radius,
        }
    }
}

impl<B: BackEnd<I>, I: ImageSize> 
Draw<B, I> 
for RoundBorderLineColorContext {
    #[inline(always)]
    fn draw(&self, back_end: &mut B) {
        if back_end.supports_tri_list_xy_f32_rgba_f32() {
            let line = self.line;
            let round_border_radius = self.round_border_radius;
            let color = self.color;
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely opaque.
            let needs_alpha = color[3] != 1.0;
            if needs_alpha { back_end.enable_alpha_blend(); }
            with_round_border_line_tri_list_xy_f32_rgba_f32(
                64,
                self.transform,
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

