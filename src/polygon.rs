//! Draw polygon

use types::Color;
use {types, triangulation, Graphics, DrawState};
use math::{Matrix2d, Scalar};

/// A polygon
#[derive(Copy, Clone)]
pub struct Polygon {
    /// The color of the polygon
    pub color: Color,
}

impl Polygon {
    /// Creates new polygon
    pub fn new(color: Color) -> Polygon {
        Polygon { color: color }
    }

    /// Sets color.
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Draws polygon using the default method.
    #[inline(always)]
    pub fn draw<G>(&self,
                   polygon: types::Polygon,
                   draw_state: &DrawState,
                   transform: Matrix2d,
                   g: &mut G)
        where G: Graphics
    {
        g.polygon(self, polygon, draw_state, transform);
    }

    /// Draws polygon using triangulation.
    pub fn draw_tri<G>(&self,
                       polygon: types::Polygon,
                       draw_state: &DrawState,
                       transform: Matrix2d,
                       g: &mut G)
        where G: Graphics
    {
        g.tri_list(draw_state, &self.color, |f| {
            triangulation::with_polygon_tri_list(transform, polygon, |vertices| f(vertices))
        });
    }

    /// Draws tweened polygon with linear interpolation, using default method.
    #[inline(always)]
    pub fn draw_tween_lerp<G>(&self,
                              polygons: types::Polygons,
                              tween_factor: Scalar,
                              draw_state: &DrawState,
                              transform: Matrix2d,
                              g: &mut G)
        where G: Graphics
    {
        g.polygon_tween_lerp(self, polygons, tween_factor, draw_state, transform);
    }

    /// Draws tweened polygon with linear interpolation, using triangulation.
    pub fn draw_tween_lerp_tri<G>(&self,
                                  polygons: types::Polygons,
                                  tween_factor: Scalar,
                                  draw_state: &DrawState,
                                  transform: Matrix2d,
                                  g: &mut G)
        where G: Graphics
    {
        if self.color[3] == 0.0 {
            return;
        }
        g.tri_list(draw_state, &self.color, |f| {
            triangulation::with_lerp_polygons_tri_list(transform,
                                                       polygons,
                                                       tween_factor,
                                                       |vertices| f(vertices))
        });
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_polygon() {
        let _polygon = Polygon::new([1.0; 4]).color([0.0; 4]);
    }
}
