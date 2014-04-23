#![crate_id = "graphics"]
#![deny(missing_doc)]

//! Attempt of creating a cheap drawing context.

pub use context::Context;
pub use backend::BackEnd;
pub use transform2d::Transform2d;

use vecmath::{rotate_radians, multiply, translate, scale, shear};

mod context;
mod backend;
mod transform2d;
pub mod vecmath;

pub type Matrix2d = [f64, ..6];
pub type Color = [f64, ..4];
pub type Rectangle = [f64, ..4];

/// A structure that might contain a value or a borrowed value.
/// This is to used as building block to create data structure
/// that is partially based on an existing structure.
pub enum Field<'a, T> {
    /// Contains a value.
    Value(T),
    /// Contains a borrowed pointer.
    Borrowed(&'a T),
}

impl<'a, T> Field<'a, T> {
    /// Gets a read only value.
    #[inline(always)]
    pub fn get(&'a self) -> &'a T {
        match *self {
            Value(ref val) => val,
            Borrowed(rval) => rval,
        }
    }
}

/// A context with color information.
pub struct ColorContext<'a> {
    base: Field<'a, Matrix2d>,
    transform: Field<'a, Matrix2d>,
    color: Field<'a, Color>,
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

/// A rectangle color context.
pub struct RectangleColorContext<'a> {
    base: Field<'a, Matrix2d>,
    transform: Field<'a, Matrix2d>,
    rect: Field<'a, Rectangle>,
    color: Field<'a, Color>,
}

impl<'a> Transform2d<'a> for RectangleColorContext<'a> {
    #[inline(always)]
    fn trans(&'a self, x: f64, y: f64) -> RectangleColorContext<'a> {
        RectangleColorContext {
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
    fn rot_rad(&'a self, angle: f64) -> RectangleColorContext<'a> {
        RectangleColorContext {
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
    fn scale(&'a self, sx: f64, sy: f64) -> RectangleColorContext<'a> {
        RectangleColorContext {
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
    fn shear(&'a self, sx: f64, sy: f64) -> RectangleColorContext<'a> {
        RectangleColorContext {
            base: Borrowed(self.base.get()),
            transform: {
                let shear = shear(sx, sy);
                Value(multiply(&shear, self.transform.get()))
            },
            rect: Borrowed(self.rect.get()),
            color: Borrowed(self.color.get()),
        }
    }
}

