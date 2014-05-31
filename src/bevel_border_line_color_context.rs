
use {
    BackEnd,
    Borrowed,
    Clear,
    Field,
    Image,
    Stroke,
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
pub struct BevelBorderLineColorContext<'a> {
    /// View transform.
    pub view: Field<'a, Matrix2d>,
    /// Current transform.
    pub transform: Field<'a, Matrix2d>,
    /// Current line.
    pub line: Field<'a, Line>,
    /// Current color.
    pub color: Field<'a, Color>,
    /// Current bevel border.
    pub bevel_border_radius: Field<'a, Radius>,
}

impl<'a> Clone for BevelBorderLineColorContext<'a> {
    #[inline(always)]
    fn clone(&self) -> BevelBorderLineColorContext<'static> {
        BevelBorderLineColorContext {
            view: Value(*self.view.get()),
            transform: Value(*self.transform.get()),
            line: Value(*self.line.get()),
            color: Value(*self.color.get()),
            bevel_border_radius: Value(*self.bevel_border_radius.get()),
        }
    }
}

impl<'a> HasTransform<'a, Matrix2d> for BevelBorderLineColorContext<'a> {
    #[inline(always)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a> CanTransform<'a, BevelBorderLineColorContext<'a>, Matrix2d> for BevelBorderLineColorContext<'a> {
    #[inline(always)]
    fn transform(&'a self, value: Matrix2d) -> BevelBorderLineColorContext<'a> {
        BevelBorderLineColorContext {
            view: Borrowed(self.view.get()),
            transform: Value(value),
            line: Borrowed(self.line.get()),
            color: Borrowed(self.color.get()),
            bevel_border_radius: Borrowed(self.bevel_border_radius.get()),
        }
    }
}

impl<'a> HasViewTransform<'a, Matrix2d> for BevelBorderLineColorContext<'a> {
    #[inline(always)]
    fn get_view_transform(&'a self) -> &'a Matrix2d {
        self.view.get()
    }
}

impl<'a> CanViewTransform<'a, BevelBorderLineColorContext<'a>, Matrix2d> 
for BevelBorderLineColorContext<'a> {
    #[inline(always)]
    fn view_transform(&'a self, value: Matrix2d) -> BevelBorderLineColorContext<'a> {
        BevelBorderLineColorContext {
            view: Value(value),
            transform: Borrowed(self.transform.get()),
            line: Borrowed(self.line.get()),
            color: Borrowed(self.color.get()),
            bevel_border_radius: Borrowed(self.bevel_border_radius.get()),
        }
    }
}

impl<'a> HasColor<'a, Color> for BevelBorderLineColorContext<'a> {
    #[inline(always)]
    fn get_color(&'a self) -> &'a Color {
        self.color.get()
    }
}

impl<'a> CanColor<'a, BevelBorderLineColorContext<'a>, Color> for BevelBorderLineColorContext<'a> {
    #[inline(always)]
    fn color(&'a self, value: Color) -> BevelBorderLineColorContext<'a> {
        BevelBorderLineColorContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            line: Borrowed(self.line.get()),
            color: Value(value),
            bevel_border_radius: Borrowed(self.bevel_border_radius.get()),
        }
    }
}

impl<'a, B: BackEnd<I>, I: Image> Stroke<'a, B, I> for BevelBorderLineColorContext<'a> {
    #[inline(always)]
    fn stroke(&'a self, back_end: &mut B) {
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
                *self.transform.get(),
                *line,
                *bevel_border_radius,
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

impl<'a, B: BackEnd<I>, I: Image> Clear<B, I> for BevelBorderLineColorContext<'a> {
    #[inline(always)]
    fn clear(&self, back_end: &mut B) {
        if back_end.supports_clear_rgba() {
            let color = self.color.get();
            back_end.clear_rgba(color[0], color[1], color[2], color[3]);
        }
    }
}

