
use {
    AddBorder,
    BackEnd,
    Borrowed,
    Clear,
    Field,
    Fill,
    Image,
    RoundRectangleBorderColorContext,
    Value,
};
use triangulation::{
    with_round_rectangle_tri_list_xy_f32_rgba_f32
};
use internal::{
    CanColor,
    CanRectangle,
    CanTransform,
    CanViewTransform,
    Color,
    HasColor,
    HasRectangle,
    HasTransform,
    HasViewTransform,
    Matrix2d,
    Radius,
    Rectangle,
};

/// A round rectangle color context.
pub struct RoundRectangleColorContext<'a> {
    /// View transformation.
    pub view: Field<'a, Matrix2d>,
    /// Current transformation.
    pub transform: Field<'a, Matrix2d>,
    /// Current rectangle.
    pub rect: Field<'a, Rectangle>,
    /// Current roundness radius.
    pub round_radius: Field<'a, Radius>,
    /// Current color.
    pub color: Field<'a, Color>,
}

impl<'a> Clone for RoundRectangleColorContext<'a> {
    #[inline(always)]
    fn clone(&self) -> RoundRectangleColorContext<'static> {
        RoundRectangleColorContext {
            view: Value(*self.view.get()),
            transform: Value(*self.transform.get()),
            rect: Value(*self.rect.get()),
            round_radius: Value(*self.round_radius.get()),
            color: Value(*self.color.get()),
        }
    }
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
            view: Borrowed(self.view.get()),
            transform: Value(value),
            rect: Borrowed(self.rect.get()),
            round_radius: Borrowed(self.round_radius.get()),
            color: Borrowed(self.color.get()),
        }
    }
}

impl<'a> HasViewTransform<'a, Matrix2d> for RoundRectangleColorContext<'a> {
    #[inline(always)]
    fn get_view_transform(&'a self) -> &'a Matrix2d {
        self.view.get()
    }
}

impl<'a> CanViewTransform<'a, RoundRectangleColorContext<'a>, Matrix2d> 
for RoundRectangleColorContext<'a> {
    #[inline(always)]
    fn view_transform(&'a self, value: Matrix2d) -> RoundRectangleColorContext<'a> {
        RoundRectangleColorContext {
            view: Value(value),
            transform: Borrowed(self.transform.get()),
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
            view: Borrowed(self.view.get()),
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
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            rect: Value(rect),
            round_radius: Borrowed(self.round_radius.get()),
            color: Borrowed(self.color.get()),
        }
    }
}

impl<'a, B: BackEnd<I>, I: Image> Clear<B, I> for RoundRectangleColorContext<'a> {
    fn clear(&self, back_end: &mut B) {
        if back_end.supports_clear_rgba() {
            let color = self.color.get();
            back_end.clear_rgba(color[0], color[1], color[2], color[3]);
        } else {
            unimplemented!();
        }
    }
}

impl<'a> Fill<'a> for RoundRectangleColorContext<'a> {
    #[inline(always)]
    fn fill<B: BackEnd<I>, I: Image>(&'a self, back_end: &mut B) {
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
                *self.transform.get(),
                *rect,
                *round_radius,
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

impl<'a> AddBorder<'a, RoundRectangleBorderColorContext<'a>> 
for RoundRectangleColorContext<'a> {
    #[inline(always)]
    fn border_radius(&'a self, radius: f64) -> RoundRectangleBorderColorContext<'a> {
        RoundRectangleBorderColorContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            rect: Borrowed(self.rect.get()),
            round_radius: Borrowed(self.round_radius.get()),
            color: Borrowed(self.color.get()),
            border: Value(radius),
        }
    }
}

