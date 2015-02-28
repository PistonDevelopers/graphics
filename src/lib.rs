#![crate_name = "graphics"]
#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

//! A library for 2D graphics that works with multiple back-ends.

extern crate "vecmath" as vecmath_lib;
extern crate texture;
extern crate read_color;
#[macro_use]
extern crate quack;
extern crate interpolation;
extern crate draw_state;

pub use texture::ImageSize;

pub use graphics::Graphics;
pub use graphics::Graphics as BackEnd;
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
pub static BACK_END_MAX_VERTEX_COUNT: usize = 1024;

mod graphics;
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
#[derive(Copy)]
pub struct Color(pub internal::Color);

/// A rectangle property
#[derive(Copy)]
pub struct Rect(pub internal::Rectangle);

/// A source rectangle property
#[derive(Copy)]
pub struct SrcRect(pub internal::SourceRectangle);

/// Clears the screen.
pub fn clear<B>(
    color: internal::Color, back_end: &mut B
)
    where B: Graphics
{
    back_end.clear(color);
}

/// Draws image.
pub fn image<B>(
    image: &<B as Graphics>::Texture,
    c: &Context,
    back_end: &mut B
)
    where B: Graphics
{
    Image::new().draw(image, c, back_end);
}

/// Draws ellipse.
pub fn ellipse<B>(
    color: internal::Color,
    rect: internal::Rectangle,
    c: &Context,
    back_end: &mut B
)
    where B: Graphics
{
    Ellipse::new(color).draw(rect, c, back_end);
}

/// Draws rectangle.
pub fn rectangle<B>(
    color: internal::Color,
    rect: internal::Rectangle,
    c: &Context,
    back_end: &mut B
)
    where B: Graphics
{
    Rectangle::new(color).draw(rect, c, back_end);
}

/// Draws polygon.
pub fn polygon<B>(
    color: internal::Color,
    polygon: internal::Polygon,
    c: &Context,
    back_end: &mut B
)
    where B: Graphics
{
    Polygon::new(color).draw(polygon, c, back_end);
}
