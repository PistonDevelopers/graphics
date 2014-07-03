
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

/// A line context with bevel border information.
pub struct BevelBorderLineColorContext {
    /// View transform.
    pub view: Matrix2d,
    /// Current transform.
    pub transform: Matrix2d,
    /// Current line.
    pub line: Line,
    /// Current color.
    pub color: Color,
    /// Current bevel border.
    pub bevel_border_radius: Radius,
}

impl
Clone 
for BevelBorderLineColorContext {
    #[inline(always)]
    fn clone(&self) -> BevelBorderLineColorContext {
        BevelBorderLineColorContext {
            view: self.view,
            transform: self.transform,
            line: self.line,
            color: self.color,
            bevel_border_radius: self.bevel_border_radius,
        }
    }
}

impl
HasTransform<Matrix2d> 
for BevelBorderLineColorContext {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform
    }
}

impl
CanTransform<BevelBorderLineColorContext, Matrix2d> 
for BevelBorderLineColorContext {
    #[inline(always)]
    fn transform(
        &self, 
        value: Matrix2d
    ) -> BevelBorderLineColorContext {
        BevelBorderLineColorContext {
            view: self.view,
            transform: value,
            line: self.line,
            color: self.color,
            bevel_border_radius: self.bevel_border_radius,
        }
    }
}

impl
HasViewTransform<Matrix2d> 
for BevelBorderLineColorContext {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view
    }
}

impl
CanViewTransform<BevelBorderLineColorContext, Matrix2d> 
for BevelBorderLineColorContext {
    #[inline(always)]
    fn view_transform(
        &self, 
        value: Matrix2d
    ) -> BevelBorderLineColorContext {
        BevelBorderLineColorContext {
            view: value,
            transform: self.transform,
            line: self.line,
            color: self.color,
            bevel_border_radius: self.bevel_border_radius,
        }
    }
}

impl
HasColor<Color> 
for BevelBorderLineColorContext {
    #[inline(always)]
    fn get_color(&self) -> Color {
        self.color
    }
}

impl
CanColor<BevelBorderLineColorContext, Color> 
for BevelBorderLineColorContext {
    #[inline(always)]
    fn color(&self, value: Color) -> BevelBorderLineColorContext {
        BevelBorderLineColorContext {
            view: self.view,
            transform: self.transform,
            line: self.line,
            color: value,
            bevel_border_radius: self.bevel_border_radius,
        }
    }
}

impl<B: BackEnd<I>, I: ImageSize> 
Draw<B, I> 
for BevelBorderLineColorContext {
    #[inline(always)]
    fn draw(&self, back_end: &mut B) {
        if back_end.supports_tri_list_xy_f32_rgba_f32() {
            let line = self.line;
            let bevel_border_radius = self.bevel_border_radius;
            let color = self.color;
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely opaque.
            let needs_alpha = color[3] != 1.0;
            if needs_alpha { back_end.enable_alpha_blend(); }
            with_round_border_line_tri_list_xy_f32_rgba_f32(
                3,
                self.transform,
                line,
                bevel_border_radius,
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

