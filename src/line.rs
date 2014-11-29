use internal;
use triangulation;
use BackEnd;
use Context;
use ImageSize;

/// A colored line with a default border radius
pub struct Line {
    /// The line shape
    pub line: internal::Line,
    /// The line color
    pub color: internal::Color,
}

impl Line {
    /// Draw the line.
    pub fn draw<B: BackEnd<I>, I: ImageSize>(&self, c: &Context, back_end: &mut B) {
        // Complete transparency does not need to be rendered.
        if self.color[3] == 0.0 { return; }
        back_end.color(self.color);
        triangulation::with_round_border_line_tri_list(
            2,
            c.transform,
            self.line,
            1.0,
            |vertices| back_end.tri_list(vertices)
        );
    }
}

/// A line that is round at the edges
pub struct RoundLine {
    /// The line shape
    pub line: internal::Line,
    /// The line color
    pub color: internal::Color,
    /// The border radius
    pub border_radius: internal::Radius,
}

impl RoundLine {
    /// Draws the line.
    pub fn draw<B: BackEnd<I>, I: ImageSize>(&self, c: &Context, back_end: &mut B) {
        // Complete transparency does not need to be rendered.
        if self.color[3] == 0.0 { return; }
        back_end.color(self.color);
        triangulation::with_round_border_line_tri_list(
            64,
            c.transform,
            self.line,
            self.border_radius,
            |vertices| back_end.tri_list(vertices)
        );
    }
}

/// A bevel shaped line
pub struct BevelLine {
    /// The line shape
    pub line: internal::Line,
    /// The line color
    pub color: internal::Color,
    /// The border radius
    pub border_radius: internal::Radius,
}

impl BevelLine {
    /// Draws the line.
    pub fn draw<B: BackEnd<I>, I: ImageSize>(&self, c: &Context, back_end: &mut B) {
        // Complete transparency does not need to be rendered.
        if self.color[3] == 0.0 { return; }
        back_end.color(self.color);
        triangulation::with_round_border_line_tri_list(
            3,
            c.transform,
            self.line,
            self.border_radius,
            |vertices| back_end.tri_list(vertices)
        );
    }
}

