#![crate_name = "graphics"]
#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

//! A library for 2D graphics that works with multiple back-ends.

extern crate vecmath as vecmath_lib;
extern crate texture;
extern crate read_color;
extern crate interpolation;
extern crate draw_state as draw_state_lib;
extern crate num;

pub use texture::ImageSize;
pub use draw_state_lib as draw_state;
pub use draw_state::DrawState;

pub use graphics::Graphics;
pub use relative::{
    Rectangled,
    SourceRectangled,
};
pub use transformed::Transformed;
pub use colored::Colored;
pub use rectangle::Rectangle;
pub use line::Line;
pub use ellipse::Ellipse;
pub use image::Image;
pub use polygon::Polygon;
pub use math::abs_transform;
pub use default_draw_state::default_draw_state;

pub use context::Context;

/// Any triangulation method called on the back-end
/// never exceeds this number of vertices.
/// This can be used to initialize buffers that fit the chunk size.
pub static BACK_END_MAX_VERTEX_COUNT: usize = 1024;

mod graphics;
mod relative;
mod transformed;
mod colored;
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
pub mod math;
pub mod deform;
pub mod grid;

pub mod radians {
    //! Reexport radians helper trait from vecmath

    pub use vecmath_lib::consts::Radians;
}

/// A rectangle property
#[derive(Copy, Clone)]
pub struct Rect(pub internal::Rectangle);

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
    transform: math::Matrix2d,
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
    transform: math::Matrix2d,
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
    transform: math::Matrix2d,
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
    transform: math::Matrix2d,
    g: &mut G
)
    where G: Graphics
{
    Polygon::new(color).draw(polygon, default_draw_state(), transform, g);
}
