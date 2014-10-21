
//! Contains internal type aliases and traits.
//!
//! These do not affect the normal usage of the library.
//! Using these might lead to breaking changes.
//! Made public to clarify documentation.

pub use vecmath::{ Scalar, Vec2d };

/// The type used for area.
pub type Area = Scalar;

/// [red, green, blue, alpha]
pub type Color = [ColorComponent, ..4];

/// The type used for color component.
pub type ColorComponent = f32;

/// [x1, y1, x2, y2]
pub type Line = [Scalar, ..4];

/// [x, y, w, h]
pub type SourceRectangle = [i32, ..4];

/// [x0, y0, x1, y1, ...]
pub type Polygon<'a> = &'a [Scalar];

/// A slice of polygons.
pub type Polygons<'a> = &'a [Polygon<'a>];

/// The type used for radius.
pub type Radius = Scalar;

/// [x, y, dir_x, dir_y]
pub type Ray = [Scalar, ..4];

/// [x, y, w, h]
pub type Rectangle = [Scalar, ..4];

/// [x1, y1, x2, y2, x3, y3]
pub type Triangle = [Scalar, ..6];

/// The type used for width.
pub type Width = Scalar;
