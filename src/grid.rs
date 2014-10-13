//! A flat grid with square cells.

use vecmath::Scalar;
use { 
    AddLine,
    AddSquareBorder,
    BackEnd,
    ColorContext,
    Draw,
    ImageSize,
};

/// Represents a flat grid with square cells.
pub struct Grid {
    /// Number of columns.
    pub cols: u32,
    /// Number of rows.
    pub rows: u32,
    /// The width and height of each grid cell.
    pub units: Scalar,
    /// The line radius.
    pub radius: Scalar,
}

impl Grid {
    /// Draws the grid.
    pub fn draw<B: BackEnd<I>, I: ImageSize>(
        &self,
        c: &ColorContext,
        g: &mut B
    ) {
        let &Grid {
            cols, rows, units, radius
        } = self;
        for x in range(0, cols + 1) {
            let x1 = x as Scalar * units;
            let y1 = 0.0;
            let x2 = x1;
            let y2 = rows as Scalar * units;
            c.line(x1, y1, x2, y2).square_border_radius(radius).draw(g);
        }
        for y in range(0, rows + 1) {
            let x1 = 0.0;
            let y1 = y as Scalar * units;
            let x2 = cols as Scalar * units;
            let y2 = y1;
            c.line(x1, y1, x2, y2).square_border_radius(radius).draw(g);
        }
    }
}

