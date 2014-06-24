use {
    BackEnd,
    Borrowed,
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
pub struct PolygonColorContext<'a, 'b> {
    /// View transform.
    pub view: Field<'a, Matrix2d>,
    /// Current transform.
    pub transform: Field<'a, Matrix2d>,
    /// Current color.
    pub color: Field<'a, Color>,
    /// Current polygon.
    pub polygon: Field<'a, Polygon<'b>>,
}

impl<'a, 'b> 
Clone 
for PolygonColorContext<'a, 'b> {
    #[inline(always)]
    fn clone(&self) -> PolygonColorContext<'static, 'b> {
        PolygonColorContext {
            view: Value(*self.view.get()),
            transform: Value(*self.transform.get()),
            polygon: Value(*self.polygon.get()),
            color: Value(*self.color.get()),
        }
    }
}

impl<'a, 'b> 
HasTransform<'a, Matrix2d> 
for PolygonColorContext<'a, 'b> {
    #[inline(always)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a, 'b> 
CanTransform<'a, PolygonColorContext<'a, 'b>, Matrix2d> 
for PolygonColorContext<'a, 'b> {
    #[inline(always)]
    fn transform(
        &'a self, 
        value: Matrix2d
    ) -> PolygonColorContext<'a, 'b> {
        PolygonColorContext {
            view: Borrowed(self.view.get()),
            transform: Value(value),
            polygon: Borrowed(self.polygon.get()),
            color: Borrowed(self.color.get()),
        }
    }
}

impl<'a, 'b> 
HasViewTransform<'a, Matrix2d> 
for PolygonColorContext<'a, 'b> {
    #[inline(always)]
    fn get_view_transform(&'a self) -> &'a Matrix2d {
        self.view.get()
    }
}

impl<'a, 'b> 
CanViewTransform<'a, PolygonColorContext<'a, 'b>, Matrix2d> 
for PolygonColorContext<'a, 'b> {
    #[inline(always)]
    fn view_transform(
        &'a self, 
        value: Matrix2d
    ) -> PolygonColorContext<'a, 'b> {
        PolygonColorContext {
            view: Value(value),
            transform: Borrowed(self.transform.get()),
            polygon: Borrowed(self.polygon.get()),
            color: Borrowed(self.color.get()),
        }
    }
}

impl<'a, 'b> 
HasColor<'a, Color> 
for PolygonColorContext<'a, 'b> {
    #[inline(always)]
    fn get_color(&'a self) -> &'a Color {
        self.color.get()
    }
}

impl<'a, 'b> 
CanColor<'a, PolygonColorContext<'a, 'b>, Color> 
for PolygonColorContext<'a, 'b> {
    #[inline(always)]
    fn color(&'a self, value: Color) -> PolygonColorContext<'a, 'b> {
        PolygonColorContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            color: Value(value),
            polygon: Borrowed(self.polygon.get()),
        }
    }
}

impl<'a, 'b, B: BackEnd<I>, I: ImageSize> 
Draw<'a, B, I> 
for PolygonColorContext<'a, 'b> {
    #[inline(always)]
    fn draw(&'a self, back_end: &mut B) {
        if back_end.supports_tri_list_xy_f32_rgba_f32() {
            let polygon = self.polygon.get();
            let color = self.color.get();
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely opaque.
            let needs_alpha = color[3] != 1.0;
            if needs_alpha { back_end.enable_alpha_blend(); }
            with_polygon_tri_list_xy_f32_rgba_f32(
                *self.transform.get(),
                *polygon,
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

