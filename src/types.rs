//! Contains type aliases used in this library

pub use crate::math::{Matrix2d, Scalar, Vec2d};

/// The type used for area.
pub type Area<T = Scalar> = T;

/// [red, green, blue, alpha]
///
/// All values are between 0.0 and 1.0.
/// For example, black is `[0.0, 0.0, 0.0, 1.0]` and white is `[1.0, 1.0, 1.0, 1.0]`.
pub type Color = [ColorComponent; 4];

/// The type used for color component.
pub type ColorComponent = f32;

/// [x1, y1, x2, y2]
pub type Line<T = Scalar> = [T; 4];

/// [x, y, w, h]
pub type SourceRectangle<T = Scalar> = [T; 4];

/// [p0, p1, ...]
pub type Polygon<'a, T = Scalar> = &'a [Vec2d<T>];

/// A slice of polygons.
pub type Polygons<'a, T = Scalar> = &'a [Polygon<'a, T>];

/// The type used for radius.
pub type Radius<T = Scalar> = T;

/// The type used for resolution.
pub type Resolution = u32;

/// [x, y, dir_x, dir_y]
pub type Ray<T = Scalar> = [T; 4];

/// Rectangle dimensions: [x, y, w, h]
pub type Rectangle<T = Scalar> = [T; 4];

/// [x1, y1, x2, y2, x3, y3]
pub type Triangle<T = Scalar> = [Vec2d<T>; 3];

/// The type used for width.
pub type Width<T = Scalar> = T;

/// The type used for font size.
pub type FontSize = u32;
