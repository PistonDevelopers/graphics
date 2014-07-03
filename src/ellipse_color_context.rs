
use {
    AddBorder,
    BackEnd,
    EllipseBorderColorContext,
    Draw,
    ImageSize,
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
    pub view: Matrix2d,
    /// Current transformation.
    pub transform: Matrix2d,
    /// Current rectangle.
    pub rect: Rectangle,
    /// Current color.
    pub color: Color,
}

impl
Clone 
for EllipseColorContext {
    #[inline(always)]
    fn clone(&self) -> EllipseColorContext {
        EllipseColorContext {
            view: self.view,
            transform: self.transform,
            rect: self.rect,
            color: self.color,
        }
    }
}

impl
HasTransform<Matrix2d> 
for EllipseColorContext {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform
    }
}

impl
CanTransform<EllipseColorContext, Matrix2d> 
for EllipseColorContext {
    #[inline(always)]
    fn transform(&self, value: Matrix2d) -> EllipseColorContext {
        EllipseColorContext {
            view: self.view,
            transform: value,
            rect: self.rect,
            color: self.color,
        }
    }
}

impl
HasViewTransform<Matrix2d> 
for EllipseColorContext {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view
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
            view: value,
            transform: self.transform,
            rect: self.rect,
            color: self.color,
        }
    }
}

impl
HasColor<Color> 
for EllipseColorContext {
    #[inline(always)]
    fn get_color(&self) -> Color {
        self.color
    }
}

impl
CanColor<EllipseColorContext, Color> 
for EllipseColorContext {
    #[inline(always)]
    fn color(&self, value: Color) -> EllipseColorContext {
        EllipseColorContext {
            view: self.view,
            transform: self.transform,
            color: value,
            rect: self.rect,
        }
    }
}

impl
HasRectangle<Rectangle> 
for EllipseColorContext {
    #[inline(always)]
    fn get_rectangle(&self) -> Rectangle {
        self.rect
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
            view: self.view,
            transform: self.transform,
            rect: rect,
            color: self.color,
        }
    }
}

impl<B: BackEnd<I>, I: ImageSize> 
Draw<B, I> 
for EllipseColorContext {
    #[inline(always)]
    fn draw(&self, back_end: &mut B) {
        if back_end.supports_tri_list_xy_f32_rgba_f32() {
            let rect = self.rect;
            let color = self.color;
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely opaque.
            let needs_alpha = color[3] != 1.0;
            if needs_alpha { back_end.enable_alpha_blend(); }
            with_ellipse_tri_list_xy_f32_rgba_f32(
                128,
                self.transform,
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
            view: self.view,
            transform: self.transform,
            rect: self.rect,
            color: self.color,
            border: radius,
        }
    }
}

