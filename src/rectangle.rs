//! Draw rectangle
//!
//! This module contains the definintion of a rectangle with possibly rounded
//! corners. It contains the code to draw the rectangle and defines properties
//! like color and shape. The rectangle dimensions and location are specified by
//! `types::Rectangle`.
//!
//! To draw a square with side 10 and top left corner in (0, 0), do the
//! following:
//! ```ignore
//! let rectangle = Rectangle::new(color::BLACK);
//! let dims = square(0.0, 0.0, 10.0);
//! rectangle.draw(dims, &draw_state::Default::default(), transform, g);
//! ```

pub use crate::math::margin_rectangle as margin;
use crate::{
    math::{Matrix2d, Scalar},
    triangulation, types,
    types::{Color, Radius, Resolution},
    DrawState, Graphics,
};

/// Create `types::Rectangle` by the two opposite corners.
///
/// The corners are in (x0, y0) and (x1, y1).
pub fn rectangle_by_corners(x0: Scalar, y0: Scalar, x1: Scalar, y1: Scalar) -> types::Rectangle {
    let (xmin, w) = if x0 <= x1 {
        (x0, x1 - x0)
    } else {
        (x1, x0 - x1)
    };

    let (ymin, h) = if y0 <= y1 {
        (y0, y1 - y0)
    } else {
        (y1, y0 - y1)
    };

    [xmin, ymin, w, h]
}

/// Create a centered rectangle.
///
/// `rect` is interpreted as `[x, y, half_width, half_height]`
///
/// Note that passing an existing `types::Rectangle` in here will not produce a rectangle of the
/// same
/// dimensions
///
/// # Example
/// ```
/// use graphics::rectangle::centered;
///
/// // We create a rectangle centered on [0.0, 0.0] with width 2.0 and height 6.0
/// let centered_rect = centered([0.0, 0.0, 1.0, 3.0]);
/// assert_eq!(centered_rect, [-1.0, -3.0, 2.0, 6.0]);
/// ```
pub fn centered(rect: types::Rectangle) -> types::Rectangle {
    [
        rect[0] - rect[2],
        rect[1] - rect[3],
        2.0 * rect[2],
        2.0 * rect[3],
    ]
}

/// Create `types::Rectangle` for a square with a center in (`x`, `y`) and side
/// `2 * radius`.
pub fn centered_square(x: Scalar, y: Scalar, radius: Scalar) -> types::Rectangle {
    [x - radius, y - radius, 2.0 * radius, 2.0 * radius]
}

/// Create `types::Rectangle` for a square with a top-left corner in (`x`, `y`)
/// and side `size`.
pub fn square(x: Scalar, y: Scalar, size: Scalar) -> types::Rectangle {
    [x, y, size, size]
}

/// The shape of the rectangle corners
#[derive(Copy, Clone)]
pub enum Shape {
    /// Square corners
    Square,
    /// Round corners, with resolution per corner.
    Round(Radius, Resolution),
    /// Bevel corners
    Bevel(Radius),
}

/// The border of the rectangle
#[derive(Copy, Clone)]
pub struct Border {
    /// The color of the border
    pub color: Color,
    /// The radius of the border. The half-width of the line by which border is
    /// drawn.
    pub radius: Radius,
}

/// A filled rectangle
#[derive(Copy, Clone)]
pub struct Rectangle {
    /// The rectangle color
    pub color: Color,
    /// The roundness of the rectangle
    pub shape: Shape,
    /// The border
    pub border: Option<Border>,
}

impl Rectangle {
    /// Creates a new rectangle.
    pub fn new(color: Color) -> Rectangle {
        Rectangle {
            color,
            shape: Shape::Square,
            border: None,
        }
    }

    /// Creates a new rectangle with rounded corners.
    pub fn new_round(color: Color, round_radius: Radius) -> Rectangle {
        Rectangle {
            color,
            shape: Shape::Round(round_radius, 32),
            border: None,
        }
    }

    /// Creates a new rectangle border.
    pub fn new_border(color: Color, radius: Radius) -> Rectangle {
        Rectangle {
            color: [0.0; 4],
            shape: Shape::Square,
            border: Some(Border { color, radius }),
        }
    }

    /// Creates a new rectangle border with rounded corners.
    pub fn new_round_border(
        color: Color,
        round_radius: Radius,
        border_radius: Radius,
    ) -> Rectangle {
        Rectangle {
            color: [0.0; 4],
            shape: Shape::Round(round_radius, 32),
            border: Some(Border {
                color,
                radius: border_radius,
            }),
        }
    }

    /// Sets color.
    pub fn color(mut self, value: Color) -> Self {
        self.color = value;
        self
    }

    /// Sets shape of the corners.
    pub fn shape(mut self, value: Shape) -> Self {
        self.shape = value;
        self
    }

    /// Sets border properties.
    pub fn border(mut self, value: Border) -> Self {
        self.border = Some(value);
        self
    }

    /// Sets optional border.
    pub fn maybe_border(mut self, value: Option<Border>) -> Self {
        self.border = value;
        self
    }

    /// Draws the rectangle by corners using the default method.
    #[inline(always)]
    pub fn draw_from_to<P: Into<types::Vec2d>, G>(
        &self,
        from: P,
        to: P,
        draw_state: &DrawState,
        transform: Matrix2d,
        g: &mut G,
    ) where
        G: Graphics,
    {
        let from = from.into();
        let to = to.into();
        g.rectangle(
            self,
            rectangle_by_corners(from[0], from[1], to[0], to[1]),
            draw_state,
            transform,
        );
    }

    /// Draws the rectangle using the default method.
    ///
    /// `rectangle` defines the rectangle's location and dimensions,
    /// `draw_state` draw state, `draw_state::Default::default()` can be used
    /// as a default, `transform` is the transformation matrix, `g` is the
    /// `Graphics` implementation, that is used to actually draw the rectangle.s
    #[inline(always)]
    pub fn draw<R: Into<types::Rectangle>, G>(
        &self,
        rectangle: R,
        draw_state: &DrawState,
        transform: Matrix2d,
        g: &mut G,
    ) where
        G: Graphics,
    {
        g.rectangle(self, rectangle, draw_state, transform);
    }

    /// Draws the rectangle using triangulation.
    ///
    /// This is the default implementation of draw() that will be used if `G`
    /// does not redefine `Graphics::rectangle()`.
    pub fn draw_tri<R: Into<types::Rectangle>, G>(
        &self,
        rectangle: R,
        draw_state: &DrawState,
        transform: Matrix2d,
        g: &mut G,
    ) where
        G: Graphics,
    {
        let rectangle = rectangle.into();
        if self.color[3] != 0.0 {
            match self.shape {
                Shape::Square => {
                    g.tri_list(draw_state, &self.color, |f| {
                        f(&triangulation::rect_tri_list_xy(transform, rectangle))
                    });
                }
                Shape::Round(round_radius, resolution) => {
                    g.tri_list(draw_state, &self.color, |f| {
                        triangulation::with_round_rectangle_tri_list(
                            resolution,
                            transform,
                            rectangle,
                            round_radius,
                            |vertices| f(vertices),
                        )
                    });
                }
                Shape::Bevel(bevel_radius) => {
                    g.tri_list(draw_state, &self.color, |f| {
                        triangulation::with_round_rectangle_tri_list(
                            2,
                            transform,
                            rectangle,
                            bevel_radius,
                            |vertices| f(vertices),
                        )
                    });
                }
            }
        }

        if let Some(Border {
            color,
            radius: border_radius,
        }) = self.border
        {
            if color[3] == 0.0 {
                return;
            }
            match self.shape {
                Shape::Square => {
                    g.tri_list(draw_state, &color, |f| {
                        f(&triangulation::rect_border_tri_list_xy(
                            transform,
                            rectangle,
                            border_radius,
                        ))
                    });
                }
                Shape::Round(round_radius, resolution) => {
                    g.tri_list(draw_state, &color, |f| {
                        triangulation::with_round_rectangle_border_tri_list(
                            resolution,
                            transform,
                            rectangle,
                            round_radius,
                            border_radius,
                            |vertices| f(vertices),
                        )
                    });
                }
                Shape::Bevel(bevel_radius) => {
                    g.tri_list(draw_state, &color, |f| {
                        triangulation::with_round_rectangle_border_tri_list(
                            2,
                            transform,
                            rectangle,
                            bevel_radius,
                            border_radius,
                            |vertices| f(vertices),
                        )
                    });
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rectangle() {
        let _rectangle = Rectangle::new([1.0; 4])
            .color([0.0; 4])
            .shape(Shape::Round(10.0, 32))
            .border(Border {
                color: [0.0; 4],
                radius: 4.0,
            });
    }

    #[test]
    fn test_rectangle_by_corners() {
        assert_eq!(
            rectangle_by_corners(1.0, -1.0, 2.0, 3.0),
            [1.0, -1.0, 1.0, 4.0]
        );
        assert_eq!(
            rectangle_by_corners(2.0, 3.0, 1.0, -1.0),
            [1.0, -1.0, 1.0, 4.0]
        );
    }
}
