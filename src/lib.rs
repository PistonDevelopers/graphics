#![crate_id = "graphics"]
#![deny(missing_doc)]

//! A library for 2D graphics that works with multiple back-ends.

pub use add_bevel::AddBevel;
pub use add_bevel_border::AddBevelBorder;
pub use add_border::AddBorder;
pub use add_color::AddColor;
pub use add_ellipse::AddEllipse;
pub use add_image::AddImage;
pub use add_line::AddLine;
pub use add_polygon::AddPolygon;
pub use add_polygons::AddPolygons;
pub use add_rectangle::AddRectangle;
pub use add_round::AddRound;
pub use add_round_border::AddRoundBorder;
pub use add_square_border::AddSquareBorder;
pub use add_tween::AddTween;
pub use back_end::BackEnd;
pub use bevel_border_line_color_context::BevelBorderLineColorContext;
pub use bevel_border_line_context::BevelBorderLineContext;
pub use bevel_rectangle_border_color_context::BevelRectangleBorderColorContext;
pub use bevel_rectangle_border_context::BevelRectangleBorderContext;
pub use bevel_rectangle_color_context::BevelRectangleColorContext;
pub use bevel_rectangle_context::BevelRectangleContext;
pub use color_context::ColorContext;
pub use draw::Draw;
pub use ellipse_border_context::EllipseBorderContext;
pub use ellipse_color_context::EllipseColorContext;
pub use ellipse_border_color_context::EllipseBorderColorContext;
pub use image_size::ImageSize;
pub use image_context::ImageContext;
pub use image_color_context::ImageColorContext;
pub use image_rectangle_context::ImageRectangleContext;
pub use image_rectangle_color_context::ImageRectangleColorContext;
pub use lerp_tween_context::LerpTweenContext;
pub use lerp_tween_color_context::LerpTweenColorContext;
pub use lerp_tween_polygons_context::LerpTweenPolygonsContext;
pub use lerp_tween_polygons_color_context::LerpTweenPolygonsColorContext;
pub use line_context::LineContext;
pub use line_color_context::LineColorContext;
pub use rectangle_border_color_context::RectangleBorderColorContext;
pub use rectangle_color_context::RectangleColorContext;
pub use relative_color::RelativeColor;
pub use relative_rectangle::RelativeRectangle;
pub use relative_source_rectangle::RelativeSourceRectangle;
pub use relative_transform2d::RelativeTransform2d;
pub use round_border_line_context::RoundBorderLineContext;
pub use round_border_line_color_context::RoundBorderLineColorContext;
pub use round_rectangle_border_color_context::RoundRectangleBorderColorContext;
pub use round_rectangle_border_context::RoundRectangleBorderContext;
pub use round_rectangle_color_context::RoundRectangleColorContext;
pub use round_rectangle_context::RoundRectangleContext;
pub use square_border_line_color_context::SquareBorderLineColorContext;
pub use square_border_line_context::SquareBorderLineContext;
pub use view::View;

pub use context::ctx_id;
pub use context::ctx_abs;

use shape::Shape;
use internal::{
    Color,
    Polygon,
    Radius,
    Rectangle,
};

mod add_bevel;
mod add_bevel_border;
mod add_border;
mod add_color;
mod add_ellipse;
mod add_image;
mod add_line;
mod add_polygon;
mod add_polygons;
mod add_rectangle;
mod add_round;
mod add_round_border;
mod add_square_border;
mod add_tween;
mod back_end;
mod bevel_border_line_color_context;
mod bevel_border_line_context;
mod bevel_rectangle_border_color_context;
mod bevel_rectangle_border_context;
mod bevel_rectangle_color_context;
mod bevel_rectangle_context;
mod color_context;
mod context;
mod draw;
mod ellipse_border_context;
mod ellipse_border_color_context;
mod ellipse_color_context;
mod image_size;
mod image_context;
mod image_color_context;
mod image_rectangle_color_context;
mod image_rectangle_context;
mod line_color_context;
mod line_context;
mod rectangle_border_color_context;
mod rectangle_color_context;
mod relative_color;
mod relative_rectangle;
mod relative_source_rectangle;
mod relative_transform2d;
mod round_border_line_color_context;
mod round_border_line_context;
mod round_rectangle_border_color_context;
mod round_rectangle_border_context;
mod round_rectangle_color_context;
mod round_rectangle_context;
mod square_border_line_color_context;
mod square_border_line_context;
mod lerp_tween_color_context;
mod lerp_tween_context;
mod lerp_tween_polygons_color_context;
mod lerp_tween_polygons_context;
mod view;

pub mod internal;
pub mod interpolation;
pub mod modular_index;
pub mod triangulation;
pub mod vecmath;

mod shape;

pub type Context = context::Context<(), ()>;
pub type ColorContext = context::Context<(), Color>;
pub type RectangleContext = context::Context<shape::RectangleShape, ()>;
pub type RectangleBorderContext = context::Context<
    shape::RectangleBorderShape, ()>;
pub type EllipseContext = context::Context<shape::EllipseShape, ()>;
pub type EllipseBorderContext = context::Context<
    shape::EllipseBorderShape, ()>;
pub type EllipseColorContext = context::Context<
    shape::EllipseShape, Color>;
pub type ImageContext<'a, I> = context::Context<
    shape::ImageShape<'a, I>, ()>;
pub type ImageColorContext<'a, I> = context::Context<
    shape::ImageShape<'a, I>, Color>;
pub type PolygonContext<'a> = context::Context<Polygon<'a>, ()>;
pub type PolygonColorContext<'a> = context::Context<
    Polygon<'a>, Color>;

