//! Draw ellipse

use internal;
use triangulation;
use BackEnd;
use Context;
use ImageSize;

/// Ellipse border
pub struct Border {
    /// The border color
    pub color: internal::Color,
    /// The border radius
    pub radius: internal::Radius,
}

/// An ellipse with filled color
pub struct Ellipse {
    /// The ellipse color
    pub color: internal::Color,
    /// The ellipse border
    pub border: Option<Border>,
}

impl Ellipse {
    /// Creates a new ellipse
    pub fn new(color: internal::Color) -> Ellipse {
        Ellipse {
            color: color,
            border: None
        }
    }

    /// Draws the ellipse.
    pub fn draw<B: BackEnd<I>, I: ImageSize>(
        &self, 
        rectangle: internal::Rectangle,
        c: &Context,
        back_end: &mut B
    ) {
        if self.color[3] != 0.0 {
            back_end.color(self.color);
            triangulation::with_ellipse_tri_list(
                128,
                c.transform,
                rectangle,
                |vertices| {
                    back_end.tri_list(vertices)
                }
            );
        }

        if let Some(Border { color, radius: border_radius }) = self.border {
            if color[3] == 0.0 { return; }
            back_end.color(self.color);
            triangulation::with_ellipse_border_tri_list(
                128,
                c.transform,
                rectangle,
                border_radius,
                |vertices| back_end.tri_list(vertices)
            );
        }
    }
}

