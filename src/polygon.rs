//! Draw polygon

use current::Modifier;
use internal;
use triangulation;
use BackEnd;
use Context;
use ImageSize;
use Color;

/// A polygon
pub struct Polygon {
    /// The color of the polygon
    pub color: internal::Color,
}

impl Polygon {
    /// Creates new polygon
    pub fn new(color: internal::Color) -> Polygon {
        Polygon {
            color: color,
        }
    }

    /// Draws polygon
    pub fn draw<B: BackEnd<I>, I: ImageSize>(
        &self,
        polygon: internal::Polygon,
        c: &Context,
        back_end: &mut B
    ) {
        // Complete transparency does not need to be rendered.
        if self.color[3] == 0.0 { return; }
        back_end.color(self.color);
        triangulation::with_polygon_tri_list(
            c.transform,
            polygon,
            |vertices| back_end.tri_list(vertices)
        );
    }
}

impl Modifier<Polygon> for Color {
    fn modify(self, p: &mut Polygon) {
        let Color(val) = self;
        p.color = val;
    }
}

#[cfg(test)]
mod test {
    use current::Set;
    use super::Polygon;
    use Color;

    #[test]
    fn test_polygon() {
        let _polygon = Polygon::new([1.0, ..4])
            .set(Color([0.0, ..4]));
    }
}
