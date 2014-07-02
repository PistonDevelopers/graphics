
use {
    AddBevel,
    AddBorder,
    AddImage,
    AddRound,
    BackEnd,
    BevelRectangleColorContext,
    Field,
    Draw,
    ImageSize,
    ImageRectangleColorContext,
    RectangleBorderColorContext,
    RoundRectangleColorContext,
    Value,
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
for RectangleColorContext {
    #[inline(always)]
    fn clone(&self) -> RectangleColorContext {
        RectangleColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            rect: Value(self.rect.get()),
            color: Value(self.color.get()),
        }
    }
}

impl
HasTransform<Matrix2d> 
for RectangleColorContext {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform.get()
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
            view: Value(self.view.get()),
            transform: Value(value),
            rect: Value(self.rect.get()),
            color: Value(self.color.get()),
        }
    }
}

impl
HasViewTransform<Matrix2d> 
for RectangleColorContext {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view.get()
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
            view: Value(value),
            transform: Value(self.transform.get()),
            rect: Value(self.rect.get()),
            color: Value(self.color.get()),
        }
    }
}

impl
HasColor<Color> 
for RectangleColorContext {
    #[inline(always)]
    fn get_color(&self) -> Color {
        self.color.get()
    }
}

impl
CanColor<RectangleColorContext, Color> 
for RectangleColorContext {
    #[inline(always)]
    fn color(&self, value: Color) -> RectangleColorContext {
        RectangleColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            color: Value(value),
            rect: Value(self.rect.get()),
        }
    }
}

impl
HasRectangle<Rectangle> 
for RectangleColorContext {
    #[inline(always)]
    fn get_rectangle(&self) -> Rectangle {
        self.rect.get()
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
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            rect: Value(rect),
            color: Value(self.color.get()),
        }
    }
}

impl<B: BackEnd<I>, I: ImageSize> 
Draw<B, I> 
for RectangleColorContext {
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
            back_end.tri_list_xy_f32_rgba_f32(
                rect_tri_list_xy_f32(self.transform.get(), rect),
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
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            color: Value(self.color.get()),
            rect: Value(self.rect.get()),
            round_radius: Value(radius),
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
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            color: Value(self.color.get()),
            rect: Value(self.rect.get()),
            bevel_radius: Value(radius),
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
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            rect: Value(self.rect.get()),
            image: Value(image),
            source_rect: Value([0, 0, w as i32, h as i32]),
            color: Value(self.color.get()),
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
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            rect: Value(self.rect.get()),
            color: Value(self.color.get()),
            border: Value(radius),
        }
    }
}

