
use {
    BackEnd,
    Field,
    ImageSize,
    Draw,
    Value,
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
    pub view: Field<Matrix2d>,
    /// Current transform.
    pub transform: Field<Matrix2d>,
    /// Current line.
    pub line: Field<Line>,
    /// Current color.
    pub color: Field<Color>,
    /// Current bevel border.
    pub bevel_border_radius: Field<Radius>,
}

impl
Clone 
for BevelBorderLineColorContext {
    #[inline(always)]
    fn clone(&self) -> BevelBorderLineColorContext {
        BevelBorderLineColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            line: Value(self.line.get()),
            color: Value(self.color.get()),
            bevel_border_radius: Value(self.bevel_border_radius.get()),
        }
    }
}

impl
HasTransform<Matrix2d> 
for BevelBorderLineColorContext {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform.get()
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
            view: Value(self.view.get()),
            transform: Value(value),
            line: Value(self.line.get()),
            color: Value(self.color.get()),
            bevel_border_radius: Value(self.bevel_border_radius.get()),
        }
    }
}

impl
HasViewTransform<Matrix2d> 
for BevelBorderLineColorContext {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view.get()
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
            view: Value(value),
            transform: Value(self.transform.get()),
            line: Value(self.line.get()),
            color: Value(self.color.get()),
            bevel_border_radius: Value(self.bevel_border_radius.get()),
        }
    }
}

impl
HasColor<Color> 
for BevelBorderLineColorContext {
    #[inline(always)]
    fn get_color(&self) -> Color {
        self.color.get()
    }
}

impl
CanColor<BevelBorderLineColorContext, Color> 
for BevelBorderLineColorContext {
    #[inline(always)]
    fn color(&self, value: Color) -> BevelBorderLineColorContext {
        BevelBorderLineColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            line: Value(self.line.get()),
            color: Value(value),
            bevel_border_radius: Value(self.bevel_border_radius.get()),
        }
    }
}

impl<B: BackEnd<I>, I: ImageSize> 
Draw<B, I> 
for BevelBorderLineColorContext {
    #[inline(always)]
    fn draw(&self, back_end: &mut B) {
        if back_end.supports_tri_list_xy_f32_rgba_f32() {
            let line = self.line.get();
            let bevel_border_radius = self.bevel_border_radius.get();
            let color = self.color.get();
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely opaque.
            let needs_alpha = color[3] != 1.0;
            if needs_alpha { back_end.enable_alpha_blend(); }
            with_round_border_line_tri_list_xy_f32_rgba_f32(
                3,
                self.transform.get(),
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

