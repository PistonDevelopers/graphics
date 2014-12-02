//! Draw ellipse

pub use rectangle::centered;
pub use rectangle::centered_square as circle;

use current::{ Get, Modifier, Set };
use internal;
use triangulation;
use BackEnd;
use Context;
use ImageSize;
use Color;

/// Ellipse border
pub struct Border {
    /// The border color
    pub color: internal::Color,
    /// The border radius
    pub radius: internal::Radius,
}

/// Wrapper trait for `Get<Border>`
pub trait GetBorder: Get<Border> {
    /// Get border
    #[inline(always)]
    fn get_border(&self) -> Border {
        self.get()
    }
}

impl<T: Get<Border>> GetBorder for T {}

/// Wrapper trait for `Set<Border>`
pub trait SetBorder: Set<Border> {
    /// Set border
    #[inline(always)]
    fn set_border(&mut self, val: Border) {
        self.set_mut(val);
    }
}

impl<T: Set<Border>> SetBorder for T {}

/// Maybe border property
pub struct MaybeBorder(pub Option<Border>);

/// Wrapper trait for `Get<MaybeBorder>`
pub trait GetMaybeBorder: Get<MaybeBorder> {
    /// Get maybe border
    #[inline(always)]
    fn get_maybe_border(&self) -> MaybeBorder {
        self.get()
    }
}

impl<T: Get<MaybeBorder>> GetMaybeBorder for T {}

/// Wrapper trait for `Set<MaybeBorder>`
pub trait SetMaybeBorder: Set<MaybeBorder> {
    /// Set maybe border
    #[inline(always)]
    fn set_maybe_border(&mut self, val: MaybeBorder) {
        self.set_mut(val);
    }
}

impl<T: Set<MaybeBorder>> SetMaybeBorder for T {}

/// An ellipse with filled color
#[deriving(Copy)]
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
            color: [0.0, ..4],
            border: Some(Border {
                    color: color,
                    radius: radius,
                })
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

impl Modifier<Ellipse> for Color {
    #[inline(always)]
    fn modify(self, e: &mut Ellipse) {
        let Color(val) = self;
        e.color = val;
    }
}

impl Get<Color> for Ellipse {
    #[inline(always)]
    fn get(&self) -> Color {
        Color(self.color)
    }
}

impl Modifier<Ellipse> for Border {
    #[inline(always)]
    fn modify(self, e: &mut Ellipse) {
        e.border = Some(self);
    }
}

impl Modifier<Ellipse> for MaybeBorder {
    #[inline(always)]
    fn modify(self, e: &mut Ellipse) {
        let MaybeBorder(val) = self;
        e.border = val;
    }
}

impl Get<MaybeBorder> for Ellipse {
    #[inline(always)]
    fn get(&self) -> MaybeBorder {
        MaybeBorder(self.border)
    }
}

#[cfg(test)]
mod test {
    use super::Ellipse;
    use super::Border;
    use Color;
    use current::Set;

    #[test]
    fn test_ellipse() {
        let _ellipse = Ellipse::new([1.0, ..4])
            .set(Color([0.0, ..4]))
            .set(Border { color: [1.0, ..4], radius: 3.0 });
    }
}

