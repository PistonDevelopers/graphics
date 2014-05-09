
use {
    AddColor,
    Borrowed,
    Field,
    Matrix2d,
    TweenPolygonsColorContext,
    Value,
};
use internal::{
    CanTransform,
    HasTransform,
};

/// An animation inbetweening context with color.
pub struct TweenPolygonsContext<'a> {
    /// Base/origin transform.
    pub base: Field<'a, Matrix2d>,
    /// Current transform.
    pub transform: Field<'a, Matrix2d>,
    /// Animation inbetweening factor.
    pub tween_factor: Field<'a, f64>,
    /// The animated polygons.
    pub polygons: Field<'a, &'a [&'a [f64]]>,
}

impl<'a> AddColor<'a, TweenPolygonsColorContext<'a>> for TweenPolygonsContext<'a> {
    /// Creates a RectangleColorContext.
    #[inline(always)]
    fn rgba(&'a self, r: f32, g: f32, b: f32, a: f32) -> TweenPolygonsColorContext<'a> {
        TweenPolygonsColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            color: Value([r, g, b, a]),
            tween_factor: Borrowed(self.tween_factor.get()),
            polygons: Borrowed(self.polygons.get()),
        }
    }
}

impl<'a> HasTransform<'a, Matrix2d> for TweenPolygonsContext<'a> {
    #[inline(alwyas)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a> CanTransform<'a, TweenPolygonsContext<'a>, Matrix2d> for TweenPolygonsContext<'a> {
    #[inline(always)]
    fn transform(&'a self, value: Matrix2d) -> TweenPolygonsContext<'a> {
        TweenPolygonsContext {
            base: Borrowed(self.base.get()),
            transform: Value(value),
            tween_factor: Borrowed(self.tween_factor.get()),
            polygons: Borrowed(self.polygons.get()),
        }
    }
}

