use {
    AddRectangle,
    BackEnd,
    Draw,
    Field,
    ImageSize,
    ImageRectangleColorContext,
    Value,
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
    pub view: Field<Matrix2d>,
    /// Current transformation.
    pub transform: Field<Matrix2d>,
    /// Current image.
    pub image: Field<&'b I>,
    /// Current source rectangle.
    pub source_rect: Field<SourceRectangle>,
    /// Current color.
    pub color: Field<Color>,
}

impl<'b, I> 
Clone 
for ImageColorContext<'b, I> {
    #[inline(always)]
    fn clone(&self) -> ImageColorContext<'b, I> {
        ImageColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            image: Value(self.image.get()),
            source_rect: Value(self.source_rect.get()),
            color: Value(self.color.get()),
        }
    }
}

impl<'b, I> 
HasTransform<Matrix2d> 
for ImageColorContext<'b, I> {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform.get()
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
            view: Value(self.view.get()),
            transform: Value(value),
            image: Value(self.image.get()),
            source_rect: Value(self.source_rect.get()),
            color: Value(self.color.get()),
        }
    }
}

impl<'b, I> 
HasViewTransform<Matrix2d> 
for ImageColorContext<'b, I> {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view.get()
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
            view: Value(value),
            transform: Value(self.transform.get()),
            image: Value(self.image.get()),
            source_rect: Value(self.source_rect.get()),
            color: Value(self.color.get()),
        }
    }
}

impl<'b, I> 
HasColor<Color> 
for ImageColorContext<'b, I> {
    #[inline(always)]
    fn get_color(&self) -> Color {
        self.color.get()
    }
}

impl<'b, I> 
CanColor<ImageColorContext<'b, I>, Color> 
for ImageColorContext<'b, I> {
    #[inline(always)]
    fn color(&self, value: Color) -> ImageColorContext<'b, I> {
        ImageColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            color: Value(value),
            image: Value(self.image.get()),
            source_rect: Value(self.source_rect.get()),
        }
    }
}

impl<'b, I> 
HasSourceRectangle<SourceRectangle> 
for ImageColorContext<'b, I> {
    #[inline(always)]
    fn get_source_rectangle(&self) -> SourceRectangle {
        self.source_rect.get()
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
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            image: Value(self.image.get()),
            source_rect: Value(source_rect),
            color: Value(self.color.get()),
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
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            rect: Value([x, y, w, h]),
            image: Value(self.image.get()),
            source_rect: Value(self.source_rect.get()),
            color: Value(self.color.get()),
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
            let color = self.color.get();
            let texture = self.image.get();
            let source_rect = self.source_rect.get();
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

