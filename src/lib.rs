#![crate_id = "graphics"]
#![deny(missing_doc)]

//! A library for 2D graphics that works with multiple back-ends.
//!
//! To implement your own back-end, use the BackEnd trait.
//!
//! ## Usage
//!
//! If you are using [Rust-Empty](https://github.com/bvssvni/rust-empty) 
//! or Cargo, put the compiled library in 
//! the "target/cpu-vendor-platform/lib/" directory, 
//! then add the following to your source:
//!
//! ```Rust
//! #![feature(globs)] // Allow global imports
//!
//! extern crate graphics; // Link to 'graphics' library
//!
//! use graphics::*; // Use the graphics types in the module
//! ```
//!
//! To draw to the back-end, you need a context.
//! The context contains the information necessary to perform the drawing.
//! Unlike other graphics libraries, this library is not bound to the back-end.
//! You do not have to specify which back-end to use 
//! before doing the actual drawing.
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
//! it is not possible to do two or more steps at a time and 
//! assign it to a variable:
//!
//! ```
//! // ERROR: Borrowed value does not live long enough
//! let rect = Context::new().rect(x, y, w, h);
//! ```
//!
//! This is because the lifetime of the first step only 
//! lives inside the expression.
//! To solve this problem, break the statement into two parts, 
//! one for each step:
//!
//! ```
//! let c = Context::new();
//! let rect = c.rect(x, y, w, h);
//! ```
//!
//! This is only the case when you are assigning the context to a variable.

#[cfg(gl)]
extern crate gl;
#[cfg(gl)]
extern crate libc;
#[cfg(gl)]
extern crate image;
extern crate core;
extern crate std;

#[cfg(gl)]
pub use Gl = gl_back_end::Gl;
#[cfg(gl)]
pub use Texture = texture::Texture;

pub use AddBevel 
    = add_bevel::AddBevel;
pub use AddBevelBorder 
    = add_bevel_border::AddBevelBorder;
pub use AddBorder 
    = add_border::AddBorder;
pub use AddColor 
    = add_color::AddColor;
pub use AddEllipse 
    = add_ellipse::AddEllipse;
pub use AddImage 
    = add_image::AddImage;
pub use AddLine 
    = add_line::AddLine;
pub use AddPolygon 
    = add_polygon::AddPolygon;
pub use AddPolygons 
    = add_polygons::AddPolygons;
pub use AddRectangle 
    = add_rectangle::AddRectangle;
pub use AddRound 
    = add_round::AddRound;
pub use AddRoundBorder 
    = add_round_border::AddRoundBorder;
pub use AddSquareBorder 
    = add_square_border::AddSquareBorder;
pub use AddTween 
    = add_tween::AddTween;
pub use BackEnd 
    = back_end::BackEnd;
pub use BevelBorderLineColorContext 
    = bevel_border_line_color_context::BevelBorderLineColorContext;
pub use BevelBorderLineContext 
    = bevel_border_line_context::BevelBorderLineContext;
pub use BevelRectangleBorderColorContext 
    = bevel_rectangle_border_color_context::BevelRectangleBorderColorContext;
pub use BevelRectangleBorderContext 
    = bevel_rectangle_border_context::BevelRectangleBorderContext;
pub use BevelRectangleColorContext 
    = bevel_rectangle_color_context::BevelRectangleColorContext;
pub use BevelRectangleContext 
    = bevel_rectangle_context::BevelRectangleContext;
pub use Clear 
    = clear::Clear;
pub use ColorContext 
    = color_context::ColorContext;
pub use Context 
    = context::Context;
pub use Draw 
    = draw::Draw;
pub use EllipseContext 
    = ellipse_context::EllipseContext;
pub use EllipseBorderContext 
    = ellipse_border_context::EllipseBorderContext;
pub use EllipseColorContext 
    = ellipse_color_context::EllipseColorContext;
pub use EllipseBorderColorContext 
    = ellipse_border_color_context::EllipseBorderColorContext;
pub use Fill 
    = fill::Fill;
pub use ImageSize 
    = image_size::ImageSize;
pub use ImageContext 
    = image_context::ImageContext;
pub use ImageColorContext 
    = image_color_context::ImageColorContext;
pub use ImageRectangleContext 
    = image_rectangle_context::ImageRectangleContext;
pub use ImageRectangleColorContext 
    = image_rectangle_color_context::ImageRectangleColorContext;
pub use LerpTweenContext 
    = lerp_tween_context::LerpTweenContext;
pub use LerpTweenColorContext 
    = lerp_tween_color_context::LerpTweenColorContext;
pub use LerpTweenPolygonsContext 
    = lerp_tween_polygons_context::LerpTweenPolygonsContext;
pub use LerpTweenPolygonsColorContext 
    = lerp_tween_polygons_color_context::LerpTweenPolygonsColorContext;
pub use LineContext 
    = line_context::LineContext;
pub use LineColorContext 
    = line_color_context::LineColorContext;
pub use PolygonContext 
    = polygon_context::PolygonContext;
pub use PolygonColorContext 
    = polygon_color_context::PolygonColorContext;
pub use RectangleBorderContext 
    = rectangle_border_context::RectangleBorderContext;
pub use RectangleBorderColorContext 
    = rectangle_border_color_context::RectangleBorderColorContext;
pub use RectangleContext 
    = rectangle_context::RectangleContext;
pub use RectangleColorContext 
    = rectangle_color_context::RectangleColorContext;
pub use RelativeColor 
    = relative_color::RelativeColor;
pub use RelativeRectangle 
    = relative_rectangle::RelativeRectangle;
pub use RelativeSourceRectangle 
    = relative_source_rectangle::RelativeSourceRectangle;
pub use RelativeTransform2d 
    = relative_transform2d::RelativeTransform2d;
pub use RoundBorderLineContext 
    = round_border_line_context::RoundBorderLineContext;
pub use RoundBorderLineColorContext 
    = round_border_line_color_context::RoundBorderLineColorContext;
pub use RoundRectangleBorderColorContext 
    = round_rectangle_border_color_context::RoundRectangleBorderColorContext;
pub use RoundRectangleBorderContext 
    = round_rectangle_border_context::RoundRectangleBorderContext;
pub use RoundRectangleColorContext 
    = round_rectangle_color_context::RoundRectangleColorContext;
pub use RoundRectangleContext 
    = round_rectangle_context::RoundRectangleContext;
pub use Stroke 
    = stroke::Stroke;
pub use SquareBorderLineColorContext 
    = square_border_line_color_context::SquareBorderLineColorContext;
pub use SquareBorderLineContext 
    = square_border_line_context::SquareBorderLineContext;
pub use View 
    = view::View;

#[cfg(gl)]
mod gl_back_end;
#[cfg(gl)]
mod texture;
#[cfg(gl)]
pub mod shader_utils;

mod add_bevel;
mod add_bevel_border;
mod add_border;
mod add_color;
mod add_ellipse;
mod add_image;
mod add_line;
mod add_polygon;
mod add_polygons;
mod add_rectangle;
mod add_round;
mod add_round_border;
mod add_square_border;
mod add_tween;
mod back_end;
mod bevel_border_line_color_context;
mod bevel_border_line_context;
mod bevel_rectangle_border_color_context;
mod bevel_rectangle_border_context;
mod bevel_rectangle_color_context;
mod bevel_rectangle_context;
mod clear;
mod color_context;
mod context;
mod draw;
mod ellipse_border_context;
mod ellipse_border_color_context;
mod ellipse_color_context;
mod ellipse_context;
mod fill;
mod image_size;
mod image_context;
mod image_color_context;
mod image_rectangle_color_context;
mod image_rectangle_context;
mod line_color_context;
mod line_context;
mod polygon_color_context;
mod polygon_context;
mod rectangle_border_context;
mod rectangle_border_color_context;
mod rectangle_color_context;
mod rectangle_context;
mod relative_color;
mod relative_rectangle;
mod relative_source_rectangle;
mod relative_transform2d;
mod round_border_line_color_context;
mod round_border_line_context;
mod round_rectangle_border_color_context;
mod round_rectangle_border_context;
mod round_rectangle_color_context;
mod round_rectangle_context;
mod square_border_line_color_context;
mod square_border_line_context;
mod stroke;
mod lerp_tween_color_context;
mod lerp_tween_context;
mod lerp_tween_polygons_color_context;
mod lerp_tween_polygons_context;
mod view;

pub mod internal;
pub mod interpolation;
pub mod modular_index;
pub mod triangulation;
pub mod vecmath;

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

