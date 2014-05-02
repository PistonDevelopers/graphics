use {Field, Color, Matrix2d};
use {Fill, BackEnd};
use triangulation::{
    with_lerp_polygons_tri_list_xy_f32_rgba_f32
};

/// An animation inbetweening context with color.
pub struct TweenPolygonsColorContext<'a> {
    /// Base/origin transform.
    pub base: Field<'a, Matrix2d>,
    /// Current transform.
    pub transform: Field<'a, Matrix2d>,
    /// Current color.
    pub color: Field<'a, Color>,
    /// Animation inbetweening factor.
    pub tween_factor: Field<'a, f64>,
    /// The animated polygons.
    pub polygons: Field<'a, &'a [&'a [f64]]>,
}

impl<'a> Fill<'a> for TweenPolygonsColorContext<'a> {
    fn fill<B: BackEnd>(&'a self, back_end: &mut B) {
        if back_end.supports_tri_list_xy_rgba_f32() {
            let polygons = self.polygons.get();
            let color = self.color.get();
            let color: [f32, ..4] = [color[0], color[1], color[2], color[3]];
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely opaque.
            let needs_alpha = color[3] != 1.0;
            if needs_alpha { back_end.enable_alpha_blend(); }
            with_lerp_polygons_tri_list_xy_f32_rgba_f32(
                self.transform.get(),
                *polygons,
                *self.tween_factor.get(),                
                color,
                |vertices, colors| {
                    back_end.tri_list_xy_rgba_f32(vertices, colors)
                }
            );
            if needs_alpha { back_end.disable_alpha_blend(); }
        } else {
            unimplemented!();
        }
    }
}
