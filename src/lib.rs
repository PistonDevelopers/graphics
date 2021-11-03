#![crate_name = "graphics"]
#![deny(
    rust_2018_compatibility,
    rust_2018_idioms,
    unused,
    nonstandard_style,
    future_incompatible,
    missing_docs,
    missing_copy_implementations
)]

//! A library for 2D graphics that works with multiple back-ends.
//!
//! Piston-Graphics was started in 2014 by Sven Nilsen to test
//! back-end agnostic design for 2D in Rust.
//! This means generic code can be reused across projects and platforms.
//!
//! ### Design
//!
//! A graphics back-end implements the `Graphics` trait.
//!
//! This library uses immediate design for flexibility.
//! By default, triangles are generated from 2D shapes and passed in chunks
//! to the back-end. This behavior can be overridden by a back-end library.
//!
//! The structures used for drawing 2D shapes contains settings for rendering.
//! The separation of shapes and settings allows more reuse and flexibility.
//! For example, to render an image, you use an `Image` object.
//!
//! The `math` module contains useful methods for 2D geometry.
//!
//! `Context` stores settings that are commonly shared when rendering.
//! It can be copied and changed without affecting any global state.
//!
//! At top level, there are some shortcut methods for common operations.
//! For example, `ellipse` is a simplified version of `Ellipse`.

pub use character::{Character, CharacterCache};
pub use circle_arc::CircleArc;
pub use colored::Colored;
pub use context::Context;
pub use draw_state::DrawState;
pub use ellipse::Ellipse;
pub use crate::graphics::Graphics;
pub use image::Image;
pub use line::Line;
pub use polygon::Polygon;
pub use radians::Radians;
pub use rectangle::Rectangle;
pub use rectangled::Rectangled;
pub use source_rectangled::SourceRectangled;
pub use text::Text;
pub use texture::ImageSize;
pub use transformed::Transformed;
pub use viewport::Viewport;

/// Any triangulation method called on the back-end
/// never exceeds this number of vertices.
/// This can be used to initialize buffers that fit the chunk size.
///
/// Must be a multiple of 3 because you need 3 vertices per triangle
/// in a triangle list.
pub const BACK_END_MAX_VERTEX_COUNT: usize = 1023;

mod colored;
mod graphics;
mod rectangled;
mod source_rectangled;
mod transformed;

pub mod character;
pub mod circle_arc;
pub mod color;
pub mod context;
pub mod draw_state;
pub mod ellipse;
pub mod glyph_cache;
pub mod grid;
pub mod image;
pub mod line;
pub mod math;
pub mod modular_index;
pub mod polygon;
pub mod rectangle;
pub mod text;
pub mod texture_packer;
pub mod triangulation;
pub mod types;

pub mod radians {
    //! Reexport radians helper trait from vecmath

    pub use vecmath::traits::Radians;
}

/// Clears the screen.
pub fn clear<G>(color: types::Color, g: &mut G)
where
    G: Graphics,
{
    g.clear_color(color);
    g.clear_stencil(0);
}

/// Draws image.
pub fn image<G>(image: &<G as Graphics>::Texture, transform: math::Matrix2d, g: &mut G)
where
    G: Graphics,
{
    Image::new().draw(image, &Default::default(), transform, g);
}

/// Draws ellipse by corners.
pub fn ellipse_from_to<P: Into<types::Vec2d>, G>(
    color: types::Color,
    from: P,
    to: P,
    transform: math::Matrix2d,
    g: &mut G,
) where
    G: Graphics,
{
    Ellipse::new(color).draw_from_to(from, to, &Default::default(), transform, g);
}

/// Draws ellipse.
pub fn ellipse<R: Into<types::Rectangle>, G>(
    color: types::Color,
    rect: R,
    transform: math::Matrix2d,
    g: &mut G,
) where
    G: Graphics,
{
    Ellipse::new(color).draw(rect, &Default::default(), transform, g);
}

/// Draws arc
pub fn circle_arc<R: Into<types::Rectangle>, G>(
    color: types::Color,
    radius: types::Radius,
    start: types::Scalar,
    end: types::Scalar,
    rect: R,
    transform: math::Matrix2d,
    g: &mut G,
) where
    G: Graphics,
{
    CircleArc::new(color, radius, start, end).draw(rect, &Default::default(), transform, g);
}

/// Draws rectangle.
pub fn rectangle_from_to<P: Into<types::Vec2d>, G>(
    color: types::Color,
    from: P,
    to: P,
    transform: math::Matrix2d,
    g: &mut G,
) where
    G: Graphics,
{
    Rectangle::new(color).draw_from_to(from, to, &Default::default(), transform, g);
}

/// Draws rectangle.
pub fn rectangle<R: Into<types::Rectangle>, G>(
    color: types::Color,
    rect: R,
    transform: math::Matrix2d,
    g: &mut G,
) where
    G: Graphics,
{
    Rectangle::new(color).draw(rect, &Default::default(), transform, g);
}

/// Draws polygon.
pub fn polygon<G>(
    color: types::Color,
    polygon: types::Polygon<'_>,
    transform: math::Matrix2d,
    g: &mut G,
) where
    G: Graphics,
{
    Polygon::new(color).draw(polygon, &Default::default(), transform, g);
}

/// Draws line between points.
pub fn line_from_to<P: Into<types::Vec2d>, G>(
    color: types::Color,
    radius: types::Radius,
    from: P,
    to: P,
    transform: math::Matrix2d,
    g: &mut G,
) where
    G: Graphics,
{
    Line::new(color, radius).draw_from_to(from, to, &Default::default(), transform, g)
}

/// Draws line.
pub fn line<L: Into<types::Line>, G>(
    color: types::Color,
    radius: types::Radius,
    line: L,
    transform: math::Matrix2d,
    g: &mut G,
) where
    G: Graphics,
{
    Line::new(color, radius).draw(line, &Default::default(), transform, g)
}

/// Draws text.
pub fn text<C, G>(
    color: types::Color,
    font_size: types::FontSize,
    text: &str,
    cache: &mut C,
    transform: math::Matrix2d,
    g: &mut G,
) -> Result<(), C::Error>
where
    C: character::CharacterCache,
    G: Graphics<Texture = <C as character::CharacterCache>::Texture>,
{
    Text::new_color(color, font_size).draw(text, cache, &Default::default(), transform, g)
}
