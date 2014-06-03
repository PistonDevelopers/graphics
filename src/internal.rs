
//! Contains internal type aliases and traits.
//!
//! These do not affect the normal usage of the library.
//! Using these might lead to breaking changes.
//! Made public to clarify documentation.

/// The type used for area.
pub type Area = Scalar;

/// [red, green, blue, alpha]
pub type Color = [ColorComponent, ..4];

/// The type used for color component.
pub type ColorComponent = f32;

/// [x1, y1, x2, y2]
pub type Line = [Scalar, ..4];

/// [m00, m01, m02, m10, m11, m12]
///
/// The first 3 numbers transforms `x`,
/// the last 3 numbers transforms `y`:
///
/// ```
/// tx = m00 * x + m01 * y + m02;
/// ty = m10 * x + m11 * y + m12;
/// ```
pub type Matrix2d = [Scalar, ..6];

/// [x, y, w, h]
pub type SourceRectangle = [u32, ..4];

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

/// The type used for scalars.
pub type Scalar = f64;

/// [x1, y1, x2, y2, x3, y3]
pub type Triangle = [Scalar, ..6];

/// [x, y]
pub type Vec2d = [Scalar, ..2];

/// The type used for width.
pub type Width = Scalar;

/// Implemented by contexts that can color.
///
/// The context can color to type `T` by adding value `U`.
pub trait CanColor<'a, T, U> {
    /// Create a new context with color.
    fn color(&'a self, value: U) -> T;
}

/// Implemented by contexts that can set rectangle information.
pub trait CanRectangle<'a, T, U> {
    /// Create new context with rectangle information.
    fn rectangle(&'a self, value: U) -> T;
}

/// Implemented by contexts that can set source rectangle information.
pub trait CanSourceRectangle<'a, T, U> {
    /// Create new context with source rectangle information.
    fn source_rectangle(&'a self, value: U) -> T;
}

/// Implemented by contexts that can transform.
///
/// The context can transform to type `T` by adding value `U`.
pub trait CanTransform<'a, T, U> {
    /// Create a new context with transformation.
    fn transform(&'a self, value: U) -> T;
}

/// Implemented by contexts that can view transform.
///
/// The context can view transform to type `T` by adding value `U`.
pub trait CanViewTransform<'a, T, U> {
    /// Create a new context with view transformation.
    fn view_transform(&'a self, value: U) -> T;
}

/// Implemented by contexts that uses type `U` as current color.
pub trait HasColor<'a, U> {
    /// Returns the current color.
    fn get_color(&'a self) -> &'a U;
}

/// Implemented by contexts that uses type `U` as current rectangle.
pub trait HasRectangle<'a, U> {
    /// Returns the current rectangle.
    fn get_rectangle(&'a self) -> &'a U;
}

/// Implemented by contexts that uses type `U` as current source rectangle.
pub trait HasSourceRectangle<'a, U> {
    /// Returns the current source rectangle.
    fn get_source_rectangle(&'a self) -> &'a U;
}

/// Implemented by contexts that uses type `U` as current transform.
///
/// This helps to remove redundant code.
pub trait HasTransform<'a, U> {
    /// Returns the current transform.
    fn get_transform(&'a self) -> &'a U;
}

/// Implemented by contexts that uses type `U` as current view transform.
///
/// This helps to remove redundant code.
pub trait HasViewTransform<'a, U> {
    /// Returns the current view transform.
    fn get_view_transform(&'a self) -> &'a U;
}

