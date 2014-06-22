use {
    AddBorder,
    BackEnd,
    BevelRectangleBorderColorContext,
    Borrowed,
    Clear,
    Field,
    Fill,
    ImageSize,
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

/// A rectangle color context.
pub struct BevelRectangleColorContext<'a> {
    /// View transformation.
    pub view: Field<'a, Matrix2d>,
    /// Current transformation.
    pub transform: Field<'a, Matrix2d>,
    /// Current rectangle.
    pub rect: Field<'a, Rectangle>,
    /// Current bevel radius.
    pub bevel_radius: Field<'a, Radius>,
    /// Current color.
    pub color: Field<'a, Color>,
}

impl<'a> Clone for BevelRectangleColorContext<'a> {
    #[inline(always)]
    fn clone(&self) -> BevelRectangleColorContext<'static> {
        BevelRectangleColorContext {
            view: Value(*self.view.get()),
            transform: Value(*self.transform.get()),
            rect: Value(*self.rect.get()),
            bevel_radius: Value(*self.bevel_radius.get()),
            color: Value(*self.color.get()),
        }
    }
}

impl<'a> HasTransform<'a, Matrix2d> for BevelRectangleColorContext<'a> {
    #[inline(always)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a> CanTransform<'a, BevelRectangleColorContext<'a>, Matrix2d> for BevelRectangleColorContext<'a> {
    #[inline(always)]
    fn transform(&'a self, value: Matrix2d) -> BevelRectangleColorContext<'a> {
        BevelRectangleColorContext {
            view: Borrowed(self.view.get()),
            transform: Value(value),
            rect: Borrowed(self.rect.get()),
            bevel_radius: Borrowed(self.bevel_radius.get()),
            color: Borrowed(self.color.get()),
        }
    }
}

impl<'a> HasViewTransform<'a, Matrix2d> for BevelRectangleColorContext<'a> {
    #[inline(always)]
    fn get_view_transform(&'a self) -> &'a Matrix2d {
        self.view.get()
    }
}

impl<'a> CanViewTransform<'a, BevelRectangleColorContext<'a>, Matrix2d> 
for BevelRectangleColorContext<'a> {
    #[inline(always)]
    fn view_transform(&'a self, value: Matrix2d) -> BevelRectangleColorContext<'a> {
        BevelRectangleColorContext {
            view: Value(value),
            transform: Borrowed(self.transform.get()),
            rect: Borrowed(self.rect.get()),
            bevel_radius: Borrowed(self.bevel_radius.get()),
            color: Borrowed(self.color.get()),
        }
    }
}

impl<'a> HasColor<'a, Color> for BevelRectangleColorContext<'a> {
    #[inline(always)]
    fn get_color(&'a self) -> &'a Color {
        self.color.get()
    }
}

impl<'a> CanColor<'a, BevelRectangleColorContext<'a>, Color> for BevelRectangleColorContext<'a> {
    #[inline(always)]
    fn color(&'a self, value: Color) -> BevelRectangleColorContext<'a> {
        BevelRectangleColorContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            color: Value(value),
            rect: Borrowed(self.rect.get()),
            bevel_radius: Borrowed(self.bevel_radius.get()),
        }
    }
}

impl<'a> HasRectangle<'a, Rectangle> for BevelRectangleColorContext<'a> {
    #[inline(always)]
    fn get_rectangle(&'a self) -> &'a Rectangle {
        self.rect.get()
    }
}

impl<'a> CanRectangle<'a, BevelRectangleColorContext<'a>, Rectangle> for BevelRectangleColorContext<'a> {
    #[inline(always)]
    fn rectangle(&'a self, rect: Rectangle) -> BevelRectangleColorContext<'a> {
        BevelRectangleColorContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            rect: Value(rect),
            bevel_radius: Borrowed(self.bevel_radius.get()),
            color: Borrowed(self.color.get()),
        }
    }
}

impl<'a, B: BackEnd<I>, I: ImageSize> 
Clear<B, I> 
for BevelRectangleColorContext<'a> {
    #[inline(always)]
    fn clear(&self, back_end: &mut B) {
        if back_end.supports_clear_rgba() {
            let color = self.color.get();
            back_end.clear_rgba(color[0], color[1], color[2], color[3]);
        } else {
            unimplemented!();
        }
    }
}

impl<'a> Fill<'a> for BevelRectangleColorContext<'a> {
    #[inline(always)]
    fn fill<B: BackEnd<I>, I: ImageSize>(&'a self, back_end: &mut B) {
        if back_end.supports_tri_list_xy_f32_rgba_f32() {
            let rect = self.rect.get();
            let bevel_radius = self.bevel_radius.get();
            let color = self.color.get();
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely opaque.
            let needs_alpha = color[3] != 1.0;
            if needs_alpha { back_end.enable_alpha_blend(); }
            with_round_rectangle_tri_list_xy_f32_rgba_f32(
                2,
                *self.transform.get(),
                *rect,
                *bevel_radius,
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

impl<'a> AddBorder<'a, BevelRectangleBorderColorContext<'a>> 
for BevelRectangleColorContext<'a> {
    #[inline(always)]
    fn border_radius(&'a self, radius: f64) -> BevelRectangleBorderColorContext<'a> {
        BevelRectangleBorderColorContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            rect: Borrowed(self.rect.get()),
            bevel_radius: Borrowed(self.bevel_radius.get()),
            color: Borrowed(self.color.get()),
            border: Value(radius),
        }
    }
}

