use {Field, Borrowed, Value, Color, Matrix2d};
use {AddPolygons};
use {TweenPolygonsColorContext};

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

impl<'a> AddPolygons<'a, TweenPolygonsColorContext<'a>> for TweenColorContext<'a> {
    #[inline(always)]
    fn polygons(&'a self, polygons: &'a [&'a [f64]]) -> TweenPolygonsColorContext<'a> {
        TweenPolygonsColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            color: Borrowed(self.color.get()),
            tween_factor: Borrowed(self.tween_factor.get()),
            polygons: Value(polygons),
        }
    }
}

