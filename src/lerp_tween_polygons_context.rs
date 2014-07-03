
use {
    AddColor,
    LerpTweenPolygonsColorContext,
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
    pub view: Matrix2d,
    /// Current transform.
    pub transform: Matrix2d,
    /// Animation inbetweening factor.
    pub tween_factor: Scalar,
    /// The animated polygons.
    pub polygons: Polygons<'b>,
}

impl<'b> 
Clone 
for LerpTweenPolygonsContext<'b> {
    #[inline(always)]
    fn clone(&self) -> LerpTweenPolygonsContext<'b> {
        LerpTweenPolygonsContext {
            view: self.view,
            transform: self.transform,
            tween_factor: self.tween_factor,
            polygons: self.polygons,
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
            view: self.view,
            transform: self.transform,
            color: [r, g, b, a],
            tween_factor: self.tween_factor,
            polygons: self.polygons,
        }
    }
}

impl<'a, 'b> 
HasTransform<Matrix2d> 
for LerpTweenPolygonsContext<'b> {
    #[inline(alwyas)]
    fn get_transform(&self) -> Matrix2d {
        self.transform
    }
}

impl<'b> 
CanTransform<LerpTweenPolygonsContext<'b>, Matrix2d> 
for LerpTweenPolygonsContext<'b> {
    #[inline(always)]
    fn transform(&self, value: Matrix2d) -> LerpTweenPolygonsContext<'b> {
        LerpTweenPolygonsContext {
            view: self.view,
            transform: value,
            tween_factor: self.tween_factor,
            polygons: self.polygons,
        }
    }
}

impl<'b> 
HasViewTransform<Matrix2d> 
for LerpTweenPolygonsContext<'b> {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view
    }
}

impl<'b> 
CanViewTransform<LerpTweenPolygonsContext<'b>, Matrix2d> 
for LerpTweenPolygonsContext<'b> {
    #[inline(always)]
    fn view_transform(&self, value: Matrix2d) -> LerpTweenPolygonsContext<'b> {
        LerpTweenPolygonsContext {
            view: value,
            transform: self.transform,
            tween_factor: self.tween_factor,
            polygons: self.polygons,
        }
    }
}

