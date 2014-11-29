use internal;
use triangulation;
use BackEnd;
use Context;
use ImageSize;

/// An ellipse with filled color
pub struct Ellipse {
    /// The ellipse rectangle
    pub rectangle: internal::Rectangle,
    /// The ellipse color
    pub color: internal::Color,
}

impl Ellipse {
    /// Draws the ellipse.
    pub fn draw<B: BackEnd<I>, I: ImageSize>(&self, c: &Context, back_end: &mut B) {
        if self.color[3] == 0.0 { return; }
        back_end.color(self.color);
        triangulation::with_ellipse_tri_list(
            128,
            c.transform,
            self.rectangle,
            |vertices| {
                back_end.tri_list(vertices)
            }
        );
    }
}

