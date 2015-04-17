//! Draw rectangle

use types::{ Color, Radius };
use { types, triangulation, Graphics, DrawState };
use math::{ Matrix2d, Scalar };

pub use math::margin_rectangle as margin;

/// Use x, y, half-width, half-height
pub fn centered(rect: types::Rect) -> types::Rect {
    let rect = rect.to_array();
    [rect[0] - rect[2], rect[1] - rect[3], 2.0 * rect[2], 2.0 * rect[3]].into()
}

/// Use centered square
pub fn centered_square(x: Scalar, y: Scalar, radius: Scalar) -> types::Rect {
    [x - radius, y - radius, 2.0 * radius, 2.0 * radius].into()
}

/// Use square with x, y in upper left corner
pub fn square(x: Scalar, y: Scalar, size: Scalar) -> types::Rect {
    [x, y, size, size].into()
}

/// The shape of the rectangle
#[derive(Copy, Clone)]
pub enum Shape {
    /// Square corners
    Square,
    /// Round corners
    Round(Radius),
    /// Bevel corners
    Bevel(Radius),
}

/// The border of the rectangle
#[derive(Copy, Clone)]
pub struct Border {
    /// The color of the border
    pub color: Color,
    /// The radius of the border
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
            color: color,
            shape: Shape::Square,
            border: None,
        }
    }

    /// Creates a new round rectangle.
    pub fn new_round(
        color: Color,
        round_radius: Radius
    ) -> Rectangle {
        Rectangle {
            color: color,
            shape: Shape::Round(round_radius),
            border: None
        }
    }

    /// Creates a new rectangle border.
    pub fn new_border(
        color: Color,
        radius: Radius
    ) -> Rectangle {
        Rectangle {
            color: [0.0; 4],
            shape: Shape::Square,
            border: Some(Border {
                    color: color,
                    radius: radius
                })
        }
    }

    /// Creates a new round rectangle border.
    pub fn new_round_border(
        color: Color,
        round_radius: Radius,
        border_radius: Radius
    ) -> Rectangle {
        Rectangle {
            color: [0.0; 4],
            shape: Shape::Round(round_radius),
            border: Some(Border {
                    color: color,
                    radius: border_radius
                })
        }
    }

    /// Sets color.
    pub fn color(mut self, value: Color) -> Self {
        self.color = value;
        self
    }

    /// Sets shape.
    pub fn shape(mut self, value: Shape) -> Self {
        self.shape = value;
        self
    }

    /// Sets border.
    pub fn border(mut self, value: Border) -> Self {
        self.border = Some(value);
        self
    }

    /// Sets optional border.
    pub fn maybe_border(mut self, value: Option<Border>) -> Self {
        self.border = value;
        self
    }

    /// Draws the rectangle
    pub fn draw<G>(
        &self,
        rectangle: types::Rect,
        draw_state: &DrawState,
        transform: Matrix2d,
        g: &mut G
    )
        where G: Graphics
    {
        if self.color[3] != 0.0 {
            match self.shape {
                Shape::Square => {
                    g.tri_list(
                        draw_state,
                        &self.color,
                        |f|
                            f(&triangulation::rect_tri_list_xy(
                                transform,
                                rectangle
                            )),
                    );
                }
                Shape::Round(round_radius) => {
                    g.tri_list(
                        draw_state,
                        &self.color,
                        |f|
                    triangulation::with_round_rectangle_tri_list(
                        32,
                        transform,
                        rectangle,
                        round_radius,
                        |vertices| f(vertices)
                    ));
                }
                Shape::Bevel(bevel_radius) => {
                    g.tri_list(
                        draw_state,
                        &self.color,
                        |f|
                    triangulation::with_round_rectangle_tri_list(
                        2,
                        transform,
                        rectangle,
                        bevel_radius,
                        |vertices| f(vertices)
                    ));
                }
            }
        }

        if let Some(Border { color, radius: border_radius }) = self.border {
            if color[3] == 0.0 { return; }
            match self.shape {
                Shape::Square => {
                    g.tri_list(
                        draw_state,
                        &color,
                        |f| f(
                            &triangulation::rect_border_tri_list_xy(
                                transform, rectangle, border_radius),
                        )
                    );
                }
                Shape::Round(round_radius) => {
                    g.tri_list(
                        draw_state,
                        &color,
                        |f|
                    triangulation::with_round_rectangle_border_tri_list(
                        128,
                        transform,
                        rectangle,
                        round_radius,
                        border_radius,
                        |vertices| f(vertices)
                    ));
                }
                Shape::Bevel(bevel_radius) => {
                    g.tri_list(
                        draw_state,
                        &color,
                        |f|
                    triangulation::with_round_rectangle_border_tri_list(
                        2,
                        transform,
                        rectangle,
                        bevel_radius,
                        border_radius,
                        |vertices| f(vertices)
                    ));
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
            .shape(Shape::Round(10.0))
            .border(Border { color: [0.0; 4], radius: 4.0 });
    }
}
