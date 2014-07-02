use {
    BackEnd,
    Field,
    Draw,
    ImageSize,
    Value,
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
    pub view: Field<Matrix2d>,
    /// Current transform.
    pub transform: Field<Matrix2d>,
    /// Current color.
    pub color: Field<Color>,
    /// Current polygon.
    pub polygon: Field<Polygon<'b>>,
}

impl<'b> 
Clone 
for PolygonColorContext<'b> {
    #[inline(always)]
    fn clone(&self) -> PolygonColorContext<'b> {
        PolygonColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            polygon: Value(self.polygon.get()),
            color: Value(self.color.get()),
        }
    }
}

impl<'b> 
HasTransform<Matrix2d> 
for PolygonColorContext<'b> {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform.get()
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
            view: Value(self.view.get()),
            transform: Value(value),
            polygon: Value(self.polygon.get()),
            color: Value(self.color.get()),
        }
    }
}

impl<'b> 
HasViewTransform<Matrix2d> 
for PolygonColorContext<'b> {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view.get()
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
            view: Value(value),
            transform: Value(self.transform.get()),
            polygon: Value(self.polygon.get()),
            color: Value(self.color.get()),
        }
    }
}

impl<'b> 
HasColor<Color> 
for PolygonColorContext<'b> {
    #[inline(always)]
    fn get_color(&self) -> Color {
        self.color.get()
    }
}

impl<'b> 
CanColor<PolygonColorContext<'b>, Color> 
for PolygonColorContext<'b> {
    #[inline(always)]
    fn color(&self, value: Color) -> PolygonColorContext<'b> {
        PolygonColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            color: Value(value),
            polygon: Value(self.polygon.get()),
        }
    }
}

impl<'b, B: BackEnd<I>, I: ImageSize> 
Draw<B, I> 
for PolygonColorContext<'b> {
    #[inline(always)]
    fn draw(&self, back_end: &mut B) {
        if back_end.supports_tri_list_xy_f32_rgba_f32() {
            let polygon = self.polygon.get();
            let color = self.color.get();
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely opaque.
            let needs_alpha = color[3] != 1.0;
            if needs_alpha { back_end.enable_alpha_blend(); }
            with_polygon_tri_list_xy_f32_rgba_f32(
                self.transform.get(),
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

