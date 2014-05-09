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
pub use RelativeTransform2d = relative_transform2d::RelativeTransform2d;
pub use AddColor = add_color::AddColor;
pub use AddEllipse = add_ellipse::AddEllipse;
pub use AddImage = add_image::AddImage;
pub use AddPolygon = add_polygon::AddPolygon;
pub use AddPolygons = add_polygons::AddPolygons;
pub use AddRectangle = add_rectangle::AddRectangle;
pub use AddRound = add_round::AddRound;
pub use AddTween = add_tween::AddTween;
pub use RelativeColor = relative_color::RelativeColor;
pub use RelativeRectangle = relative_rectangle::RelativeRectangle;
pub use Draw = draw::Draw;
pub use Fill = fill::Fill;
pub use Clear = clear::Clear;
pub use ColorContext = color_context::ColorContext;
pub use EllipseContext = ellipse_context::EllipseContext;
pub use EllipseColorContext = ellipse_color_context::EllipseColorContext;
pub use ImageRectangleContext = image_rectangle_context::ImageRectangleContext;
pub use ImageRectangleColorContext = image_rectangle_color_context::ImageRectangleColorContext;
pub use LineContext = line_context::LineContext;
pub use PolygonContext = polygon_context::PolygonContext;
pub use PolygonColorContext = polygon_color_context::PolygonColorContext;
pub use RectangleContext = rectangle_context::RectangleContext;
pub use RectangleColorContext = rectangle_color_context::RectangleColorContext;
pub use RoundRectangleContext = round_rectangle_context::RoundRectangleContext;
pub use RoundRectangleColorContext = round_rectangle_color_context::RoundRectangleColorContext;
pub use TweenContext = tween_context::TweenContext;
pub use TweenColorContext = tween_color_context::TweenColorContext;
pub use TweenPolygonsContext = tween_polygons_context::TweenPolygonsContext;
pub use TweenPolygonsColorContext = tween_polygons_color_context::TweenPolygonsColorContext;
pub use View = view::View;

mod internal;
mod context;
mod back_end;
mod relative_transform2d;
mod add_color;
mod add_ellipse;
mod add_image;
mod add_polygon;
mod add_polygons;
mod add_rectangle;
mod add_round;
mod add_tween;
mod relative_color;
mod relative_rectangle;
mod draw;
mod fill;
mod clear;
mod color_context;
mod ellipse_context;
mod ellipse_color_context;
mod image_rectangle_context;
mod image_rectangle_color_context;
mod line_context;
mod polygon_context;
mod polygon_color_context;
mod rectangle_context;
mod rectangle_color_context;
mod round_rectangle_context;
mod round_rectangle_color_context;
mod tween_context;
mod tween_color_context;
mod tween_polygons_context;
mod tween_polygons_color_context;
mod view;
pub mod vecmath;
pub mod triangulation;
pub mod interpolation;
pub mod modular_index;

/// [red, green, blue, alpha]
pub type Color = [f32, ..4];

/// [x1, y1, x2, y2]
pub type Line = [f64, ..4];

/// [x, y, dir_x, dir_y]
pub type Ray = [f64, ..4];

/// [m00, m01, m02, m10, m11, m12]  
///
/// The first 3 numbers transforms `x`,  
/// the last 3 numbers transforms `y`:
///
/// ```
/// tx = m00 * x + m01 * y + m02;
/// ty = m10 * x + m11 * y + m12;
/// ```
pub type Matrix2d = [f64, ..6];

/// [x, y, w, h]
pub type PixelRectangle = [u32, ..4];

/// [x, y, w, h]
pub type Rectangle = [f64, ..4];

/// [x1, y1, x2, y2, x3, y3]
pub type Triangle = [f64, ..6];

/// [x, y]
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

/// Represents an image.
///
/// Images are often packed together in sprite sheets.  
/// For this reason it refers to a rectangle within a texture.  
///
/// The texture is a unique identifier recognized by the back-end.  
/// An image contains the size of a texture to be able to  
/// compute normalized coordinates.  
///
/// There is no garbage collection of textures,  
/// this responsibility is given to the back-end.  
pub struct Image {
    /// A unique identifier of the texture, recognizable by back-end.
    pub texture_id: uint,
    /// The pixel width of the texture.
    pub texture_width: u32,
    /// The pixel height of the texture.
    pub texture_height: u32,
    /// The source rectangle in the texture.
    pub source_rect: PixelRectangle,
}

