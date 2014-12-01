#![crate_name = "graphics"]
#![deny(missing_docs)]
#![feature(default_type_params)]
#![feature(globs)]
#![feature(if_let)]

//! A library for 2D graphics that works with multiple back-ends.

extern crate "vecmath" as vecmath_lib;
extern crate texture;
extern crate read_color;
extern crate current;

pub use texture::ImageSize;

pub use add::{
    AddBevel,
    AddBevelBorder,
    AddBorder,
    AddColor,
    AddEllipse,
    AddImage,
    AddLine,
    AddPolygon,
    AddPolygons,
    AddRectangle,
    AddRound,
    AddRoundBorder,
    AddSquareBorder,
    AddTween,
};
pub use back_end::BackEnd;
pub use draw::Draw;
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

pub use context::Context as Context;

/// Any triangulation method called on the back-end
/// never exceeds this number of vertices.
/// This can be used to initialize buffers that fit the chunk size.
pub static BACK_END_MAX_VERTEX_COUNT: uint = 1024;

mod add;
mod back_end;
mod context;
mod draw;
mod relative;

pub mod polygon;
pub mod line;
pub mod ellipse;
pub mod rectangle;
pub mod image;
pub mod can;
pub mod has;
pub mod internal;
pub mod interpolation;
pub mod modular_index;
pub mod triangulation;
pub mod vecmath;
pub mod deform;
pub mod grid;
pub mod radians;

/// A color property
pub struct Color(pub internal::Color);

/// A rectangle property
pub struct Rect(pub internal::Rectangle);

/// A source rectangle property
pub struct SrcRect(pub internal::SourceRectangle);

