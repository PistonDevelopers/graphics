use internal;
use triangulation;
use BackEnd;
use Context;
use ImageSize;

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
