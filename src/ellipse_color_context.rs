
use {
    AddBorder,
    BackEnd,
    Clear,
    Borrowed,
    EllipseBorderColorContext,
    Field,
    Fill,
    ImageSize,
    Value,
};
use triangulation::{
    with_ellipse_tri_list_xy_f32_rgba_f32
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
    Rectangle,
};

/// An ellipse color context.
pub struct EllipseColorContext<'a> {
    /// View transformation.
    pub view: Field<'a, Matrix2d>,
    /// Current transformation.
    pub transform: Field<'a, Matrix2d>,
    /// Current rectangle.
    pub rect: Field<'a, Rectangle>,
    /// Current color.
    pub color: Field<'a, Color>,
}

impl<'a> 
Clone 
for EllipseColorContext<'a> {
    #[inline(always)]
    fn clone(&self) -> EllipseColorContext<'static> {
        EllipseColorContext {
            view: Value(*self.view.get()),
            transform: Value(*self.transform.get()),
            rect: Value(*self.rect.get()),
            color: Value(*self.color.get()),
        }
    }
}

impl<'a> 
HasTransform<'a, Matrix2d> 
for EllipseColorContext<'a> {
    #[inline(always)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a> 
CanTransform<'a, EllipseColorContext<'a>, Matrix2d> 
for EllipseColorContext<'a> {
    #[inline(always)]
    fn transform(&'a self, value: Matrix2d) -> EllipseColorContext<'a> {
        EllipseColorContext {
            view: Borrowed(self.view.get()),
            transform: Value(value),
            rect: Borrowed(self.rect.get()),
            color: Borrowed(self.color.get()),
        }
    }
}

impl<'a> 
HasViewTransform<'a, Matrix2d> 
for EllipseColorContext<'a> {
    #[inline(always)]
    fn get_view_transform(&'a self) -> &'a Matrix2d {
        self.view.get()
    }
}

impl<'a> 
CanViewTransform<'a, EllipseColorContext<'a>, Matrix2d> 
for EllipseColorContext<'a> {
    #[inline(always)]
    fn view_transform(
        &'a self, 
        value: Matrix2d
    ) -> EllipseColorContext<'a> {
        EllipseColorContext {
            view: Value(value),
            transform: Borrowed(self.transform.get()),
            rect: Borrowed(self.rect.get()),
            color: Borrowed(self.color.get()),
        }
    }
}

impl<'a> 
HasColor<'a, Color> 
for EllipseColorContext<'a> {
    #[inline(always)]
    fn get_color(&'a self) -> &'a Color {
        self.color.get()
    }
}

impl<'a> 
CanColor<'a, EllipseColorContext<'a>, Color> 
for EllipseColorContext<'a> {
    #[inline(always)]
    fn color(&'a self, value: Color) -> EllipseColorContext<'a> {
        EllipseColorContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            color: Value(value),
            rect: Borrowed(self.rect.get()),
        }
    }
}

impl<'a> 
HasRectangle<'a, Rectangle> 
for EllipseColorContext<'a> {
    #[inline(always)]
    fn get_rectangle(&'a self) -> &'a Rectangle {
        self.rect.get()
    }
}

impl<'a> 
CanRectangle<'a, EllipseColorContext<'a>, Rectangle> 
for EllipseColorContext<'a> {
    #[inline(always)]
    fn rectangle(
        &'a self, 
        rect: Rectangle
    ) -> EllipseColorContext<'a> {
        EllipseColorContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            rect: Value(rect),
            color: Borrowed(self.color.get()),
        }
    }
}

impl<'a> 
Fill<'a> 
for EllipseColorContext<'a> {
    #[inline(always)]
    fn fill<B: BackEnd<I>, I: ImageSize>(
        &'a self, 
        back_end: &mut B
    ) {
        if back_end.supports_tri_list_xy_f32_rgba_f32() {
            let rect = self.rect.get();
            let color = self.color.get();
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely opaque.
            let needs_alpha = color[3] != 1.0;
            if needs_alpha { back_end.enable_alpha_blend(); }
            with_ellipse_tri_list_xy_f32_rgba_f32(
                128,
                *self.transform.get(),
                *rect,
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

impl<'a, B: BackEnd<I>, I: ImageSize> 
Clear<B, I> 
for EllipseColorContext<'a> {
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

impl<'a> 
AddBorder<'a, EllipseBorderColorContext<'a>> 
for EllipseColorContext<'a> {
    #[inline(always)]
    fn border_radius(
        &'a self, 
        radius: f64
    ) -> EllipseBorderColorContext<'a> {
        EllipseBorderColorContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            rect: Borrowed(self.rect.get()),
            color: Borrowed(self.color.get()),
            border: Value(radius),
        }
    }
}

