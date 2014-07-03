
use {
    BackEnd,
    Draw,
    ImageSize,
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
    pub view: Matrix2d,
    /// Current transformation.
    pub transform: Matrix2d,
    /// Current rectangle.
    pub rect: Rectangle,
    /// Current color.
    pub color: Color,
    /// Current border.
    pub border: Radius,
}

impl
Clone 
for EllipseBorderColorContext {
    #[inline(always)]
    fn clone(&self) -> EllipseBorderColorContext {
        EllipseBorderColorContext {
            view: self.view,
            transform: self.transform,
            rect: self.rect,
            color: self.color,
            border: self.border,
        }
    }
}

impl
HasTransform<Matrix2d> 
for EllipseBorderColorContext {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform
    }
}

impl
CanTransform<EllipseBorderColorContext, Matrix2d> 
for EllipseBorderColorContext {
    #[inline(always)]
    fn transform(&self, value: Matrix2d) -> EllipseBorderColorContext {
        EllipseBorderColorContext {
            view: self.view,
            transform: value,
            rect: self.rect,
            color: self.color,
            border: self.border,
        }
    }
}

impl
HasViewTransform<Matrix2d> 
for EllipseBorderColorContext {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view
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
            view: value,
            transform: self.transform,
            rect: self.rect,
            color: self.color,
            border: self.border,
        }
    }
}

impl
HasColor<Color> 
for EllipseBorderColorContext {
    #[inline(always)]
    fn get_color(&self) -> Color {
        self.color
    }
}

impl
CanColor<EllipseBorderColorContext, Color> 
for EllipseBorderColorContext {
    #[inline(always)]
    fn color(&self, value: Color) -> EllipseBorderColorContext {
        EllipseBorderColorContext {
            view: self.view,
            transform: self.transform,
            color: value,
            rect: self.rect,
            border: self.border,
        }
    }
}

impl
HasRectangle<Rectangle> 
for EllipseBorderColorContext {
    #[inline(always)]
    fn get_rectangle(&self) -> Rectangle {
        self.rect
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
            view: self.view,
            transform: self.transform,
            rect: rect,
            color: self.color,
            border: self.border,
        }
    }
}

impl<B: BackEnd<I>, I: ImageSize>
Draw<B, I>
for EllipseBorderColorContext {
    #[inline(always)]
    fn draw( &self, back_end: &mut B) {
        if back_end.supports_tri_list_xy_f32_rgba_f32() {
            let rect = self.rect;
            let color = self.color;
            let border_radius = self.border;
            // Complte transparency does  not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not complete opaque.
            let needs_alpha = color[3] != 1.0;
            if needs_alpha { back_end.enable_alpha_blend(); }
            with_ellipse_border_tri_list_xy_f32_rgba_f32(
                128,
                self.transform,
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


