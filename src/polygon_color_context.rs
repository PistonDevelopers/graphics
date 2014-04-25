use {Field, Value, Borrowed, Matrix2d, Color};
use vecmath::{multiply, translate, rotate_radians, scale, shear};
use {Transform2d};

/// A polygon color context.
pub struct PolygonColorContext<'a> {
    /// Base/origin transform.
    base: Field<'a, Matrix2d>,
    /// Current transform.
    transform: Field<'a, Matrix2d>,
    /// Current color.
    color: Field<'a, Color>,
    /// Current polygon.
    polygon: Field<'a, &'a [f64]>,
}

impl<'a> Transform2d<'a> for PolygonColorContext<'a> {
    #[inline(always)]
    fn trans(&'a self, x: f64, y: f64) -> PolygonColorContext<'a> {
        PolygonColorContext {
            base: Borrowed(self.base.get()),
            transform: {
                let trans = translate(x, y);
                Value(multiply(&trans, self.transform.get()))
            },
            polygon: Borrowed(self.polygon.get()),
            color: Borrowed(self.color.get()),
        }
    }

    #[inline(always)]
    fn rot_rad(&'a self, angle: f64) -> PolygonColorContext<'a> {
        PolygonColorContext {
            base: Borrowed(self.base.get()),
            transform: {
                let rot = rotate_radians(angle);
                Value(multiply(&rot, self.transform.get()))
            },
            polygon: Borrowed(self.polygon.get()),
            color: Borrowed(self.color.get()),
        }
    }

    #[inline(always)]
    fn scale(&'a self, sx: f64, sy: f64) -> PolygonColorContext<'a> {
        PolygonColorContext {
            base: Borrowed(self.base.get()),
            transform: {
                let scale = scale(sx, sy);
                Value(multiply(&scale, self.transform.get()))
            },
            polygon: Borrowed(self.polygon.get()),
            color: Borrowed(self.color.get()),
        }
    }

    #[inline(always)]
    fn shear(&'a self, sx: f64, sy: f64) -> PolygonColorContext<'a> {
        PolygonColorContext {
            base: Borrowed(self.base.get()),
            transform: {
                let shear = shear(sx, sy);
                Value(multiply(&shear, self.transform.get()))
            },
            polygon: Borrowed(self.polygon.get()),
            color: Borrowed(self.color.get()),
        }
    }
}
