use {
    AddPolygons,
    BackEnd,
    Borrowed,
    Clear,
    Color,
    Field,
    Matrix2d,
    Polygons,
    TweenPolygonsColorContext,
    Value,
};
use internal::{
    CanColor,
    CanTransform,
    CanViewTransform,
    HasColor,
    HasTransform,
    HasViewTransform,
};

/// An animation inbetweening context with color.
pub struct TweenColorContext<'a> {
    /// Base/origin transform.
    pub base: Field<'a, Matrix2d>,
    /// Current transform.
    pub transform: Field<'a, Matrix2d>,
    /// Current color.
    pub color: Field<'a, Color>,
    /// Animation inbetweening factor.
    pub tween_factor: Field<'a, f64>,
}

impl<'a> Clone for TweenColorContext<'a> {
    #[inline(always)]
    fn clone(&self) -> TweenColorContext<'static> {
        TweenColorContext {
            base: self.base.clone(),
            transform: self.transform.clone(),
            color: self.color.clone(),
            tween_factor: self.tween_factor.clone(),
        }
    }
}

impl<'a> HasColor<'a, Color> for TweenColorContext<'a> {
    #[inline(always)]
    fn get_color(&'a self) -> &'a Color {
        self.color.get()
    }
}

impl<'a> CanColor<'a, TweenColorContext<'a>, Color> for TweenColorContext<'a> {
    #[inline(always)]
    fn color(&'a self, value: Color) -> TweenColorContext<'a> {
        TweenColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            color: Value(value),
            tween_factor: Borrowed(self.tween_factor.get()),
        }
    }
}

impl<'a> HasTransform<'a, Matrix2d> for TweenColorContext<'a> {
    #[inline(always)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a> CanTransform<'a, TweenColorContext<'a>, Matrix2d> for TweenColorContext<'a> {
    #[inline(always)]
    fn transform(&'a self, value: Matrix2d) -> TweenColorContext<'a> {
        TweenColorContext {
            base: Borrowed(self.base.get()),
            transform: Value(value),
            color: Borrowed(self.color.get()),
            tween_factor: Borrowed(self.tween_factor.get()),
        }
    }
}

impl<'a> HasViewTransform<'a, Matrix2d> for TweenColorContext<'a> {
    #[inline(always)]
    fn get_view_transform(&'a self) -> &'a Matrix2d {
        self.base.get()
    }
}

impl<'a> CanViewTransform<'a, TweenColorContext<'a>, Matrix2d> for TweenColorContext<'a> {
    #[inline(always)]
    fn view_transform(&'a self, value: Matrix2d) -> TweenColorContext<'a> {
        TweenColorContext {
            base: Value(value),
            transform: Borrowed(self.transform.get()),
            tween_factor: Borrowed(self.tween_factor.get()),
            color: Borrowed(self.color.get()),
        }
    }
}


impl<'a, 'b> AddPolygons<'a, TweenPolygonsColorContext<'a, 'b>> for TweenColorContext<'a> {
    #[inline(always)]
    fn polygons(&'a self, polygons: Polygons<'b>) -> TweenPolygonsColorContext<'a, 'b> {
        TweenPolygonsColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            color: Borrowed(self.color.get()),
            tween_factor: Borrowed(self.tween_factor.get()),
            polygons: Value(polygons),
        }
    }
}

impl<'a> Clear for TweenColorContext<'a> {
    #[inline(always)]
    fn clear<B: BackEnd>(&self, back_end: &mut B) {
        if back_end.supports_clear_rgba() {
            let &Color(color) = self.color.get();
            back_end.clear_rgba(color[0], color[1], color[2], color[3]);
        }
    }
}

