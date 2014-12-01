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
pub use line::{ Line, RoundLine, BevelLine, SquareLine };
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
mod line;

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

mod shape;

pub type BevelBorderLineContext
    = context::Context<shape::BevelBorderLineShape>;
pub type BevelBorderLineColorContext
    = context::Context<shape::BevelBorderLineShape, internal::Color>;
pub type BevelRectangleContext
    = context::Context<shape::BevelRectangleShape>;
pub type BevelRectangleColorContext
    = context::Context<shape::BevelRectangleShape, internal::Color>;
pub type BevelRectangleBorderContext
    = context::Context<shape::BevelRectangleBorderShape>;
pub type BevelRectangleBorderColorContext
    = context::Context<shape::BevelRectangleBorderShape, internal::Color>;
pub type ColorContext
    = context::Context<(), internal::Color>;
pub type EllipseContext
    = context::Context<shape::EllipseShape>;
pub type EllipseBorderContext
    = context::Context<shape::EllipseBorderShape>;
pub type EllipseColorContext
    = context::Context<shape::EllipseShape, internal::Color>;
pub type EllipseBorderColorContext
    = context::Context<shape::EllipseBorderShape, internal::Color>;
pub type LerpTweenContext
    = context::Context<shape::LerpTweenShape>;
pub type LerpTweenColorContext
    = context::Context<shape::LerpTweenShape, internal::Color>;
pub type LerpTweenPolygonsContext<'a>
    = context::Context<shape::LerpTweenPolygonsShape<'a>>;
pub type LerpTweenPolygonsColorContext<'a>
    = context::Context<shape::LerpTweenPolygonsShape<'a>, internal::Color>;
pub type LineContext
    = context::Context<shape::LineShape>;
pub type LineColorContext
    = context::Context<shape::LineShape, internal::Color>;
pub type ImageContext<'a, I>
    = context::Context<shape::ImageShape<'a, I>>;
pub type ImageColorContext<'a, I>
    = context::Context<shape::ImageShape<'a, I>, internal::Color>;
pub type ImageRectangleContext<'a, I>
    = context::Context<shape::ImageRectangleShape<'a, I>>;
pub type ImageRectangleColorContext<'a, I>
    = context::Context<shape::ImageRectangleShape<'a, I>, internal::Color>;
pub type PolygonContext<'a>
    = context::Context<shape::PolygonShape<'a>>;
pub type PolygonColorContext<'a>
    = context::Context<shape::PolygonShape<'a>, internal::Color>;
pub type RectangleContext
    = context::Context<shape::RectangleShape>;
pub type RectangleBorderContext
    = context::Context<shape::RectangleBorderShape>;
pub type RectangleColorContext
    = context::Context<shape::RectangleShape, internal::Color>;
pub type RectangleBorderColorContext
    = context::Context<shape::RectangleBorderShape, internal::Color>;
pub type RoundBorderLineContext
    = context::Context<shape::RoundBorderLineShape>;
pub type RoundBorderLineColorContext
    = context::Context<shape::RoundBorderLineShape, internal::Color>;
pub type RoundRectangleContext
    = context::Context<shape::RoundRectangleShape>;
pub type RoundRectangleColorContext
    = context::Context<shape::RoundRectangleShape, internal::Color>;
pub type RoundRectangleBorderContext
    = context::Context<shape::RoundRectangleBorderShape>;
pub type RoundRectangleBorderColorContext
    = context::Context<shape::RoundRectangleBorderShape, internal::Color>;
pub type SquareBorderLineContext
    = context::Context<shape::SquareBorderLineShape>;
pub type SquareBorderLineColorContext
    = context::Context<shape::SquareBorderLineShape, internal::Color>;

/// A color property
pub struct Color(pub internal::Color);

/// A rectangle property
pub struct Rect(pub internal::Rectangle);

/// A source rectangle property
pub struct SrcRect(pub internal::SourceRectangle);

