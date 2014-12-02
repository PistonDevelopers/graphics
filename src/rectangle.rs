//! Draw rectangle

use current::{ Get, Modifier, Set };
use internal;
use triangulation;
use Context;
use BackEnd;
use ImageSize;
use Color;

/// Use x, y, half-width, half-height
pub fn centered(rect: internal::Rectangle) -> internal::Rectangle {
    let [cx, cy, rw, rh] = rect;
    [cx - rw, cy - rh, 2.0 * rw, 2.0 * rh]
}

/// Use centered square
pub fn centered_square(
    x: internal::Scalar, 
    y: internal::Scalar, 
    radius: internal::Scalar
) -> internal::Rectangle {
    [x - radius, y - radius, 2.0 * radius, 2.0 * radius]
}

/// Use square with x, y in upper left corner
pub fn square(
    x: internal::Scalar, 
    y: internal::Scalar, 
    size: internal::Scalar
) -> internal::Rectangle {
    [x, y, size, size]
}

/// The shape of the rectangle
pub enum Shape {
    /// Square corners
    Square,
    /// Round corners
    Round(internal::Radius),
    /// Bevel corners
    Bevel(internal::Radius),
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

/// The border of the rectangle
pub struct Border {
    /// The color of the border
    pub color: internal::Color,
    /// The radius of the border
    pub radius: internal::Radius,
}

/// Wrapper trait for `Get<Border>`
pub trait GetBorder: Get<Border> {
    /// Get border
    #[inline(always)]
    fn get_border(&self) -> Border {
        self.get()
    }
}

impl<T: Get<Border>> GetBorder for T {}

/// Wrapper trait for `Set<Border>`
pub trait SetBorder: Set<Border> {
    /// Set border
    #[inline(always)]
    fn set_border(&mut self, val: Border) {
        self.set_mut(val);
    }
}

impl<T: Set<Border>> SetBorder for T {}

/// Maybe border property
pub struct MaybeBorder(pub Option<Border>);

/// Wrapper trait for `Get<MaybeBorder>`
pub trait GetMaybeBorder: Get<MaybeBorder> {
    /// Get maybe border
    #[inline(always)]
    fn get_maybe_border(&self) -> MaybeBorder {
        self.get()
    }
}

impl<T: Get<MaybeBorder>> GetMaybeBorder for T {}

/// Wrapper trait for `Set<MaybeBorder>`
pub trait SetMaybeBorder: Set<MaybeBorder> {
    /// Set maybe border
    #[inline(always)]
    fn set_maybe_border(&mut self, val: MaybeBorder) {
        self.set_mut(val);
    }
}

impl<T: Set<MaybeBorder>> SetMaybeBorder for T {}

/// A filled rectangle
#[deriving(Copy)]
pub struct Rectangle {
    /// The rectangle color
    pub color: internal::Color,
    /// The roundness of the rectangle
    pub shape: Shape,
    /// The border
    pub border: Option<Border>,
}

impl Rectangle {
    /// Creates a new rectangle.
    pub fn new(color: internal::Color) -> Rectangle {
        Rectangle {
            color: color,
            shape: Shape::Square,
            border: None,
        }
    }

    /// Creates a new rectangle border.
    pub fn border(
        color: internal::Color, 
        radius: internal::Radius
    ) -> Rectangle {
        Rectangle {
            color: [0.0, ..4],
            shape: Shape::Square,
            border: Some(Border {
                    color: color,
                    radius: radius
                })
        }
    }

    /// Draws the rectangle
    pub fn draw<B: BackEnd<I>, I: ImageSize>(
        &self, 
        rectangle: internal::Rectangle, 
        c: &Context, 
        back_end: &mut B
    ) {
        if self.color[3] != 0.0 {
            back_end.color(self.color);
            match self.shape {
                Shape::Square => {
                    back_end.tri_list(
                        &triangulation::rect_tri_list_xy(c.transform, rectangle),
                    );
                }
                Shape::Round(round_radius) => {
                    triangulation::with_round_rectangle_tri_list(
                        32,
                        c.transform,
                        rectangle,
                        round_radius,
                        |vertices| back_end.tri_list(vertices)
                    );
                }
                Shape::Bevel(bevel_radius) => {
                    triangulation::with_round_rectangle_tri_list(
                        2,
                        c.transform,
                        rectangle,
                        bevel_radius,
                        |vertices| back_end.tri_list(vertices)
                    );
                }
            }
        }
       
        if let Some(Border { color, radius: border_radius }) = self.border {
            if color[3] == 0.0 { return; }
            back_end.color(color);
            match self.shape {
                Shape::Square => {
                    back_end.tri_list(
                        &triangulation::rect_border_tri_list_xy(
                            c.transform, rectangle, border_radius),
                    );
                }
                Shape::Round(round_radius) => {
                    triangulation::with_round_rectangle_border_tri_list(
                        128,
                        c.transform,
                        rectangle,
                        round_radius,
                        border_radius,
                        |vertices| back_end.tri_list(vertices)
                    );
                }
                Shape::Bevel(bevel_radius) => {
                    triangulation::with_round_rectangle_border_tri_list(
                        2,
                        c.transform,
                        rectangle,
                        bevel_radius,
                        border_radius,
                        |vertices| back_end.tri_list(vertices)
                    );
                }
            }
        } 
    }
}

impl Modifier<Rectangle> for Color {
    #[inline(always)]
    fn modify(self, r: &mut Rectangle) {
        let Color(val) = self;
        r.color = val;
    }
}

impl Get<Color> for Rectangle {
    #[inline(always)]
    fn get(&self) -> Color {
        Color(self.color)
    }
}

impl Modifier<Rectangle> for Shape {
    #[inline(always)]
    fn modify(self, r: &mut Rectangle) {
        r.shape = self;
    }
}

impl Get<Shape> for Rectangle {
    #[inline(always)]
    fn get(&self) -> Shape {
        self.shape
    }
}

impl Modifier<Rectangle> for Border {
    #[inline(always)]
    fn modify(self, r: &mut Rectangle) {
        r.border = Some(self);
    }
}

impl Modifier<Rectangle> for MaybeBorder {
    #[inline(always)]
    fn modify(self, r: &mut Rectangle) {
        let MaybeBorder(val) = self;
        r.border = val;
    }
}

impl Get<MaybeBorder> for Rectangle {
    #[inline(always)]
    fn get(&self) -> MaybeBorder {
        MaybeBorder(self.border)
    }
}

#[cfg(test)]
mod test {
    use super::Rectangle;
    use super::Shape;
    use super::Border;
    use Color;
    use current::Set;

    #[test]
    fn test_rectangle() {
        let _rectangle = Rectangle::new([1.0, ..4])
            .set(Color([0.0, ..4]))
            .set(Shape::Round(10.0))
            .set(Border { color: [0.0, ..4], radius: 4.0 });
    }
}

