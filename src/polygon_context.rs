use {Matrix2d, Field, Value, Borrowed};
use vecmath::{multiply, translate, rotate_radians, scale, shear};
use {Transform2d};

/// A polygon context.
pub struct PolygonContext<'a> {
    /// Base/origin transform.
    pub base: Field<'a, Matrix2d>,
    /// Current transform.
    pub transform: Field<'a, Matrix2d>,
    /// Current polygon.
    pub polygon: Field<'a, &'a [f64]>
}

impl<'a> Transform2d<'a> for PolygonContext<'a> {
    #[inline(always)]
    fn trans(&'a self, x: f64, y: f64) -> PolygonContext<'a> {
        PolygonContext {
            base: Borrowed(self.base.get()),
            transform: {
                let trans = translate(x, y);
                Value(multiply(&trans, self.transform.get()))
            },
            polygon: Borrowed(self.polygon.get()),
        }
    }

    #[inline(always)]
    fn rot_rad(&'a self, angle: f64) -> PolygonContext<'a> {
        PolygonContext {
            base: Borrowed(self.base.get()),
            transform: {
                let rot = rotate_radians(angle);
                Value(multiply(&rot, self.transform.get()))
            },
            polygon: Borrowed(self.polygon.get()),
        }
    }

    #[inline(always)]
    fn scale(&'a self, sx: f64, sy: f64) -> PolygonContext<'a> {
        PolygonContext {
            base: Borrowed(self.base.get()),
            transform: {
                let scale = scale(sx, sy);
                Value(multiply(&scale, self.transform.get()))
            },
            polygon: Borrowed(self.polygon.get()),
        }
    }

    #[inline(always)]
    fn shear(&'a self, sx: f64, sy: f64) -> PolygonContext<'a> {
        PolygonContext {
            base: Borrowed(self.base.get()),
            transform: {
                let shear = shear(sx, sy);
                Value(multiply(&shear, self.transform.get()))
            },
            polygon: Borrowed(self.polygon.get()),
        }
    }
}
