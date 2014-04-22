#![crate_id = "graphics"]
#![deny(missing_doc)]

//! Attempt of creating a cheap drawing context.

use std::num::Float;

pub type Matrix2d = [f64, ..6];
pub type Color = [f64, ..4];

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
pub fn multiply(m: &[f64, ..6], b: &[f64, ..6]) -> [f64, ..6] {
    [m[0]*b[0]+m[1]*b[3]+m[2]*0.0,  m[0]*b[1]+m[1]*b[4]+m[2]*0.0,  m[0]*b[2]+m[1]*b[5]+m[2]*1.0,
     m[3]*b[0]+m[4]*b[3]+m[5]*0.0,  m[3]*b[1]+m[4]*b[4]+m[5]*0.0,  m[3]*b[2]+m[4]*b[5]+m[5]*1.0]
}

/// Drawing 2d context.
pub struct Context<'a> {
    base: Field<'a, Matrix2d>,
    transform: Field<'a, Matrix2d>,
    color: Field<'a, Color>,
}

impl<'a> Context<'a> {
    /// Creates a new drawing context.
    pub fn new() -> Context {
        Context {
            base:  Value([1.0, 0.0, 0.0,
                          0.0, 1.0, 0.0]),
            transform: Value([1.0, 0.0, 0.0,
                          0.0, 1.0, 0.0]),
            color: Value([0.0, 0.0, 0.0, 1.0]),
        }
    }

    /// Returns a translated context.
    #[inline(always)]
    pub fn trans(&'a self, x: f64, y: f64) -> Context<'a> {
        Context {
            base: Borrowed(self.base.get()),
            transform: Value({
                let trans: [f64, ..6] = [1.0, 0.0, x,
                                         0.0, 1.0, y];
                 multiply(&trans, self.transform.get())
            }),
            color: Borrowed(self.color.get()),
        }
    }

    /// Rotates with degrees.
    #[inline(always)]
    pub fn rot_deg(&'a self, angle: f64) -> Context<'a> {
        let pi: f64 = Float::pi();
        self.rot(angle * pi / 180.0)
    }

    /// Returns a rotated context.
    #[inline(always)]
    pub fn rot(&'a self, angle: f64) -> Context<'a> {
        Context {
            base: Borrowed(self.base.get()),
            transform: Value({
                let c = angle.cos();
                let s = angle.sin();
                let rot: [f64, ..6] = [c, s, 0.0,
                                      -s, c, 0.0];
                multiply(&rot, self.transform.get())
            }),
            color: Borrowed(self.color.get()),
        }
    }

    /// Returns a scaled context.
    #[inline(always)]
    pub fn scale(&'a self, sx: f64, sy: f64) -> Context<'a> {
        Context {
            base: Borrowed(self.base.get()),
            transform: Value({
                let scale: [f64, ..6] = [sx, 0.0, 0.0,
                                         0.0, sy, 0.0];
                multiply(&scale, self.transform.get())
            }),
            color: Borrowed(self.color.get()),
        }
    }
}

#[test]
fn test_context() {
    let c = Context::new();
    {
        let d = c.trans(20.0, 40.0);
        let d = d.trans(10.0, 10.0);
        assert_eq!(d.transform.get()[2], 30.0);
        assert_eq!(d.transform.get()[5], 50.0);
    }
    assert_eq!(c.transform.get()[2], 0.0);
    assert_eq!(c.transform.get()[5], 0.0);

    let c = c.rot_deg(90.0);
    assert!((c.transform.get()[0] - 0.0).abs() < 0.00001);
    assert!((c.transform.get()[1] - 1.0).abs() < 0.00001);
}

#[test]
fn test_scale() {
    let c = Context::new();
    let c = c.scale(2.0, 3.0);
    assert!((c.transform.get()[0] - 2.0).abs() < 0.00001);
    assert!((c.transform.get()[4] - 3.0).abs() < 0.00001);
}
