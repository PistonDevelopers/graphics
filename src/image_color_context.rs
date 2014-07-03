use {
    AddRectangle,
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
    CanSourceRectangle,
    CanTransform,
    CanViewTransform,
    Color,
    HasColor,
    HasSourceRectangle,
    HasTransform,
    HasViewTransform,
    SourceRectangle,
    Matrix2d,
    Scalar,
};

/// An image rectangle context.
pub struct ImageColorContext<'b, I> {
    /// View transformation.
    pub view: Matrix2d,
    /// Current transformation.
    pub transform: Matrix2d,
    /// Current image.
    pub image: &'b I,
    /// Current source rectangle.
    pub source_rect: SourceRectangle,
    /// Current color.
    pub color: Color,
}

impl<'b, I> 
Clone 
for ImageColorContext<'b, I> {
    #[inline(always)]
    fn clone(&self) -> ImageColorContext<'b, I> {
        ImageColorContext {
            view: self.view,
            transform: self.transform,
            image: self.image,
            source_rect: self.source_rect,
            color: self.color,
        }
    }
}

impl<'b, I> 
HasTransform<Matrix2d> 
for ImageColorContext<'b, I> {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform
    }
}

impl<'b, I> 
CanTransform<ImageColorContext<'b, I>, Matrix2d> 
for ImageColorContext<'b, I> {
    #[inline(always)]
    fn transform(
        &self, 
        value: Matrix2d
    ) -> ImageColorContext<'b, I> {
        ImageColorContext {
            view: self.view,
            transform: value,
            image: self.image,
            source_rect: self.source_rect,
            color: self.color,
        }
    }
}

impl<'b, I> 
HasViewTransform<Matrix2d> 
for ImageColorContext<'b, I> {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view
    }
}

impl<'b, I> 
CanViewTransform<ImageColorContext<'b, I>, Matrix2d>
for ImageColorContext<'b, I> {
    #[inline(always)]
    fn view_transform(
        &self, 
        value: Matrix2d
    ) -> ImageColorContext<'b, I> {
        ImageColorContext {
            view: value,
            transform: self.transform,
            image: self.image,
            source_rect: self.source_rect,
            color: self.color,
        }
    }
}

impl<'b, I> 
HasColor<Color> 
for ImageColorContext<'b, I> {
    #[inline(always)]
    fn get_color(&self) -> Color {
        self.color
    }
}

impl<'b, I> 
CanColor<ImageColorContext<'b, I>, Color> 
for ImageColorContext<'b, I> {
    #[inline(always)]
    fn color(&self, value: Color) -> ImageColorContext<'b, I> {
        ImageColorContext {
            view: self.view,
            transform: self.transform,
            color: value,
            image: self.image,
            source_rect: self.source_rect,
        }
    }
}

impl<'b, I> 
HasSourceRectangle<SourceRectangle> 
for ImageColorContext<'b, I> {
    #[inline(always)]
    fn get_source_rectangle(&self) -> SourceRectangle {
        self.source_rect
    }
}

impl<'b, I> 
CanSourceRectangle<ImageColorContext<'b, I>, SourceRectangle> 
for ImageColorContext<'b, I> {
    #[inline(always)]
    fn source_rectangle(
        &self, 
        source_rect: SourceRectangle
    ) -> ImageColorContext<'b, I> {
        ImageColorContext {
            view: self.view,
            transform: self.transform,
            image: self.image,
            source_rect: source_rect,
            color: self.color,
        }
    }
}

impl<'b, I> 
AddRectangle<ImageRectangleColorContext<'b, I>> 
for ImageColorContext<'b, I> {
    #[inline(always)]
    fn rect(
        &self, 
        x: Scalar, 
        y: Scalar, 
        w: Scalar, 
        h: Scalar
    ) -> ImageRectangleColorContext<'b, I> {
        ImageRectangleColorContext {
            view: self.view,
            transform: self.transform,
            rect: [x, y, w, h],
            image: self.image,
            source_rect: self.source_rect,
            color: self.color,
        }
    }
}

impl<'b, B: BackEnd<I>, I: ImageSize> 
Draw<B, I> 
for ImageColorContext<'b, I> {
    #[inline(always)]
    fn draw(&self, back_end: &mut B) {
        if back_end.supports_single_texture()
        && back_end.supports_tri_list_xy_f32_rgba_f32_uv_f32() {
            let color = self.color;
            let texture = self.image;
            let source_rect = self.source_rect;
            let rect = [
                0.0, 
                0.0, 
                source_rect[2] as f64, 
                source_rect[3] as f64
            ];
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely
            // opaque or if the texture has alpha channel.
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

