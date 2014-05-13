use {
    BackEnd,
    Borrowed,
    Clear,
    Color,
    Field,
    Fill,
    Matrix2d,
    Value,
    View,
};
use vecmath::{
    identity,
};
use triangulation::{
    with_polygon_tri_list_xy_f32_rgba_f32
};
use internal::{
    CanColor,
    CanTransform,
    HasColor,
    HasTransform,
};

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

impl<'a> HasTransform<'a, Matrix2d> for PolygonColorContext<'a> {
    #[inline(always)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a> CanTransform<'a, PolygonColorContext<'a>, Matrix2d> for PolygonColorContext<'a> {
    #[inline(always)]
    fn transform(&'a self, value: Matrix2d) -> PolygonColorContext<'a> {
        PolygonColorContext {
            base: Borrowed(self.base.get()),
            transform: Value(value),
            polygon: Borrowed(self.polygon.get()),
            color: Borrowed(self.color.get()),
        }
    }
}

impl<'a> HasColor<'a, Color> for PolygonColorContext<'a> {
    #[inline(always)]
    fn get_color(&'a self) -> &'a Color {
        self.color.get()
    }
}

impl<'a> CanColor<'a, PolygonColorContext<'a>, Color> for PolygonColorContext<'a> {
    #[inline(always)]
    fn color(&'a self, value: Color) -> PolygonColorContext<'a> {
        PolygonColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            color: Value(value),
            polygon: Borrowed(self.polygon.get()),
        }
    }
}

impl<'a> Fill<'a> for PolygonColorContext<'a> {
    fn fill<B: BackEnd>(&'a self, back_end: &mut B) {
        if back_end.supports_tri_list_xy_f32_rgba_f32() {
            let polygon = self.polygon.get();
            let color = self.color.get();
            let color: [f32, ..4] = [color[0], color[1], color[2], color[3]];
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely opaque.
            let needs_alpha = color[3] != 1.0;
            if needs_alpha { back_end.enable_alpha_blend(); }
            with_polygon_tri_list_xy_f32_rgba_f32(
                self.transform.get(),
                *polygon,
                color,
                |vertices, colors| {
                    back_end.tri_list_xy_f32_rgba_f32(vertices, colors)
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

impl<'a> View<'a> for PolygonColorContext<'a> {
    #[inline(always)]
    fn view(&'a self) -> PolygonColorContext<'a> {
        PolygonColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.base.get()),
            polygon: Borrowed(self.polygon.get()),
            color: Borrowed(self.color.get()),
        }
    }

    #[inline(always)]
    fn reset(&'a self) -> PolygonColorContext<'a> {
        PolygonColorContext {
            base: Borrowed(self.base.get()),
            transform: Value(identity()),
            polygon: Borrowed(self.polygon.get()),
            color: Borrowed(self.color.get()),
        }
    }

    #[inline(always)]
    fn store_view(&'a self) -> PolygonColorContext<'a> {
        PolygonColorContext {
            base: Borrowed(self.transform.get()),
            transform: Borrowed(self.transform.get()),
            polygon: Borrowed(self.polygon.get()),
            color: Borrowed(self.color.get()),
        }
    }
}

