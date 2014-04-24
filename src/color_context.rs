
use {Field, Borrowed, Value};
use vecmath::{rotate_radians, multiply, translate, scale, shear};
use {Transform2d, Matrix2d, Color};
use {RectangleColorContext};

/// A context with color information.
pub struct ColorContext<'a> {
    /// Base/original transformation.
    pub base: Field<'a, Matrix2d>,
    /// Current transformation.
    pub transform: Field<'a, Matrix2d>,
    /// Current color.
    pub color: Field<'a, Color>,
}

impl<'a> Transform2d<'a> for ColorContext<'a> {
    #[inline(always)]
    fn trans(&'a self, x: f64, y: f64) -> ColorContext<'a> {
        ColorContext {
            base: Borrowed(self.base.get()),
            transform: {
                let trans = translate(x, y);
                Value(multiply(&trans, self.transform.get()))
            },
            color: Borrowed(self.color.get()),
        }
    }

    #[inline(always)]
    fn rot_rad(&'a self, angle: f64) -> ColorContext<'a> {
        ColorContext {
            base: Borrowed(self.base.get()),
            transform: {
                let rot = rotate_radians(angle);
                Value(multiply(&rot, self.transform.get()))
            },
            color: Borrowed(self.color.get()),
        }
    }

    #[inline(always)]
    fn scale(&'a self, sx: f64, sy: f64) -> ColorContext<'a> {
        ColorContext {
            base: Borrowed(self.base.get()),
            transform: {
                let scale = scale(sx, sy);
                Value(multiply(&scale, self.transform.get()))
            },
            color: Borrowed(self.color.get()),
        }
    }

    #[inline(always)]
    fn shear(&'a self, sx: f64, sy: f64) -> ColorContext<'a> {
        ColorContext {
            base: Borrowed(self.base.get()),
            transform: {
                let shear = shear(sx, sy);
                Value(multiply(&shear, self.transform.get()))
            },
            color: Borrowed(self.color.get()),
        }
    }
}

impl<'a> ColorContext<'a> {
    /// Creates a RectangleColorContext.
    #[inline(always)]
    pub fn rect(&'a self, x: f64, y: f64, w: f64, h: f64) -> RectangleColorContext<'a> {
        RectangleColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            color: Borrowed(self.color.get()),
            rect: Value([x, y, w, h]),
        }
    }
}

#[test]
fn test_rect() {
    use {Context, AddColor};

    let c = Context::new();
    let color = c.rgba(1.0, 0.0, 0.0, 1.0);
    let rect_color = color.rect(0.0, 0.0, 100.0, 100.0);
    assert_eq!(rect_color.rect.get()[2], 100.0);
}

