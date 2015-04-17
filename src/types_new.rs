//! Contains types used in this library
use std::convert::From;
use std::ops::{ Add, Mul, Sub };

pub use math::{ self, Matrix2d, Scalar, Vec2d };

/// The size of a shape.
#[derive(Clone, Copy, Debug)]
pub struct Size {
    /// The horizontal length of the shape (width).
    pub w: Scalar,
    /// The vertical length of the shape (height).
    pub h: Scalar,
}

impl From<Vec2d> for Size {
    fn from(v: Vec2d) -> Size {
        Size { w: v[0], h: v[1] }
    }
}

impl From<(Scalar, Scalar)> for Size {
    fn from((w, h): (Scalar, Scalar)) -> Size {
        Size { w: w, h: h }
    }
}

impl Size {
    /// Convert size to a vector.
    pub fn to_array(self) -> Vec2d {
        [self.w, self.h]
    }
}

impl<T: Into<Size>> Mul<T> for Size {
    type Output = Size;

    fn mul(self, v: T) -> Size {
        let v: Size = v.into();
        Size { w: self.w * v.w, h: self.h * v.h }
    }
}

impl Mul<Scalar> for Size {
    type Output = Size;

    fn mul(self, s: Scalar) -> Size {
        Size { w: self.w * s, h: self.h * s }
    }
}

/// A point in the Cartesian plane.
#[derive(Clone, Copy, Debug)]
pub struct Point {
    /// The x coordinate.
    pub x: Scalar,
    /// The y coordinate.
    pub y: Scalar,
}

impl Add<Scalar> for Point {
    type Output = Point;

    fn add(self, s: Scalar) -> Point {
        Point { x: self.x + s, y: self.y + s }
    }
}

impl<T: Into<Point>> Add<T> for Point {
    type Output = Point;

    fn add(self, v: T) -> Point {
        let v: Point = v.into();
        Point { x: self.x + v.x, y: self.y + v.y }
    }
}

impl From<Vec2d> for Point {
    fn from(v: Vec2d) -> Point {
        Point { x: v[0], y: v[1] }
    }
}

impl From<(Scalar, Scalar)> for Point {
    fn from((x, y): (Scalar, Scalar)) -> Point {
        Point { x: x, y: y }
    }
}

impl Sub<Scalar> for Point {
    type Output = Point;

    fn sub(self, s: Scalar) -> Point {
        Point { x: self.x - s, y: self.y - s }
    }
}

impl<T: Into<Point>> Sub<T> for Point {
    type Output = Point;

    fn sub(self, v: T) -> Point {
        let v = v.into();
        Point { x: self.x - v.x, y: self.y - v.y }
    }
}

impl Point {
    /// Convert the point to a vector.
    pub fn to_array(self) -> Vec2d {
        [self.x, self.y]
    }
}

/// A rectangle.
#[derive(Clone, Copy, Debug)]
pub struct Rect {
    /// The position of the top left corner of the rectangle.
    pub pos: Point,
    /// The width and height of the rectangle.
    pub size: Size,
}

impl<P: Into<Point>, S: Into<Size>> From<(P, S)> for Rect {
    /// Creates a rectangle from the position of its top left corner and its size.
    fn from((pos, size): (P, S)) -> Rect {
        let (pos, size): (Point, Size) = (pos.into(), size.into());
        Rect { pos: pos, size: size }
    }
}

impl From<[Scalar; 4]> for Rect {
    /// Creates a rectangle from an array.
    fn from(v: [Scalar; 4]) -> Rect {
        Rect {
            pos: Point { x: v[0], y: v[1] },
            size: Size { w: v[2], h: v[3] },
        }
    }
}

impl From<(Scalar, Scalar, Scalar, Scalar)> for Rect {
    fn from((x, y, w, h): (Scalar, Scalar, Scalar, Scalar)) -> Rect {
        Rect {
            pos: Point { x: x, y: y },
            size: Size { w: w, h: h },
        }
    }
}

impl Rect {
    /// Returns the position of the bottom side of the rectangle.
    pub fn bottom(&self) -> Scalar {
        self.pos.y + self.size.h
    }

    /// Computes a rectangle with quadruple the surface area of self and with center
    /// (self.x, self.y).
    pub fn centered(self) -> Rect {
        Rect {
            pos: Point {
                 x: self.pos.x - self.size.w,
                 y: self.pos.y - self.size.h,
            },
            size: self.size * 2.0,
        }
    }

    /// Compute whether or not the point is inside the rectangle.
    #[inline(always)]
    pub fn contains<T: Into<Point>>(&self, point: T) -> bool {
        let point: Point = point.into();
        self.left() < point.x && point.x < self.right() &&
        self.top() < point.y && point.y < self.bottom()
    }

    /// Create a rectangle that circumscribes the given circle.
    pub fn new_circle<T: Into<Point>>(center: T, radius: Scalar) -> Rect {
        let center: Point = center.into();
        Rect {
            pos: Point {
                x: center.x - radius,
                y: center.y - radius,
            },
            size: Size {
                w: 2.0 * radius,
                h: 2.0 * radius,
            },
        }
    }

    /// Create a square rectangle with sides of length len and top left corner at pos.
    pub fn new_square<T: Into<Point>>(pos: T, len: Scalar) -> Rect {
        let pos: Point = pos.into();
        Rect {
            pos: pos,
            size: Size { w: len, h: len },
        }
    }

    /// Returns the position of the left side of the rectangle.
    pub fn left(&self) -> Scalar {
        self.pos.x
    }

    /// Computes a rectangle whose perimeter forms the inside edge of margin with size m for self.
    #[inline(always)]
    pub fn margin(self, m: Scalar) -> Rect {
        math::margin_rectangle(self.to_array(), m).into()
    }

    /// Computes a rectangle translated (slid) in the direction of the vector a distance relative
    /// to the size of the rectangle. For example, self.relative([1.0, 1.0]) returns a rectangle
    /// one rectangle to the right and down from the original.
    #[inline(always)]
    pub fn relative<T: Into<Point>>(self, v: T) -> Rect {
        let v: Point = v.into();
        Rect {
            pos: Point {
                x: self.pos.x + self.size.w * v.x,
                y: self.pos.y + self.size.h * v.y,
            },
            size: self.size,
        }
    }

    /// Returns the position of the right side of the rectangle.
    pub fn right(&self) -> Scalar {
        self.pos.x + self.size.w
    }

    /// Computes a scaled rectangle with the same position as self.
    pub fn scaled<T: Into<Size>>(self, v: T) -> Rect {
        let v: Size = v.into();
        Rect {
            pos: self.pos,
            size: self.size * v,
        }
    }

    /// Converts a rectangle to [x, y, w, h].
    pub fn to_array(self) -> [Scalar; 4] {
        [self.pos.x, self.pos.y, self.size.w, self.size.h]
    }

    /// Returns the position of the top side of the rectangle.
    pub fn top(&self) -> Scalar {
        self.pos.y
    }
}
