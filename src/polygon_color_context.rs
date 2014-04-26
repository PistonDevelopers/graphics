use {Field, Value, Borrowed, Matrix2d, Color};
use vecmath::{multiply, translate, rotate_radians, scale, shear};
use {Transform2d, Fill, BackEnd, Clear};
use triangulation::{with_polygon_tri_list_xy_rgba_f32};

/// A polygon color context.
pub struct PolygonColorContext<'a> {
    /// Base/origin transform.
    pub base: Field<'a, Matrix2d>,
    /// Current transform.
    pub transform: Field<'a, Matrix2d>,
    /// Current color.
    pub color: Field<'a, Color>,
    /// Current polygon.
    pub polygon: Field<'a, &'a [f64]>,
}

impl<'a> Transform2d<'a> for PolygonColorContext<'a> {
    #[inline(always)]
    fn trans(&'a self, x: f64, y: f64) -> PolygonColorContext<'a> {
        PolygonColorContext {
            base: Borrowed(self.base.get()),
            transform: {
                let trans = translate(x, y);
                Value(multiply(&trans, self.transform.get()))
            },
            polygon: Borrowed(self.polygon.get()),
            color: Borrowed(self.color.get()),
        }
    }

    #[inline(always)]
    fn trans_local(&'a self, x: f64, y: f64) -> PolygonColorContext<'a> {
        PolygonColorContext {
            base: Borrowed(self.base.get()),
            transform: {
                let trans = translate(x, y);
                Value(multiply(self.transform.get(), &trans))
            },
            polygon: Borrowed(self.polygon.get()),
            color: Borrowed(self.color.get()),
        }
    }
    
    #[inline(always)]
    fn rot_rad(&'a self, angle: f64) -> PolygonColorContext<'a> {
        PolygonColorContext {
            base: Borrowed(self.base.get()),
            transform: {
                let rot = rotate_radians(angle);
                Value(multiply(&rot, self.transform.get()))
            },
            polygon: Borrowed(self.polygon.get()),
            color: Borrowed(self.color.get()),
        }
    }

    #[inline(always)]
    fn rot_rad_local(&'a self, angle: f64) -> PolygonColorContext<'a> {
        PolygonColorContext {
            base: Borrowed(self.base.get()),
            transform: {
                let rot = rotate_radians(angle);
                Value(multiply(self.transform.get(), &rot))
            },
            polygon: Borrowed(self.polygon.get()),
            color: Borrowed(self.color.get()),
        }
    }

    #[inline(always)]
    fn scale(&'a self, sx: f64, sy: f64) -> PolygonColorContext<'a> {
        PolygonColorContext {
            base: Borrowed(self.base.get()),
            transform: {
                let scale = scale(sx, sy);
                Value(multiply(&scale, self.transform.get()))
            },
            polygon: Borrowed(self.polygon.get()),
            color: Borrowed(self.color.get()),
        }
    }

    #[inline(always)]
    fn scale_local(&'a self, sx: f64, sy: f64) -> PolygonColorContext<'a> {
        PolygonColorContext {
            base: Borrowed(self.base.get()),
            transform: {
                let scale = scale(sx, sy);
                Value(multiply(self.transform.get(), &scale))
            },
            polygon: Borrowed(self.polygon.get()),
            color: Borrowed(self.color.get()),
        }
    }

    #[inline(always)]
    fn shear(&'a self, sx: f64, sy: f64) -> PolygonColorContext<'a> {
        PolygonColorContext {
            base: Borrowed(self.base.get()),
            transform: {
                let shear = shear(sx, sy);
                Value(multiply(&shear, self.transform.get()))
            },
            polygon: Borrowed(self.polygon.get()),
            color: Borrowed(self.color.get()),
        }
    }
}

impl<'a> Fill<'a> for PolygonColorContext<'a> {
    fn fill<B: BackEnd>(&'a self, back_end: &mut B) {
        if back_end.supports_tri_list_xy_rgba_f32() {
            let polygon = self.polygon.get();
            let color = self.color.get();
            let color: [f32, ..4] = [color[0], color[1], color[2], color[3]];
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely opaque.
            let needs_alpha = color[3] != 1.0;
            if needs_alpha { back_end.enable_alpha_blend(); }
            with_polygon_tri_list_xy_rgba_f32(
                self.transform.get(),
                *polygon,
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

impl<'a> Clear for PolygonColorContext<'a> {
    fn clear<B: BackEnd>(&self, back_end: &mut B) {
        if back_end.supports_clear_rgba() {
            let color = self.color.get();
            back_end.clear_rgba(color[0], color[1], color[2], color[3]);
        } else {
            unimplemented!();
        }
    }
}
