//! Draw ellipse

pub use rectangle::centered;
pub use rectangle::centered_square as circle;

use internal;
use triangulation;
use BackEnd;
use Context;
use Color;

/// Ellipse border
#[derive(Copy, Clone)]
pub struct Border {
    /// The border color
    pub color: internal::Color,
    /// The border radius
    pub radius: internal::Radius,
}

/// Maybe border property
#[derive(Copy)]
pub struct MaybeBorder(pub Option<Border>);

/// An ellipse with filled color
#[derive(Copy, Clone)]
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

    /// Creates a new ellipse border
    pub fn border(
        color: internal::Color, 
        radius: internal::Radius
    ) -> Ellipse {
        Ellipse {
            color: [0.0; 4],
            border: Some(Border {
                    color: color,
                    radius: radius,
                })
        }
    }

    /// Draws the ellipse.
    pub fn draw<B>(
        &self, 
        rectangle: internal::Rectangle,
        c: &Context,
        back_end: &mut B
    )
        where B: BackEnd
    {
        if self.color[3] != 0.0 {
            back_end.tri_list(
                &self.color,
                |f|
            triangulation::with_ellipse_tri_list(
                128,
                c.transform,
                rectangle,
                |vertices| f(vertices)
            ));
        }

        if let Some(Border { color, radius: border_radius }) = self.border {
            if color[3] == 0.0 { return; }
            back_end.tri_list(
                &color,
                |f|
            triangulation::with_ellipse_border_tri_list(
                128,
                c.transform,
                rectangle,
                border_radius,
                |vertices| f(vertices)
            ));
        }
    }
}

quack! {
    e: Ellipse[]
    get:
        fn () -> Color [] { Color(e.color) }
        fn () -> MaybeBorder [] { MaybeBorder(e.border) }
    set:
        fn (val: Color) [] { e.color = val.0 }
        fn (val: Border) [] { e.border = Some(val) }
        fn (val: MaybeBorder) [] { e.border = val.0 }
    action:
}

#[cfg(test)]
mod test {
    use super::Ellipse;
    use super::Border;
    use Color;
    use quack::Set;

    #[test]
    fn test_ellipse() {
        let _ellipse = Ellipse::new([1.0; 4])
            .set(Color([0.0; 4]))
            .set(Border { color: [1.0; 4], radius: 3.0 });
    }
}

