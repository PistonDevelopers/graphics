
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

/// A line context with square border information.
pub struct SquareBorderLineColorContext {
    /// View transform.
    pub view: Matrix2d,
    /// Current transform.
    pub transform: Matrix2d,
    /// Current line.
    pub line: Line,
    /// Current color.
    pub color: Color,
    /// Current square border.
    pub square_border_radius: Radius,
}

impl
Clone 
for SquareBorderLineColorContext {
    #[inline(always)]
    fn clone(&self) -> SquareBorderLineColorContext {
        SquareBorderLineColorContext {
            view: self.view,
            transform: self.transform,
            line: self.line,
            color: self.color,
            square_border_radius: self.square_border_radius,
        }
    }
}

impl
HasTransform<Matrix2d> 
for SquareBorderLineColorContext {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform
    }
}

impl
CanTransform<SquareBorderLineColorContext, Matrix2d> 
for SquareBorderLineColorContext {
    #[inline(always)]
    fn transform(
        &self, 
        value: Matrix2d
    ) -> SquareBorderLineColorContext {
        SquareBorderLineColorContext {
            view: self.view,
            transform: value,
            line: self.line,
            color: self.color,
            square_border_radius: self.square_border_radius,
        }
    }
}

impl
HasViewTransform<Matrix2d> 
for SquareBorderLineColorContext {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view
    }
}

impl
CanViewTransform<SquareBorderLineColorContext, Matrix2d> 
for SquareBorderLineColorContext {
    #[inline(always)]
    fn view_transform(
        &self, 
        value: Matrix2d
    ) -> SquareBorderLineColorContext {
        SquareBorderLineColorContext {
            view: value,
            transform: self.transform,
            line: self.line,
            square_border_radius: self.square_border_radius,
            color: self.color,
        }
    }
}

impl
HasColor<Color> 
for SquareBorderLineColorContext {
    #[inline(always)]
    fn get_color(&self) -> Color {
        self.color
    }
}

impl
CanColor<SquareBorderLineColorContext, Color> 
for SquareBorderLineColorContext {
    #[inline(always)]
    fn color(
        &self, 
        value: Color
    ) -> SquareBorderLineColorContext {
        SquareBorderLineColorContext {
            view: self.view,
            transform: self.transform,
            line: self.line,
            color: value,
            square_border_radius: self.square_border_radius,
        }
    }
}

impl<B: BackEnd<I>, I: ImageSize> 
Draw<B, I> 
for SquareBorderLineColorContext {
    #[inline(always)]
    fn draw(&self, back_end: &mut B) {
        if back_end.supports_tri_list_xy_f32_rgba_f32() {
            let line = self.line;
            let square_border_radius = self.square_border_radius;
            let color = self.color;
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely opaque.
            let needs_alpha = color[3] != 1.0;
            if needs_alpha { back_end.enable_alpha_blend(); }
            with_round_border_line_tri_list_xy_f32_rgba_f32(
                2,
                self.transform,
                line,
                square_border_radius,
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

