use {
    BackEnd,
    Draw,
    ImageSize,
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
    pub view: Matrix2d,
    /// Current transformation.
    pub transform: Matrix2d,
    /// Current rectangle.
    pub rect: Rectangle,
    /// Current image.
    pub image: &'b I,
    /// Current source rectangle.
    pub source_rect: SourceRectangle,
    /// Current color.
    pub color: Color,
}

impl<'b, I>
Clone 
for ImageRectangleColorContext<'b, I> {
    #[inline(always)]
    fn clone(&self) -> ImageRectangleColorContext<'b, I> {
        ImageRectangleColorContext {
            view: self.view,
            transform: self.transform,
            rect: self.rect,
            image: self.image,
            source_rect: self.source_rect,
            color: self.color,
        }
    }
}

impl<'b, I> 
HasTransform<Matrix2d> 
for ImageRectangleColorContext<'b, I> {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform
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
            view: self.view,
            transform: value,
            rect: self.rect,
            image: self.image,
            source_rect: self.source_rect,
            color: self.color,
        }
    }
}

impl<'b, I> 
HasViewTransform<Matrix2d> 
for ImageRectangleColorContext<'b, I> {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view
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
            view: value,
            transform: self.transform,
            rect: self.rect,
            image: self.image,
            source_rect: self.source_rect,
            color: self.color,
        }
    }
}

impl<'b, I> 
HasColor<Color> 
for ImageRectangleColorContext<'b, I> {
    #[inline(always)]
    fn get_color(&self) -> Color {
        self.color
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
            view: self.view,
            transform: self.transform,
            color: value,
            rect: self.rect,
            image: self.image,
            source_rect: self.source_rect,
        }
    }
}

impl<'b, I> 
HasRectangle<Rectangle> 
for ImageRectangleColorContext<'b, I> {
    #[inline(always)]
    fn get_rectangle(&self) -> Rectangle {
        self.rect
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
            view: self.view,
            transform: self.transform,
            rect: rect,
            image: self.image,
            source_rect: self.source_rect,
            color: self.color,
        }
    }
}

impl<'b, I> 
HasSourceRectangle<SourceRectangle> 
for ImageRectangleColorContext<'b, I> {
    #[inline(always)]
    fn get_source_rectangle(&self) -> SourceRectangle {
        self.source_rect
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
            view: self.view,
            transform: self.transform,
            rect: self.rect,
            image: self.image,
            source_rect: source_rect,
            color: self.color,
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
            let rect = self.rect;
            let color = self.color;
            let texture = self.image;
            let source_rect = self.source_rect;
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely opaque 
            // or if the texture has alpha channel.
            let needs_alpha = color[3] != 1.0 
                || back_end.has_texture_alpha(texture);
            if needs_alpha { back_end.enable_alpha_blend(); }
            back_end.enable_single_texture(texture);
            back_end.tri_list_xy_f32_rgba_f32_uv_f32(
                rect_tri_list_xy_f32(self.transform, rect),
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

