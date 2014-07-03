use {
    AddColor,
    AddPolygons,
    LerpTweenColorContext,
    LerpTweenPolygonsContext,
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
    pub view: Matrix2d,
    /// Current transform.
    pub transform: Matrix2d,
    /// Animation inbetweening factor.
    pub tween_factor: Scalar,
}

impl
Clone 
for LerpTweenContext {
    #[inline(always)]
    fn clone(&self) -> LerpTweenContext {
        LerpTweenContext {
            view: self.view,
            transform: self.transform,
            tween_factor: self.tween_factor,
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
            view: self.view,
            transform: self.transform,
            tween_factor: self.tween_factor,
            color: [r, g, b, a],
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
            view: self.view,
            transform: self.transform,
            tween_factor: self.tween_factor,
            polygons: polygons,
        }
    }
}

impl
HasTransform<Matrix2d> 
for LerpTweenContext {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform
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
            view: self.view,
            transform: value,
            tween_factor: self.tween_factor,
        }
    }
}

impl
HasViewTransform<Matrix2d> 
for LerpTweenContext {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view
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
            view: value,
            transform: self.transform,
            tween_factor: self.tween_factor,
        }
    }
}

