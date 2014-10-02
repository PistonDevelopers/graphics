#![crate_name = "graphics"]
#![deny(missing_doc)]
#![feature(default_type_params)]
#![feature(globs)]

//! A library for 2D graphics that works with multiple back-ends.

extern crate "vecmath" as vecmath_lib;

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
pub use image_size::ImageSize;
pub use relative::{
    RelativeColor,
    RelativeRectangle,
    RelativeSourceRectangle,
    RelativeTransform2d,
};
pub use view::View;

pub use context::Context as Context;

use internal::{
    Color,
};

/// Any triangulation method called on the back-end
/// never exceeds this number of vertices.
/// This can be used to initialize buffers that fit the chunk size.
pub static BACK_END_MAX_VERTEX_COUNT: uint = 1024;

mod add;
mod back_end;
mod context;
mod draw;
mod image_size;
mod relative;
mod view;

pub mod internal;
pub mod interpolation;
pub mod modular_index;
pub mod triangulation;
pub mod vecmath;
pub mod deform;

mod shape;

pub type BevelBorderLineContext
    = context::Context<shape::BevelBorderLineShape>;
pub type BevelBorderLineColorContext
    = context::Context<shape::BevelBorderLineShape, Color>;
pub type BevelRectangleContext
    = context::Context<shape::BevelRectangleShape>;
pub type BevelRectangleColorContext
    = context::Context<shape::BevelRectangleShape, Color>;
pub type BevelRectangleBorderContext
    = context::Context<shape::BevelRectangleBorderShape>;
pub type BevelRectangleBorderColorContext
    = context::Context<shape::BevelRectangleBorderShape, Color>;
pub type ColorContext
    = context::Context<(), Color>;
pub type EllipseContext
    = context::Context<shape::EllipseShape>;
pub type EllipseBorderContext
    = context::Context<shape::EllipseBorderShape>;
pub type EllipseColorContext
    = context::Context<shape::EllipseShape, Color>;
pub type EllipseBorderColorContext
    = context::Context<shape::EllipseBorderShape, Color>;
pub type LerpTweenContext
    = context::Context<shape::LerpTweenShape>;
pub type LerpTweenColorContext
    = context::Context<shape::LerpTweenShape, Color>;
pub type LerpTweenPolygonsContext<'a>
    = context::Context<shape::LerpTweenPolygonsShape<'a>>;
pub type LerpTweenPolygonsColorContext<'a>
    = context::Context<shape::LerpTweenPolygonsShape<'a>, Color>;
pub type LineContext
    = context::Context<shape::LineShape>;
pub type LineColorContext
    = context::Context<shape::LineShape, Color>;
pub type ImageContext<'a, I>
    = context::Context<shape::ImageShape<'a, I>>;
pub type ImageColorContext<'a, I>
    = context::Context<shape::ImageShape<'a, I>, Color>;
pub type ImageRectangleContext<'a, I>
    = context::Context<shape::ImageRectangleShape<'a, I>>;
pub type ImageRectangleColorContext<'a, I>
    = context::Context<shape::ImageRectangleShape<'a, I>, Color>;
pub type PolygonContext<'a>
    = context::Context<shape::PolygonShape<'a>>;
pub type PolygonColorContext<'a>
    = context::Context<shape::PolygonShape<'a>, Color>;
pub type RectangleContext
    = context::Context<shape::RectangleShape>;
pub type RectangleBorderContext
    = context::Context<shape::RectangleBorderShape>;
pub type RectangleColorContext
    = context::Context<shape::RectangleShape, Color>;
pub type RectangleBorderColorContext
    = context::Context<shape::RectangleBorderShape, Color>;
pub type RoundBorderLineContext
    = context::Context<shape::RoundBorderLineShape>;
pub type RoundBorderLineColorContext
    = context::Context<shape::RoundBorderLineShape, Color>;
pub type RoundRectangleContext
    = context::Context<shape::RoundRectangleShape>;
pub type RoundRectangleColorContext
    = context::Context<shape::RoundRectangleShape, Color>;
pub type RoundRectangleBorderContext
    = context::Context<shape::RoundRectangleBorderShape>;
pub type RoundRectangleBorderColorContext
    = context::Context<shape::RoundRectangleBorderShape, Color>;
pub type SquareBorderLineContext
    = context::Context<shape::SquareBorderLineShape>;
pub type SquareBorderLineColorContext
    = context::Context<shape::SquareBorderLineShape, Color>;

