use {
    BackEnd,
    Borrowed,
    Color,
    Draw,
    Field,
    Image,
    Matrix2d,
    Rectangle,
    RelativeRectangle,
    Transform2d,
    Value,
    View,
};
use vecmath::{
    identity,
    margin_rectangle,
    multiply,
    relative_rectangle,
    rotate_radians,
    scale,
    shear,
    translate,
};
use triangulation::{
    rect_tri_list_xy_f32,
    rect_tri_list_rgba_f32,
    rect_tri_list_uv_f32,
};

/// An image rectangle context.
pub struct ImageRectangleColorContext<'a> {
    /// Base/original transformation.
    pub base: Field<'a, Matrix2d>,
    /// Current transformation.
    pub transform: Field<'a, Matrix2d>,
    /// Current rectangle.
    pub rect: Field<'a, Rectangle>,
    /// Current image.
    pub image: Field<'a, Image>,
    /// Current color.
    pub color: Field<'a, Color>,
}

impl<'a> Transform2d<'a> for ImageRectangleColorContext<'a> {
    #[inline(always)]
    fn trans(&'a self, x: f64, y: f64) -> ImageRectangleColorContext<'a> {
        ImageRectangleColorContext {
            base: Borrowed(self.base.get()),
            transform: {
                let trans = translate(x, y);
                Value(multiply(&trans, self.transform.get()))
            },
            rect: Borrowed(self.rect.get()),
            image: Borrowed(self.image.get()),
            color: Borrowed(self.color.get()),
        }
    }
    
    #[inline(always)]
    fn trans_local(&'a self, x: f64, y: f64) -> ImageRectangleColorContext<'a> {
        ImageRectangleColorContext {
            base: Borrowed(self.base.get()),
            transform: {
                let trans = translate(x, y);
                Value(multiply(self.transform.get(), &trans))
            },
            rect: Borrowed(self.rect.get()),
            image: Borrowed(self.image.get()),
            color: Borrowed(self.color.get()),
        }
    }

    #[inline(always)]
    fn rot_rad(&'a self, angle: f64) -> ImageRectangleColorContext<'a> {
        ImageRectangleColorContext {
            base: Borrowed(self.base.get()),
            transform: {
                let rot = rotate_radians(angle);
                Value(multiply(&rot, self.transform.get()))
            },
            rect: Borrowed(self.rect.get()),
            image: Borrowed(self.image.get()),
            color: Borrowed(self.color.get()),
        }
    }

    #[inline(always)]
    fn rot_rad_local(&'a self, angle: f64) -> ImageRectangleColorContext<'a> {
        ImageRectangleColorContext {
            base: Borrowed(self.base.get()),
            transform: {
                let rot = rotate_radians(angle);
                Value(multiply(self.transform.get(), &rot))
            },
            rect: Borrowed(self.rect.get()),
            image: Borrowed(self.image.get()),
            color: Borrowed(self.color.get()),
        }
    }

    #[inline(always)]
    fn scale(&'a self, sx: f64, sy: f64) -> ImageRectangleColorContext<'a> {
        ImageRectangleColorContext {
            base: Borrowed(self.base.get()),
            transform: {
                let scale = scale(sx, sy);
                Value(multiply(&scale, self.transform.get()))
            },
            rect: Borrowed(self.rect.get()),
            image: Borrowed(self.image.get()),
            color: Borrowed(self.color.get()),
        }
    }

    #[inline(always)]
    fn scale_local(&'a self, sx: f64, sy: f64) -> ImageRectangleColorContext<'a> {
        ImageRectangleColorContext {
            base: Borrowed(self.base.get()),
            transform: {
                let scale = scale(sx, sy);
                Value(multiply(self.transform.get(), &scale))
            },
            rect: Borrowed(self.rect.get()),
            image: Borrowed(self.image.get()),
            color: Borrowed(self.color.get()),
        }
    }

    #[inline(always)]
    fn shear(&'a self, sx: f64, sy: f64) -> ImageRectangleColorContext<'a> {
        ImageRectangleColorContext {
            base: Borrowed(self.base.get()),
            transform: {
                let shear = shear(sx, sy);
                Value(multiply(&shear, self.transform.get()))
            },
            rect: Borrowed(self.rect.get()),
            image: Borrowed(self.image.get()),
            color: Borrowed(self.color.get()),
        }
    }
    
    #[inline(always)]
    fn shear_local(&'a self, sx: f64, sy: f64) -> ImageRectangleColorContext<'a> {
        ImageRectangleColorContext {
            base: Borrowed(self.base.get()),
            transform: {
                let shear = shear(sx, sy);
                Value(multiply(self.transform.get(), &shear))
            },
            rect: Borrowed(self.rect.get()),
            image: Borrowed(self.image.get()),
            color: Borrowed(self.color.get()),
        }
    }
}

impl<'a> Draw<'a> for ImageRectangleColorContext<'a> {
    fn draw<B: BackEnd>(&'a self, back_end: &mut B) {
        if back_end.supports_single_texture()
        && back_end.supports_tri_list_xy_f32_rgba_f32_uv_f32() {
            let rect = self.rect.get();
            let color = self.color.get();
            let color: [f32, ..4] = [color[0], color[1], color[2], color[3]];
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

impl<'a> RelativeRectangle<'a> for ImageRectangleColorContext<'a> {
    #[inline(always)]
    fn margin(&'a self, m: f64) -> ImageRectangleColorContext<'a> {
        ImageRectangleColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            image: Borrowed(self.image.get()),
            rect: Value(margin_rectangle(self.rect.get(), m)),
            color: Borrowed(self.color.get()),
        }
    }

    #[inline(always)]
    fn rel(&'a self, x: f64, y: f64) -> ImageRectangleColorContext<'a> {
        ImageRectangleColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            image: Borrowed(self.image.get()),
            rect: Value(relative_rectangle(self.rect.get(), x, y)),
            color: Borrowed(self.color.get()),
        }
    }
}

impl<'a> View<'a> for ImageRectangleColorContext<'a> {
    #[inline(always)]
    fn view(&'a self) -> ImageRectangleColorContext<'a> {
        ImageRectangleColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.base.get()),
            rect: Borrowed(self.rect.get()),
            image: Borrowed(self.image.get()),
            color: Borrowed(self.color.get()),
        }
    }

    #[inline(always)]
    fn reset(&'a self) -> ImageRectangleColorContext<'a> {
        ImageRectangleColorContext {
            base: Borrowed(self.base.get()),
            transform: Value(identity()),
            rect: Borrowed(self.rect.get()),
            image: Borrowed(self.image.get()),
            color: Borrowed(self.color.get()),
        }
    }

    #[inline(always)]
    fn store_view(&'a self) -> ImageRectangleColorContext<'a> {
        ImageRectangleColorContext {
            base: Borrowed(self.transform.get()),
            transform: Borrowed(self.transform.get()),
            rect: Borrowed(self.rect.get()),
            image: Borrowed(self.image.get()),
            color: Borrowed(self.color.get()),
        }
    }
}

