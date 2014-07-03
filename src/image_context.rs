use {
    AddColor,
    AddRectangle,
    BackEnd,
    Draw,
    ImageSize,
    ImageColorContext,
    ImageRectangleContext,
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
    ColorComponent,
    HasColor,
    HasSourceRectangle,
    HasTransform,
    HasViewTransform,
    Matrix2d,
    Scalar,
    SourceRectangle,
};

/// An image context.
pub struct ImageContext<'b, I> {
    /// View transformation.
    pub view: Matrix2d,
    /// Current transformation.
    pub transform: Matrix2d,
    /// Current image.
    pub image: &'b I,
    /// Current source rectangle.
    pub source_rect: SourceRectangle,
}

impl<'b, I> 
Clone 
for ImageContext<'b, I> {
    #[inline(always)]
    fn clone(&self) -> ImageContext<'b, I> {
        ImageContext {
            view: self.view,
            transform: self.transform,
            image: self.image,
            source_rect: self.source_rect,
        }
    }
}

impl<'b, I> 
HasTransform<Matrix2d> 
for ImageContext<'b, I> {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform
    }
}

impl<'b, I> 
CanTransform<ImageContext<'b, I>, Matrix2d> 
for ImageContext<'b, I> {
    #[inline(always)]
    fn transform(
        &self, 
        value: Matrix2d
    ) -> ImageContext<'b, I> {
        ImageContext {
            view: self.view,
            transform: value,
            image: self.image,
            source_rect: self.source_rect,
        }
    }
}

impl<'b, I> 
HasViewTransform<Matrix2d> 
for ImageContext<'b, I> {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view
    }
}

impl<'b, I> 
CanViewTransform<ImageContext<'b, I>, Matrix2d>
for ImageContext<'b, I> {
    #[inline(always)]
    fn view_transform(
        &self, 
        value: Matrix2d
    ) -> ImageContext<'b, I> {
        ImageContext {
            view: value,
            transform: self.transform,
            image: self.image,
            source_rect: self.source_rect,
        }
    }
}

static WHITE: Color = [1.0, ..4];

impl<'b, I> 
HasColor<Color> 
for ImageContext<'b, I> {
    #[inline(always)]
    fn get_color(&self) -> Color {
        WHITE
    }
}

impl<'b, I> 
HasSourceRectangle<SourceRectangle> 
for ImageContext<'b, I> {
    #[inline(always)]
    fn get_source_rectangle(&self) -> SourceRectangle {
        self.source_rect
    }
}

impl<'b, I> 
CanSourceRectangle<ImageContext<'b, I>, SourceRectangle> 
for ImageContext<'b, I> {
    #[inline(always)]
    fn source_rectangle(
        &self, 
        source_rect: SourceRectangle
    ) -> ImageContext<'b, I> {
        ImageContext {
            view: self.view,
            transform: self.transform,
            image: self.image,
            source_rect: source_rect,
        }
    }
}

impl<'b, I> 
AddRectangle<ImageRectangleContext<'b, I>> 
for ImageContext<'b, I> {
    #[inline(always)]
    fn rect(
        &self, 
        x: Scalar, 
        y: Scalar, 
        w: Scalar, 
        h: Scalar
    ) -> ImageRectangleContext<'b, I> {
        ImageRectangleContext {
            view: self.view,
            transform: self.transform,
            rect: [x, y, w, h],
            image: self.image,
            source_rect: self.source_rect,
        }
    }
}

impl<'b, B: BackEnd<I>, I: ImageSize> 
Draw<B, I> 
for ImageContext<'b, I> {
    #[inline(always)]
    fn draw(&self, back_end: &mut B) {
        if back_end.supports_single_texture()
        && back_end.supports_tri_list_xy_f32_rgba_f32_uv_f32() {
            let color: [f32, ..4] = [1.0, 1.0, 1.0, 1.0];
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

