//! A flat grid with square cells.

use vecmath::Scalar;
use {
    BackEnd,
    Context,
    ImageSize,
    Line,
};

/// Represents a flat grid with square cells.
pub struct Grid {
    /// Number of columns.
    pub cols: u32,
    /// Number of rows.
    pub rows: u32,
    /// The width and height of each grid cell.
    pub units: Scalar,
}

/// Iterates through the cells of a grid as (u32, u32).
pub struct GridIterator {
    cols: u32,
    rows: u32,
    state: u64,
}

impl Grid {
    /// Draws the grid.
    pub fn draw<B: BackEnd<I>, I: ImageSize>(
        &self,
        line: &Line,
        c: &Context,
        g: &mut B
    ) {
        let &Grid {
            cols, rows, units
        } = self;
        for x in range(0, cols + 1) {
            let x1 = x as Scalar * units;
            let y1 = 0.0;
            let x2 = x1;
            let y2 = rows as Scalar * units;
            line.draw([x1, y1, x2, y2], c, g);
        }
        for y in range(0, rows + 1) {
            let x1 = 0.0;
            let y1 = y as Scalar * units;
            let x2 = cols as Scalar * units;
            let y2 = y1;
            line.draw([x1, y1, x2, y2], c, g);
        }
    }

    /// Get a GridIterator for the grid
    pub fn cells(&self) -> GridIterator {
        GridIterator {
            cols: self.cols,
            rows: self.rows,
            state: 0,
        }
    }
}

impl Iterator<(u32, u32)> for GridIterator {

    fn next(&mut self) -> Option<(u32, u32)> {
        let cols = self.cols as u64;
        let rows = self.rows as u64;

        if self.state == cols * rows {
            return None
        }
        let ret = (
            (self.state % cols) as u32,
            (self.state / rows) as u32,
        );
        self.state += 1;

        return Some(ret);
    }

}

#[test]
fn test_grid_iterator() {
    let g = Grid {cols: 2, rows: 2, units: 2.0};
    let expected : Vec<(u32, u32)> = vec![(0, 0), (1, 0), (0, 1), (1, 1)];
    assert_eq!(expected, g.cells().collect());
}
