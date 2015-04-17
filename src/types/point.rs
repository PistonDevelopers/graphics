use std::ops::{ Add, Sub };

use math::{ Scalar, Vec2d };

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
