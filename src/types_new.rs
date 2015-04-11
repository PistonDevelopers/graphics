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

/// The size of a shape.
#[derive(Clone, Copy, Debug)]
pub struct Size {
    width: Scalar,
    height: Scalar,
}

impl From<Vec2d> for Size {
    fn from(v: Vec2d) -> Size {
        Size { width: v[0], height: v[1] }
    }
}

impl Size {
    /// Convert size to a vector.
    pub fn to_vec2d(&self) -> Vec2d {
        [self.width, self.height]
    }
}

impl Mul<Vec2d> for Size {
    type Output = Size;

    fn mul(self, v: Vec2d) -> Size {
        Size { width: self.width * v[0], height: self.height * v[1] }
    }
}

impl Mul<Scalar> for Size {
    type Output = Size;

    fn mul(self, s: Scalar) -> Size {
        Size { width: self.width * s, height: self.height * s }
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
    pub fn to_vec2d(&self) -> Vec2d {
        [self.x, self.y]
    }
}

/// A rectangle whose top left corner is at pos.
#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pos: Point,
    size: Size,
}

impl From<Circle> for Rect {
    /// Convert from a circle to a rectangle.
    fn from(c: Circle) -> Rect {
        Rect {
            pos: Point {
                x: c.center.x - c.radius,
                y: c.center.y - c.radius,
            },
            size: Size {
                width: 2.0 * c.radius,
                height: 2.0 * c.radius,
            },
        }
    }
}

impl From<(Point, Size)> for Rect {
    /// Creates a rectangle from the position of its top left corner and its size.
    fn from(rectangle: (Point, Size)) -> Rect {
        let (pos, size) = rectangle;
        Rect { pos: pos, size: size }
    }
}

impl From<[Scalar; 4]> for Rect {
    /// Creates a rectangle from an array.
    fn from(v: [Scalar; 4]) -> Rect {
        Rect {
            pos: Point { x: v[0], y: v[1] },
            size: Size { width: v[2], height: v[3] },
        }
    }
}

impl From<Square> for Rect {
    /// Convert a square into a rectangle.
    fn from(s: Square) -> Rect {
        Rect {
            pos: s.pos,
            size: Size { width: s.len, height: s.len },
        }
    }
}

impl Rect {
    /// Returns the position of the bottom side of the rectangle.
    pub fn bottom(&self) -> Scalar {
        self.pos.y + self.size.height
    }

    /// Computes a rectangle with quadruple the surface area of self and with center
    /// (self.x, self.y).
    pub fn centered(&self) -> Rect {
        Rect {
            pos: Point {
                 x: self.pos.x - self.size.width,
                 y: self.pos.y - self.size.height,
            },
            size: self.size * 2.0,
        }
    }

    /// Compute whether or not the point is inside the rectangle.
    #[inline]
    pub fn contains(&self, point: Point) -> bool {
        self.left() < point.x && point.x < self.right() &&
        self.top() < point.y && point.y < self.bottom()
    }

    /// Converts a rectangle into [x, y, w, h].
    pub fn into_array(self) -> [Scalar; 4] {
        [self.pos.x, self.pos.y, self.size.width, self.size.height]
    }

    /// Returns the position of the left side of the rectangle.
    pub fn left(&self) -> Scalar {
        self.pos.x
    }

    /// Computes a rectangle whose perimeter forms the inside edge of margin with size m for self.
    #[inline(always)]
    pub fn margin(&self, m: Scalar) -> Rect {
        let w = self.size.width - 2.0 * m;
        let h = self.size.height - 2.0 * m;
        let (x, w)
            =   if w < 0.0 {
                    (self.pos.x + 0.5 * self.size.width, 0.0)
                } else {
                    (self.pos.x + m, w)
                };
        let (y, h)
            =   if h < 0.0 {
                    (self.pos.y + 0.5 * self.size.height, 0.0)
                } else {
                    (self.pos.y + m, h)
                };

        Rect {
            pos: Point { x: x, y: y },
            size: Size { width: w, height: h },
        }
    }

    /// Computes a rectangle translated (slid) in the direction of the vector a distance relative
    /// to the size of the rectangle. For example, self.relative([1.0, 1.0]) returns a rectangle
    /// one rectangle to the right and down from the original.
    #[inline(always)]
    pub fn relative(&self, v: Vec2d) -> Rect {
        Rect {
            pos: self.pos + (self.size * v).to_vec2d(),
            size: self.size,
        }
    }

    /// Returns the position of the right side of the rectangle.
    pub fn right(&self) -> Scalar {
        self.pos.x + self.size.width
    }

    /// Computes a scaled rectangle with the same position as self.
    pub fn scaled(&self, v: Vec2d) -> Rect {
        Rect {
            pos: self.pos,
            size: self.size * v,
        }
    }

    /// Converts a rectangle to [x, y, w, h].
    pub fn to_array(&self) -> [Scalar; 4] {
        [self.pos.x, self.pos.y, self.size.width, self.size.height]
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
