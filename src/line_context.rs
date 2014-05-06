
use {Field, Value, Borrowed, Matrix2d, Line};
use {Transform2d};
use vecmath::{translate, rotate_radians, scale, shear, multiply, identity};
use {View};

/// A line context.
pub struct LineContext<'a> {
    /// Base/original transform.
    pub base: Field<'a, Matrix2d>,
    /// Current transform.
    pub transform: Field<'a, Matrix2d>,
    /// Current line.
    pub line: Field<'a, Line>,
}

impl<'a> Transform2d<'a> for LineContext<'a> {
    #[inline(always)]
    fn trans(&'a self, x: f64, y: f64) -> LineContext<'a> {
        LineContext {
            base: Borrowed(self.base.get()),
            transform: {
                let trans = translate(x, y);
                Value(multiply(&trans, self.transform.get()))
            },
            line: Borrowed(self.line.get()),
        }
    }

    #[inline(always)]
    fn trans_local(&'a self, x: f64, y: f64) -> LineContext<'a> {
        LineContext {
            base: Borrowed(self.base.get()),
            transform: {
                let trans = translate(x, y);
                Value(multiply(self.transform.get(), &trans))
            },
            line: Borrowed(self.line.get()),
        }
    }

    #[inline(always)]
    fn rot_rad(&'a self, angle: f64) -> LineContext<'a> {
        LineContext {
            base: Borrowed(self.base.get()),
            transform: {
                let rot = rotate_radians(angle);
                Value(multiply(&rot, self.transform.get()))
            },
            line: Borrowed(self.line.get()),
        }
    }

    #[inline(always)]
    fn rot_rad_local(&'a self, angle: f64) -> LineContext<'a> {
        LineContext {
            base: Borrowed(self.base.get()),
            transform: {
                let rot = rotate_radians(angle);
                Value(multiply(self.transform.get(), &rot))
            },
            line: Borrowed(self.line.get()),
        }
    }

    #[inline(always)]
    fn scale(&'a self, sx: f64, sy: f64) -> LineContext<'a> {
        LineContext {
            base: Borrowed(self.base.get()),
            transform: {
                let scale = scale(sx, sy);
                Value(multiply(&scale, self.transform.get()))
            },
            line: Borrowed(self.line.get()),
        }
    }

    #[inline(always)]
    fn scale_local(&'a self, sx: f64, sy: f64) -> LineContext<'a> {
        LineContext {
            base: Borrowed(self.base.get()),
            transform: {
                let scale = scale(sx, sy);
                Value(multiply(self.transform.get(), &scale))
            },
            line: Borrowed(self.line.get()),
        }
    }

    #[inline(alwyas)]
    fn shear(&'a self, sx: f64, sy: f64) -> LineContext<'a> {
        LineContext {
            base: Borrowed(self.base.get()),
            transform: {
                let shear = shear(sx, sy);
                Value(multiply(&shear, self.transform.get()))
            },
            line: Borrowed(self.line.get()),
        }
    } 
    
    #[inline(alwyas)]
    fn shear_local(&'a self, sx: f64, sy: f64) -> LineContext<'a> {
        LineContext {
            base: Borrowed(self.base.get()),
            transform: {
                let shear = shear(sx, sy);
                Value(multiply(self.transform.get(), &shear))
            },
            line: Borrowed(self.line.get()),
        }
    } 
}

impl<'a> View<'a> for LineContext<'a> {
    #[inline(always)]
    fn view(&'a self) -> LineContext<'a> {
        LineContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.base.get()),
            line: Borrowed(self.line.get()),
        }
    }

    #[inline(always)]
    fn reset(&'a self) -> LineContext<'a> {
        LineContext {
            base: Borrowed(self.base.get()),
            transform: Value(identity()),
            line: Borrowed(self.line.get()),
        }
    }

    #[inline(always)]
    fn store_view(&'a self) -> LineContext<'a> {
        LineContext {
            base: Borrowed(self.transform.get()),
            transform: Borrowed(self.transform.get()),
            line: Borrowed(self.line.get()),
        }
    }
}


