//! Contains types used in this library
use std::convert::From;
use std::ops::{ Add, Mul };

use types::{ Point, Size };

pub use math::{ self, Matrix2d, Scalar, Vec2d };

/// A rectangle.
#[derive(Clone, Copy, Debug)]
pub struct Rect<S = Scalar> {
    /// The position of the top left corner of the rectangle.
    pub pos: Point<S>,
    /// The width and height of the rectangle.
    pub size: Size<S>,
}

impl<P: Into<Point>, S: Into<Size>> From<(P, S)> for Rect {
    /// Creates a rectangle from the position of its top left corner and its size.
    fn from((pos, size): (P, S)) -> Rect {
        let (pos, size): (Point, Size) = (pos.into(), size.into());
        Rect { pos: pos, size: size }
    }
}

impl From<Rect<f32>> for Rect {
    fn from(rect: Rect<f32>) -> Rect {
        Rect {
            pos: rect.pos.into(),
            size: rect.size.into(),
        }
    }
}

impl<S: Copy> From<[S; 4]> for Rect<S> {
    /// Creates a rectangle from an array.
    fn from(v: [S; 4]) -> Rect<S> {
        Rect {
            pos: Point { x: v[0], y: v[1] },
            size: Size { w: v[2], h: v[3] },
        }
    }
}

impl From<[f32; 4]> for Rect {
    fn from(v: [f32; 4]) -> Rect {
        Rect {
            pos: Point { x: v[0] as Scalar, y: v[1] as Scalar },
            size: Size { w: v[2] as Scalar, h: v[3] as Scalar },
        }
    }
}

impl<S> From<(S, S, S, S)> for Rect<S> {
    fn from((x, y, w, h): (S, S, S, S)) -> Rect<S> {
        Rect {
            pos: Point { x: x, y: y },
            size: Size { w: w, h: h },
        }
    }
}

impl From<(f32, f32, f32, f32)> for Rect {
    fn from((x, y, w, h): (f32, f32, f32, f32)) -> Rect {
        Rect {
            pos: Point { x: x as Scalar, y: y as Scalar },
            size: Size { w: w as Scalar, h: h as Scalar },
        }
    }
}

impl Rect {
    /// Computes a rectangle whose perimeter forms the inside edge of margin with size m for self.
    #[inline(always)]
    pub fn margin(self, m: Scalar) -> Rect {
        math::margin_rectangle(self.to_array(), m).into()
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
}

impl<S: Copy> Rect<S> {
    /// Returns the position of the bottom side of the rectangle.
    pub fn bottom(&self) -> S
        where S: Add<S, Output = S>
    {
        self.pos.y + self.size.h
    }

    /// Create a square rectangle with sides of length len and top left corner at pos.
    pub fn new_square<T: Into<Point<S>>>(pos: T, len: S) -> Rect<S> {
        let pos: Point<S> = pos.into();
        Rect {
            pos: pos,
            size: Size { w: len, h: len },
        }
    }

    /// Returns the position of the left side of the rectangle.
    pub fn left(&self) -> S {
        self.pos.x
    }

    /// Computes a rectangle translated (slid) in the direction of the vector a distance relative
    /// to the size of the rectangle. For example, self.relative([1.0, 1.0]) returns a rectangle
    /// one rectangle to the right and down from the original.
    #[inline(always)]
    pub fn relative<T: Into<Point<S>>>(self, v: T) -> Rect<S>
        where S: Add<S, Output = S> + Mul<S, Output = S>
    {
        let v: Point<S> = v.into();
        Rect {
            pos: Point {
                x: self.pos.x + self.size.w * v.x,
                y: self.pos.y + self.size.h * v.y,
            },
            size: self.size,
        }
    }

    /// Returns the position of the right side of the rectangle.
    pub fn right(&self) -> S
        where S: Add<S, Output = S>
    {
        self.pos.x + self.size.w
    }

    /// Computes a scaled rectangle with the same position as self.
    pub fn scaled<T: Into<Size<S>>>(self, v: T) -> Rect<S>
        where S: Mul<S, Output = S>
    {
        Rect {
            pos: self.pos,
            size: self.size * v,
        }
    }

    /// Converts a rectangle to [x, y, w, h].
    pub fn to_array(self) -> [S; 4] {
        [self.pos.x, self.pos.y, self.size.w, self.size.h]
    }

    /// Returns the position of the top side of the rectangle.
    pub fn top(&self) -> S {
        self.pos.y
    }
}
