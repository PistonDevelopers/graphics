use {
    AddColor,
    AddRectangle,
    BackEnd,
    Borrowed,
    Draw,
    Field,
    Image,
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
pub struct ImageContext<'a, 'b, I> {
    /// View transformation.
    pub view: Field<'a, Matrix2d>,
    /// Current transformation.
    pub transform: Field<'a, Matrix2d>,
    /// Current image.
    pub image: Field<'a, &'b I>,
    /// Current source rectangle.
    pub source_rect: Field<'a, SourceRectangle>,
}

impl<'a, 'b, I> Clone for ImageContext<'a, 'b, I> {
    #[inline(always)]
    fn clone(&self) -> ImageContext<'static, 'b, I> {
        ImageContext {
            view: Value(*self.view.get()),
            transform: Value(*self.transform.get()),
            image: Value(*self.image.get()),
            source_rect: Value(*self.source_rect.get()),
        }
    }
}

impl<'a, 'b, I> HasTransform<'a, Matrix2d> for ImageContext<'a, 'b, I> {
    #[inline(always)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a, 'b, I> CanTransform<'a, ImageContext<'a, 'b, I>, Matrix2d> 
for ImageContext<'a, 'b, I> {
    #[inline(always)]
    fn transform(&'a self, value: Matrix2d) -> ImageContext<'a, 'b, I> {
        ImageContext {
            view: Borrowed(self.view.get()),
            transform: Value(value),
            image: Borrowed(self.image.get()),
            source_rect: Borrowed(self.source_rect.get()),
        }
    }
}

impl<'a, 'b, I> HasViewTransform<'a, Matrix2d> for ImageContext<'a, 'b, I> {
    #[inline(always)]
    fn get_view_transform(&'a self) -> &'a Matrix2d {
        self.view.get()
    }
}

impl<'a, 'b, I> CanViewTransform<'a, ImageContext<'a, 'b, I>, Matrix2d>
for ImageContext<'a, 'b, I> {
    #[inline(always)]
    fn view_transform(&'a self, value: Matrix2d) -> ImageContext<'a, 'b, I> {
        ImageContext {
            view: Value(value),
            transform: Borrowed(self.transform.get()),
            image: Borrowed(self.image.get()),
            source_rect: Borrowed(self.source_rect.get()),
        }
    }
}

static WHITE: &'static Color = &[1.0, ..4];

impl<'a, 'b, I> HasColor<'a, Color> for ImageContext<'a, 'b, I> {
    #[inline(always)]
    fn get_color(&'a self) -> &'a Color {
        WHITE
    }
}

impl<'a, 'b, I> CanColor<'a, ImageColorContext<'a, 'b, I>, Color> for ImageContext<'a, 'b, I> {
    #[inline(always)]
    fn color(&'a self, value: Color) -> ImageColorContext<'a, 'b, I> {
        ImageColorContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            color: Value(value),
            image: Borrowed(self.image.get()),
            source_rect: Borrowed(self.source_rect.get()),
        }
    }
}

impl<'a, 'b, I> HasSourceRectangle<'a, SourceRectangle> 
for ImageContext<'a, 'b, I> {
    #[inline(always)]
    fn get_source_rectangle(&'a self) -> &'a SourceRectangle {
        self.source_rect.get()
    }
}

impl<'a, 'b, I> CanSourceRectangle<'a, ImageContext<'a, 'b, I>, SourceRectangle> 
for ImageContext<'a, 'b, I> {
    #[inline(always)]
    fn source_rectangle(&'a self, source_rect: SourceRectangle) -> ImageContext<'a, 'b, I> {
        ImageContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            image: Borrowed(self.image.get()),
            source_rect: Value(source_rect),
        }
    }
}

impl<'a, 'b, I> AddRectangle<'a, ImageRectangleContext<'a, 'b, I>> 
for ImageContext<'a, 'b, I> {
    #[inline(always)]
    fn rect(&'a self, x: Scalar, y: Scalar, w: Scalar, h: Scalar) -> ImageRectangleContext<'a, 'b, I> {
        ImageRectangleContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            rect: Value([x, y, w, h]),
            image: Borrowed(self.image.get()),
            source_rect: Borrowed(self.source_rect.get()),
        }
    }
}

impl<'a, 'b, B: BackEnd<I>, I: Image> Draw<'a, B, I> for ImageContext<'a, 'b, I> {
    #[inline(always)]
    fn draw(&'a self, back_end: &mut B) {
        if back_end.supports_single_texture()
        && back_end.supports_tri_list_xy_f32_rgba_f32_uv_f32() {
            let color: [f32, ..4] = [1.0, 1.0, 1.0, 1.0];
            let &texture = self.image.get();
            let source_rect = self.source_rect.get();
            let rect = [0.0, 0.0, source_rect[2] as f64, source_rect[3] as f64];
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely opaque or if the texture has alpha channel.
            let needs_alpha = color[3] != 1.0 || back_end.has_texture_alpha(texture);
            if needs_alpha { back_end.enable_alpha_blend(); }
            back_end.enable_single_texture(texture);
            back_end.tri_list_xy_f32_rgba_f32_uv_f32(
                rect_tri_list_xy_f32(*self.transform.get(), rect),
                rect_tri_list_rgba_f32(color),
                rect_tri_list_uv_f32(texture, *source_rect)
            );
            back_end.disable_single_texture();
            if needs_alpha { back_end.disable_alpha_blend(); }
        } else {
            unimplemented!();
        }
    }
}

impl<'a, 'b, I> AddColor<'a, ImageColorContext<'a, 'b, I>> for ImageContext<'a, 'b, I> {
    #[inline(always)]
    fn rgba(
        &'a self,
        r: ColorComponent,
        g: ColorComponent,
        b: ColorComponent,
        a: ColorComponent
    ) -> ImageColorContext<'a, 'b, I> {
        ImageColorContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            image: Borrowed(self.image.get()),
            source_rect: Borrowed(self.source_rect.get()),
            color: Value([r, g, b, a]),
        }
    }
}

