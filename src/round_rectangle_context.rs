use {Field, Borrowed, Value, Matrix2d, RoundRectangle};
use vecmath::{translate, rotate_radians, scale, shear, multiply};
use {Transform2d};

/// A round rectangle context.
pub struct RoundRectangleContext<'a> {
    /// Base/origin transform.
    pub base: Field<'a, Matrix2d>,
    /// Current transform.
    pub transform: Field<'a, Matrix2d>,
    /// Current round rectangle.
    pub round_rect: Field<'a, RoundRectangle>,
}

impl<'a> Transform2d<'a> for RoundRectangleContext<'a> {
    #[inline(always)]
    fn trans(&'a self, x: f64, y: f64) -> RoundRectangleContext<'a> {
        RoundRectangleContext {
            base: Borrowed(self.base.get()),
            transform: {
                let trans = translate(x, y);
                Value(multiply(&trans, self.transform.get()))
            },
            round_rect: Borrowed(self.round_rect.get()),
        }
    }
    
    #[inline(always)]
    fn trans_local(&'a self, x: f64, y: f64) -> RoundRectangleContext<'a> {
        RoundRectangleContext {
            base: Borrowed(self.base.get()),
            transform: {
                let trans = translate(x, y);
                Value(multiply(self.transform.get(), &trans))
            },
            round_rect: Borrowed(self.round_rect.get()),
        }
    }

    #[inline(always)]
    fn rot_rad(&'a self, angle: f64) -> RoundRectangleContext<'a> {
        RoundRectangleContext {
            base: Borrowed(self.base.get()),
            transform: {
                let rot = rotate_radians(angle);
                Value(multiply(&rot, self.transform.get()))
            },
            round_rect: Borrowed(self.round_rect.get()),
        }
    }

    #[inline(always)]
    fn scale(&'a self, sx: f64, sy: f64) -> RoundRectangleContext<'a> {
        RoundRectangleContext {
            base: Borrowed(self.base.get()),
            transform: {
                let scale = scale(sx, sy);
                Value(multiply(&scale, self.transform.get()))
            },
            round_rect: Borrowed(self.round_rect.get()),
        }
    }

    #[inline(always)]
    fn shear(&'a self, sx: f64, sy: f64) -> RoundRectangleContext<'a> {
        RoundRectangleContext {
            base: Borrowed(self.base.get()),
            transform: {
                let shear = shear(sx, sy);
                Value(multiply(&shear, self.transform.get()))
            },
            round_rect: Borrowed(self.round_rect.get()),
        }
    }
}
