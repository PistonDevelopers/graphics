//! Contains types used in this library
use std::convert::From;
use std::ops::{ Add, Mul, Sub };

pub use math::{ Matrix2d, Scalar, Vec2d };

/// A circle.
#[derive(Clone, Copy, Debug)]
pub struct Circle {
    center: Point,
    radius: Scalar,
}

/// The dimensions of a shape.
#[derive(Clone, Copy, Debug)]
pub struct Dimensions {
    width: Scalar,
    height: Scalar,
}

impl From<Vec2d> for Dimensions {
    fn from(v: Vec2d) -> Dimensions {
        Dimensions { width: v[0], height: v[1] }
    }
}

impl Dimensions {
    /// Convert dimenions to a vector.
    pub fn to_vec(&self) -> Vec2d {
        [self.width, self.height]
    }
}

impl Mul<Vec2d> for Dimensions {
    type Output = Dimensions;

    fn mul(self, v: Vec2d) -> Dimensions {
        Dimensions { width: self.width * v[0], height: self.height * v[1] }
    }
}

impl Mul<Scalar> for Dimensions {
    type Output = Dimensions;

    fn mul(self, s: Scalar) -> Dimensions {
        Dimensions { width: self.width * s, height: self.height * s }
    }
}

/// A point in the Cartesian plane.
#[derive(Clone, Copy, Debug)]
pub struct Point {
    x: Scalar,
    y: Scalar,
}

impl Add<Scalar> for Point {
    type Output = Point;

    fn add(self, s: Scalar) -> Point {
        Point { x: self.x + s, y: self.y + s }
    }
}

impl Add<Vec2d> for Point {
    type Output = Point;

    fn add(self, v: Vec2d) -> Point {
        Point { x: self.x + v[0], y: self.y + v[1] }
    }
}

impl From<Vec2d> for Point {
    fn from(v: Vec2d) -> Point {
        Point { x: v[0], y: v[1] }
    }
}

impl Sub<Scalar> for Point {
    type Output = Point;

    fn sub(self, s: Scalar) -> Point {
        Point { x: self.x - s, y: self.y - s }
    }
}

impl Sub<Vec2d> for Point {
    type Output = Point;

    fn sub(self, v: Vec2d) -> Point {
        Point { x: self.x - v[0], y: self.y - v[1] }
    }
}

impl Point {
    /// Convert the point to a vector.
    pub fn to_vec(&self) -> Vec2d {
        [self.x, self.y]
    }
}

/// A rectangle whose top left corner is at (x, y).
#[derive(Clone, Copy, Debug)]
pub struct Rectangle {
    pos: Point,
    dim: Dimensions,
}

impl From<Circle> for Rectangle {
    /// Convert from a circle to a rectangle.
    fn from(c: Circle) -> Rectangle {
        Rectangle {
            pos: Point {
                x: c.center.x - c.radius,
                y: c.center.y - c.radius,
            },
            dim: Dimensions {
                width: 2.0 * c.radius,
                height: 2.0 * c.radius,
            },
        }
    }
}

impl From<(Point, Dimensions)> for Rectangle {
    /// Creates a rectangle from the position of its top left corner and its dimensions.
    fn from(rectangle: (Point, Dimensions)) -> Rectangle {
        let (pos, dim) = rectangle;
        Rectangle { pos: pos, dim: dim }
    }
}

impl From<[Scalar; 4]> for Rectangle {
    /// Creates a rectangle from an array.
    fn from(v: [Scalar; 4]) -> Rectangle {
        Rectangle {
            pos: Point { x: v[0], y: v[1] },
            dim: Dimensions { width: v[2], height: v[3] },
        }
    }
}

impl From<Square> for Rectangle {
    /// Convert a square into a rectangle.
    fn from(s: Square) -> Rectangle {
        Rectangle {
            pos: s.pos,
            dim: Dimensions { width: s.len, height: s.len },
        }
    }
}

impl Rectangle {
    /// Returns the position of the bottom side of the rectangle.
    pub fn bottom(&self) -> Scalar {
        self.pos.y + self.dim.height
    }

    /// Computes a rectangle with quadruple the surface area of self and with center
    /// (self.x, self.y).
    pub fn centered(&self) -> Rectangle {
        Rectangle {
            pos: Point {
                 x: self.pos.x - self.dim.width,
                 y: self.pos.y - self.dim.height,
            },
            dim: self.dim * 2.0,
        }
    }

    /// Compute whether or not the point is inside the rectangle.
    #[inline]
    pub fn contains(&self, point: Point) -> bool {
        self.left() < point.x && point.x < self.right() &&
        self.top() < point.y && point.y < self.bottom()
    }

    /// Converts a rectangle into an array.
    pub fn into_vec(self) -> [Scalar; 4] {
        [self.pos.x, self.pos.y, self.dim.width, self.dim.height]
    }

    /// Returns the position of the left side of the rectangle.
    pub fn left(&self) -> Scalar {
        self.pos.x
    }

    /// Computes a rectangle whose perimeter forms the inside edge of margin with size m for self.
    #[inline(always)]
    pub fn margin(&self, m: Scalar) -> Rectangle {
        let w = self.dim.width - 2.0 * m;
        let h = self.dim.height - 2.0 * m;
        let (x, w)
            =   if w < 0.0 {
                    (self.pos.x + 0.5 * self.dim.width, 0.0)
                } else {
                    (self.pos.x + m, w)
                };
        let (y, h)
            =   if h < 0.0 {
                    (self.pos.y + 0.5 * self.dim.height, 0.0)
                } else {
                    (self.pos.y + m, h)
                };

        Rectangle {
            pos: Point { x: x, y: y },
            dim: Dimensions { width: w, height: h },
        }
    }

    /// Computes a rectangle translated (slid) in the direction of the vector a distance relative
    /// to the size of the rectangle. For example, self.relative([1.0, 1.0]) returns a rectangle
    /// one rectangle to the right and down from the original.
    #[inline(always)]
    pub fn relative(&self, v: Vec2d) -> Rectangle {
        Rectangle {
            pos: self.pos + (self.dim * v).to_vec(),
            dim: self.dim,
        }
    }

    /// Returns the position of the right side of the rectangle.
    pub fn right(&self) -> Scalar {
        self.pos.x + self.dim.width
    }

    /// Computes a scaled rectangle with the same position as self.
    pub fn scaled(&self, v: Vec2d) -> Rectangle {
        Rectangle {
            pos: self.pos,
            dim: self.dim * v,
        }
    }

    /// Converts a rectangle into a Vec4<Scalar> representation.
    pub fn to_vec(&self) -> [Scalar; 4] {
        [self.pos.x, self.pos.y, self.dim.width, self.dim.height]
    }

    /// Returns the position of the top side of the rectangle.
    pub fn top(&self) -> Scalar {
        self.pos.y
    }
}

/// A square.
#[derive(Clone, Copy, Debug)]
pub struct Square {
    pos: Point,
    len: Scalar,
}
