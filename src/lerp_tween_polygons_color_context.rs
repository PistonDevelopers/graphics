use {
    BackEnd,
    Field,
    Draw,
    ImageSize,
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
pub struct LerpTweenPolygonsColorContext<'b> {
    /// View transform.
    pub view: Field<Matrix2d>,
    /// Current transform.
    pub transform: Field<Matrix2d>,
    /// Current color.
    pub color: Field<Color>,
    /// Animation inbetweening factor.
    pub tween_factor: Field<Scalar>,
    /// The animated polygons.
    pub polygons: Field<Polygons<'b>>,
}

impl<'b> 
Clone 
for LerpTweenPolygonsColorContext<'b> {
    #[inline(always)]
    fn clone(&self) -> LerpTweenPolygonsColorContext<'b> {
        LerpTweenPolygonsColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            color: Value(self.color.get()),
            tween_factor: Value(self.tween_factor.get()),
            polygons: Value(self.polygons.get()),
        }
    }
}

impl<'b> 
HasColor<Color> 
for LerpTweenPolygonsColorContext<'b> {
    #[inline(always)]
    fn get_color(&self) -> Color {
        self.color.get()
    }
}

impl<'b> 
CanColor<LerpTweenPolygonsColorContext<'b>, Color> 
for LerpTweenPolygonsColorContext<'b> {
    #[inline(always)]
    fn color(
        &self, 
        value: Color
    ) -> LerpTweenPolygonsColorContext<'b> {
        LerpTweenPolygonsColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            color: Value(value),
            tween_factor: Value(self.tween_factor.get()),
            polygons: Value(self.polygons.get()),
        }
    }
}

impl<'b> 
HasTransform<Matrix2d> 
for LerpTweenPolygonsColorContext<'b> {
    #[inline(alwyas)]
    fn get_transform(&self) -> Matrix2d {
        self.transform.get()
    }
}

impl<'b> 
CanTransform<LerpTweenPolygonsColorContext<'b>, Matrix2d> 
for LerpTweenPolygonsColorContext<'b> {
    #[inline(always)]
    fn transform(
        &self, 
        value: Matrix2d
    ) -> LerpTweenPolygonsColorContext<'b> {
        LerpTweenPolygonsColorContext {
            view: Value(self.view.get()),
            transform: Value(value),
            color: Value(self.color.get()),
            tween_factor: Value(self.tween_factor.get()),
            polygons: Value(self.polygons.get()),
        }
    }
}

impl<'b> 
HasViewTransform<Matrix2d> 
for LerpTweenPolygonsColorContext<'b> {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view.get()
    }
}

impl<'b> 
CanViewTransform<LerpTweenPolygonsColorContext<'b>, Matrix2d> 
for LerpTweenPolygonsColorContext<'b> {
    #[inline(always)]
    fn view_transform(
        &self, 
        value: Matrix2d
    ) -> LerpTweenPolygonsColorContext<'b> {
        LerpTweenPolygonsColorContext {
            view: Value(value),
            transform: Value(self.transform.get()),
            tween_factor: Value(self.tween_factor.get()),
            polygons: Value(self.polygons.get()),
            color: Value(self.color.get()),
        }
    }
}


impl<'b, B: BackEnd<I>, I: ImageSize> 
Draw<B, I> 
for LerpTweenPolygonsColorContext<'b> {
    #[inline(always)]
    fn draw(&self, back_end: &mut B) {
        if back_end.supports_tri_list_xy_f32_rgba_f32() {
            let polygons = self.polygons.get();
            let color = self.color.get();
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely opaque.
            let needs_alpha = color[3] != 1.0;
            if needs_alpha { back_end.enable_alpha_blend(); }
            with_lerp_polygons_tri_list_xy_f32_rgba_f32(
                self.transform.get(),
                polygons,
                self.tween_factor.get(),
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

