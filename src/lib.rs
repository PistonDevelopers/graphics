#![crate_name = "graphics"]
#![deny(missing_docs)]
#![deny(missing_copy_implementations)]
#![feature(default_type_params)]
#![feature(globs)]

//! A library for 2D graphics that works with multiple back-ends.

extern crate "vecmath" as vecmath_lib;
extern crate texture;
extern crate read_color;
extern crate quack;

pub use texture::ImageSize;

pub use back_end::BackEnd;
pub use relative::{
    RelativeColor,
    RelativeRectangle,
    RelativeSourceRectangle,
    RelativeTransform,
    RelativeViewTransform,
};
pub use rectangle::Rectangle;
pub use line::Line;
pub use ellipse::Ellipse;
pub use image::Image;
pub use polygon::Polygon;

pub use context::Context as Context;

/// Any triangulation method called on the back-end
/// never exceeds this number of vertices.
/// This can be used to initialize buffers that fit the chunk size.
pub static BACK_END_MAX_VERTEX_COUNT: uint = 1024;

mod back_end;
mod relative;

pub mod character;
pub mod context;
pub mod color;
pub mod polygon;
pub mod line;
pub mod ellipse;
pub mod rectangle;
pub mod image;
pub mod internal;
pub mod interpolation;
pub mod modular_index;
pub mod text;
pub mod triangulation;
pub mod vecmath;
pub mod deform;
pub mod grid;

pub mod radians {
    //! Reexport radians helper trait from vecmath

    pub use vecmath_lib::consts::Radians;
}

/// A color property
#[deriving(Copy)]
pub struct Color(pub internal::Color);

/// A rectangle property
#[deriving(Copy)]
pub struct Rect(pub internal::Rectangle);

/// A source rectangle property
#[deriving(Copy)]
pub struct SrcRect(pub internal::SourceRectangle);

/// Clears the screen.
pub fn clear<B: BackEnd<I>, I: ImageSize>(
    color: internal::Color, back_end: &mut B
) {
    back_end.clear(color);
}

/// Draws image.
pub fn image<B: BackEnd<I>, I: ImageSize>(
    image: &I, c: &Context, back_end: &mut B
) {
    Image::new().draw(image, c, back_end);
}

