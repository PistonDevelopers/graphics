use math::Scalar;
use types::Point;

/// Represents a line.
#[derive(Clone, Copy, Debug)]
pub struct Line {
    /// Start point of line.
    pub s: Point,
    /// End point of line.
    pub e: Point,
}

impl From<[Scalar; 4]> for Line {
    fn from(line: [Scalar; 4]) -> Line {
        Line {
            s: Point { x: line[0], y: line[1] },
            e: Point { x: line[2], y: line[3] },
        }
    }
}

impl From<(Scalar, Scalar, Scalar, Scalar)> for Line {
    fn from((x1, y1, x2, y2): (Scalar, Scalar, Scalar, Scalar)) -> Line {
        Line {
            s: Point { x: x1, y: y1 },
            e: Point { x: x2, y: y2 },
        }
    }
}

impl<T: Copy + Into<Point>> From<[T; 2]> for Line {
    fn from(line: [T; 2]) -> Line {
        Line {
            s: line[0].into(),
            e: line[1].into(),
        }
    }
}

impl<T: Into<Point>, U: Into<Point>> From<(T, U)> for Line {
    fn from((s, e): (T, U)) -> Line {
        Line {
            s: s.into(),
            e: e.into(),
        }
    }
}
