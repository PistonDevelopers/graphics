
use types;
use shapes::Point;
use math::Scalar;

/// A line.
#[derive(Copy, Clone, Debug)]
pub struct Line {
    /// Start point.
    pub start: Point,
    /// End point.
    pub end: Point,
}

impl From<Line> for types::Line {
    fn from(line: Line) -> types::Line {
        [line.start.x, line.start.y, line.end.x, line.end.y]
    }
}

impl From<types::Line> for Line {
    fn from(line: types::Line) -> Line {
        Line {
            start: (line[0], line[1]).into(),
            end: (line[2], line[3]).into()
        }
    }
}

impl<T: Into<Point>, U: Into<Point>> From<(T, U)> for Line {
    fn from((start, end): (T, U)) -> Line {
        Line {
            start: start.into(),
            end: end.into(),
        }
    }
}

impl From<Line> for (Point, Point) {
    fn from(line: Line) -> (Point, Point) {
        (line.start, line.end)
    }
}

impl From<(Scalar, Scalar, Scalar, Scalar)> for Line {
    fn from((ax, ay, bx, by): (Scalar, Scalar, Scalar, Scalar)) -> Line {
        Line {
            start: (ax, ay).into(),
            end: (bx, by).into(),
        }
    }
}

impl<T: Copy + Into<Point>> From<[T; 2]> for Line {
    fn from(line: [T; 2]) -> Line {
        Line {
            start: line[0].into(),
            end: line[1].into(),
        }
    }
}
