use internal;
use triangulation;
use Context;
use BackEnd;
use ImageSize;

/// A filled rectangle
pub struct Rectangle {
    /// The rectangle shape
    pub rectangle: internal::Rectangle,
    /// The rectangle color
    pub color: internal::Color
}

impl Rectangle {
    /// Draws the rectangle
    pub fn draw<B: BackEnd<I>, I: ImageSize>(&self, c: &Context, back_end: &mut B) {
        // Complete transparency does not need to be rendered.
        if self.color[3] == 0.0 { return; }
        back_end.color(self.color);
        back_end.tri_list(
            &triangulation::rect_tri_list_xy(c.transform, self.rectangle),
        );
    }
}

/// A round rectangle filled with a color
pub struct RoundRectangle {
    /// The rectangle shape
    pub rectangle: internal::Rectangle,
    /// The rectangle color
    pub color: internal::Color,
    /// The roundness radius
    pub round_radius: internal::Radius,
}

impl RoundRectangle {
    /// Draws the round rectangle
    pub fn draw<B: BackEnd<I>, I: ImageSize>(&self, c: &Context, back_end: &mut B) {
        if self.color[3] == 0.0 { return; }
        back_end.color(self.color);
        triangulation::with_round_rectangle_tri_list(
            32,
            c.transform,
            self.rectangle,
            self.round_radius,
            |vertices| back_end.tri_list(vertices)
        );
    }
}

/// A rectangle border.
pub struct RectangleBorder {
    /// The rectangle shape.
    pub rectangle: internal::Rectangle,
    /// The rectangle color.
    pub color: internal::Color,
    /// The border radius.
    pub border_radius: internal::Radius,
}

impl RectangleBorder {
    /// Draw the rectangle border.
    pub fn draw<B: BackEnd<I>, I: ImageSize>(&self, c: &Context, back_end: &mut B) {
        if self.color[3] == 0.0 { return; }
        back_end.color(self.color);
        back_end.tri_list(
            &triangulation::rect_border_tri_list_xy(
                c.transform, self.rectangle, self.border_radius),
        );
    }
}
