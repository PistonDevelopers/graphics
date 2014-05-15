use {
    AddColor,
    AddPolygons,
    Borrowed,
    Color,
    Field,
    Matrix2d,
    TweenColorContext,
    TweenPolygonsContext,
    Value,
    View,
};
use vecmath::{
    identity
};
use internal::{
    CanTransform,
    HasTransform,
};

/// An animation inbetweening context.
pub struct TweenContext<'a> {
    /// Base/origin transform.
    pub base: Field<'a, Matrix2d>,
    /// Current transform.
    pub transform: Field<'a, Matrix2d>,
    /// Animation inbetweening factor.
    pub tween_factor: Field<'a, f64>,
}

impl<'a> Clone for TweenContext<'a> {
    #[inline(always)]
    fn clone(&self) -> TweenContext<'static> {
        TweenContext {
            base: self.base.clone(),
            transform: self.transform.clone(),
            tween_factor: self.tween_factor.clone(),
        }
    }
}

impl<'a> View<'a> for TweenContext<'a> {
    #[inline(always)]
    fn view(&'a self) -> TweenContext<'a> {
        TweenContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.base.get()),
            tween_factor: Borrowed(self.tween_factor.get()),
        }
    }

    #[inline(always)]
    fn reset(&'a self) -> TweenContext<'a> {
        TweenContext {
            base: Borrowed(self.base.get()),
            transform: Value(identity()),
            tween_factor: Borrowed(self.tween_factor.get()),
        }
    }

    #[inline(always)]
    fn store_view(&'a self) -> TweenContext<'a> {
        TweenContext {
            base: Borrowed(self.transform.get()),
            transform: Borrowed(self.transform.get()),
            tween_factor: Borrowed(self.tween_factor.get()),
        }
    }
}

impl<'a> AddColor<'a, TweenColorContext<'a>> for TweenContext<'a> {
    #[inline(always)]
    fn rgba(&'a self, r: f32, g: f32, b: f32, a: f32) -> TweenColorContext<'a> {
        TweenColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            tween_factor: Borrowed(self.tween_factor.get()),
            color: Value(Color([r, g, b, a])),
        }
    }
}

impl<'a, 'b> AddPolygons<'a, TweenPolygonsContext<'a, 'b>> for TweenContext<'a> {
    #[inline(always)]
    fn polygons(&'a self, polygons: &'b [&'b [f64]]) -> TweenPolygonsContext<'a, 'b> {
        TweenPolygonsContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            tween_factor: Borrowed(self.tween_factor.get()),
            polygons: Value(polygons),
        }
    }
}

impl<'a> HasTransform<'a, Matrix2d> for TweenContext<'a> {
    #[inline(always)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a> CanTransform<'a, TweenContext<'a>, Matrix2d> for TweenContext<'a> {
    #[inline(always)]
    fn transform(&'a self, value: Matrix2d) -> TweenContext<'a> {
        TweenContext {
            base: Borrowed(self.base.get()),
            transform: Value(value),
            tween_factor: Borrowed(self.tween_factor.get()),
        }
    }
}

