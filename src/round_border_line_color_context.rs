
use {
    BackEnd,
    Borrowed,
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

/// A line context with round border information.
pub struct RoundBorderLineColorContext<'a> {
    /// View transform.
    pub view: Field<'a, Matrix2d>,
    /// Current transform.
    pub transform: Field<'a, Matrix2d>,
    /// Current line.
    pub line: Field<'a, Line>,
    /// Current color.
    pub color: Field<'a, Color>,
    /// Current round border.
    pub round_border_radius: Field<'a, Radius>,
}

impl<'a> 
Clone 
for RoundBorderLineColorContext<'a> {
    #[inline(always)]
    fn clone(&self) -> RoundBorderLineColorContext<'static> {
        RoundBorderLineColorContext {
            view: Value(*self.view.get()),
            transform: Value(*self.transform.get()),
            line: Value(*self.line.get()),
            color: Value(*self.color.get()),
            round_border_radius: Value(*self.round_border_radius.get()),
        }
    }
}

impl<'a> 
HasTransform<'a, Matrix2d> 
for RoundBorderLineColorContext<'a> {
    #[inline(always)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a> 
CanTransform<'a, RoundBorderLineColorContext<'a>, Matrix2d> 
for RoundBorderLineColorContext<'a> {
    #[inline(always)]
    fn transform(
        &'a self, 
        value: Matrix2d
    ) -> RoundBorderLineColorContext<'a> {
        RoundBorderLineColorContext {
            view: Borrowed(self.view.get()),
            transform: Value(value),
            line: Borrowed(self.line.get()),
            color: Borrowed(self.color.get()),
            round_border_radius: Borrowed(self.round_border_radius.get()),
        }
    }
}

impl<'a> 
HasViewTransform<'a, Matrix2d> 
for RoundBorderLineColorContext<'a> {
    #[inline(always)]
    fn get_view_transform(&'a self) -> &'a Matrix2d {
        self.view.get()
    }
}

impl<'a> 
CanViewTransform<'a, RoundBorderLineColorContext<'a>, Matrix2d> 
for RoundBorderLineColorContext<'a> {
    #[inline(always)]
    fn view_transform(
        &'a self, 
        value: Matrix2d
    ) -> RoundBorderLineColorContext<'a> {
        RoundBorderLineColorContext {
            view: Value(value),
            transform: Borrowed(self.transform.get()),
            line: Borrowed(self.line.get()),
            round_border_radius: Borrowed(self.round_border_radius.get()),
            color: Borrowed(self.color.get()),
        }
    }
}

impl<'a> 
HasColor<'a, Color> 
for RoundBorderLineColorContext<'a> {
    #[inline(always)]
    fn get_color(&'a self) -> &'a Color {
        self.color.get()
    }
}

impl<'a> 
CanColor<'a, RoundBorderLineColorContext<'a>, Color> 
for RoundBorderLineColorContext<'a> {
    #[inline(always)]
    fn color(
        &'a self, 
        value: Color
    ) -> RoundBorderLineColorContext<'a> {
        RoundBorderLineColorContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            line: Borrowed(self.line.get()),
            color: Value(value),
            round_border_radius: Borrowed(self.round_border_radius.get()),
        }
    }
}

impl<'a, B: BackEnd<I>, I: ImageSize> 
Draw<'a, B, I> 
for RoundBorderLineColorContext<'a> {
    #[inline(always)]
    fn draw(&'a self, back_end: &mut B) {
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
                *self.transform.get(),
                *line,
                *round_border_radius,
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

