use {
    AddPolygons,
    BackEnd,
    Borrowed,
    Clear,
    Field,
    Image,
    LerpTweenPolygonsColorContext,
    Value,
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
};

/// An animation inbetweening context with color.
pub struct LerpTweenColorContext<'a> {
    /// View transform.
    pub view: Field<'a, Matrix2d>,
    /// Current transform.
    pub transform: Field<'a, Matrix2d>,
    /// Current color.
    pub color: Field<'a, Color>,
    /// Animation inbetweening factor.
    pub tween_factor: Field<'a, f64>,
}

impl<'a> Clone for LerpTweenColorContext<'a> {
    #[inline(always)]
    fn clone(&self) -> LerpTweenColorContext<'static> {
        LerpTweenColorContext {
            view: Value(*self.view.get()),
            transform: Value(*self.transform.get()),
            color: Value(*self.color.get()),
            tween_factor: Value(*self.tween_factor.get()),
        }
    }
}

impl<'a> HasColor<'a, Color> for LerpTweenColorContext<'a> {
    #[inline(always)]
    fn get_color(&'a self) -> &'a Color {
        self.color.get()
    }
}

impl<'a> CanColor<'a, LerpTweenColorContext<'a>, Color> for LerpTweenColorContext<'a> {
    #[inline(always)]
    fn color(&'a self, value: Color) -> LerpTweenColorContext<'a> {
        LerpTweenColorContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            color: Value(value),
            tween_factor: Borrowed(self.tween_factor.get()),
        }
    }
}

impl<'a> HasTransform<'a, Matrix2d> for LerpTweenColorContext<'a> {
    #[inline(always)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a> CanTransform<'a, LerpTweenColorContext<'a>, Matrix2d> for LerpTweenColorContext<'a> {
    #[inline(always)]
    fn transform(&'a self, value: Matrix2d) -> LerpTweenColorContext<'a> {
        LerpTweenColorContext {
            view: Borrowed(self.view.get()),
            transform: Value(value),
            color: Borrowed(self.color.get()),
            tween_factor: Borrowed(self.tween_factor.get()),
        }
    }
}

impl<'a> HasViewTransform<'a, Matrix2d> for LerpTweenColorContext<'a> {
    #[inline(always)]
    fn get_view_transform(&'a self) -> &'a Matrix2d {
        self.view.get()
    }
}

impl<'a> CanViewTransform<'a, LerpTweenColorContext<'a>, Matrix2d> for LerpTweenColorContext<'a> {
    #[inline(always)]
    fn view_transform(&'a self, value: Matrix2d) -> LerpTweenColorContext<'a> {
        LerpTweenColorContext {
            view: Value(value),
            transform: Borrowed(self.transform.get()),
            tween_factor: Borrowed(self.tween_factor.get()),
            color: Borrowed(self.color.get()),
        }
    }
}


impl<'a, 'b> AddPolygons<'a, LerpTweenPolygonsColorContext<'a, 'b>> for LerpTweenColorContext<'a> {
    #[inline(always)]
    fn polygons(&'a self, polygons: Polygons<'b>) -> LerpTweenPolygonsColorContext<'a, 'b> {
        LerpTweenPolygonsColorContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            color: Borrowed(self.color.get()),
            tween_factor: Borrowed(self.tween_factor.get()),
            polygons: Value(polygons),
        }
    }
}

impl<'a, B: BackEnd<I>, I: Image> Clear<B, I> for LerpTweenColorContext<'a> {
    #[inline(always)]
    fn clear(&self, back_end: &mut B) {
        if back_end.supports_clear_rgba() {
            let color = self.color.get();
            back_end.clear_rgba(color[0], color[1], color[2], color[3]);
        }
    }
}

