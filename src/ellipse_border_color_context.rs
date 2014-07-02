
use {
    BackEnd,
    Draw,
    Field,
    ImageSize,
    Value,
};
use triangulation::{
    with_ellipse_border_tri_list_xy_f32_rgba_f32,
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

/// An ellipse border color context.
pub struct EllipseBorderColorContext {
    /// View transformation.
    pub view: Field<Matrix2d>,
    /// Current transformation.
    pub transform: Field<Matrix2d>,
    /// Current rectangle.
    pub rect: Field<Rectangle>,
    /// Current color.
    pub color: Field<Color>,
    /// Current border.
    pub border: Field<Radius>,
}

impl
Clone 
for EllipseBorderColorContext {
    #[inline(always)]
    fn clone(&self) -> EllipseBorderColorContext {
        EllipseBorderColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            rect: Value(self.rect.get()),
            color: Value(self.color.get()),
            border: Value(self.border.get()),
        }
    }
}

impl
HasTransform<Matrix2d> 
for EllipseBorderColorContext {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform.get()
    }
}

impl
CanTransform<EllipseBorderColorContext, Matrix2d> 
for EllipseBorderColorContext {
    #[inline(always)]
    fn transform(&self, value: Matrix2d) -> EllipseBorderColorContext {
        EllipseBorderColorContext {
            view: Value(self.view.get()),
            transform: Value(value),
            rect: Value(self.rect.get()),
            color: Value(self.color.get()),
            border: Value(self.border.get()),
        }
    }
}

impl
HasViewTransform<Matrix2d> 
for EllipseBorderColorContext {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view.get()
    }
}

impl
CanViewTransform<EllipseBorderColorContext, Matrix2d> 
for EllipseBorderColorContext {
    #[inline(always)]
    fn view_transform(
        &self, 
        value: Matrix2d
    ) -> EllipseBorderColorContext {
        EllipseBorderColorContext {
            view: Value(value),
            transform: Value(self.transform.get()),
            rect: Value(self.rect.get()),
            color: Value(self.color.get()),
            border: Value(self.border.get()),
        }
    }
}

impl
HasColor<Color> 
for EllipseBorderColorContext {
    #[inline(always)]
    fn get_color(&self) -> Color {
        self.color.get()
    }
}

impl
CanColor<EllipseBorderColorContext, Color> 
for EllipseBorderColorContext {
    #[inline(always)]
    fn color(&self, value: Color) -> EllipseBorderColorContext {
        EllipseBorderColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            color: Value(value),
            rect: Value(self.rect.get()),
            border: Value(self.border.get()),
        }
    }
}

impl
HasRectangle<Rectangle> 
for EllipseBorderColorContext {
    #[inline(always)]
    fn get_rectangle(&self) -> Rectangle {
        self.rect.get()
    }
}

impl
CanRectangle<EllipseBorderColorContext, Rectangle> 
for EllipseBorderColorContext {
    #[inline(always)]
    fn rectangle(
        &self, 
        rect: Rectangle
    ) -> EllipseBorderColorContext {
        EllipseBorderColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            rect: Value(rect),
            color: Value(self.color.get()),
            border: Value(self.border.get()),
        }
    }
}

impl<B: BackEnd<I>, I: ImageSize>
Draw<B, I>
for EllipseBorderColorContext {
    #[inline(always)]
    fn draw( &self, back_end: &mut B) {
        if back_end.supports_tri_list_xy_f32_rgba_f32() {
            let rect = self.rect.get();
            let color = self.color.get();
            let border_radius = self.border.get();
            // Complte transparency does  not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not complete opaque.
            let needs_alpha = color[3] != 1.0;
            if needs_alpha { back_end.enable_alpha_blend(); }
            with_ellipse_border_tri_list_xy_f32_rgba_f32(
                128,
                self.transform.get(),
                rect,
                color,
                border_radius,
                |vertices, colors| {
                    back_end.tri_list_xy_f32_rgba_f32(vertices, colors)
                }
            );
            if needs_alpha { back_end.disable_alpha_blend(); }
        }
    }
}


