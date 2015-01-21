//! Draw Line

use quack::{ GetFrom, SetAt, Me };
use internal;
use triangulation;
use BackEnd;
use Context;
use Color;

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
#[derive(Copy)]
pub struct Radius(pub internal::Radius);

/// The line border width
#[derive(Copy)]
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
    pub fn draw<B>(
        &self, 
        line: internal::Line,
        c: &Context, 
        back_end: &mut B
    )
        where B: BackEnd
    {
        // Complete transparency does not need to be rendered.
        if self.color[3] == 0.0 { return; }
        back_end.color(self.color);
        match self.shape {
            Shape::Square => {
                triangulation::with_round_border_line_tri_list(
                    2,
                    c.transform,
                    line,
                    self.radius,
                    |vertices| back_end.tri_list(vertices)
                );
            }
            Shape::Round => {
                triangulation::with_round_border_line_tri_list(
                    64,
                    c.transform,
                    line,
                    self.radius,
                    |vertices| back_end.tri_list(vertices)
                );
            }
            Shape::Bevel => {
                triangulation::with_round_border_line_tri_list(
                    3,
                    c.transform,
                    line,
                    self.radius,
                    |vertices| back_end.tri_list(vertices)
                );
            }
        }
    }

    /// Draws an arrow
    ///
    /// Head size is the sides of the triangle
    /// between the arrow hooks and the line
    pub fn draw_arrow<B>(
        &self,
        line: internal::Line,
        head_size: internal::Scalar,
        c: &Context,
        back_end: &mut B
    )
        where B: BackEnd
    {
        use RelativeTransform;

        self.draw(line, c, back_end);
        let diff = [line[2] - line[0], line[3] - line[1]];
        let arrow_head = c.trans(line[2], line[3]).orient(diff[0], diff[1]);
        self.draw([-head_size, head_size, 0.0, 0.0], &arrow_head, back_end);
        self.draw([-head_size, -head_size, 0.0, 0.0], &arrow_head, back_end);
    }
}

impl SetAt for (Color, Line) {
    type Property = Color;
    type Object = Line;

    #[inline(always)]
    fn set_at(_: Me<Self>, Color(val): Color, l: &mut Line) {
        l.color = val;
    }
}

impl GetFrom for (Color, Line) {
    type Property = Color;
    type Object = Line;

    #[inline(always)]
    fn get_from(_: Me<Self>, obj: &Line) -> Color {
        Color(obj.color)
    }
}

impl SetAt for (Radius, Line) {
    type Property = Radius;
    type Object = Line;

    #[inline(always)]
    fn set_at(_: Me<Self>, Radius(val): Radius, l: &mut Line) {
        l.radius = val;
    }
}

impl GetFrom for (Radius, Line) {
    type Property = Radius;
    type Object = Line;

    #[inline(always)]
    fn get_from(_: Me<Self>, obj: &Line) -> Radius {
        Radius(obj.radius)
    }
}

impl SetAt for (Width, Line) {
    type Property = Width;
    type Object = Line;

    #[inline(always)]
    fn set_at(_: Me<Self>, Width(val): Width, l: &mut Line) {
        l.radius = 0.5 * val;
    }
}

impl GetFrom for (Width, Line) {
    type Property = Width;
    type Object = Line;

    #[inline(always)]
    fn get_from(_: Me<Self>, obj: &Line) -> Width {
        Width(2.0 * obj.radius)
    }
}

impl SetAt for (Shape, Line) {
    type Property = Shape;
    type Object = Line;

    #[inline(always)]
    fn set_at(_: Me<Self>, val: Shape, l: &mut Line) {
        l.shape = val;
    }
}

impl GetFrom for (Shape, Line) {
    type Property = Shape;
    type Object = Line;

    #[inline(always)]
    fn get_from(_: Me<Self>, obj: &Line) -> Shape {
        obj.shape
    }
}

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

