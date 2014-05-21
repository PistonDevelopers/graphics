use {
    AddColor,
    AddPolygons,
    Borrowed,
    Field,
    LerpTweenColorContext,
    LerpTweenPolygonsContext,
    Value,
};
use internal::{
    CanTransform,
    CanViewTransform,
    ColorComponent,
    HasTransform,
    HasViewTransform,
    Matrix2d,
    Polygons,
    Scalar,
};

/// An animation inbetweening context.
pub struct LerpTweenContext<'a> {
    /// Base/origin transform.
    pub base: Field<'a, Matrix2d>,
    /// Current transform.
    pub transform: Field<'a, Matrix2d>,
    /// Animation inbetweening factor.
    pub tween_factor: Field<'a, Scalar>,
}

impl<'a> Clone for LerpTweenContext<'a> {
    #[inline(always)]
    fn clone(&self) -> LerpTweenContext<'static> {
        LerpTweenContext {
            base: Value(*self.base.get()),
            transform: Value(*self.transform.get()),
            tween_factor: Value(*self.tween_factor.get()),
        }
    }
}

impl<'a> AddColor<'a, LerpTweenColorContext<'a>> for LerpTweenContext<'a> {
    #[inline(always)]
    fn rgba(
        &'a self, 
        r: ColorComponent, 
        g: ColorComponent, 
        b: ColorComponent, 
        a: ColorComponent
    ) -> LerpTweenColorContext<'a> {
        LerpTweenColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            tween_factor: Borrowed(self.tween_factor.get()),
            color: Value([r, g, b, a]),
        }
    }
}

impl<'a, 'b> AddPolygons<'a, LerpTweenPolygonsContext<'a, 'b>> for LerpTweenContext<'a> {
    #[inline(always)]
    fn polygons(&'a self, polygons: Polygons<'b>) -> LerpTweenPolygonsContext<'a, 'b> {
        LerpTweenPolygonsContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            tween_factor: Borrowed(self.tween_factor.get()),
            polygons: Value(polygons),
        }
    }
}

impl<'a> HasTransform<'a, Matrix2d> for LerpTweenContext<'a> {
    #[inline(always)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a> CanTransform<'a, LerpTweenContext<'a>, Matrix2d> for LerpTweenContext<'a> {
    #[inline(always)]
    fn transform(&'a self, value: Matrix2d) -> LerpTweenContext<'a> {
        LerpTweenContext {
            base: Borrowed(self.base.get()),
            transform: Value(value),
            tween_factor: Borrowed(self.tween_factor.get()),
        }
    }
}

impl<'a> HasViewTransform<'a, Matrix2d> for LerpTweenContext<'a> {
    #[inline(always)]
    fn get_view_transform(&'a self) -> &'a Matrix2d {
        self.base.get()
    }
}

impl<'a> CanViewTransform<'a, LerpTweenContext<'a>, Matrix2d> for LerpTweenContext<'a> {
    #[inline(always)]
    fn view_transform(&'a self, value: Matrix2d) -> LerpTweenContext<'a> {
        LerpTweenContext {
            base: Value(value),
            transform: Borrowed(self.transform.get()),
            tween_factor: Borrowed(self.tween_factor.get()),
        }
    }
}

