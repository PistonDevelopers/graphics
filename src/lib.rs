#![crate_name = "graphics"]
#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

//! A library for 2D graphics that works with multiple back-ends.

extern crate vecmath as vecmath_lib;
extern crate texture;
extern crate read_color;
extern crate interpolation;
extern crate viewport;

pub use texture::ImageSize;
pub use draw_state::DrawState;
pub use viewport::Viewport;

pub use graphics::Graphics;
pub use source_rectangled::SourceRectangled;
pub use rectangled::Rectangled;
pub use transformed::Transformed;
pub use colored::Colored;
pub use rectangle::Rectangle;
pub use line::Line;
pub use ellipse::Ellipse;
pub use circle_arc::CircleArc;
pub use image::Image;
pub use polygon::Polygon;
pub use text::Text;
pub use context::Context;

/// Any triangulation method called on the back-end
/// never exceeds this number of vertices.
/// This can be used to initialize buffers that fit the chunk size.
pub static BACK_END_MAX_VERTEX_COUNT: usize = 1024;

mod graphics;
mod source_rectangled;
mod rectangled;
mod transformed;
mod colored;

pub mod character;
pub mod context;
pub mod color;
pub mod draw_state;
pub mod polygon;
pub mod line;
pub mod circle_arc;
pub mod ellipse;
pub mod rectangle;
pub mod image;
pub mod types;
pub mod modular_index;
pub mod text;
pub mod triangulation;
pub mod math;
pub mod deform;
pub mod grid;

pub mod radians {
    //! Reexport radians helper trait from vecmath

    pub use vecmath_lib::traits::Radians;
}

/// Clears the screen.
pub fn clear<G>(
    color: types::Color, g: &mut G
)
    where G: Graphics
{
    g.clear_color(color);
}

/// Draws image.
pub fn image<G>(
    image: &<G as Graphics>::Texture,
    transform: math::Matrix2d,
    g: &mut G
)
    where G: Graphics
{
    Image::new().draw(image, &Default::default(), transform, g);
}

/// Draws ellipse.
pub fn ellipse<R: Into<types::Rectangle>, G>(
    color: types::Color,
    rect: R,
    transform: math::Matrix2d,
    g: &mut G
)
    where G: Graphics
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
    g: &mut G
)
    where G: Graphics
{
    CircleArc::new(color, radius, start, end).draw(rect, &Default::default(), transform, g);
}

/// Draws rectangle.
pub fn rectangle<R: Into<types::Rectangle>, G>(
    color: types::Color,
    rect: R,
    transform: math::Matrix2d,
    g: &mut G
)
    where G: Graphics
{
    Rectangle::new(color).draw(rect, &Default::default(), transform, g);
}

/// Draws polygon.
pub fn polygon<G>(
    color: types::Color,
    polygon: types::Polygon,
    transform: math::Matrix2d,
    g: &mut G
)
    where G: Graphics
{
    Polygon::new(color).draw(polygon, &Default::default(), transform, g);
}

/// Draws line.
pub fn line<L: Into<types::Line>, G>(
    color: types::Color,
    radius: types::Radius,
    line: L,
    transform: math::Matrix2d,
    g: &mut G
)
    where G: Graphics
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
    g: &mut G
)
    where
        C: character::CharacterCache,
        G: Graphics<Texture = <C as character::CharacterCache>::Texture>
{
    Text::new_color(color, font_size).draw(text, cache, &Default::default(), transform, g)
}
