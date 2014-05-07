use {
    AddColor,
    Borrowed, 
    EllipseColorContext,
    Field, 
    Matrix2d, 
    Rectangle,
    RelativeRectangle, 
    Value,
    View,
    Transform2d,
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

/// An ellipse context.
pub struct EllipseContext<'a> {
    /// Base/origin transform.
    pub base: Field<'a, Matrix2d>,
    /// Current transformation.
    pub transform: Field<'a, Matrix2d>,
    /// Current rectangle enclosing the ellipse.
    pub rect: Field<'a, Rectangle>,
}

impl<'a> Transform2d<'a> for EllipseContext<'a> {
    #[inline(always)]
    fn trans(&'a self, x: f64, y: f64) -> EllipseContext<'a> {
        EllipseContext {
            base: Borrowed(self.base.get()),
            transform: {
                let trans = translate(x, y);
                Value(multiply(&trans, self.transform.get()))
            },
            rect: Borrowed(self.rect.get()),
        }
    }

    #[inline(always)]
    fn trans_local(&'a self, x: f64, y: f64) -> EllipseContext<'a> {
        EllipseContext {
            base: Borrowed(self.base.get()),
            transform: {
                let trans = translate(x, y);
                Value(multiply(self.transform.get(), &trans))
            },
            rect: Borrowed(self.rect.get()),
        }
    }

    #[inline(always)]
    fn rot_rad(&'a self, angle: f64) -> EllipseContext<'a> {
        EllipseContext {
            base: Borrowed(self.base.get()),
            transform: {
                let rot = rotate_radians(angle);
                Value(multiply(&rot, self.transform.get()))
            },
            rect: Borrowed(self.rect.get()),
        }
    }

    #[inline(always)]
    fn rot_rad_local(&'a self, angle: f64) -> EllipseContext<'a> {
        EllipseContext {
            base: Borrowed(self.base.get()),
            transform: {
                let rot = rotate_radians(angle);
                Value(multiply(self.transform.get(), &rot))
            },
            rect: Borrowed(self.rect.get()),
        }
    }

    #[inline(always)]
    fn scale(&'a self, sx: f64, sy: f64) -> EllipseContext<'a> {
        EllipseContext {
            base: Borrowed(self.base.get()),
            transform: {
                let scale = scale(sx, sy);
                Value(multiply(&scale, self.transform.get()))
            },
            rect: Borrowed(self.rect.get()),
        }
    }

    #[inline(always)]
    fn scale_local(&'a self, sx: f64, sy: f64) -> EllipseContext<'a> {
        EllipseContext {
            base: Borrowed(self.base.get()),
            transform: {
                let scale = scale(sx, sy);
                Value(multiply(self.transform.get(), &scale))
            },
            rect: Borrowed(self.rect.get()),
        }
    }

    #[inline(always)]
    fn shear(&'a self, sx: f64, sy: f64) -> EllipseContext<'a> {
        EllipseContext {
            base: Borrowed(self.base.get()),
            transform: {
                let shear = shear(sx, sy);
                Value(multiply(&shear, self.transform.get()))
            },
            rect: Borrowed(self.rect.get()),
        }
    }
    
    #[inline(always)]
    fn shear_local(&'a self, sx: f64, sy: f64) -> EllipseContext<'a> {
        EllipseContext {
            base: Borrowed(self.base.get()),
            transform: {
                let shear = shear(sx, sy);
                Value(multiply(self.transform.get(), &shear))
            },
            rect: Borrowed(self.rect.get()),
        }
    }
}

impl<'a> AddColor<'a, EllipseColorContext<'a>> for EllipseContext<'a> {
    #[inline(always)]
    fn rgba(&'a self, r: f32, g: f32, b: f32, a: f32) -> EllipseColorContext<'a> {
        EllipseColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            rect: Borrowed(self.rect.get()),
            color: Value([r, g, b, a]),
        }
    }
}

impl<'a> RelativeRectangle<'a> for EllipseContext<'a> {
    #[inline(always)]
    fn margin(&'a self, m: f64) -> EllipseContext<'a> {
        EllipseContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            rect: Value(margin_rectangle(self.rect.get(), m)),
        }
    }

    #[inline(always)]
    fn rel(&'a self, x: f64, y: f64) -> EllipseContext<'a> {
        EllipseContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            rect: Value(relative_rectangle(self.rect.get(), x, y)),
        }
    }
}

impl<'a> View<'a> for EllipseContext<'a> {
    #[inline(always)]
    fn view(&'a self) -> EllipseContext<'a> {
        EllipseContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.base.get()),
            rect: Borrowed(self.rect.get()),
        }
    }

    #[inline(always)]
    fn reset(&'a self) -> EllipseContext<'a> {
        EllipseContext {
            base: Borrowed(self.base.get()),
            transform: Value(identity()),
            rect: Borrowed(self.rect.get()),
        }
    }

    #[inline(always)]
    fn store_view(&'a self) -> EllipseContext<'a> {
        EllipseContext {
            base: Borrowed(self.transform.get()),
            transform: Borrowed(self.transform.get()),
            rect: Borrowed(self.rect.get()),
        }
    }
}


