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

/// The line border width
pub struct Width(pub internal::Width);

/// Wrapper trait for `Get<Width>`
pub trait GetWidth: Get<Width> {
    /// Get width
    #[inline(always)]
    fn get_width(&self) -> Width {
        self.get()
    }
}

impl<T: Get<Width>> GetWidth for T {}

/// Wrapper trait for `Set<Width>`
pub trait SetWidth: Set<Width> {
    /// Set width
    #[inline(always)]
    fn set_width(&mut self, val: Width) {
        self.set_mut(val);
    }
}

impl<T: Set<Width>> SetWidth for T {}

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
    #[inline(always)]
    fn modify(self, l: &mut Line) {
        let Color(val) = self;
        l.color = val;
    }
}

impl Get<Color> for Line {
    #[inline(always)]
    fn get(&self) -> Color {
        Color(self.color)
    }
}

impl Modifier<Line> for Radius {
    #[inline(always)]
    fn modify(self, l: &mut Line) {
        let Radius(val) = self;
        l.radius = val;
    }
}

impl Get<Radius> for Line {
    #[inline(always)]
    fn get(&self) -> Radius {
        Radius(self.radius)
    }
}

impl Modifier<Line> for Width {
    #[inline(always)]
    fn modify(self, l: &mut Line) {
        let Width(val) = self;
        l.radius = 0.5 * val;
    }
}

impl Get<Width> for Line {
    #[inline(always)]
    fn get(&self) -> Width {
        Width(2.0 * self.radius)
    }
}

impl Modifier<Line> for Shape {
    #[inline(always)]
    fn modify(self, l: &mut Line) {
        l.shape = self;
    }
}

impl Get<Shape> for Line {
    #[inline(always)]
    fn get(&self) -> Shape {
        self.shape
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
