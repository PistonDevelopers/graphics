
use {
    AddEllipse, 
    AddPolygon, 
    AddRectangle, 
    AddTween,
    BackEnd, 
    Borrowed, 
    Clear, 
    Color,
    EllipseColorContext, 
    Field, 
    Matrix2d, 
    PolygonColorContext, 
    RectangleColorContext,
    TweenColorContext,
    View,
    Value,
};
use vecmath::{
    identity,
};
use internal::{
    CanColor,
    CanTransform,
    HasColor,
    HasTransform,
};

/// A context with color information.
pub struct ColorContext<'a> {
    /// Base/original transformation.
    pub base: Field<'a, Matrix2d>,
    /// Current transformation.
    pub transform: Field<'a, Matrix2d>,
    /// Current color.
    pub color: Field<'a, Color>,
}

impl<'a> HasTransform<'a, Matrix2d> for ColorContext<'a> {
    #[inline(always)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a> CanTransform<'a, ColorContext<'a>, Matrix2d> for ColorContext<'a> {
    #[inline(always)]
    fn transform(&'a self, value: Matrix2d) -> ColorContext<'a> {
        ColorContext {
            base: Borrowed(self.base.get()),
            transform: Value(value),
            color: Borrowed(self.color.get()),
        }
    }
}

impl<'a> HasColor<'a, Color> for ColorContext<'a> {
    #[inline(always)]
    fn get_color(&'a self) -> &'a Color {
        self.color.get()
    }
}

impl<'a> CanColor<'a, ColorContext<'a>, Color> for ColorContext<'a> {
    #[inline(always)]
    fn color(&'a self, value: Color) -> ColorContext<'a> {
        ColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            color: Value(value),
        }
    }
}

impl<'a> AddRectangle<'a, RectangleColorContext<'a>> for ColorContext<'a> {
    #[inline(always)]
    fn rect(&'a self, x: f64, y: f64, w: f64, h: f64) -> RectangleColorContext<'a> {
        RectangleColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            color: Borrowed(self.color.get()),
            rect: Value([x, y, w, h]),
        }
    }
}

#[test]
fn test_rect() {
    use {Context, AddColor};

    let c = Context::new();
    let color = c.rgba(1.0, 0.0, 0.0, 1.0);
    let rect_color = color.rect(0.0, 0.0, 100.0, 100.0);
    assert_eq!(rect_color.rect.get()[2], 100.0);
}

impl<'a> AddEllipse<'a, EllipseColorContext<'a>> for ColorContext<'a> {
    #[inline(always)]
    fn ellipse(&'a self, x: f64, y: f64, w: f64, h: f64) -> EllipseColorContext<'a> {
        EllipseColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            color: Borrowed(self.color.get()),
            rect: Value([x, y, w, h]),
        }
    }
}

impl<'a> AddPolygon<'a, PolygonColorContext<'a>> for ColorContext<'a> {
    #[inline(always)]
    fn polygon(&'a self, polygon: &'a [f64]) -> PolygonColorContext<'a> {
        PolygonColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            color: Borrowed(self.color.get()),
            polygon: Value(polygon),
        }
    }
}

impl<'a> AddTween<'a, TweenColorContext<'a>> for ColorContext<'a> {
    #[inline(always)]
    fn lerp(&'a self, tween_factor: f64) -> TweenColorContext<'a> {
        TweenColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            color: Borrowed(self.color.get()),
            tween_factor: Value(tween_factor),
        }
    }
}

impl<'a> Clear for ColorContext<'a> {
    fn clear<B: BackEnd>(&self, back_end: &mut B) {
        if back_end.supports_clear_rgba() {
            let color = self.color.get();
            back_end.clear_rgba(color[0], color[1], color[2], color[3]);
        }        
    }
}

impl<'a> View<'a> for ColorContext<'a> {
    #[inline(always)]
    fn view(&'a self) -> ColorContext<'a> {
        ColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.base.get()),
            color: Borrowed(self.color.get()),
        }
    }

    #[inline(always)]
    fn reset(&'a self) -> ColorContext<'a> {
        ColorContext {
            base: Borrowed(self.base.get()),
            transform: Value(identity()),
            color: Borrowed(self.color.get()),
        }
    }

    #[inline(always)]
    fn store_view(&'a self) -> ColorContext<'a> {
        ColorContext {
            base: Borrowed(self.transform.get()),
            transform: Borrowed(self.transform.get()),
            color: Borrowed(self.color.get()),
        }
    }
}

