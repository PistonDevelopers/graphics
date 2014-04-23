#![crate_id = "graphics"]
#![deny(missing_doc)]

//! Attempt of creating a cheap drawing context.

use std::num::Float;
pub use context::Context;

mod context;

pub type Matrix2d = [f64, ..6];
pub type Color = [f64, ..4];
pub type Rectangle = [f64, ..4];

/// Implemented by all graphics back-ends.
/// This trait uses default methods to simplify implementation.
pub trait BackEnd {
    /// Returns true if feature is supported.
    #[inline(always)]
    fn supports_tri_list_xy_rgba_f64(&self) -> bool { false }

    /// Renders list of 2d triangles with color assigned per vertex.
    fn tri_list_xy_rgba_f64(&mut self, _vertices: &[f64], _colors: &[f64]) {}

    /// Returns true if feature is supported.
    #[inline(always)]
    fn supports_tri_list_xy_rgba_f32(&self) -> bool { false }

    /// Renders list of 2d triangles with color assigned per vertex.
    fn tri_list_xy_rgba_f32(&mut self, _vertices: &[f32], _colors: &[f32]) {}
}

/// Implemented by contexts that can transform.
pub trait Transform2d<'a> {
    /// Translate x and y.
    fn trans(&'a self, x: f64, y: f64) -> Self;
   
    /// Rotates degrees.
    #[inline(always)]
    fn rot_deg(&'a self, angle: f64) -> Self {
        let pi: f64 = Float::pi();
        self.rot_rad(angle * pi / 180.0)
    }
    
    /// Rotate radians.
    fn rot_rad(&'a self, angle: f64) -> Self;
    /// Scale.
    fn scale(&'a self, sx: f64, sy: f64) -> Self;
    /// Shear.
    fn shear(&'a self, sx: f64, sy: f64) -> Self;
}

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

/// Multiplies two matrices.
#[inline(always)]
pub fn multiply(m: &[f64, ..6], b: &[f64, ..6]) -> Matrix2d {
    [m[0]*b[0]+m[1]*b[3]+m[2]*0.0,  m[0]*b[1]+m[1]*b[4]+m[2]*0.0,  m[0]*b[2]+m[1]*b[5]+m[2]*1.0,
     m[3]*b[0]+m[4]*b[3]+m[5]*0.0,  m[3]*b[1]+m[4]*b[4]+m[5]*0.0,  m[3]*b[2]+m[4]*b[5]+m[5]*1.0]
}

/// Creates a translation matrix.
#[inline(always)]
pub fn translate(x: f64, y: f64) -> Matrix2d {
    [1.0, 0.0, x,
     0.0, 1.0, y]
}

/// Creates a rotation matrix.
#[inline(always)]
pub fn rotate_radians(angle: f64) -> Matrix2d {
    let c = angle.cos();
    let s = angle.sin();
    [c, s, 0.0,
    -s, c, 0.0]
}

/// Create a scale matrix.
#[inline(always)]
pub fn scale(sx: f64, sy: f64) -> Matrix2d {
    [sx, 0.0, 0.0,
     0.0, sy, 0.0]
}

/// Create a shear matrix.
#[inline(always)]
pub fn shear(sx: f64, sy: f64) -> Matrix2d {
    [1.0, sx, 0.0,
     sy, 1.0, 0.0]
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

