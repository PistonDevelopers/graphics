use {
    AddColor,
    AddRectangle,
    BackEnd,
    Draw,
    Field,
    ImageSize,
    ImageColorContext,
    ImageRectangleContext,
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
    pub view: Field<Matrix2d>,
    /// Current transformation.
    pub transform: Field<Matrix2d>,
    /// Current image.
    pub image: Field<&'b I>,
    /// Current source rectangle.
    pub source_rect: Field<SourceRectangle>,
}

impl<'b, I> 
Clone 
for ImageContext<'b, I> {
    #[inline(always)]
    fn clone(&self) -> ImageContext<'b, I> {
        ImageContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            image: Value(self.image.get()),
            source_rect: Value(self.source_rect.get()),
        }
    }
}

impl<'b, I> 
HasTransform<Matrix2d> 
for ImageContext<'b, I> {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform.get()
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
            view: Value(self.view.get()),
            transform: Value(value),
            image: Value(self.image.get()),
            source_rect: Value(self.source_rect.get()),
        }
    }
}

impl<'b, I> 
HasViewTransform<Matrix2d> 
for ImageContext<'b, I> {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view.get()
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
            view: Value(value),
            transform: Value(self.transform.get()),
            image: Value(self.image.get()),
            source_rect: Value(self.source_rect.get()),
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
CanColor<ImageColorContext<'b, I>, Color> 
for ImageContext<'b, I> {
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
for ImageContext<'b, I> {
    #[inline(always)]
    fn get_source_rectangle(&self) -> SourceRectangle {
        self.source_rect.get()
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
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            image: Value(self.image.get()),
            source_rect: Value(source_rect),
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
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            rect: Value([x, y, w, h]),
            image: Value(self.image.get()),
            source_rect: Value(self.source_rect.get()),
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

impl<'b, I> 
AddColor<ImageColorContext<'b, I>> 
for ImageContext<'b, I> {
    #[inline(always)]
    fn rgba(
        &self,
        r: ColorComponent,
        g: ColorComponent,
        b: ColorComponent,
        a: ColorComponent
    ) -> ImageColorContext<'b, I> {
        ImageColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            image: Value(self.image.get()),
            source_rect: Value(self.source_rect.get()),
            color: Value([r, g, b, a]),
        }
    }
}

