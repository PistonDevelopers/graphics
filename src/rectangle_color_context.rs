
use {
    AddBevel,
    AddBorder,
    AddImage,
    AddRound,
    BackEnd,
    BevelRectangleColorContext,
    Borrowed,
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
pub struct RectangleColorContext<'a> {
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
for RectangleColorContext<'a> {
    #[inline(always)]
    fn clone(&self) -> RectangleColorContext<'static> {
        RectangleColorContext {
            view: Value(*self.view.get()),
            transform: Value(*self.transform.get()),
            rect: Value(*self.rect.get()),
            color: Value(*self.color.get()),
        }
    }
}

impl<'a> 
HasTransform<'a, Matrix2d> 
for RectangleColorContext<'a> {
    #[inline(always)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a> 
CanTransform<'a, RectangleColorContext<'a>, Matrix2d> 
for RectangleColorContext<'a> {
    #[inline(always)]
    fn transform(
        &'a self, 
        value: Matrix2d
    ) -> RectangleColorContext<'a> {
        RectangleColorContext {
            view: Borrowed(self.view.get()),
            transform: Value(value),
            rect: Borrowed(self.rect.get()),
            color: Borrowed(self.color.get()),
        }
    }
}

impl<'a> 
HasViewTransform<'a, Matrix2d> 
for RectangleColorContext<'a> {
    #[inline(always)]
    fn get_view_transform(&'a self) -> &'a Matrix2d {
        self.view.get()
    }
}

impl<'a> 
CanViewTransform<'a, RectangleColorContext<'a>, Matrix2d> 
for RectangleColorContext<'a> {
    #[inline(always)]
    fn view_transform(
        &'a self, 
        value: Matrix2d
    ) -> RectangleColorContext<'a> {
        RectangleColorContext {
            view: Value(value),
            transform: Borrowed(self.transform.get()),
            rect: Borrowed(self.rect.get()),
            color: Borrowed(self.color.get()),
        }
    }
}

impl<'a> 
HasColor<'a, Color> 
for RectangleColorContext<'a> {
    #[inline(always)]
    fn get_color(&'a self) -> &'a Color {
        self.color.get()
    }
}

impl<'a> 
CanColor<'a, RectangleColorContext<'a>, Color> 
for RectangleColorContext<'a> {
    #[inline(always)]
    fn color(&'a self, value: Color) -> RectangleColorContext<'a> {
        RectangleColorContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            color: Value(value),
            rect: Borrowed(self.rect.get()),
        }
    }
}

impl<'a> 
HasRectangle<'a, Rectangle> 
for RectangleColorContext<'a> {
    #[inline(always)]
    fn get_rectangle(&'a self) -> &'a Rectangle {
        self.rect.get()
    }
}

impl<'a> 
CanRectangle<'a, RectangleColorContext<'a>, Rectangle> 
for RectangleColorContext<'a> {
    #[inline(always)]
    fn rectangle(
        &'a self, 
        rect: Rectangle
    ) -> RectangleColorContext<'a> {
        RectangleColorContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            rect: Value(rect),
            color: Borrowed(self.color.get()),
        }
    }
}

impl<'a, B: BackEnd<I>, I: ImageSize> 
Draw<'a, B, I> 
for RectangleColorContext<'a> {
    #[inline(always)]
    fn draw(&'a self, back_end: &mut B) {
        if back_end.supports_tri_list_xy_f32_rgba_f32() {
            let rect = self.rect.get();
            let color = self.color.get();
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely opaque.
            let needs_alpha = color[3] != 1.0;
            if needs_alpha { back_end.enable_alpha_blend(); }
            back_end.tri_list_xy_f32_rgba_f32(
                rect_tri_list_xy_f32(*self.transform.get(), *rect),
                rect_tri_list_rgba_f32(*color)
            );
            if needs_alpha { back_end.disable_alpha_blend(); }
        } else {
            unimplemented!();
        }
    }
}

impl<'a> 
AddRound<'a, RoundRectangleColorContext<'a>> 
for RectangleColorContext<'a> {
    #[inline(always)]
    fn round(
        &'a self, 
        radius: f64
    ) -> RoundRectangleColorContext<'a> {
        RoundRectangleColorContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            color: Borrowed(self.color.get()),
            rect: Borrowed(self.rect.get()),
            round_radius: Value(radius),
        }
    }
}

impl<'a> 
AddBevel<'a, BevelRectangleColorContext<'a>> 
for RectangleColorContext<'a> {
    #[inline(always)]
    fn bevel(
        &'a self, 
        radius: f64
    ) -> BevelRectangleColorContext<'a> {
        BevelRectangleColorContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            color: Borrowed(self.color.get()),
            rect: Borrowed(self.rect.get()),
            bevel_radius: Value(radius),
        }
    }
}

impl<'a, 'b, I: ImageSize> 
AddImage<'a, 'b, ImageRectangleColorContext<'a, 'b, I>, I> 
for RectangleColorContext<'a> {
    fn image(
        &'a self, 
        image: &'b I
    ) -> ImageRectangleColorContext<'a, 'b, I> {
        let (w, h) = image.get_size();
        ImageRectangleColorContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            rect: Borrowed(self.rect.get()),
            image: Value(image),
            source_rect: Value([0, 0, w as i32, h as i32]),
            color: Borrowed(self.color.get()),
        }
    }
}

impl<'a> 
AddBorder<'a, RectangleBorderColorContext<'a>> 
for RectangleColorContext<'a> {
    #[inline(always)]
    fn border_radius(
        &'a self, 
        radius: f64
    ) -> RectangleBorderColorContext<'a> {
        RectangleBorderColorContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            rect: Borrowed(self.rect.get()),
            color: Borrowed(self.color.get()),
            border: Value(radius),
        }
    }
}

