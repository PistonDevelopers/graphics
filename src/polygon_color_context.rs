use {
    BackEnd,
    Draw,
    ImageSize,
};
use triangulation::{
    with_polygon_tri_list_xy_f32_rgba_f32
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
    Polygon,
};

/// A polygon color context.
pub struct PolygonColorContext<'b> {
    /// View transform.
    pub view: Matrix2d,
    /// Current transform.
    pub transform: Matrix2d,
    /// Current color.
    pub color: Color,
    /// Current polygon.
    pub polygon: Polygon<'b>,
}

impl<'b> 
Clone 
for PolygonColorContext<'b> {
    #[inline(always)]
    fn clone(&self) -> PolygonColorContext<'b> {
        PolygonColorContext {
            view: self.view,
            transform: self.transform,
            polygon: self.polygon,
            color: self.color,
        }
    }
}

impl<'b> 
HasTransform<Matrix2d> 
for PolygonColorContext<'b> {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform
    }
}

impl<'b> 
CanTransform<PolygonColorContext<'b>, Matrix2d> 
for PolygonColorContext<'b> {
    #[inline(always)]
    fn transform(
        &self, 
        value: Matrix2d
    ) -> PolygonColorContext<'b> {
        PolygonColorContext {
            view: self.view,
            transform: value,
            polygon: self.polygon,
            color: self.color,
        }
    }
}

impl<'b> 
HasViewTransform<Matrix2d> 
for PolygonColorContext<'b> {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view
    }
}

impl<'b> 
CanViewTransform<PolygonColorContext<'b>, Matrix2d> 
for PolygonColorContext<'b> {
    #[inline(always)]
    fn view_transform(
        &self, 
        value: Matrix2d
    ) -> PolygonColorContext<'b> {
        PolygonColorContext {
            view: value,
            transform: self.transform,
            polygon: self.polygon,
            color: self.color,
        }
    }
}

impl<'b> 
HasColor<Color> 
for PolygonColorContext<'b> {
    #[inline(always)]
    fn get_color(&self) -> Color {
        self.color
    }
}

impl<'b> 
CanColor<PolygonColorContext<'b>, Color> 
for PolygonColorContext<'b> {
    #[inline(always)]
    fn color(&self, value: Color) -> PolygonColorContext<'b> {
        PolygonColorContext {
            view: self.view,
            transform: self.transform,
            color: value,
            polygon: self.polygon,
        }
    }
}

impl<'b, B: BackEnd<I>, I: ImageSize> 
Draw<B, I> 
for PolygonColorContext<'b> {
    #[inline(always)]
    fn draw(&self, back_end: &mut B) {
        if back_end.supports_tri_list_xy_f32_rgba_f32() {
            let polygon = self.polygon;
            let color = self.color;
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely opaque.
            let needs_alpha = color[3] != 1.0;
            if needs_alpha { back_end.enable_alpha_blend(); }
            with_polygon_tri_list_xy_f32_rgba_f32(
                self.transform,
                polygon,
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

