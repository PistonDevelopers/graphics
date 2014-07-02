
use {
    AddBorder,
    BackEnd,
    EllipseBorderColorContext,
    Field,
    Draw,
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
pub struct EllipseColorContext {
    /// View transformation.
    pub view: Field<Matrix2d>,
    /// Current transformation.
    pub transform: Field<Matrix2d>,
    /// Current rectangle.
    pub rect: Field<Rectangle>,
    /// Current color.
    pub color: Field<Color>,
}

impl
Clone 
for EllipseColorContext {
    #[inline(always)]
    fn clone(&self) -> EllipseColorContext {
        EllipseColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            rect: Value(self.rect.get()),
            color: Value(self.color.get()),
        }
    }
}

impl
HasTransform<Matrix2d> 
for EllipseColorContext {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform.get()
    }
}

impl
CanTransform<EllipseColorContext, Matrix2d> 
for EllipseColorContext {
    #[inline(always)]
    fn transform(&self, value: Matrix2d) -> EllipseColorContext {
        EllipseColorContext {
            view: Value(self.view.get()),
            transform: Value(value),
            rect: Value(self.rect.get()),
            color: Value(self.color.get()),
        }
    }
}

impl
HasViewTransform<Matrix2d> 
for EllipseColorContext {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view.get()
    }
}

impl
CanViewTransform<EllipseColorContext, Matrix2d> 
for EllipseColorContext {
    #[inline(always)]
    fn view_transform(
        &self, 
        value: Matrix2d
    ) -> EllipseColorContext {
        EllipseColorContext {
            view: Value(value),
            transform: Value(self.transform.get()),
            rect: Value(self.rect.get()),
            color: Value(self.color.get()),
        }
    }
}

impl
HasColor<Color> 
for EllipseColorContext {
    #[inline(always)]
    fn get_color(&self) -> Color {
        self.color.get()
    }
}

impl
CanColor<EllipseColorContext, Color> 
for EllipseColorContext {
    #[inline(always)]
    fn color(&self, value: Color) -> EllipseColorContext {
        EllipseColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            color: Value(value),
            rect: Value(self.rect.get()),
        }
    }
}

impl
HasRectangle<Rectangle> 
for EllipseColorContext {
    #[inline(always)]
    fn get_rectangle(&self) -> Rectangle {
        self.rect.get()
    }
}

impl
CanRectangle<EllipseColorContext, Rectangle> 
for EllipseColorContext {
    #[inline(always)]
    fn rectangle(
        &self, 
        rect: Rectangle
    ) -> EllipseColorContext {
        EllipseColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            rect: Value(rect),
            color: Value(self.color.get()),
        }
    }
}

impl<B: BackEnd<I>, I: ImageSize> 
Draw<B, I> 
for EllipseColorContext {
    #[inline(always)]
    fn draw(&self, back_end: &mut B) {
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
                self.transform.get(),
                rect,
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

impl
AddBorder<EllipseBorderColorContext> 
for EllipseColorContext {
    #[inline(always)]
    fn border_radius(
        &self, 
        radius: f64
    ) -> EllipseBorderColorContext {
        EllipseBorderColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            rect: Value(self.rect.get()),
            color: Value(self.color.get()),
            border: Value(radius),
        }
    }
}

