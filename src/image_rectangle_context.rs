use {
    AddColor,
    BackEnd,
    Borrowed,
    CanColor,
    CanTransform,
    Color,
    Draw,
    Field,
    HasColor,
    HasTransform,
    Image,
    ImageRectangleColorContext,
    Matrix2d,
    Rectangle,
    RelativeRectangle,
    Value,
    View,
};
use vecmath::{
    identity,
    margin_rectangle,
    relative_rectangle,
};
use triangulation::{
    rect_tri_list_xy_f32,
    rect_tri_list_rgba_f32,
    rect_tri_list_uv_f32,
};

/// An image rectangle context.
pub struct ImageRectangleContext<'a> {
    /// Base/original transformation.
    pub base: Field<'a, Matrix2d>,
    /// Current transformation.
    pub transform: Field<'a, Matrix2d>,
    /// Current rectangle.
    pub rect: Field<'a, Rectangle>,
    /// Current image.
    pub image: Field<'a, Image>,
}

impl<'a> HasTransform<'a, Matrix2d> for ImageRectangleContext<'a> {
    #[inline(always)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a> CanTransform<'a, ImageRectangleContext<'a>, Matrix2d> for ImageRectangleContext<'a> {
    #[inline(always)]
    fn transform(&'a self, value: Matrix2d) -> ImageRectangleContext<'a> {
        ImageRectangleContext {
            base: Borrowed(self.base.get()),
            transform: Value(value),
            rect: Borrowed(self.rect.get()),
            image: Borrowed(self.image.get()),
        }
    }
}

static WHITE: Color = [1.0, ..4];

impl<'a> HasColor<'a, Color> for ImageRectangleContext<'a> {
    #[inline(always)]
    fn get_color(&'a self) -> &'a Color {
        &WHITE
    }
}

impl<'a> CanColor<'a, ImageRectangleColorContext<'a>, Color> for ImageRectangleContext<'a> {
    #[inline(always)]
    fn color(&'a self, value: Color) -> ImageRectangleColorContext<'a> {
        ImageRectangleColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            color: Value(value),
            rect: Borrowed(self.rect.get()),
            image: Borrowed(self.image.get()),
        }
    }
}

impl<'a> Draw<'a> for ImageRectangleContext<'a> {
    fn draw<B: BackEnd>(&'a self, back_end: &mut B) {
        if back_end.supports_single_texture()
        && back_end.supports_tri_list_xy_f32_rgba_f32_uv_f32() {
            let rect = self.rect.get();
            let color: [f32, ..4] = [1.0, 1.0, 1.0, 1.0];
            let texture_id = self.image.get().texture_id;
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely opaque or if the texture has alpha channel.
            let needs_alpha = color[3] != 1.0 || back_end.has_texture_alpha(texture_id);
            if needs_alpha { back_end.enable_alpha_blend(); }
            back_end.enable_single_texture(texture_id);
            back_end.tri_list_xy_f32_rgba_f32_uv_f32(
                rect_tri_list_xy_f32(self.transform.get(), rect),
                rect_tri_list_rgba_f32(color),
                rect_tri_list_uv_f32(self.image.get())
            );
            back_end.disable_single_texture();
            if needs_alpha { back_end.disable_alpha_blend(); }
        } else {
            unimplemented!();
        }
    }
}

impl<'a> RelativeRectangle<'a> for ImageRectangleContext<'a> {
    #[inline(always)]
    fn margin(&'a self, m: f64) -> ImageRectangleContext<'a> {
        ImageRectangleContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            image: Borrowed(self.image.get()),
            rect: Value(margin_rectangle(self.rect.get(), m)),
        }
    }

    #[inline(always)]
    fn rel(&'a self, x: f64, y: f64) -> ImageRectangleContext<'a> {
        ImageRectangleContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            image: Borrowed(self.image.get()),
            rect: Value(relative_rectangle(self.rect.get(), x, y)),
        }
    }
}

impl<'a> View<'a> for ImageRectangleContext<'a> {
    #[inline(always)]
    fn view(&'a self) -> ImageRectangleContext<'a> {
        ImageRectangleContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.base.get()),
            rect: Borrowed(self.rect.get()),
            image: Borrowed(self.image.get()),
        }
    }

    #[inline(always)]
    fn reset(&'a self) -> ImageRectangleContext<'a> {
        ImageRectangleContext {
            base: Borrowed(self.base.get()),
            transform: Value(identity()),
            rect: Borrowed(self.rect.get()),
            image: Borrowed(self.image.get()),
        }
    }

    #[inline(always)]
    fn store_view(&'a self) -> ImageRectangleContext<'a> {
        ImageRectangleContext {
            base: Borrowed(self.transform.get()),
            transform: Borrowed(self.transform.get()),
            rect: Borrowed(self.rect.get()),
            image: Borrowed(self.image.get()),
        }
    }
}

impl<'a> AddColor<'a, ImageRectangleColorContext<'a>> for ImageRectangleContext<'a> {
    #[inline(always)]
    fn rgba(&'a self, r: f32, g: f32, b: f32, a: f32) -> ImageRectangleColorContext<'a> {
        ImageRectangleColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            rect: Borrowed(self.rect.get()),
            image: Borrowed(self.image.get()),
            color: Value([r, g, b, a]),
        }
    }
}

