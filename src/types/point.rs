use std::ops::{ Add, Sub };

use math::{ Scalar, Vec2d };

/// A point in the Cartesian plane.
#[derive(Clone, Copy, Debug)]
pub struct Point<S = Scalar> {
    /// The x coordinate.
    pub x: S,
    /// The y coordinate.
    pub y: S,
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

impl From<Point<f32>> for Point {
    fn from(point: Point<f32>) -> Point {
        Point { x: point.x as Scalar, y: point.y as Scalar }
    }
}

impl From<Vec2d> for Point {
    fn from(v: Vec2d) -> Point {
        Point { x: v[0], y: v[1] }
    }
}

impl From<[f32; 2]> for Point {
    fn from(v: [f32; 2]) -> Point {
        Point { x: v[0] as Scalar, y: v[1] as Scalar }
    }
}

impl From<(Scalar, Scalar)> for Point {
    fn from((x, y): (Scalar, Scalar)) -> Point {
        Point { x: x, y: y }
    }
}

impl From<(f32, f32)> for Point {
    fn from((x, y): (f32, f32)) -> Point {
        Point { x: x as f64, y: y as f64 }
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

    /// Computes length from origin.
    pub fn len(&self) -> Scalar {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}
