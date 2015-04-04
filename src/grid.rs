//! A flat grid with square cells.

use math::{ Matrix2d, Scalar };
use { DrawState, Graphics, Line };

/// Represents a flat grid with square cells.
#[derive(Copy, Clone)]
pub struct Grid {
    /// Number of columns.
    pub cols: u32,
    /// Number of rows.
    pub rows: u32,
    /// The width and height of each grid cell.
    pub units: Scalar,
}

/// Iterates through the cells of a grid as (u32, u32).
#[derive(Copy, Clone)]
pub struct GridCells {
    cols: u32,
    rows: u32,
    state: u64,
}

impl Grid {
    /// Draws the grid.
    pub fn draw<G>(
        &self,
        line: &Line,
        draw_state: &DrawState,
        transform: Matrix2d,
        g: &mut G
    )
        where G: Graphics
    {
        let &Grid {
            cols, rows, units
        } = self;
        for x in 0..cols + 1 {
            let x1 = x as Scalar * units;
            let y1 = 0.0;
            let x2 = x1;
            let y2 = rows as Scalar * units;
            line.draw([x1, y1, x2, y2], draw_state, transform, g);
        }
        for y in 0..rows + 1 {
            let x1 = 0.0;
            let y1 = y as Scalar * units;
            let x2 = cols as Scalar * units;
            let y2 = y1;
            line.draw([x1, y1, x2, y2], draw_state, transform, g);
        }
    }

    /// Get a GridIterator for the grid
    pub fn cells(&self) -> GridCells {
        GridCells {
            cols: self.cols,
            rows: self.rows,
            state: 0,
        }
    }
}

impl Iterator for GridCells {
    type Item = (u32, u32);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_iterator() {
        let g: Grid = Grid {cols: 2, rows: 2, units: 2.0};
        let expected: Vec<(u32, u32)> = vec![(0, 0), (1, 0), (0, 1), (1, 1)];
        let cells: Vec<(u32, u32)> = g.cells().collect();
        assert_eq!(expected, cells);
    }
}
