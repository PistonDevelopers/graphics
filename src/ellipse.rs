//! Draw ellipse

use types::{Color, Radius, Rectangle, Resolution};
use {triangulation, DrawState, Graphics};
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
    /// The resolution for the shape, 360 degrees.
    pub resolution: Resolution,
}

impl Ellipse {
    /// Creates a new ellipse
    pub fn new(color: Color) -> Ellipse {
        Ellipse {
            color: color,
            border: None,
            resolution: 128,
        }
    }

    /// Creates a new ellipse border
    pub fn new_border(color: Color, radius: Radius) -> Ellipse {
        Ellipse {
            color: [0.0; 4],
            border: Some(Border {
                color: color,
                radius: radius,
            }),
            resolution: 128,
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

    /// Sets resolution of the ellipse smoothness.
    pub fn resolution(mut self, value: Resolution) -> Self {
        self.resolution = value;
        self
    }

    /// Draws ellipse by corners using default method.
    #[inline(always)]
    pub fn draw_from_to<P: Into<crate::types::Vec2d>, G>(&self,
                                       from: P,
                                       to: P,
                                       draw_state: &DrawState,
                                       transform: Matrix2d,
                                       g: &mut G)
        where G: Graphics
    {
        use rectangle::rectangle_by_corners;

        let from = from.into();
        let to = to.into();
        g.ellipse(self,
                  rectangle_by_corners(from[0], from[1], to[0], to[1]),
                  draw_state,
                  transform);
    }

    /// Draws ellipse using default method.
    #[inline(always)]
    pub fn draw<R: Into<Rectangle>, G>(&self,
                                       rectangle: R,
                                       draw_state: &DrawState,
                                       transform: Matrix2d,
                                       g: &mut G)
        where G: Graphics
    {
        g.ellipse(self, rectangle, draw_state, transform);
    }

    /// Draws ellipse using triangulation.
    pub fn draw_tri<R: Into<Rectangle>, G>(&self,
                                           rectangle: R,
                                           draw_state: &DrawState,
                                           transform: Matrix2d,
                                           g: &mut G)
        where G: Graphics
    {
        let rectangle = rectangle.into();
        g.tri_list(draw_state, &self.color, |f| {
            triangulation::with_ellipse_tri_list(self.resolution,
                                                 transform,
                                                 rectangle,
                                                 |vertices| f(vertices))
        });

        if let Some(Border { color, radius: border_radius }) = self.border {
            g.tri_list(&draw_state, &color, |f| {
                triangulation::with_ellipse_border_tri_list(self.resolution,
                                                            transform,
                                                            rectangle,
                                                            border_radius,
                                                            |vertices| f(vertices))
            });
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
            .border(Border {
                color: [1.0; 4],
                radius: 3.0,
            });
    }
}
