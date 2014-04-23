
use {Field, Borrowed, Value};
use vecmath::{rotate_radians, multiply, translate, scale, shear};
use {Transform2d, Matrix2d, Rectangle};

/// A rectangle context.
pub struct RectangleContext<'a> {
    base: Field<'a, Matrix2d>,
    transform: Field<'a, Matrix2d>,
    rect: Field<'a, Rectangle>,
}

impl<'a> Transform2d<'a> for RectangleContext<'a> {
    #[inline(always)]
    fn trans(&'a self, x: f64, y: f64) -> RectangleContext<'a> {
        RectangleContext {
            base: Borrowed(self.base.get()),
            transform: {
                let trans = translate(x, y);
                Value(multiply(&trans, self.transform.get()))
            },
            rect: Borrowed(self.rect.get()),
        }
    }

    #[inline(always)]
    fn rot_rad(&'a self, angle: f64) -> RectangleContext<'a> {
        RectangleContext {
            base: Borrowed(self.base.get()),
            transform: {
                let rot = rotate_radians(angle);
                Value(multiply(&rot, self.transform.get()))
            },
            rect: Borrowed(self.rect.get()),
        }
    }

    #[inline(always)]
    fn scale(&'a self, sx: f64, sy: f64) -> RectangleContext<'a> {
        RectangleContext {
            base: Borrowed(self.base.get()),
            transform: {
                let scale = scale(sx, sy);
                Value(multiply(&scale, self.transform.get()))
            },
            rect: Borrowed(self.rect.get()),
        }
    }

    #[inline(always)]
    fn shear(&'a self, sx: f64, sy: f64) -> RectangleContext<'a> {
        RectangleContext {
            base: Borrowed(self.base.get()),
            transform: {
                let shear = shear(sx, sy);
                Value(multiply(&shear, self.transform.get()))
            },
            rect: Borrowed(self.rect.get()),
        }
    }
}

