use {
    AddColor,
    AddPolygons,
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
pub struct LerpTweenContext {
    /// View transform.
    pub view: Field<Matrix2d>,
    /// Current transform.
    pub transform: Field<Matrix2d>,
    /// Animation inbetweening factor.
    pub tween_factor: Field<Scalar>,
}

impl
Clone 
for LerpTweenContext {
    #[inline(always)]
    fn clone(&self) -> LerpTweenContext {
        LerpTweenContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            tween_factor: Value(self.tween_factor.get()),
        }
    }
}

impl
AddColor<LerpTweenColorContext> 
for LerpTweenContext {
    #[inline(always)]
    fn rgba(
        &self, 
        r: ColorComponent, 
        g: ColorComponent, 
        b: ColorComponent, 
        a: ColorComponent
    ) -> LerpTweenColorContext {
        LerpTweenColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            tween_factor: Value(self.tween_factor.get()),
            color: Value([r, g, b, a]),
        }
    }
}

impl<'b> 
AddPolygons<'b, LerpTweenPolygonsContext<'b>> 
for LerpTweenContext {
    #[inline(always)]
    fn polygons(
        &self, 
        polygons: Polygons<'b>
    ) -> LerpTweenPolygonsContext<'b> {
        LerpTweenPolygonsContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            tween_factor: Value(self.tween_factor.get()),
            polygons: Value(polygons),
        }
    }
}

impl
HasTransform<Matrix2d> 
for LerpTweenContext {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform.get()
    }
}

impl
CanTransform<LerpTweenContext, Matrix2d> 
for LerpTweenContext {
    #[inline(always)]
    fn transform(
        &self, 
        value: Matrix2d
    ) -> LerpTweenContext {
        LerpTweenContext {
            view: Value(self.view.get()),
            transform: Value(value),
            tween_factor: Value(self.tween_factor.get()),
        }
    }
}

impl
HasViewTransform<Matrix2d> 
for LerpTweenContext {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view.get()
    }
}

impl
CanViewTransform<LerpTweenContext, Matrix2d> 
for LerpTweenContext {
    #[inline(always)]
    fn view_transform(
        &self, 
        value: Matrix2d
    ) -> LerpTweenContext {
        LerpTweenContext {
            view: Value(value),
            transform: Value(self.transform.get()),
            tween_factor: Value(self.tween_factor.get()),
        }
    }
}

