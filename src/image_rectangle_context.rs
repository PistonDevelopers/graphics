use {
    BackEnd,
    Borrowed,
    Draw,
    Field,
    Image,
    Matrix2d,
    Rectangle,
    RelativeRectangle,
    Value,
    Transform2d,
};
use vecmath::{
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
pub struct ImageRectangleContext<'a> {
    /// Base/original transformation.
    pub base: Field<'a, Matrix2d>,
    /// Current transformation.
    pub transform: Field<'a, Matrix2d>,
    /// Current rectangle.
    pub rect: Field<'a, Rectangle>,
    /// Current color.
    pub image: Field<'a, Image>,
}

impl<'a> Transform2d<'a> for ImageRectangleContext<'a> {
    #[inline(always)]
    fn trans(&'a self, x: f64, y: f64) -> ImageRectangleContext<'a> {
        ImageRectangleContext {
            base: Borrowed(self.base.get()),
            transform: {
                let trans = translate(x, y);
                Value(multiply(&trans, self.transform.get()))
            },
            rect: Borrowed(self.rect.get()),
            image: Borrowed(self.image.get()),
        }
    }
    
    #[inline(always)]
    fn trans_local(&'a self, x: f64, y: f64) -> ImageRectangleContext<'a> {
        ImageRectangleContext {
            base: Borrowed(self.base.get()),
            transform: {
                let trans = translate(x, y);
                Value(multiply(self.transform.get(), &trans))
            },
            rect: Borrowed(self.rect.get()),
            image: Borrowed(self.image.get()),
        }
    }

    #[inline(always)]
    fn rot_rad(&'a self, angle: f64) -> ImageRectangleContext<'a> {
        ImageRectangleContext {
            base: Borrowed(self.base.get()),
            transform: {
                let rot = rotate_radians(angle);
                Value(multiply(&rot, self.transform.get()))
            },
            rect: Borrowed(self.rect.get()),
            image: Borrowed(self.image.get()),
        }
    }

    #[inline(always)]
    fn rot_rad_local(&'a self, angle: f64) -> ImageRectangleContext<'a> {
        ImageRectangleContext {
            base: Borrowed(self.base.get()),
            transform: {
                let rot = rotate_radians(angle);
                Value(multiply(self.transform.get(), &rot))
            },
            rect: Borrowed(self.rect.get()),
            image: Borrowed(self.image.get()),
        }
    }

    #[inline(always)]
    fn scale(&'a self, sx: f64, sy: f64) -> ImageRectangleContext<'a> {
        ImageRectangleContext {
            base: Borrowed(self.base.get()),
            transform: {
                let scale = scale(sx, sy);
                Value(multiply(&scale, self.transform.get()))
            },
            rect: Borrowed(self.rect.get()),
            image: Borrowed(self.image.get()),
        }
    }

    #[inline(always)]
    fn scale_local(&'a self, sx: f64, sy: f64) -> ImageRectangleContext<'a> {
        ImageRectangleContext {
            base: Borrowed(self.base.get()),
            transform: {
                let scale = scale(sx, sy);
                Value(multiply(self.transform.get(), &scale))
            },
            rect: Borrowed(self.rect.get()),
            image: Borrowed(self.image.get()),
        }
    }

    #[inline(always)]
    fn shear(&'a self, sx: f64, sy: f64) -> ImageRectangleContext<'a> {
        ImageRectangleContext {
            base: Borrowed(self.base.get()),
            transform: {
                let shear = shear(sx, sy);
                Value(multiply(&shear, self.transform.get()))
            },
            rect: Borrowed(self.rect.get()),
            image: Borrowed(self.image.get()),
        }
    }
    
    #[inline(always)]
    fn shear_local(&'a self, sx: f64, sy: f64) -> ImageRectangleContext<'a> {
        ImageRectangleContext {
            base: Borrowed(self.base.get()),
            transform: {
                let shear = shear(sx, sy);
                Value(multiply(self.transform.get(), &shear))
            },
            rect: Borrowed(self.rect.get()),
            image: Borrowed(self.image.get()),
        }
    }
}

impl<'a> Draw<'a> for ImageRectangleContext<'a> {
    fn draw<B: BackEnd>(&'a self, back_end: &mut B) {
        if back_end.supports_tri_list_xy_f32_rgba_f32_uv_f32() {
            let rect = self.rect.get();
            let color: [f32, ..4] = [1.0, 1.0, 1.0, 1.0];
            let texture_id = self.image.get().texture_id;
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely opaque or if the texture has alpha channel.
            let needs_alpha = color[3] != 1.0 || back_end.has_texture_alpha(texture_id);
            if needs_alpha { back_end.enable_alpha_blend(); }
            back_end.tri_list_xy_f32_rgba_f32_uv_f32(
                rect_tri_list_xy_f32(self.transform.get(), rect),
                rect_tri_list_rgba_f32(color),
                rect_tri_list_uv_f32(self.image.get())
            );
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
