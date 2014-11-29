use internal;
use triangulation;
use Context;
use BackEnd;
use ImageSize;

/// A filled rectangle
pub struct Rectangle {
    /// The rectangle shape
    pub rect: internal::Rectangle,
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
            &triangulation::rect_tri_list_xy(c.transform, self.rect),
        );
    }
}

