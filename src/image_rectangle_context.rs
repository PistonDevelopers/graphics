use {
    AddColor,
    BackEnd,
    Borrowed,
    Color,
    Draw,
    Field,
    Image,
    ImageRectangleColorContext,
    Matrix2d,
    Rectangle,
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
    CanTransform,
    CanViewTransform,
    HasColor,
    HasRectangle,
    HasTransform,
    HasViewTransform,
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

impl<'a> Clone for ImageRectangleContext<'a> {
    #[inline(always)]
    fn clone(&self) -> ImageRectangleContext<'static> {
        ImageRectangleContext {
            base: self.base.clone(),
            transform: self.transform.clone(),
            rect: self.rect.clone(),
            image: self.image.clone(),
        }
    }
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

impl<'a> HasViewTransform<'a, Matrix2d> for ImageRectangleContext<'a> {
    #[inline(always)]
    fn get_view_transform(&'a self) -> &'a Matrix2d {
        self.base.get()
    }
}

impl<'a> CanViewTransform<'a, ImageRectangleContext<'a>, Matrix2d> 
for ImageRectangleContext<'a> {
    #[inline(always)]
    fn view_transform(&'a self, value: Matrix2d) -> ImageRectangleContext<'a> {
        ImageRectangleContext {
            base: Value(value),
            transform: Borrowed(self.transform.get()),
            rect: Borrowed(self.rect.get()),
            image: Borrowed(self.image.get()),
        }
    }
}

static WHITE: &'static Color = &Color([1.0, ..4]);

impl<'a> HasColor<'a, Color> for ImageRectangleContext<'a> {
    #[inline(always)]
    fn get_color(&'a self) -> &'a Color {
        WHITE
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

impl<'a> HasRectangle<'a, Rectangle> for ImageRectangleContext<'a> {
    #[inline(always)]
    fn get_rectangle(&'a self) -> &'a Rectangle {
        self.rect.get()
    }
}

impl<'a> CanRectangle<'a, ImageRectangleContext<'a>, Rectangle> for ImageRectangleContext<'a> {
    #[inline(always)]
    fn rectangle(&'a self, rect: Rectangle) -> ImageRectangleContext<'a> {
        ImageRectangleContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            rect: Value(rect),
            image: Borrowed(self.image.get()),
        }
    }
}

impl<'a> Draw<'a> for ImageRectangleContext<'a> {
    #[inline(always)]
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
                rect_tri_list_rgba_f32(&Color(color)),
                rect_tri_list_uv_f32(self.image.get())
            );
            back_end.disable_single_texture();
            if needs_alpha { back_end.disable_alpha_blend(); }
        } else {
            unimplemented!();
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
            color: Value(Color([r, g, b, a])),
        }
    }
}

