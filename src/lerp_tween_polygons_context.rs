
use {
    AddColor,
    Borrowed,
    Field,
    LerpTweenPolygonsColorContext,
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

/// An animation inbetweening context with color.
pub struct LerpTweenPolygonsContext<'a, 'b> {
    /// View transform.
    pub view: Field<'a, Matrix2d>,
    /// Current transform.
    pub transform: Field<'a, Matrix2d>,
    /// Animation inbetweening factor.
    pub tween_factor: Field<'a, Scalar>,
    /// The animated polygons.
    pub polygons: Field<'a, Polygons<'b>>,
}

impl<'a, 'b> Clone for LerpTweenPolygonsContext<'a, 'b> {
    #[inline(always)]
    fn clone(&self) -> LerpTweenPolygonsContext<'static, 'b> {
        LerpTweenPolygonsContext {
            view: Value(*self.view.get()),
            transform: Value(*self.transform.get()),
            tween_factor: Value(*self.tween_factor.get()),
            polygons: Value(*self.polygons.get()),
        }
    }
}

impl<'a, 'b> AddColor<'a, LerpTweenPolygonsColorContext<'a, 'b>> 
for LerpTweenPolygonsContext<'a, 'b> {
    /// Creates a RectangleColorContext.
    #[inline(always)]
    fn rgba(
        &'a self, 
        r: ColorComponent, 
        g: ColorComponent, 
        b: ColorComponent, 
        a: ColorComponent
    ) -> LerpTweenPolygonsColorContext<'a, 'b> {
        LerpTweenPolygonsColorContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            color: Value([r, g, b, a]),
            tween_factor: Borrowed(self.tween_factor.get()),
            polygons: Borrowed(self.polygons.get()),
        }
    }
}

impl<'a, 'b> HasTransform<'a, Matrix2d> 
for LerpTweenPolygonsContext<'a, 'b> {
    #[inline(alwyas)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a, 'b> CanTransform<'a, LerpTweenPolygonsContext<'a, 'b>, Matrix2d> 
for LerpTweenPolygonsContext<'a, 'b> {
    #[inline(always)]
    fn transform(&'a self, value: Matrix2d) -> LerpTweenPolygonsContext<'a, 'b> {
        LerpTweenPolygonsContext {
            view: Borrowed(self.view.get()),
            transform: Value(value),
            tween_factor: Borrowed(self.tween_factor.get()),
            polygons: Borrowed(self.polygons.get()),
        }
    }
}

impl<'a, 'b> HasViewTransform<'a, Matrix2d> 
for LerpTweenPolygonsContext<'a, 'b> {
    #[inline(always)]
    fn get_view_transform(&'a self) -> &'a Matrix2d {
        self.view.get()
    }
}

impl<'a, 'b> CanViewTransform<'a, LerpTweenPolygonsContext<'a, 'b>, Matrix2d> 
for LerpTweenPolygonsContext<'a, 'b> {
    #[inline(always)]
    fn view_transform(&'a self, value: Matrix2d) -> LerpTweenPolygonsContext<'a, 'b> {
        LerpTweenPolygonsContext {
            view: Value(value),
            transform: Borrowed(self.transform.get()),
            tween_factor: Borrowed(self.tween_factor.get()),
            polygons: Borrowed(self.polygons.get()),
        }
    }
}

