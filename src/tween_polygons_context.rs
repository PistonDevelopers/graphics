
use {
    AddColor,
    Borrowed,
    Color,
    Field,
    Matrix2d,
    Polygons,
    TweenPolygonsColorContext,
    Value,
};
use internal::{
    CanTransform,
    CanViewTransform,
    HasTransform,
    HasViewTransform,
};

/// An animation inbetweening context with color.
pub struct TweenPolygonsContext<'a, 'b> {
    /// Base/origin transform.
    pub base: Field<'a, Matrix2d>,
    /// Current transform.
    pub transform: Field<'a, Matrix2d>,
    /// Animation inbetweening factor.
    pub tween_factor: Field<'a, f64>,
    /// The animated polygons.
    pub polygons: Field<'a, Polygons<'b>>,
}

impl<'a, 'b> Clone for TweenPolygonsContext<'a, 'b> {
    #[inline(always)]
    fn clone(&self) -> TweenPolygonsContext<'static, 'b> {
        TweenPolygonsContext {
            base: self.base.clone(),
            transform: self.transform.clone(),
            tween_factor: self.tween_factor.clone(),
            polygons: self.polygons.clone(),
        }
    }
}

impl<'a, 'b> AddColor<'a, TweenPolygonsColorContext<'a, 'b>> for TweenPolygonsContext<'a, 'b> {
    /// Creates a RectangleColorContext.
    #[inline(always)]
    fn rgba(&'a self, r: f32, g: f32, b: f32, a: f32) -> TweenPolygonsColorContext<'a, 'b> {
        TweenPolygonsColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            color: Value(Color([r, g, b, a])),
            tween_factor: Borrowed(self.tween_factor.get()),
            polygons: Borrowed(self.polygons.get()),
        }
    }
}

impl<'a, 'b> HasTransform<'a, Matrix2d> for TweenPolygonsContext<'a, 'b> {
    #[inline(alwyas)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a, 'b> CanTransform<'a, TweenPolygonsContext<'a, 'b>, Matrix2d> for TweenPolygonsContext<'a, 'b> {
    #[inline(always)]
    fn transform(&'a self, value: Matrix2d) -> TweenPolygonsContext<'a, 'b> {
        TweenPolygonsContext {
            base: Borrowed(self.base.get()),
            transform: Value(value),
            tween_factor: Borrowed(self.tween_factor.get()),
            polygons: Borrowed(self.polygons.get()),
        }
    }
}

impl<'a, 'b> HasViewTransform<'a, Matrix2d> for TweenPolygonsContext<'a, 'b> {
    #[inline(always)]
    fn get_view_transform(&'a self) -> &'a Matrix2d {
        self.base.get()
    }
}

impl<'a, 'b> CanViewTransform<'a, TweenPolygonsContext<'a, 'b>, Matrix2d> for TweenPolygonsContext<'a, 'b> {
    #[inline(always)]
    fn view_transform(&'a self, value: Matrix2d) -> TweenPolygonsContext<'a, 'b> {
        TweenPolygonsContext {
            base: Value(value),
            transform: Borrowed(self.transform.get()),
            tween_factor: Borrowed(self.tween_factor.get()),
            polygons: Borrowed(self.polygons.get()),
        }
    }
}

