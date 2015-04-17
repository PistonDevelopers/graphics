
use types::Point;
use math::Scalar;

/// Represents a triangle.
pub struct Triangle<S = Scalar> {
    /// First corner of the triangle.
    pub a: Point<S>,
    /// Second corner of the triangle.
    pub b: Point<S>,
    /// Third corner of the triangle.
    pub c: Point<S>,
}

impl<S, T: Copy + Into<Point<S>>> From<[T; 3]> for Triangle<S> {
    fn from(triangle: [T; 3]) -> Triangle<S> {
        Triangle {
            a: triangle[0].into(),
            b: triangle[1].into(),
            c: triangle[2].into(),
        }
    }
}

impl<S> From<(S, S, S, S, S, S)> for Triangle<S> {
    fn from((ax, ay, bx, by, cx, cy): (S, S, S, S, S, S)) -> Triangle<S> {
        Triangle {
            a: Point { x: ax, y: ay },
            b: Point { x: bx, y: by },
            c: Point { x: cx, y: cy },
        }
    }
}

impl<S> Triangle<S> {
    /// Returns the triangle as a tuple.
    pub fn to_tuple(self) -> (S, S, S, S, S, S) {
        (self.a.x, self.a.y, self.b.x, self.b.y, self.c.x, self.c.y)
    }
}
