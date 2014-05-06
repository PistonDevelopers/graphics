use {Field, Borrowed, Value, Color, Matrix2d};
use {AddPolygons, View};
use {TweenPolygonsColorContext};
use vecmath::{identity};

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

impl<'a> View<'a> for TweenColorContext<'a> {
    #[inline(always)]
    fn view(&'a self) -> TweenColorContext<'a> {
        TweenColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.base.get()),
            tween_factor: Borrowed(self.tween_factor.get()),
            color: Borrowed(self.color.get()),
        }
    }

    #[inline(always)]
    fn reset(&'a self) -> TweenColorContext<'a> {
        TweenColorContext {
            base: Borrowed(self.base.get()),
            transform: Value(identity()),
            tween_factor: Borrowed(self.tween_factor.get()),
            color: Borrowed(self.color.get()),
        }
    }

    #[inline(always)]
    fn store_view(&'a self) -> TweenColorContext<'a> {
        TweenColorContext {
            base: Borrowed(self.transform.get()),
            transform: Borrowed(self.transform.get()),
            color: Borrowed(self.color.get()),
            tween_factor: Borrowed(self.tween_factor.get()),
        }
    }
}

