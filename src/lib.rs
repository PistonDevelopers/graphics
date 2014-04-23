#![crate_id = "graphics"]
#![deny(missing_doc)]

//! Attempt of creating a cheap drawing context.

pub use context::Context;
pub use back_end::BackEnd;
pub use transform2d::Transform2d;
pub use color_context::ColorContext;
pub use rectangle_context::RectangleContext;
pub use rectangle_color_context::RectangleColorContext;

mod context;
mod back_end;
mod transform2d;
mod color_context;
mod rectangle_context;
mod rectangle_color_context;
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

