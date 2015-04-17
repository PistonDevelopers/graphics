use std::ops::{ Add, Mul, Sub };

use math::Scalar;

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

impl Add<f32> for Point<f32> {
    type Output = Point<f32>;

    fn add(self, s: f32) -> Point<f32> {
        Point { x: self.x + s, y: self.y + s }
    }
}

impl<S: Add<S, Output = S>, T: Into<Point<S>>> Add<T> for Point<S> {
    type Output = Point<S>;

    fn add(self, v: T) -> Point<S> {
        let v: Point<S> = v.into();
        Point { x: self.x + v.x, y: self.y + v.y }
    }
}

impl From<Point<f32>> for Point {
    fn from(point: Point<f32>) -> Point {
        Point { x: point.x as Scalar, y: point.y as Scalar }
    }
}

impl<S: Copy> From<[S; 2]> for Point<S> {
    fn from(v: [S; 2]) -> Point<S> {
        Point { x: v[0], y: v[1] }
    }
}

impl From<[f32; 2]> for Point {
    fn from(v: [f32; 2]) -> Point {
        Point { x: v[0] as Scalar, y: v[1] as Scalar }
    }
}

impl<S> From<(S, S)> for Point<S> {
    fn from((x, y): (S, S)) -> Point<S> {
        Point { x: x, y: y }
    }
}

impl From<(f32, f32)> for Point {
    fn from((x, y): (f32, f32)) -> Point {
        Point { x: x as Scalar, y: y as Scalar }
    }
}

impl Sub<Scalar> for Point {
    type Output = Point;

    fn sub(self, s: Scalar) -> Point {
        Point { x: self.x - s, y: self.y - s }
    }
}

impl Sub<f32> for Point<f32> {
    type Output = Point<f32>;

    fn sub(self, s: f32) -> Point<f32> {
        Point { x: self.x - s, y: self.y - s }
    }
}

impl<S: Sub<S, Output = S>, T: Into<Point<S>>> Sub<T> for Point<S> {
    type Output = Point<S>;

    fn sub(self, v: T) -> Point<S> {
        let v = v.into();
        Point { x: self.x - v.x, y: self.y - v.y }
    }
}

impl<S: Mul<S, Output = S>, T: Into<Point<S>>> Mul<T> for Point<S> {
    type Output = Point<S>;

    fn mul(self, v: T) -> Point<S> {
        let v = v.into();
        Point { x: self.x * v.x, y: self.y * v.y }
    }
}

impl<S> Point<S> {
    /// Convert the point to an array.
    pub fn to_array(self) -> [S; 2] {
        [self.x, self.y]
    }
}

impl Point {
    /// Computes length from origin.
    pub fn len(&self) -> Scalar {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl Point<f32> {
    /// Computes length from origin.
    pub fn len(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}
