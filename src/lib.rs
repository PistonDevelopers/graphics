#![crate_id = "graphics"]
#![deny(missing_doc)]

//! A library for 2D graphics that works with multiple back-ends.
//!
//! To implement your own back-end, use the BackEnd trait.
//!
//! ## Usage
//!
//! To draw to the back-end, you need a context.
//! The context contains the information necessary to perform the drawing.
//! Unlike other graphics libraries, this library is not bound to the back-end.
//! You do not have to specify which back-end to use before doing the actual drawing.
//!
//! For example, assuming we have a back-end called `back_end`:
//!
//! ```
//! Context::new().rect(x, y, w, h).rgba(r, g, b, a).fill(&mut back_end);
//! ```
//!
//! ## Important!
//!
//! Because the context is built using borrowed pointers,
//! it is not possible to do two or more steps at a time and assign it to a variable:
//!
//! ```
//! // ERROR: Borrowed value does not live long enough
//! let rect = Context::new().rect(x, y, w, h); 
//! ```
//!
//! This is because the lifetime of the first step only lives inside the expression.
//! To solve this problem, break the statement into two parts, one for each step:
//! 
//! ```
//! let c = Context::new();
//! let rect = c.rect(x, y, w, h);
//! ```
//!
//! This is only the case when you are assigning the context to a variable.

extern crate std;

pub use Context = context::Context;
pub use BackEnd = back_end::BackEnd;
pub use Transform2d = transform2d::Transform2d;
pub use AddColor = add_color::AddColor;
pub use AddEllipse = add_ellipse::AddEllipse;
pub use AddPolygon = add_polygon::AddPolygon;
pub use AddRectangle = add_rectangle::AddRectangle;
pub use RelativeRectangle = relative_rectangle::RelativeRectangle;
pub use Fill = fill::Fill;
pub use Clear = clear::Clear;
pub use ColorContext = color_context::ColorContext;
pub use EllipseContext = ellipse_context::EllipseContext;
pub use EllipseColorContext = ellipse_color_context::EllipseColorContext;
pub use LineContext = line_context::LineContext;
pub use PolygonContext = polygon_context::PolygonContext;
pub use PolygonColorContext = polygon_color_context::PolygonColorContext;
pub use RectangleContext = rectangle_context::RectangleContext;
pub use RectangleColorContext = rectangle_color_context::RectangleColorContext;
pub use RoundRectangleContext = round_rectangle_context::RoundRectangleContext;
pub use RoundRectangleColorContext = round_rectangle_color_context::RoundRectangleColorContext;

mod context;
mod back_end;
mod transform2d;
mod add_color;
mod add_ellipse;
mod add_polygon;
mod add_rectangle;
mod relative_rectangle;
mod fill;
mod clear;
mod color_context;
mod ellipse_context;
mod ellipse_color_context;
mod line_context;
mod polygon_context;
mod polygon_color_context;
mod rectangle_context;
mod rectangle_color_context;
mod round_rectangle_context;
mod round_rectangle_color_context;
pub mod vecmath;
pub mod triangulation;
pub mod interpolation;

pub type Color = [f32, ..4];
pub type Line = [f64, ..4];
pub type Ray = [f64, ..4];
pub type Matrix2d = [f64, ..6];
pub type Rectangle = [f64, ..4];
pub type RoundRectangle = [f64, ..5];
pub type Vec2d = [f64, ..2];

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

