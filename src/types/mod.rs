//! Contains type aliases used in this library

pub use math::{ Matrix2d, Scalar, Vec2d };

pub use self::point::Point;
pub use self::rect::Rect;
pub use self::size::Size;
pub use self::line::Line;
pub use self::triangle::Triangle;

mod point;
mod rect;
mod size;
mod line;
mod triangle;

/// The type used for area.
pub type Area = Scalar;

/// [red, green, blue, alpha]
pub type Color = [ColorComponent; 4];

/// The type used for color component.
pub type ColorComponent = f32;

/// [x, y, w, h]
pub type SourceRectangle = [i32; 4];

/// [p0, p1, ...]
pub type Polygon<'a> = &'a [Vec2d];

/// A slice of polygons.
pub type Polygons<'a> = &'a [Polygon<'a>];

/// The type used for radius.
pub type Radius = Scalar;

/// [x, y, dir_x, dir_y]
pub type Ray = [Scalar; 4];

/// A rectangle.
pub type Rectangle = Rect;

/// The type used for width.
pub type Width = Scalar;

/// The type used for font size.
pub type FontSize = u32;
