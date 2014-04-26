
use {Field, Borrowed, Value};
use vecmath::{rotate_radians, multiply, translate, scale, shear};
use {Clear, Fill, BackEnd, Transform2d, Matrix2d, Rectangle, Color};
use triangulation::{with_ellipse_tri_list_xy_rgba_f32};

/// An ellipse color context.
pub struct EllipseColorContext<'a> {
    /// Base/original transformation.
    pub base: Field<'a, Matrix2d>,
    /// Current transformation.
    pub transform: Field<'a, Matrix2d>,
    /// Current rectangle.
    pub rect: Field<'a, Rectangle>,
    /// Current color.
    pub color: Field<'a, Color>,
}

impl<'a> Transform2d<'a> for EllipseColorContext<'a> {
    #[inline(always)]
    fn trans(&'a self, x: f64, y: f64) -> EllipseColorContext<'a> {
        EllipseColorContext {
            base: Borrowed(self.base.get()),
            transform: {
                let trans = translate(x, y);
                Value(multiply(&trans, self.transform.get()))
            },
            rect: Borrowed(self.rect.get()),
            color: Borrowed(self.color.get()),
        }
    }

    #[inline(always)]
    fn trans_local(&'a self, x: f64, y: f64) -> EllipseColorContext<'a> {
        EllipseColorContext {
            base: Borrowed(self.base.get()),
            transform: {
                let trans = translate(x, y);
                Value(multiply(self.transform.get(), &trans))
            },
            rect: Borrowed(self.rect.get()),
            color: Borrowed(self.color.get()),
        }
    }
    
    #[inline(always)]
    fn rot_rad(&'a self, angle: f64) -> EllipseColorContext<'a> {
        EllipseColorContext {
            base: Borrowed(self.base.get()),
            transform: {
                let rot = rotate_radians(angle);
                Value(multiply(&rot, self.transform.get()))
            },
            rect: Borrowed(self.rect.get()),
            color: Borrowed(self.color.get()),
        }
    }

    #[inline(always)]
    fn rot_rad_local(&'a self, angle: f64) -> EllipseColorContext<'a> {
        EllipseColorContext {
            base: Borrowed(self.base.get()),
            transform: {
                let rot = rotate_radians(angle);
                Value(multiply(self.transform.get(), &rot))
            },
            rect: Borrowed(self.rect.get()),
            color: Borrowed(self.color.get()),
        }
    }

    #[inline(always)]
    fn scale(&'a self, sx: f64, sy: f64) -> EllipseColorContext<'a> {
        EllipseColorContext {
            base: Borrowed(self.base.get()),
            transform: {
                let scale = scale(sx, sy);
                Value(multiply(&scale, self.transform.get()))
            },
            rect: Borrowed(self.rect.get()),
            color: Borrowed(self.color.get()),
        }
    }

    #[inline(always)]
    fn scale_local(&'a self, sx: f64, sy: f64) -> EllipseColorContext<'a> {
        EllipseColorContext {
            base: Borrowed(self.base.get()),
            transform: {
                let scale = scale(sx, sy);
                Value(multiply(self.transform.get(), &scale))
            },
            rect: Borrowed(self.rect.get()),
            color: Borrowed(self.color.get()),
        }
    }

    #[inline(always)]
    fn shear(&'a self, sx: f64, sy: f64) -> EllipseColorContext<'a> {
        EllipseColorContext {
            base: Borrowed(self.base.get()),
            transform: {
                let shear = shear(sx, sy);
                Value(multiply(&shear, self.transform.get()))
            },
            rect: Borrowed(self.rect.get()),
            color: Borrowed(self.color.get()),
        }
    }
    
    #[inline(always)]
    fn shear_local(&'a self, sx: f64, sy: f64) -> EllipseColorContext<'a> {
        EllipseColorContext {
            base: Borrowed(self.base.get()),
            transform: {
                let shear = shear(sx, sy);
                Value(multiply(self.transform.get(), &shear))
            },
            rect: Borrowed(self.rect.get()),
            color: Borrowed(self.color.get()),
        }
    }
}

impl<'a> Fill<'a> for EllipseColorContext<'a> {
    fn fill<B: BackEnd>(&'a self, back_end: &mut B) {
        if back_end.supports_tri_list_xy_rgba_f32() {
            let rect = self.rect.get();
            let color = self.color.get();
            let color: [f32, ..4] = [color[0], color[1], color[2], color[3]];
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely opaque.
            let needs_alpha = color[3] != 1.0;
            if needs_alpha { back_end.enable_alpha_blend(); }
            with_ellipse_tri_list_xy_rgba_f32(
                128,
                self.transform.get(),
                rect,
                color,
                |vertices, colors| {
                    back_end.tri_list_xy_rgba_f32(vertices, colors)
                }
            );
            if needs_alpha { back_end.disable_alpha_blend(); }
        } else {
            unimplemented!();
        }
    }
}

impl<'a> Clear for EllipseColorContext<'a> {
    fn clear<B: BackEnd>(&self, back_end: &mut B) {
        if back_end.supports_clear_rgba() {
            let color = self.color.get();
            back_end.clear_rgba(color[0], color[1], color[2], color[3]);
        } else {
            unimplemented!();
        }
    }
}

