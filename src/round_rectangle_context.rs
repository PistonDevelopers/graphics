use {
    AddColor, 
    Borrowed, 
    Field, 
    Matrix2d, 
    RelativeRectangle,
    RoundRectangle,
    RoundRectangleColorContext,
    Transform2d,
    Value, 
    View,
};
use vecmath::{
    identity,
    margin_round_rectangle, 
    multiply,
    relative_round_rectangle, 
    rotate_radians, 
    scale, 
    shear, 
    translate, 
};

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
    fn rot_rad_local(&'a self, angle: f64) -> RoundRectangleContext<'a> {
        RoundRectangleContext {
            base: Borrowed(self.base.get()),
            transform: {
                let rot = rotate_radians(angle);
                Value(multiply(self.transform.get(), &rot))
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
    fn scale_local(&'a self, sx: f64, sy: f64) -> RoundRectangleContext<'a> {
        RoundRectangleContext {
            base: Borrowed(self.base.get()),
            transform: {
                let scale = scale(sx, sy);
                Value(multiply(self.transform.get(), &scale))
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
    
    #[inline(always)]
    fn shear_local(&'a self, sx: f64, sy: f64) -> RoundRectangleContext<'a> {
        RoundRectangleContext {
            base: Borrowed(self.base.get()),
            transform: {
                let shear = shear(sx, sy);
                Value(multiply(self.transform.get(), &shear))
            },
            round_rect: Borrowed(self.round_rect.get()),
        }
    }
}

impl<'a> RelativeRectangle<'a> for RoundRectangleContext<'a> {
    #[inline(always)]
    fn margin(&'a self, m: f64) -> RoundRectangleContext<'a> {
        RoundRectangleContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            round_rect: Value(margin_round_rectangle(self.round_rect.get(), m)),
        }
    }

    #[inline(always)]
    fn rel(&'a self, x: f64, y: f64) -> RoundRectangleContext<'a> {
        RoundRectangleContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            round_rect: Value(relative_round_rectangle(self.round_rect.get(), x, y)),
        }
    }
}

impl<'a> AddColor<'a, RoundRectangleColorContext<'a>> for RoundRectangleContext<'a> {
    /// Creates a RectangleColorContext.
    #[inline(always)]
    fn rgba(&'a self, r: f32, g: f32, b: f32, a: f32) -> RoundRectangleColorContext<'a> {
        RoundRectangleColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            color: Value([r, g, b, a]),
            round_rect: Borrowed(self.round_rect.get()),
        }
    }
}

impl<'a> View<'a> for RoundRectangleContext<'a> {
    #[inline(always)]
    fn view(&'a self) -> RoundRectangleContext<'a> {
        RoundRectangleContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.base.get()),
            round_rect: Borrowed(self.round_rect.get()),
        }
    }

    #[inline(always)]
    fn reset(&'a self) -> RoundRectangleContext<'a> {
        RoundRectangleContext {
            base: Borrowed(self.base.get()),
            transform: Value(identity()),
            round_rect: Borrowed(self.round_rect.get()),
        }
    }

    #[inline(always)]
    fn store_view(&'a self) -> RoundRectangleContext<'a> {
        RoundRectangleContext {
            base: Borrowed(self.transform.get()),
            transform: Borrowed(self.transform.get()),
            round_rect: Borrowed(self.round_rect.get()),
        }
    }
}

