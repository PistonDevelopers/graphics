//! Draw rectangle

use internal;
use triangulation;
use Graphics;
use Color;
use DrawState;
use vecmath::Matrix2d;

pub use vecmath::margin_rectangle as margin;

/// Use x, y, half-width, half-height
pub fn centered(rect: internal::Rectangle) -> internal::Rectangle {
    let [cx, cy, rw, rh] = rect;
    [cx - rw, cy - rh, 2.0 * rw, 2.0 * rh]
}

/// Use centered square
pub fn centered_square(
    x: internal::Scalar,
    y: internal::Scalar,
    radius: internal::Scalar
) -> internal::Rectangle {
    [x - radius, y - radius, 2.0 * radius, 2.0 * radius]
}

/// Use square with x, y in upper left corner
pub fn square(
    x: internal::Scalar,
    y: internal::Scalar,
    size: internal::Scalar
) -> internal::Rectangle {
    [x, y, size, size]
}

/// The shape of the rectangle
#[derive(Copy, Clone)]
pub enum Shape {
    /// Square corners
    Square,
    /// Round corners
    Round(internal::Radius),
    /// Bevel corners
    Bevel(internal::Radius),
}

/// The border of the rectangle
#[derive(Copy, Clone)]
pub struct Border {
    /// The color of the border
    pub color: internal::Color,
    /// The radius of the border
    pub radius: internal::Radius,
}

/// Maybe border property
#[derive(Copy)]
pub struct MaybeBorder(pub Option<Border>);

/// A filled rectangle
#[derive(Copy, Clone)]
pub struct Rectangle {
    /// The rectangle color
    pub color: internal::Color,
    /// The roundness of the rectangle
    pub shape: Shape,
    /// The border
    pub border: Option<Border>,
}

impl Rectangle {
    /// Creates a new rectangle.
    pub fn new(color: internal::Color) -> Rectangle {
        Rectangle {
            color: color,
            shape: Shape::Square,
            border: None,
        }
    }

    /// Creates a new round rectangle.
    pub fn round(
        color: internal::Color,
        round_radius: internal::Radius
    ) -> Rectangle {
        Rectangle {
            color: color,
            shape: Shape::Round(round_radius),
            border: None
        }
    }

    /// Creates a new rectangle border.
    pub fn border(
        color: internal::Color,
        radius: internal::Radius
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
    pub fn round_border(
        color: internal::Color,
        round_radius: internal::Radius,
        border_radius: internal::Radius
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

    /// Draws the rectangle
    pub fn draw<G>(
        &self,
        rectangle: internal::Rectangle,
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

quack! {
    r: Rectangle[]
    get:
        fn () -> Color [] { Color(r.color) }
        fn () -> Shape [] { r.shape }
        fn () -> MaybeBorder [] { MaybeBorder(r.border) }
    set:
        fn (val: Color) [] { r.color = val.0 }
        fn (val: Shape) [] { r.shape = val }
        fn (val: Border) [] { r.border = Some(val) }
        fn (val: MaybeBorder) [] { r.border = val.0 }
    action:
}

#[cfg(test)]
mod test {
    use super::Rectangle;
    use super::Shape;
    use super::Border;
    use Color;
    use quack::Set;

    #[test]
    fn test_rectangle() {
        let _rectangle = Rectangle::new([1.0; 4])
            .set(Color([0.0; 4]))
            .set(Shape::Round(10.0))
            .set(Border { color: [0.0; 4], radius: 4.0 });
    }
}
