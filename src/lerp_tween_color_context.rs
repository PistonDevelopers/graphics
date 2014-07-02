use {
    AddPolygons,
    Field,
    LerpTweenPolygonsColorContext,
    Value,
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
    pub view: Field<Matrix2d>,
    /// Current transform.
    pub transform: Field<Matrix2d>,
    /// Current color.
    pub color: Field<Color>,
    /// Animation inbetweening factor.
    pub tween_factor: Field<f64>,
}

impl
Clone 
for LerpTweenColorContext {
    #[inline(always)]
    fn clone(&self) -> LerpTweenColorContext {
        LerpTweenColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            color: Value(self.color.get()),
            tween_factor: Value(self.tween_factor.get()),
        }
    }
}

impl
HasColor<Color> 
for LerpTweenColorContext {
    #[inline(always)]
    fn get_color(&self) -> Color {
        self.color.get()
    }
}

impl
CanColor<LerpTweenColorContext, Color> 
for LerpTweenColorContext {
    #[inline(always)]
    fn color(&self, value: Color) -> LerpTweenColorContext {
        LerpTweenColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            color: Value(value),
            tween_factor: Value(self.tween_factor.get()),
        }
    }
}

impl
HasTransform<Matrix2d> 
for LerpTweenColorContext {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform.get()
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
            view: Value(self.view.get()),
            transform: Value(value),
            color: Value(self.color.get()),
            tween_factor: Value(self.tween_factor.get()),
        }
    }
}

impl
HasViewTransform<Matrix2d> 
for LerpTweenColorContext {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view.get()
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
            view: Value(value),
            transform: Value(self.transform.get()),
            tween_factor: Value(self.tween_factor.get()),
            color: Value(self.color.get()),
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
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            color: Value(self.color.get()),
            tween_factor: Value(self.tween_factor.get()),
            polygons: Value(polygons),
        }
    }
}

