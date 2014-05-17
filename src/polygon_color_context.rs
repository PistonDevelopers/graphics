use {
    BackEnd,
    Borrowed,
    Clear,
    Color,
    Field,
    Fill,
    Matrix2d,
    Value,
};
use triangulation::{
    with_polygon_tri_list_xy_f32_rgba_f32
};
use internal::{
    CanColor,
    CanTransform,
    CanViewTransform,
    HasColor,
    HasTransform,
    HasViewTransform,
};

/// A polygon color context.
pub struct PolygonColorContext<'a, 'b> {
    /// Base/origin transform.
    pub base: Field<'a, Matrix2d>,
    /// Current transform.
    pub transform: Field<'a, Matrix2d>,
    /// Current color.
    pub color: Field<'a, Color>,
    /// Current polygon.
    pub polygon: Field<'a, &'b [f64]>,
}

impl<'a, 'b> Clone for PolygonColorContext<'a, 'b> {
    #[inline(always)]
    fn clone(&self) -> PolygonColorContext<'static, 'b> {
        PolygonColorContext {
            base: self.base.clone(),
            transform: self.transform.clone(),
            polygon: Value(*self.polygon.get()),
            color: self.color.clone(),
        }
    }
}

impl<'a, 'b> HasTransform<'a, Matrix2d> for PolygonColorContext<'a, 'b> {
    #[inline(always)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a, 'b> CanTransform<'a, PolygonColorContext<'a, 'b>, Matrix2d> for PolygonColorContext<'a, 'b> {
    #[inline(always)]
    fn transform(&'a self, value: Matrix2d) -> PolygonColorContext<'a, 'b> {
        PolygonColorContext {
            base: Borrowed(self.base.get()),
            transform: Value(value),
            polygon: Borrowed(self.polygon.get()),
            color: Borrowed(self.color.get()),
        }
    }
}

impl<'a, 'b> HasViewTransform<'a, Matrix2d> for PolygonColorContext<'a, 'b> {
    #[inline(always)]
    fn get_view_transform(&'a self) -> &'a Matrix2d {
        self.base.get()
    }
}

impl<'a, 'b> CanViewTransform<'a, PolygonColorContext<'a, 'b>, Matrix2d> 
for PolygonColorContext<'a, 'b> {
    #[inline(always)]
    fn view_transform(&'a self, value: Matrix2d) -> PolygonColorContext<'a, 'b> {
        PolygonColorContext {
            base: Value(value),
            transform: Borrowed(self.transform.get()),
            polygon: Borrowed(self.polygon.get()),
            color: Borrowed(self.color.get()),
        }
    }
}

impl<'a, 'b> HasColor<'a, Color> for PolygonColorContext<'a, 'b> {
    #[inline(always)]
    fn get_color(&'a self) -> &'a Color {
        self.color.get()
    }
}

impl<'a, 'b> CanColor<'a, PolygonColorContext<'a, 'b>, Color> for PolygonColorContext<'a, 'b> {
    #[inline(always)]
    fn color(&'a self, value: Color) -> PolygonColorContext<'a, 'b> {
        PolygonColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            color: Value(value),
            polygon: Borrowed(self.polygon.get()),
        }
    }
}

impl<'a, 'b> Fill<'a> for PolygonColorContext<'a, 'b> {
    #[inline(always)]
    fn fill<B: BackEnd>(&'a self, back_end: &mut B) {
        if back_end.supports_tri_list_xy_f32_rgba_f32() {
            let polygon = self.polygon.get().as_slice();
            let &Color(color) = self.color.get();
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely opaque.
            let needs_alpha = color[3] != 1.0;
            if needs_alpha { back_end.enable_alpha_blend(); }
            with_polygon_tri_list_xy_f32_rgba_f32(
                self.transform.get(),
                polygon,
                &Color(color),
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

impl<'a, 'b> Clear for PolygonColorContext<'a, 'b> {
    #[inline(always)]
    fn clear<B: BackEnd>(&self, back_end: &mut B) {
        if back_end.supports_clear_rgba() {
            let &Color(color) = self.color.get();
            back_end.clear_rgba(color[0], color[1], color[2], color[3]);
        } else {
            unimplemented!();
        }
    }
}

