//! A flat grid with square cells.

use crate::{
    math::{Matrix2d, Scalar, Vec2d},
    DrawState, Graphics, Line,
};

/// Represents a flat grid with square cells.
#[derive(Debug, Copy, Clone)]
pub struct Grid {
    /// Number of columns.
    pub cols: u32,
    /// Number of rows.
    pub rows: u32,
    /// The width and height of each grid cell.
    pub units: Scalar,
}

/// Iterates through the cells of a grid as (u32, u32).
#[derive(Debug, Copy, Clone)]
pub struct GridCells {
    cols: u32,
    rows: u32,
    state: u64,
}

impl Grid {
    /// Draws the grid.
    pub fn draw<G>(&self, line: &Line, draw_state: &DrawState, transform: Matrix2d, g: &mut G)
    where
        G: Graphics,
    {
        let &Grid { cols, rows, units } = self;
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

    /// Get on-screen position of a grid cell
    pub fn cell_position(&self, cell: (u32, u32)) -> Vec2d {
        [
            cell.0 as Scalar * &self.units,
            cell.1 as Scalar * &self.units,
        ]
    }

    /// Get on-screen x position of a grid cell
    pub fn x_pos(&self, cell: (u32, u32)) -> Scalar {
        self.cell_position(cell)[0]
    }

    /// Get on-screen y position of a grid cell
    pub fn y_pos(&self, cell: (u32, u32)) -> Scalar {
        self.cell_position(cell)[1]
    }
}

impl Iterator for GridCells {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<(u32, u32)> {
        let cols = self.cols as u64;
        let rows = self.rows as u64;

        if self.state == cols * rows {
            return None;
        }

        // reverse of: state = x + (y * cols)
        let ret = ((self.state % cols) as u32, (self.state / cols) as u32);
        self.state += 1;

        return Some(ret);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_iterator() {
        let combinations = vec![(2, 2), (2, 3), (3, 2)];

        for (cols, rows) in combinations {
            let grid = Grid {
                cols,
                rows,
                units: 2.0,
            };
            println!("Testing {:?}", grid);

            let mut iter = grid.cells();
            for y in 0..rows {
                for x in 0..cols {
                    assert_eq!(iter.next(), Some((x, y)));
                    println!("Got: {:?}", (x, y));
                }
            }

            assert_eq!(iter.next(), None);
        }
    }

    #[test]
    fn test_cell_positions() {
        let g: Grid = Grid {
            cols: 2,
            rows: 3,
            units: 2.0,
        };
        assert_eq!(4.0, g.x_pos((2, 3)));
        assert_eq!(6.0, g.y_pos((2, 3)));
    }
}
