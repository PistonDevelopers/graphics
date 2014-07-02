
use {
    AddColor,
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
pub struct LerpTweenPolygonsContext<'b> {
    /// View transform.
    pub view: Field<Matrix2d>,
    /// Current transform.
    pub transform: Field<Matrix2d>,
    /// Animation inbetweening factor.
    pub tween_factor: Field<Scalar>,
    /// The animated polygons.
    pub polygons: Field<Polygons<'b>>,
}

impl<'b> 
Clone 
for LerpTweenPolygonsContext<'b> {
    #[inline(always)]
    fn clone(&self) -> LerpTweenPolygonsContext<'b> {
        LerpTweenPolygonsContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            tween_factor: Value(self.tween_factor.get()),
            polygons: Value(self.polygons.get()),
        }
    }
}

impl<'b> 
AddColor<LerpTweenPolygonsColorContext<'b>> 
for LerpTweenPolygonsContext<'b> {
    /// Creates a RectangleColorContext.
    #[inline(always)]
    fn rgba(
        &self, 
        r: ColorComponent, 
        g: ColorComponent, 
        b: ColorComponent, 
        a: ColorComponent
    ) -> LerpTweenPolygonsColorContext<'b> {
        LerpTweenPolygonsColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            color: Value([r, g, b, a]),
            tween_factor: Value(self.tween_factor.get()),
            polygons: Value(self.polygons.get()),
        }
    }
}

impl<'a, 'b> 
HasTransform<Matrix2d> 
for LerpTweenPolygonsContext<'b> {
    #[inline(alwyas)]
    fn get_transform(&self) -> Matrix2d {
        self.transform.get()
    }
}

impl<'b> 
CanTransform<LerpTweenPolygonsContext<'b>, Matrix2d> 
for LerpTweenPolygonsContext<'b> {
    #[inline(always)]
    fn transform(&self, value: Matrix2d) -> LerpTweenPolygonsContext<'b> {
        LerpTweenPolygonsContext {
            view: Value(self.view.get()),
            transform: Value(value),
            tween_factor: Value(self.tween_factor.get()),
            polygons: Value(self.polygons.get()),
        }
    }
}

impl<'b> 
HasViewTransform<Matrix2d> 
for LerpTweenPolygonsContext<'b> {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view.get()
    }
}

impl<'b> 
CanViewTransform<LerpTweenPolygonsContext<'b>, Matrix2d> 
for LerpTweenPolygonsContext<'b> {
    #[inline(always)]
    fn view_transform(&self, value: Matrix2d) -> LerpTweenPolygonsContext<'b> {
        LerpTweenPolygonsContext {
            view: Value(value),
            transform: Value(self.transform.get()),
            tween_factor: Value(self.tween_factor.get()),
            polygons: Value(self.polygons.get()),
        }
    }
}

