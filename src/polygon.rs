//! Draw polygon

use quack::{ GetFrom, SetAt };
use internal;
use triangulation;
use BackEnd;
use Context;
use ImageSize;
use Color;

/// A polygon
#[deriving(Copy, Clone)]
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
        if self.color[3] == 0.0 { return; }
        back_end.color(self.color);
        triangulation::with_polygon_tri_list(
            c.transform,
            polygon,
            |vertices| back_end.tri_list(vertices)
        );
    }

    /// Draws tweened polygon with linear interpolation
    pub fn draw_tween_lerp<B: BackEnd<I>, I: ImageSize>(
        &self,
        polygons: internal::Polygons,
        tween_factor: internal::Scalar,
        c: &Context,
        back_end: &mut B
    ) {
        if self.color[3] == 0.0 { return; }
        back_end.color(self.color);
        triangulation::with_lerp_polygons_tri_list(
            c.transform,
            polygons,
            tween_factor,
            |vertices| back_end.tri_list(vertices)
        );
    }
}

impl SetAt<Polygon> for Color {
    #[inline(always)]
    fn set_at(self, p: &mut Polygon) {
        let Color(val) = self;
        p.color = val;
    }
}

impl GetFrom<Polygon> for Color {
    #[inline(always)]
    fn get_from(obj: &Polygon) -> Color {
        Color(obj.color)
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
