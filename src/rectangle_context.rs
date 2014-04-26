
use {Field, Borrowed, Value};
use vecmath::{rotate_radians, multiply, translate, scale, shear};
use {Transform2d, Matrix2d, Rectangle};
use {RectangleColorContext};
use {AddColor};

/// A rectangle context.
pub struct RectangleContext<'a> {
    /// Base/original transformation.
    pub base: Field<'a, Matrix2d>,
    /// Current transformation.
    pub transform: Field<'a, Matrix2d>,
    /// Current rectangle.
    pub rect: Field<'a, Rectangle>,
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
    fn trans_local(&'a self, x: f64, y: f64) -> RectangleContext<'a> {
        RectangleContext {
            base: Borrowed(self.base.get()),
            transform: {
                let trans = translate(x, y);
                Value(multiply(self.transform.get(), &trans))
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

impl<'a> AddColor<'a, RectangleColorContext<'a>> for RectangleContext<'a> {
    /// Creates a RectangleColorContext.
    #[inline(always)]
    fn rgba(&'a self, r: f32, g: f32, b: f32, a: f32) -> RectangleColorContext<'a> {
        RectangleColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            color: Value([r, g, b, a]),
            rect: Borrowed(self.rect.get()),
        }
    }
}

#[test]
fn test_rgba() {
    use {Context, AddRectangle};

    let c = Context::new();
    let d = c.rect(0.0, 0.0, 100.0, 100.0);
    let e = d.rgba(1.0, 0.0, 0.0, 1.0);
    assert_eq!(e.color.get()[0], 1.0);
}

