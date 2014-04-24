#![crate_id = "graphics"]
#![deny(missing_doc)]

//! Attempt of creating a cheap drawing context.

pub use Context = context::Context;
pub use BackEnd = back_end::BackEnd;
pub use Transform2d = transform2d::Transform2d;
pub use AddColor = add_color::AddColor;
pub use AddRectangle = add_rectangle::AddRectangle;
pub use Fill = fill::Fill;
pub use Clear = clear::Clear;
pub use ColorContext = color_context::ColorContext;
pub use EllipseContext = ellipse_context::EllipseContext;
pub use LineContext = line_context::LineContext;
pub use RectangleContext = rectangle_context::RectangleContext;
pub use RectangleColorContext = rectangle_color_context::RectangleColorContext;

mod context;
mod back_end;
mod transform2d;
mod add_color;
mod add_rectangle;
mod fill;
mod clear;
mod color_context;
mod ellipse_context;
mod line_context;
mod rectangle_context;
mod rectangle_color_context;
pub mod vecmath;
pub mod triangulation;

pub type Color = [f32, ..4];
pub type Line = [f64, ..4];
pub type Matrix2d = [f64, ..6];
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

