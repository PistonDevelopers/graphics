use {
    BackEnd,
    Draw,
    ImageSize,
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
    pub view: Matrix2d,
    /// Current transform.
    pub transform: Matrix2d,
    /// Current color.
    pub color: Color,
    /// Animation inbetweening factor.
    pub tween_factor: Scalar,
    /// The animated polygons.
    pub polygons: Polygons<'b>,
}

impl<'b> 
Clone 
for LerpTweenPolygonsColorContext<'b> {
    #[inline(always)]
    fn clone(&self) -> LerpTweenPolygonsColorContext<'b> {
        LerpTweenPolygonsColorContext {
            view: self.view,
            transform: self.transform,
            color: self.color,
            tween_factor: self.tween_factor,
            polygons: self.polygons,
        }
    }
}

impl<'b> 
HasColor<Color> 
for LerpTweenPolygonsColorContext<'b> {
    #[inline(always)]
    fn get_color(&self) -> Color {
        self.color
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
            view: self.view,
            transform: self.transform,
            color: value,
            tween_factor: self.tween_factor,
            polygons: self.polygons,
        }
    }
}

impl<'b> 
HasTransform<Matrix2d> 
for LerpTweenPolygonsColorContext<'b> {
    #[inline(alwyas)]
    fn get_transform(&self) -> Matrix2d {
        self.transform
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
            view: self.view,
            transform: value,
            color: self.color,
            tween_factor: self.tween_factor,
            polygons: self.polygons,
        }
    }
}

impl<'b> 
HasViewTransform<Matrix2d> 
for LerpTweenPolygonsColorContext<'b> {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view
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
            view: value,
            transform: self.transform,
            tween_factor: self.tween_factor,
            polygons: self.polygons,
            color: self.color,
        }
    }
}


impl<'b, B: BackEnd<I>, I: ImageSize> 
Draw<B, I> 
for LerpTweenPolygonsColorContext<'b> {
    #[inline(always)]
    fn draw(&self, back_end: &mut B) {
        if back_end.supports_tri_list_xy_f32_rgba_f32() {
            let polygons = self.polygons;
            let color = self.color;
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely opaque.
            let needs_alpha = color[3] != 1.0;
            if needs_alpha { back_end.enable_alpha_blend(); }
            with_lerp_polygons_tri_list_xy_f32_rgba_f32(
                self.transform,
                polygons,
                self.tween_factor,
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

