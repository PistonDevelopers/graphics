
use {
    AddBevel,
    AddBorder,
    AddImage,
    AddRound,
    BackEnd,
    BevelRectangleColorContext,
    Draw,
    ImageSize,
    ImageRectangleColorContext,
    RectangleBorderColorContext,
    RoundRectangleColorContext,
};
use triangulation::{
    rect_tri_list_xy_f32,
    rect_tri_list_rgba_f32,
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

/// A rectangle color context.
pub struct RectangleColorContext {
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
for RectangleColorContext {
    #[inline(always)]
    fn clone(&self) -> RectangleColorContext {
        RectangleColorContext {
            view: self.view,
            transform: self.transform,
            rect: self.rect,
            color: self.color,
        }
    }
}

impl
HasTransform<Matrix2d> 
for RectangleColorContext {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform
    }
}

impl
CanTransform<RectangleColorContext, Matrix2d> 
for RectangleColorContext {
    #[inline(always)]
    fn transform(
        &self, 
        value: Matrix2d
    ) -> RectangleColorContext {
        RectangleColorContext {
            view: self.view,
            transform: value,
            rect: self.rect,
            color: self.color,
        }
    }
}

impl
HasViewTransform<Matrix2d> 
for RectangleColorContext {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view
    }
}

impl
CanViewTransform<RectangleColorContext, Matrix2d> 
for RectangleColorContext {
    #[inline(always)]
    fn view_transform(
        &self, 
        value: Matrix2d
    ) -> RectangleColorContext {
        RectangleColorContext {
            view: value,
            transform: self.transform,
            rect: self.rect,
            color: self.color,
        }
    }
}

impl
HasColor<Color> 
for RectangleColorContext {
    #[inline(always)]
    fn get_color(&self) -> Color {
        self.color
    }
}

impl
CanColor<RectangleColorContext, Color> 
for RectangleColorContext {
    #[inline(always)]
    fn color(&self, value: Color) -> RectangleColorContext {
        RectangleColorContext {
            view: self.view,
            transform: self.transform,
            color: value,
            rect: self.rect,
        }
    }
}

impl
HasRectangle<Rectangle> 
for RectangleColorContext {
    #[inline(always)]
    fn get_rectangle(&self) -> Rectangle {
        self.rect
    }
}

impl
CanRectangle<RectangleColorContext, Rectangle> 
for RectangleColorContext {
    #[inline(always)]
    fn rectangle(
        &self, 
        rect: Rectangle
    ) -> RectangleColorContext {
        RectangleColorContext {
            view: self.view,
            transform: self.transform,
            rect: rect,
            color: self.color,
        }
    }
}

impl<B: BackEnd<I>, I: ImageSize> 
Draw<B, I> 
for RectangleColorContext {
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
            back_end.tri_list_xy_f32_rgba_f32(
                rect_tri_list_xy_f32(self.transform, rect),
                rect_tri_list_rgba_f32(color)
            );
            if needs_alpha { back_end.disable_alpha_blend(); }
        } else {
            unimplemented!();
        }
    }
}

impl
AddRound<RoundRectangleColorContext> 
for RectangleColorContext {
    #[inline(always)]
    fn round(
        &self, 
        radius: f64
    ) -> RoundRectangleColorContext {
        RoundRectangleColorContext {
            view: self.view,
            transform: self.transform,
            color: self.color,
            rect: self.rect,
            round_radius: radius,
        }
    }
}

impl
AddBevel<BevelRectangleColorContext> 
for RectangleColorContext {
    #[inline(always)]
    fn bevel(
        &self, 
        radius: f64
    ) -> BevelRectangleColorContext {
        BevelRectangleColorContext {
            view: self.view,
            transform: self.transform,
            color: self.color,
            rect: self.rect,
            bevel_radius: radius,
        }
    }
}

impl<'b, I: ImageSize> 
AddImage<'b, ImageRectangleColorContext<'b, I>, I> 
for RectangleColorContext {
    fn image(
        &self, 
        image: &'b I
    ) -> ImageRectangleColorContext<'b, I> {
        let (w, h) = image.get_size();
        ImageRectangleColorContext {
            view: self.view,
            transform: self.transform,
            rect: self.rect,
            image: image,
            source_rect: [0, 0, w as i32, h as i32],
            color: self.color,
        }
    }
}

impl
AddBorder<RectangleBorderColorContext> 
for RectangleColorContext {
    #[inline(always)]
    fn border_radius(
        &self, 
        radius: f64
    ) -> RectangleBorderColorContext {
        RectangleBorderColorContext {
            view: self.view,
            transform: self.transform,
            rect: self.rect,
            color: self.color,
            border: radius,
        }
    }
}

