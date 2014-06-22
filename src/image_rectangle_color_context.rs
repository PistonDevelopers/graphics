use {
    BackEnd,
    Borrowed,
    Clear,
    Draw,
    Field,
    ImageSize,
    Value,
};
use triangulation::{
    rect_tri_list_xy_f32,
    rect_tri_list_rgba_f32,
    rect_tri_list_uv_f32,
};
use internal::{
    CanColor,
    CanRectangle,
    CanSourceRectangle,
    CanTransform,
    CanViewTransform,
    Color,
    HasColor,
    HasRectangle,
    HasSourceRectangle,
    HasTransform,
    HasViewTransform,
    SourceRectangle,
    Matrix2d,
    Rectangle,
};

/// An image rectangle context.
pub struct ImageRectangleColorContext<'a, 'b, I> {
    /// View transformation.
    pub view: Field<'a, Matrix2d>,
    /// Current transformation.
    pub transform: Field<'a, Matrix2d>,
    /// Current rectangle.
    pub rect: Field<'a, Rectangle>,
    /// Current image.
    pub image: Field<'a, &'b I>,
    /// Current source rectangle.
    pub source_rect: Field<'a, SourceRectangle>,
    /// Current color.
    pub color: Field<'a, Color>,
}

impl<'a, 'b, I> Clone for ImageRectangleColorContext<'a, 'b, I> {
    #[inline(always)]
    fn clone(&self) -> ImageRectangleColorContext<'static, 'b, I> {
        ImageRectangleColorContext {
            view: Value(*self.view.get()),
            transform: Value(*self.transform.get()),
            rect: Value(*self.rect.get()),
            image: Value(*self.image.get()),
            source_rect: Value(*self.source_rect.get()),
            color: Value(*self.color.get()),
        }
    }
}

impl<'a, 'b, I> HasTransform<'a, Matrix2d> for ImageRectangleColorContext<'a, 'b, I> {
    #[inline(always)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a, 'b, I> CanTransform<'a, ImageRectangleColorContext<'a, 'b, I>, Matrix2d> for ImageRectangleColorContext<'a, 'b, I> {
    #[inline(always)]
    fn transform(&'a self, value: Matrix2d) -> ImageRectangleColorContext<'a, 'b, I> {
        ImageRectangleColorContext {
            view: Borrowed(self.view.get()),
            transform: Value(value),
            rect: Borrowed(self.rect.get()),
            image: Borrowed(self.image.get()),
            source_rect: Borrowed(self.source_rect.get()),
            color: Borrowed(self.color.get()),
        }
    }
}

impl<'a, 'b, I> HasViewTransform<'a, Matrix2d> for ImageRectangleColorContext<'a, 'b, I> {
    #[inline(always)]
    fn get_view_transform(&'a self) -> &'a Matrix2d {
        self.view.get()
    }
}

impl<'a, 'b, I> CanViewTransform<'a, ImageRectangleColorContext<'a, 'b, I>, Matrix2d>
for ImageRectangleColorContext<'a, 'b, I> {
    #[inline(always)]
    fn view_transform(&'a self, value: Matrix2d) -> ImageRectangleColorContext<'a, 'b, I> {
        ImageRectangleColorContext {
            view: Value(value),
            transform: Borrowed(self.transform.get()),
            rect: Borrowed(self.rect.get()),
            image: Borrowed(self.image.get()),
            source_rect: Borrowed(self.source_rect.get()),
            color: Borrowed(self.color.get()),
        }
    }
}

impl<'a, 'b, I> HasColor<'a, Color> for ImageRectangleColorContext<'a, 'b, I> {
    #[inline(always)]
    fn get_color(&'a self) -> &'a Color {
        self.color.get()
    }
}

impl<'a, 'b, I> CanColor<'a, ImageRectangleColorContext<'a, 'b, I>, Color> for ImageRectangleColorContext<'a, 'b, I> {
    #[inline(always)]
    fn color(&'a self, value: Color) -> ImageRectangleColorContext<'a, 'b, I> {
        ImageRectangleColorContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            color: Value(value),
            rect: Borrowed(self.rect.get()),
            image: Borrowed(self.image.get()),
            source_rect: Borrowed(self.source_rect.get()),
        }
    }
}

impl<'a, 'b, I> HasRectangle<'a, Rectangle> for ImageRectangleColorContext<'a, 'b, I> {
    #[inline(always)]
    fn get_rectangle(&'a self) -> &'a Rectangle {
        self.rect.get()
    }
}

impl<'a, 'b, I> CanRectangle<'a, ImageRectangleColorContext<'a, 'b, I>, Rectangle> for ImageRectangleColorContext<'a, 'b, I> {
    #[inline(always)]
    fn rectangle(&'a self, rect: Rectangle) -> ImageRectangleColorContext<'a, 'b, I> {
        ImageRectangleColorContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            rect: Value(rect),
            image: Borrowed(self.image.get()),
            source_rect: Borrowed(self.source_rect.get()),
            color: Borrowed(self.color.get()),
        }
    }
}

impl<'a, 'b, I> HasSourceRectangle<'a, SourceRectangle> 
for ImageRectangleColorContext<'a, 'b, I> {
    #[inline(always)]
    fn get_source_rectangle(&'a self) -> &'a SourceRectangle {
        self.source_rect.get()
    }
}

impl<'a, 'b, I> CanSourceRectangle<'a, ImageRectangleColorContext<'a, 'b, I>, SourceRectangle> 
for ImageRectangleColorContext<'a, 'b, I> {
    #[inline(always)]
    fn source_rectangle(&'a self, source_rect: SourceRectangle) -> ImageRectangleColorContext<'a, 'b, I> {
        ImageRectangleColorContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            rect: Borrowed(self.rect.get()),
            image: Borrowed(self.image.get()),
            source_rect: Value(source_rect),
            color: Borrowed(self.color.get()),
        }
    }
}

impl<'a, 'b, B: BackEnd<I>, I: ImageSize> 
Draw<'a, B, I> 
for ImageRectangleColorContext<'a, 'b, I> {
    #[inline(always)]
    fn draw(&'a self, back_end: &mut B) {
        if back_end.supports_single_texture()
        && back_end.supports_tri_list_xy_f32_rgba_f32_uv_f32() {
            let rect = self.rect.get();
            let color = self.color.get();
            let &texture = self.image.get();
            let source_rect = self.source_rect.get();
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely opaque or if the texture has alpha channel.
            let needs_alpha = color[3] != 1.0 || back_end.has_texture_alpha(texture);
            if needs_alpha { back_end.enable_alpha_blend(); }
            back_end.enable_single_texture(texture);
            back_end.tri_list_xy_f32_rgba_f32_uv_f32(
                rect_tri_list_xy_f32(*self.transform.get(), *rect),
                rect_tri_list_rgba_f32(*color),
                rect_tri_list_uv_f32(texture, *source_rect)
            );
            back_end.disable_single_texture();
            if needs_alpha { back_end.disable_alpha_blend(); }
        } else {
            unimplemented!();
        }
    }
}

impl<'a, 'b, B: BackEnd<I>, I: ImageSize> 
Clear<B, I> 
for ImageRectangleColorContext<'a, 'b, I> {
    #[inline(always)]
    fn clear(&self, back_end: &mut B) {
        if back_end.supports_clear_rgba() {
            let color = self.color.get();
            back_end.clear_rgba(color[0], color[1], color[2], color[3]);
        }
    }
}

