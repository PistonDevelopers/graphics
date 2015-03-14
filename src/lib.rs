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
pub use draw_state::DrawState;

pub use graphics::Graphics;
pub use graphics::Graphics as BackEnd;
pub use relative::Colored as RelativeColor;
pub use relative::Rectangled as RelativeRectangle;
pub use relative::SourceRectangled as RelativeSourceRectangle;
pub use relative::Transformed as RelativeTransform;
pub use relative::ViewTransformed as RelativeViewTransform;
pub use relative::{
    Colored,
    Rectangled,
    SourceRectangled,
    Transformed,
    ViewTransformed,
};
pub use rectangle::Rectangle;
pub use line::Line;
pub use ellipse::Ellipse;
pub use image::Image;
pub use polygon::Polygon;
pub use vecmath::abs_transform;
pub use default_draw_state::default_draw_state;

pub use context::Context as Context;

/// Any triangulation method called on the back-end
/// never exceeds this number of vertices.
/// This can be used to initialize buffers that fit the chunk size.
pub static BACK_END_MAX_VERTEX_COUNT: usize = 1024;

mod graphics;
mod relative;
mod default_draw_state;

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
pub fn clear<G>(
    color: internal::Color, g: &mut G
)
    where G: Graphics
{
    g.clear(color);
}

/// Draws image.
pub fn image<G>(
    image: &<G as Graphics>::Texture,
    transform: vecmath::Matrix2d,
    g: &mut G
)
    where G: Graphics
{
    Image::new().draw(image, default_draw_state(), transform, g);
}

/// Draws ellipse.
pub fn ellipse<G>(
    color: internal::Color,
    rect: internal::Rectangle,
    transform: vecmath::Matrix2d,
    g: &mut G
)
    where G: Graphics
{
    Ellipse::new(color).draw(rect, default_draw_state(), transform, g);
}

/// Draws rectangle.
pub fn rectangle<G>(
    color: internal::Color,
    rect: internal::Rectangle,
    transform: vecmath::Matrix2d,
    g: &mut G
)
    where G: Graphics
{
    Rectangle::new(color).draw(rect, default_draw_state(), transform, g);
}

/// Draws polygon.
pub fn polygon<G>(
    color: internal::Color,
    polygon: internal::Polygon,
    transform: vecmath::Matrix2d,
    g: &mut G
)
    where G: Graphics
{
    Polygon::new(color).draw(polygon, default_draw_state(), transform, g);
}
