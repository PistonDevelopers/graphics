use {
    BackEnd,
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
pub struct ImageRectangleColorContext<'b, I> {
    /// View transformation.
    pub view: Field<Matrix2d>,
    /// Current transformation.
    pub transform: Field<Matrix2d>,
    /// Current rectangle.
    pub rect: Field<Rectangle>,
    /// Current image.
    pub image: Field<&'b I>,
    /// Current source rectangle.
    pub source_rect: Field<SourceRectangle>,
    /// Current color.
    pub color: Field<Color>,
}

impl<'b, I>
Clone 
for ImageRectangleColorContext<'b, I> {
    #[inline(always)]
    fn clone(&self) -> ImageRectangleColorContext<'b, I> {
        ImageRectangleColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            rect: Value(self.rect.get()),
            image: Value(self.image.get()),
            source_rect: Value(self.source_rect.get()),
            color: Value(self.color.get()),
        }
    }
}

impl<'b, I> 
HasTransform<Matrix2d> 
for ImageRectangleColorContext<'b, I> {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform.get()
    }
}

impl<'b, I> 
CanTransform<ImageRectangleColorContext<'b, I>, Matrix2d> 
for ImageRectangleColorContext<'b, I> {
    #[inline(always)]
    fn transform(
        &self, 
        value: Matrix2d
    ) -> ImageRectangleColorContext<'b, I> {
        ImageRectangleColorContext {
            view: Value(self.view.get()),
            transform: Value(value),
            rect: Value(self.rect.get()),
            image: Value(self.image.get()),
            source_rect: Value(self.source_rect.get()),
            color: Value(self.color.get()),
        }
    }
}

impl<'b, I> 
HasViewTransform<Matrix2d> 
for ImageRectangleColorContext<'b, I> {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view.get()
    }
}

impl<'b, I> 
CanViewTransform<ImageRectangleColorContext<'b, I>, Matrix2d>
for ImageRectangleColorContext<'b, I> {
    #[inline(always)]
    fn view_transform(
        &self, 
        value: Matrix2d
    ) -> ImageRectangleColorContext<'b, I> {
        ImageRectangleColorContext {
            view: Value(value),
            transform: Value(self.transform.get()),
            rect: Value(self.rect.get()),
            image: Value(self.image.get()),
            source_rect: Value(self.source_rect.get()),
            color: Value(self.color.get()),
        }
    }
}

impl<'b, I> 
HasColor<Color> 
for ImageRectangleColorContext<'b, I> {
    #[inline(always)]
    fn get_color(&self) -> Color {
        self.color.get()
    }
}

impl<'b, I> 
CanColor<ImageRectangleColorContext<'b, I>, Color> 
for ImageRectangleColorContext<'b, I> {
    #[inline(always)]
    fn color(
        &self, 
        value: Color
    ) -> ImageRectangleColorContext<'b, I> {
        ImageRectangleColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            color: Value(value),
            rect: Value(self.rect.get()),
            image: Value(self.image.get()),
            source_rect: Value(self.source_rect.get()),
        }
    }
}

impl<'b, I> 
HasRectangle<Rectangle> 
for ImageRectangleColorContext<'b, I> {
    #[inline(always)]
    fn get_rectangle(&self) -> Rectangle {
        self.rect.get()
    }
}

impl<'b, I> 
CanRectangle<ImageRectangleColorContext<'b, I>, Rectangle> 
for ImageRectangleColorContext<'b, I> {
    #[inline(always)]
    fn rectangle(
        &self, 
        rect: Rectangle
    ) -> ImageRectangleColorContext<'b, I> {
        ImageRectangleColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            rect: Value(rect),
            image: Value(self.image.get()),
            source_rect: Value(self.source_rect.get()),
            color: Value(self.color.get()),
        }
    }
}

impl<'b, I> 
HasSourceRectangle<SourceRectangle> 
for ImageRectangleColorContext<'b, I> {
    #[inline(always)]
    fn get_source_rectangle(&self) -> SourceRectangle {
        self.source_rect.get()
    }
}

impl<'b, I> 
CanSourceRectangle<ImageRectangleColorContext<'b, I>, SourceRectangle> 
for ImageRectangleColorContext<'b, I> {
    #[inline(always)]
    fn source_rectangle(
        &self, 
        source_rect: SourceRectangle
    ) -> ImageRectangleColorContext<'b, I> {
        ImageRectangleColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            rect: Value(self.rect.get()),
            image: Value(self.image.get()),
            source_rect: Value(source_rect),
            color: Value(self.color.get()),
        }
    }
}

impl<'b, B: BackEnd<I>, I: ImageSize> 
Draw<B, I> 
for ImageRectangleColorContext<'b, I> {
    #[inline(always)]
    fn draw(&self, back_end: &mut B) {
        if back_end.supports_single_texture()
        && back_end.supports_tri_list_xy_f32_rgba_f32_uv_f32() {
            let rect = self.rect.get();
            let color = self.color.get();
            let texture = self.image.get();
            let source_rect = self.source_rect.get();
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely opaque 
            // or if the texture has alpha channel.
            let needs_alpha = color[3] != 1.0 
                || back_end.has_texture_alpha(texture);
            if needs_alpha { back_end.enable_alpha_blend(); }
            back_end.enable_single_texture(texture);
            back_end.tri_list_xy_f32_rgba_f32_uv_f32(
                rect_tri_list_xy_f32(self.transform.get(), rect),
                rect_tri_list_rgba_f32(color),
                rect_tri_list_uv_f32(texture, source_rect)
            );
            back_end.disable_single_texture();
            if needs_alpha { back_end.disable_alpha_blend(); }
        } else {
            unimplemented!();
        }
    }
}

