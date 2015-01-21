//! Draw ellipse

pub use rectangle::centered;
pub use rectangle::centered_square as circle;

use quack::{ GetFrom, SetAt, Me };
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

impl SetAt for (Color, Ellipse) {
    type Property = Color;
    type Object = Ellipse;

    #[inline(always)]
    fn set_at(_: Me<Self>, Color(val): Color, e: &mut Ellipse) {
        e.color = val;
    }
}

impl GetFrom for (Color, Ellipse) {
    type Property = Color;
    type Object = Ellipse;

    #[inline(always)]
    fn get_from(_: Me<Self>, obj: &Ellipse) -> Color {
        Color(obj.color)
    }
}

impl SetAt for (Border, Ellipse) {
    type Property = Border;
    type Object = Ellipse;

    #[inline(always)]
    fn set_at(_: Me<Self>, val: Border, e: &mut Ellipse) {
        e.border = Some(val);
    }
}

impl SetAt for (MaybeBorder, Ellipse) {
    type Property = MaybeBorder;
    type Object = Ellipse;

    #[inline(always)]
    fn set_at(_: Me<Self>, MaybeBorder(val): MaybeBorder, e: &mut Ellipse) {
        e.border = val;
    }
}

impl GetFrom for (MaybeBorder, Ellipse) {
    type Property = MaybeBorder;
    type Object = Ellipse;

    #[inline(always)]
    fn get_from(_: Me<Self>, obj: &Ellipse) -> MaybeBorder {
        MaybeBorder(obj.border)
    }
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

