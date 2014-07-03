use {
    AddPolygons,
    LerpTweenPolygonsColorContext,
};
use internal::{
    CanColor,
    CanTransform,
    CanViewTransform,
    Color,
    HasColor,
    HasTransform,
    HasViewTransform,
    Matrix2d,
    Polygons,
};

/// An animation inbetweening context with color.
pub struct LerpTweenColorContext {
    /// View transform.
    pub view: Matrix2d,
    /// Current transform.
    pub transform: Matrix2d,
    /// Current color.
    pub color: Color,
    /// Animation inbetweening factor.
    pub tween_factor: f64,
}

impl
Clone 
for LerpTweenColorContext {
    #[inline(always)]
    fn clone(&self) -> LerpTweenColorContext {
        LerpTweenColorContext {
            view: self.view,
            transform: self.transform,
            color: self.color,
            tween_factor: self.tween_factor,
        }
    }
}

impl
HasColor<Color> 
for LerpTweenColorContext {
    #[inline(always)]
    fn get_color(&self) -> Color {
        self.color
    }
}

impl
CanColor<LerpTweenColorContext, Color> 
for LerpTweenColorContext {
    #[inline(always)]
    fn color(&self, value: Color) -> LerpTweenColorContext {
        LerpTweenColorContext {
            view: self.view,
            transform: self.transform,
            color: value,
            tween_factor: self.tween_factor,
        }
    }
}

impl
HasTransform<Matrix2d> 
for LerpTweenColorContext {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform
    }
}

impl
CanTransform<LerpTweenColorContext, Matrix2d> 
for LerpTweenColorContext {
    #[inline(always)]
    fn transform(
        &self, 
        value: Matrix2d
    ) -> LerpTweenColorContext {
        LerpTweenColorContext {
            view: self.view,
            transform: value,
            color: self.color,
            tween_factor: self.tween_factor,
        }
    }
}

impl
HasViewTransform<Matrix2d> 
for LerpTweenColorContext {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view
    }
}

impl
CanViewTransform<LerpTweenColorContext, Matrix2d> 
for LerpTweenColorContext {
    #[inline(always)]
    fn view_transform(
        &self, 
        value: Matrix2d
    ) -> LerpTweenColorContext {
        LerpTweenColorContext {
            view: value,
            transform: self.transform,
            tween_factor: self.tween_factor,
            color: self.color,
        }
    }
}


impl<'b> 
AddPolygons<'b, LerpTweenPolygonsColorContext<'b>> 
for LerpTweenColorContext {
    #[inline(always)]
    fn polygons(
        &self, 
        polygons: Polygons<'b>
    ) -> LerpTweenPolygonsColorContext<'b> {
        LerpTweenPolygonsColorContext {
            view: self.view,
            transform: self.transform,
            color: self.color,
            tween_factor: self.tween_factor,
            polygons: polygons,
        }
    }
}

