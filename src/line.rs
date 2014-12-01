//! Draw Line

use current::{ Get, Modifier, Set };
use internal;
use triangulation;
use BackEnd;
use Context;
use ImageSize;
use Color;

/// The shape of the line
pub enum Shape {
    /// Square edges
    Square,
    /// Round edges
    Round,
    /// Bevel edges
    Bevel,
}

/// Wrapper trait for `Get<Shape>`
pub trait GetShape: Get<Shape> {
    /// Get shape
    #[inline(always)]
    fn get_shape(&self) -> Shape {
        self.get()
    }
}

impl<T: Get<Shape>> GetShape for T {}

/// Wrapper trait for `Set<Shape>`
pub trait SetShape: Set<Shape> {
    /// Set shape
    #[inline(always)]
    fn set_shape(&mut self, val: Shape) {
        self.set_mut(val);
    }
}

impl<T: Set<Shape>> SetShape for T {}

/// The line border radius
pub struct Radius(pub internal::Radius);

/// Wrapper trait for `Get<Radius>`
pub trait GetRadius: Get<Radius> {
    /// Get radius
    #[inline(always)]
    fn get_radius(&self) -> Radius {
        self.get()
    }
}

impl<T: Get<Radius>> GetRadius for T {}

/// Wrapper trait for `Set<Radius>`
pub trait SetRadius: Set<Radius> {
    /// Set radius
    #[inline(always)]
    fn set_radius(&mut self, val: Radius) {
        self.set_mut(val);
    }
}

impl<T: Set<Radius>> SetRadius for T {}

/// A colored line with a default border radius
#[deriving(Copy)]
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

    /// Draw the line.
    pub fn draw<B: BackEnd<I>, I: ImageSize>(
        &self, 
        line: internal::Line,
        c: &Context, 
        back_end: &mut B
    ) {
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
}

impl Modifier<Line> for Color {
    fn modify(self, l: &mut Line) {
        let Color(val) = self;
        l.color = val;
    }
}

impl Modifier<Line> for Radius {
    fn modify(self, l: &mut Line) {
        let Radius(val) = self;
        l.radius = val;
    }
}

impl Modifier<Line> for Shape {
    fn modify(self, l: &mut Line) {
        l.shape = self;
    }
}

#[cfg(test)]
mod test {
    use current::Set;
    use super::Line;
    use super::Shape;
    use super::Radius;
    use Color;

    #[test]
    fn test_line() {
        let _line = Line::new([0.0, ..4], 3.0)
            .set(Color([1.0, ..4]))
            .set(Radius(3.0))
            .set(Shape::Round);
    }
}
