//! Draw Line

use internal;
use triangulation;
use Graphics;
use Color;
use vecmath::Matrix2d;
use DrawState;

/// The shape of the line
#[derive(Copy, Clone)]
pub enum Shape {
    /// Square edges
    Square,
    /// Round edges
    Round,
    /// Bevel edges
    Bevel,
}

/// The line border radius
#[derive(Copy, Clone)]
pub struct Radius(pub internal::Radius);

/// The line border width
#[derive(Copy, Clone)]
pub struct Width(pub internal::Width);

/// A colored line with a default border radius
#[derive(Copy, Clone)]
pub struct Line {
    /// The line color
    pub color: internal::Color,
    /// The line radius
    pub radius: internal::Radius,
    /// The line shape
    pub shape: Shape,
}

impl Line {
    /// Creates a new line
    pub fn new(color: internal::Color, radius: internal::Radius) -> Line {
        Line {
            color: color,
            radius: radius,
            shape: Shape::Square,
        }
    }

    /// Creates a new line
    pub fn round(color: internal::Color, radius: internal::Radius) -> Line {
        Line {
            color: color,
            radius: radius,
            shape: Shape::Round,
        }
    }

    /// Draw the line.
    pub fn draw<G>(
        &self,
        line: internal::Line,
        draw_state: &DrawState,
        transform: Matrix2d,
        g: &mut G
    )
        where G: Graphics
    {
        match self.shape {
            Shape::Square => {
                g.tri_list(
                    draw_state,
                    &self.color,
                    |f|
                triangulation::with_round_border_line_tri_list(
                    2,
                    transform,
                    line,
                    self.radius,
                    |vertices| f(vertices)
                ));
            }
            Shape::Round => {
                g.tri_list(
                    draw_state,
                    &self.color,
                    |f|
                triangulation::with_round_border_line_tri_list(
                    64,
                    transform,
                    line,
                    self.radius,
                    |vertices| f(vertices)
                ));
            }
            Shape::Bevel => {
                g.tri_list(
                    draw_state,
                    &self.color,
                    |f|
                triangulation::with_round_border_line_tri_list(
                    3,
                    transform,
                    line,
                    self.radius,
                    |vertices| f(vertices)
                ));
            }
        }
    }

    /// Draws an arrow
    ///
    /// Head size is the sides of the triangle
    /// between the arrow hooks and the line
    pub fn draw_arrow<G>(
        &self,
        line: internal::Line,
        head_size: internal::Scalar,
        draw_state: &DrawState,
        transform: Matrix2d,
        g: &mut G
    )
        where G: Graphics
    {
        use Transformed;

        self.draw(line, draw_state, transform, g);
        let diff = [line[2] - line[0], line[3] - line[1]];
        let arrow_head = transform
            .trans(line[2], line[3])
            .orient(diff[0], diff[1]);
        self.draw([-head_size, head_size, 0.0, 0.0], draw_state, arrow_head, g);
        self.draw([-head_size, -head_size, 0.0, 0.0], draw_state, arrow_head, g);
    }
}

/*
quack! {
    l: Line[]
    get:
        fn () -> Color [] { Color(l.color) }
        fn () -> Radius [] { Radius(l.radius) }
        fn () -> Width [] { Width(2.0 * l.radius) }
        fn () -> Shape [] { l.shape }
    set:
        fn (val: Color) [] { l.color = val.0 }
        fn (val: Radius) [] { l.radius = val.0 }
        fn (val: Width) [] { l.radius = 0.5 * val.0 }
        fn (val: Shape) [] { l.shape = val }
    action:
}
*/

#[cfg(test)]
mod test {
    use quack::{ Get, Set };
    use super::Line;
    use super::Shape;
    use super::Radius;
    use Color;

    #[test]
    fn test_line() {
        use RelativeColor;

        let _line = Line::new([0.0; 4], 3.0)
            .set(Color([1.0; 4]))
            .set(Radius(3.0))
            .set(Shape::Round)
            .hue_deg(1.0);
        let Color(_) = _line.get();
    }
}
