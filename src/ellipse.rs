//! Draw ellipse

use types::{ Color, Radius, Rect };
use { triangulation, DrawState, Graphics };
use math::Matrix2d;

pub use rectangle::centered;
pub use rectangle::centered_square as circle;

/// Ellipse border
#[derive(Copy, Clone)]
pub struct Border {
    /// The border color
    pub color: Color,
    /// The border radius
    pub radius: Radius,
}

/// An ellipse with filled color
#[derive(Copy, Clone)]
pub struct Ellipse {
    /// The ellipse color
    pub color: Color,
    /// The ellipse border
    pub border: Option<Border>,
}

impl Ellipse {
    /// Creates a new ellipse
    pub fn new(color: Color) -> Ellipse {
        Ellipse {
            color: color,
            border: None
        }
    }

    /// Creates a new ellipse border
    pub fn new_border(
        color: Color,
        radius: Radius
    ) -> Ellipse {
        Ellipse {
            color: [0.0; 4],
            border: Some(Border {
                    color: color,
                    radius: radius,
                })
        }
    }

    /// Sets ellipse color.
    pub fn color(mut self, value: Color) -> Self {
        self.color = value;
        self
    }

    /// Sets ellipse border.
    pub fn border(mut self, value: Border) -> Self {
        self.border = Some(value);
        self
    }

    /// Sets optional ellipse border.
    pub fn maybe_border(mut self, value: Option<Border>) -> Self {
        self.border = value;
        self
    }

    /// Draws the ellipse.
    pub fn draw<G>(
        &self,
        rectangle: Rect,
        draw_state: &DrawState,
        transform: Matrix2d,
        g: &mut G
    )
        where G: Graphics
    {
        g.tri_list(
            draw_state,
            &self.color,
            |f|
        triangulation::with_ellipse_tri_list(
            128,
            transform,
            rectangle,
            |vertices| f(vertices)
        ));

        if let Some(Border { color, radius: border_radius }) = self.border {
            g.tri_list(
                &draw_state,
                &color,
                |f|
            triangulation::with_ellipse_border_tri_list(
                128,
                transform,
                rectangle,
                border_radius,
                |vertices| f(vertices)
            ));
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ellipse() {
        let _ellipse = Ellipse::new([1.0; 4])
            .color([0.0; 4])
            .border(Border { color: [1.0; 4], radius: 3.0 });
    }
}
