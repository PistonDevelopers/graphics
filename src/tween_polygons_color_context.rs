use {
    BackEnd,
    Borrowed,
    Clear,
    Field,
    Fill,
    Value,
};
use triangulation::{
    with_lerp_polygons_tri_list_xy_f32_rgba_f32
};
use internal::{
    CanColor,
    CanTransform,
    CanViewTransform,
    Color,
    HasColor,
    HasTransform,
    HasViewTransform,
    Matrix2d,
    Polygons,
    Scalar,
};

/// An animation inbetweening context with color.
pub struct TweenPolygonsColorContext<'a, 'b> {
    /// Base/origin transform.
    pub base: Field<'a, Matrix2d>,
    /// Current transform.
    pub transform: Field<'a, Matrix2d>,
    /// Current color.
    pub color: Field<'a, Color>,
    /// Animation inbetweening factor.
    pub tween_factor: Field<'a, Scalar>,
    /// The animated polygons.
    pub polygons: Field<'a, Polygons<'b>>,
}

impl<'a, 'b> Clone for TweenPolygonsColorContext<'a, 'b> {
    #[inline(always)]
    fn clone(&self) -> TweenPolygonsColorContext<'static, 'b> {
        TweenPolygonsColorContext {
            base: Value(*self.base.get()),
            transform: Value(*self.transform.get()),
            color: Value(*self.color.get()),
            tween_factor: Value(*self.tween_factor.get()),
            polygons: Value(*self.polygons.get()),
        }
    }
}

impl<'a, 'b> HasColor<'a, Color> for TweenPolygonsColorContext<'a, 'b> {
    #[inline(always)]
    fn get_color(&'a self) -> &'a Color {
        self.color.get()
    }
}

impl<'a, 'b> CanColor<'a, TweenPolygonsColorContext<'a, 'b>, Color> for TweenPolygonsColorContext<'a, 'b> {
    #[inline(always)]
    fn color(&'a self, value: Color) -> TweenPolygonsColorContext<'a, 'b> {
        TweenPolygonsColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            color: Value(value),
            tween_factor: Borrowed(self.tween_factor.get()),
            polygons: Borrowed(self.polygons.get()),
        }
    }
}

impl<'a, 'b> HasTransform<'a, Matrix2d> for TweenPolygonsColorContext<'a, 'b> {
    #[inline(alwyas)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a, 'b> CanTransform<'a, TweenPolygonsColorContext<'a, 'b>, Matrix2d> for TweenPolygonsColorContext<'a, 'b> {
    #[inline(always)]
    fn transform(&'a self, value: Matrix2d) -> TweenPolygonsColorContext<'a, 'b> {
        TweenPolygonsColorContext {
            base: Borrowed(self.base.get()),
            transform: Value(value),
            color: Borrowed(self.color.get()),
            tween_factor: Borrowed(self.tween_factor.get()),
            polygons: Borrowed(self.polygons.get()),
        }
    }
}

impl<'a, 'b> HasViewTransform<'a, Matrix2d> for TweenPolygonsColorContext<'a, 'b> {
    #[inline(always)]
    fn get_view_transform(&'a self) -> &'a Matrix2d {
        self.base.get()
    }
}

impl<'a, 'b> CanViewTransform<'a, TweenPolygonsColorContext<'a, 'b>, Matrix2d> 
for TweenPolygonsColorContext<'a, 'b> {
    #[inline(always)]
    fn view_transform(&'a self, value: Matrix2d) -> TweenPolygonsColorContext<'a, 'b> {
        TweenPolygonsColorContext {
            base: Value(value),
            transform: Borrowed(self.transform.get()),
            tween_factor: Borrowed(self.tween_factor.get()),
            polygons: Borrowed(self.polygons.get()),
            color: Borrowed(self.color.get()),
        }
    }
}


impl<'a, 'b> Fill<'a> for TweenPolygonsColorContext<'a, 'b> {
    #[inline(always)]
    fn fill<B: BackEnd>(&'a self, back_end: &mut B) {
        if back_end.supports_tri_list_xy_f32_rgba_f32() {
            let polygons = self.polygons.get();
            let color = self.color.get();
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely opaque.
            let needs_alpha = color[3] != 1.0;
            if needs_alpha { back_end.enable_alpha_blend(); }
            with_lerp_polygons_tri_list_xy_f32_rgba_f32(
                *self.transform.get(),
                *polygons,
                *self.tween_factor.get(),
                *color,
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

impl<'a, 'b> Clear for TweenPolygonsColorContext<'a, 'b> {
    #[inline(always)]
    fn clear<B: BackEnd>(&self, back_end: &mut B) {
        if back_end.supports_clear_rgba() {
            let color = self.color.get();
            back_end.clear_rgba(color[0], color[1], color[2], color[3]);
        }
    }
}

