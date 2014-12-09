#![crate_name = "graphics"]
#![deny(missing_docs)]
#![feature(default_type_params)]
#![feature(globs)]

//! A library for 2D graphics that works with multiple back-ends.

extern crate "vecmath" as vecmath_lib;
extern crate texture;
extern crate read_color;
extern crate current;

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

use current::{ Get, Set };

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
pub struct Color(pub internal::Color);

/// Wrapper trait for `Get<Color>`
pub trait GetColor: Get<Color> {
    /// Get color
    #[inline(always)]
    fn get_color(&self) -> Color {
        self.get()
    }
}

impl<T: Get<Color>> GetColor for T {}

/// Wrapper trait for `Set<Color>`
pub trait SetColor: Set<Color> {
    /// Set color
    #[inline(always)]
    fn set_color(&mut self, val: Color) {
        self.set_mut(val);
    }
}

impl<T: Set<Color>> SetColor for T {}

/// A rectangle property
pub struct Rect(pub internal::Rectangle);

/// Wrapper trait for `Get<Rect>`
pub trait GetRect: Get<Rect> {
    /// Get rectangle
    #[inline(always)]
    fn get_rect(&self) -> Rect {
        self.get()
    }
}

impl<T: Get<Rect>> GetRect for T {}

/// Wrapper trait for `Set<Rect>`
pub trait SetRect: Set<Rect> {
    /// Set rectangle
    #[inline(always)]
    fn set_rect(&mut self, val: Rect) {
        self.set_mut(val);
    }
}

impl<T: Set<Rect>> SetRect for T {}

/// A source rectangle property
pub struct SrcRect(pub internal::SourceRectangle);

/// Wrapper trait for `Get<SrcRect>`
pub trait GetSrcRect: Get<SrcRect> {
    /// Get source rectangle
    #[inline(always)]
    fn get_src_rect(&self) -> SrcRect {
        self.get()
    }
}

impl<T: Get<SrcRect>> GetSrcRect for T {}

/// Wrapper trait for `Set<SrcRect>`
pub trait SetSrcRect: Set<SrcRect> {
    /// Set source rectangle
    #[inline(always)]
    fn set_src_rect(&mut self, val: SrcRect) {
        self.set_mut(val);
    }
}

impl<T: Set<SrcRect>> SetSrcRect for T {}

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

