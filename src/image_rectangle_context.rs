use {
    AddColor,
    BackEnd,
    Draw,
    ImageSize,
    ImageRectangleColorContext,
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
    ColorComponent,
    HasColor,
    HasRectangle,
    HasSourceRectangle,
    HasTransform,
    HasViewTransform,
    Matrix2d,
    SourceRectangle,
    Rectangle,
};

/// An image rectangle context.
pub struct ImageRectangleContext<'b, I> {
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
}

impl<'b, I> 
Clone 
for ImageRectangleContext<'b, I> {
    #[inline(always)]
    fn clone(&self) -> ImageRectangleContext<'b, I> {
        ImageRectangleContext {
            view: self.view,
            transform: self.transform,
            rect: self.rect,
            image: self.image,
            source_rect: self.source_rect,
        }
    }
}

impl<'b, I> 
HasTransform<Matrix2d> 
for ImageRectangleContext<'b, I> {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform
    }
}

impl<'b, I> 
CanTransform<ImageRectangleContext<'b, I>, Matrix2d> 
for ImageRectangleContext<'b, I> {
    #[inline(always)]
    fn transform(
        &self, 
        value: Matrix2d
    ) -> ImageRectangleContext<'b, I> {
        ImageRectangleContext {
            view: self.view,
            transform: value,
            rect: self.rect,
            image: self.image,
            source_rect: self.source_rect,
        }
    }
}

impl<'b, I> 
HasViewTransform<Matrix2d> 
for ImageRectangleContext<'b, I> {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view
    }
}

impl<'b, I> 
CanViewTransform<ImageRectangleContext<'b, I>, Matrix2d>
for ImageRectangleContext<'b, I> {
    #[inline(always)]
    fn view_transform(
        &self, 
        value: Matrix2d
    ) -> ImageRectangleContext<'b, I> {
        ImageRectangleContext {
            view: value,
            transform: self.transform,
            rect: self.rect,
            image: self.image,
            source_rect: self.source_rect,
        }
    }
}

static WHITE: Color = [1.0, ..4];

impl<'b, I> 
HasColor<Color> 
for ImageRectangleContext<'b, I> {
    #[inline(always)]
    fn get_color(&self) -> Color {
        WHITE
    }
}

impl<'b, I> 
CanColor<ImageRectangleColorContext<'b, I>, Color> 
for ImageRectangleContext<'b, I> {
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
for ImageRectangleContext<'b, I> {
    #[inline(always)]
    fn get_rectangle(&self) -> Rectangle {
        self.rect
    }
}

impl<'b, I> 
CanRectangle<ImageRectangleContext<'b, I>, Rectangle> 
for ImageRectangleContext<'b, I> {
    #[inline(always)]
    fn rectangle(
        &self, 
        rect: Rectangle
    ) -> ImageRectangleContext<'b, I> {
        ImageRectangleContext {
            view: self.view,
            transform: self.transform,
            rect: rect,
            image: self.image,
            source_rect: self.source_rect,
        }
    }
}

impl<'b, I> 
HasSourceRectangle<SourceRectangle> 
for ImageRectangleContext<'b, I> {
    #[inline(always)]
    fn get_source_rectangle(&self) -> SourceRectangle {
        self.source_rect
    }
}

impl<'b, I> 
CanSourceRectangle<ImageRectangleContext<'b, I>, SourceRectangle> 
for ImageRectangleContext<'b, I> {
    #[inline(always)]
    fn source_rectangle(
        &self, 
        source_rect: SourceRectangle
    ) -> ImageRectangleContext<'b, I> {
        ImageRectangleContext {
            view: self.view,
            transform: self.transform,
            rect: self.rect,
            image: self.image,
            source_rect: source_rect,
        }
    }
}

impl<'b, B: BackEnd<I>, I: ImageSize> 
Draw<B, I> 
for ImageRectangleContext<'b, I> {
    #[inline(always)]
    fn draw(&self, back_end: &mut B) {
        if back_end.supports_single_texture()
        && back_end.supports_tri_list_xy_f32_rgba_f32_uv_f32() {
            let rect = self.rect;
            let color: [f32, ..4] = [1.0, 1.0, 1.0, 1.0];
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

impl<'b, I> 
AddColor<ImageRectangleColorContext<'b, I>> 
for ImageRectangleContext<'b, I> {
    #[inline(always)]
    fn rgba(
        &self,
        r: ColorComponent,
        g: ColorComponent,
        b: ColorComponent,
        a: ColorComponent
    ) -> ImageRectangleColorContext<'b, I> {
        ImageRectangleColorContext {
            view: self.view,
            transform: self.transform,
            rect: self.rect,
            image: self.image,
            source_rect: self.source_rect,
            color: [r, g, b, a],
        }
    }
}

