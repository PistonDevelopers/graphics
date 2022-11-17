//! Draw Line

use crate::{
    math::{Matrix2d, Scalar},
    triangulation, types,
    types::{Color, Radius, Resolution},
    DrawState, Graphics,
};

/// The shape of the line
#[derive(Copy, Clone)]
pub enum Shape {
    /// Square edges
    Square,
    /// Round edges
    Round,
    /// Round edges with specified resolution
    RoundWithResolution(Resolution),
    /// Bevel edges
    Bevel,
}

impl Shape {
    /// Gets the round resolution of the shape.
    fn resolution(&self) -> u32 {
        use Shape::*;

        match self {
            Square => 2,
            Bevel => 3,
            Round => 64,
            RoundWithResolution(n) => *n,
        }
    }
}

/// A colored line with a default border radius
#[derive(Copy, Clone)]
pub struct Line {
    /// The line color
    pub color: Color,
    /// The line radius
    pub radius: Radius,
    /// The line shape
    pub shape: Shape,
}

impl Line {
    /// Creates a new line
    pub fn new(color: Color, radius: Radius) -> Line {
        Line {
            color,
            radius,
            shape: Shape::Square,
        }
    }

    /// Creates a new line
    pub fn new_round(color: Color, radius: Radius) -> Line {
        Line {
            color,
            radius,
            shape: Shape::Round,
        }
    }

    /// Sets color.
    pub fn color(mut self, value: Color) -> Self {
        self.color = value;
        self
    }

    /// Sets radius.
    pub fn radius(mut self, value: Radius) -> Self {
        self.radius = value;
        self
    }

    /// Sets width.
    pub fn width(mut self, value: types::Width) -> Self {
        self.radius = 0.5 * value;
        self
    }

    /// Sets shape.
    pub fn shape(mut self, value: Shape) -> Self {
        self.shape = value;
        self
    }

    /// Draws line using default method between points.
    #[inline(always)]
    pub fn draw_from_to<P: Into<types::Vec2d>, G>(
        &self,
        from: P,
        to: P,
        draw_state: &DrawState,
        transform: Matrix2d,
        g: &mut G,
    ) where
        G: Graphics,
    {
        let from: types::Vec2d = from.into();
        let to: types::Vec2d = to.into();
        g.line(
            self,
            [from[0], from[1], to[0], to[1]],
            draw_state,
            transform,
        );
    }

    /// Draws line using default method.
    #[inline(always)]
    pub fn draw<L: Into<types::Line>, G>(
        &self,
        line: L,
        draw_state: &DrawState,
        transform: Matrix2d,
        g: &mut G,
    ) where
        G: Graphics,
    {
        g.line(self, line, draw_state, transform);
    }

    /// Draws line using triangulation.
    pub fn draw_tri<L: Into<types::Line>, G>(
        &self,
        line: L,
        draw_state: &DrawState,
        transform: Matrix2d,
        g: &mut G,
    ) where
        G: Graphics,
    {
        let line = line.into();
        g.tri_list(draw_state, &self.color, |f| {
            triangulation::with_round_border_line_tri_list(
                self.shape.resolution(),
                transform,
                line,
                self.radius,
                |vertices| f(vertices),
            )
        });
    }

    /// Draws an arrow
    ///
    /// Head size is the sides of the triangle
    /// between the arrow hooks and the line
    pub fn draw_arrow<L: Into<types::Line>, G>(
        &self,
        line: L,
        head_size: Scalar,
        draw_state: &DrawState,
        transform: Matrix2d,
        g: &mut G,
    ) where
        G: Graphics,
    {
        use crate::Transformed;

        let line = line.into();
        self.draw(line, draw_state, transform, g);
        let diff = [line[2] - line[0], line[3] - line[1]];
        let arrow_head = transform.trans(line[2], line[3]).orient(diff[0], diff[1]);
        self.draw([-head_size, head_size, 0.0, 0.0], draw_state, arrow_head, g);
        self.draw(
            [-head_size, -head_size, 0.0, 0.0],
            draw_state,
            arrow_head,
            g,
        );
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_line() {
        use crate::Colored;

        let _line = Line::new([0.0; 4], 3.0)
            .color([1.0; 4])
            .radius(3.0)
            .shape(Shape::Round)
            .hue_deg(1.0);
    }
}
