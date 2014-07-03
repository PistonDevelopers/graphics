
use {
    AddBevel,
    AddRound,
    BackEnd,
    BevelRectangleBorderColorContext,
    Draw,
    ImageSize,
    RoundRectangleBorderColorContext,
};
use triangulation::{
    rect_border_tri_list_rgba_f32,
    rect_border_tri_list_xy_f32,
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
pub struct RectangleBorderColorContext {
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
for RectangleBorderColorContext {
    #[inline(always)]
    fn clone(&self) -> RectangleBorderColorContext {
        RectangleBorderColorContext {
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
for RectangleBorderColorContext {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform
    }
}

impl
CanTransform<RectangleBorderColorContext, Matrix2d> 
for RectangleBorderColorContext {
    #[inline(always)]
    fn transform(
        &self, 
        value: Matrix2d
    ) -> RectangleBorderColorContext {
        RectangleBorderColorContext {
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
for RectangleBorderColorContext {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view
    }
}

impl
CanViewTransform<RectangleBorderColorContext, Matrix2d> 
for RectangleBorderColorContext {
    #[inline(always)]
    fn view_transform(
        &self, 
        value: Matrix2d
    ) -> RectangleBorderColorContext {
        RectangleBorderColorContext {
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
for RectangleBorderColorContext {
    #[inline(always)]
    fn get_color(&self) -> Color {
        self.color
    }
}

impl
CanColor<RectangleBorderColorContext, Color> 
for RectangleBorderColorContext {
    #[inline(always)]
    fn color(
        &self, 
        value: Color
    ) -> RectangleBorderColorContext {
        RectangleBorderColorContext {
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
for RectangleBorderColorContext {
    #[inline(always)]
    fn get_rectangle(&self) -> Rectangle {
        self.rect
    }
}

impl
CanRectangle<RectangleBorderColorContext, Rectangle> 
for RectangleBorderColorContext {
    #[inline(always)]
    fn rectangle(
        &self, 
        rect: Rectangle
    ) -> RectangleBorderColorContext {
        RectangleBorderColorContext {
            view: self.view,
            transform: self.transform,
            rect: rect,
            color: self.color,
            border: self.border,
        }
    }
}

impl
AddRound<RoundRectangleBorderColorContext> 
for RectangleBorderColorContext {
    #[inline(always)]
    fn round(
        &self, 
        radius: f64
    ) -> RoundRectangleBorderColorContext {
        RoundRectangleBorderColorContext {
            view: self.view,
            transform: self.transform,
            color: self.color,
            rect: self.rect,
            round_radius: radius,
            border: self.border,
        }
    }
}

impl
AddBevel<BevelRectangleBorderColorContext> 
for RectangleBorderColorContext {
    #[inline(always)]
    fn bevel(
        &self, 
        radius: f64
    ) -> BevelRectangleBorderColorContext {
        BevelRectangleBorderColorContext {
            view: self.view,
            transform: self.transform,
            color: self.color,
            rect: self.rect,
            bevel_radius: radius,
            border: self.border,
        }
    }
}

impl<B: BackEnd<I>, I: ImageSize> 
Draw<B, I> 
for RectangleBorderColorContext {
    #[inline(always)]
    fn draw(&self, back_end: &mut B) {
        if back_end.supports_tri_list_xy_f32_rgba_f32() {
            let rect = self.rect;
            let color = self.color;
            let border = self.border;
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely opaque.
            let needs_alpha = color[3] != 1.0;
            if needs_alpha { back_end.enable_alpha_blend(); }
            back_end.tri_list_xy_f32_rgba_f32(
                rect_border_tri_list_xy_f32(
                    self.transform, rect, border),
                rect_border_tri_list_rgba_f32(color)
            );
            if needs_alpha { back_end.disable_alpha_blend(); }
        } else {
            unimplemented!();
        }
    }
}

