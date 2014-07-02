
use {
    AddEllipse,
    AddImage,
    AddLine,
    AddPolygon,
    AddRectangle,
    AddTween,
    BackEnd,
    Draw,
    EllipseColorContext,
    Field,
    ImageSize,
    ImageColorContext,
    LineColorContext,
    PolygonColorContext,
    RectangleColorContext,
    LerpTweenColorContext,
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
    Polygon,
    Scalar,
};

/// A context with color information.
pub struct ColorContext {
    /// View transformation.
    pub view: Field<Matrix2d>,
    /// Current transformation.
    pub transform: Field<Matrix2d>,
    /// Current color.
    pub color: Field<Color>,
}

impl
Clone 
for ColorContext {
    #[inline(always)]
    fn clone(&self) -> ColorContext {
        ColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            color: Value(self.color.get()),
        }
    }
}

impl
HasTransform<Matrix2d> 
for ColorContext {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform.get()
    }
}

impl
CanTransform<ColorContext, Matrix2d> 
for ColorContext {
    #[inline(always)]
    fn transform(&self, value: Matrix2d) -> ColorContext {
        ColorContext {
            view: Value(self.view.get()),
            transform: Value(value),
            color: Value(self.color.get()),
        }
    }
}

impl
HasViewTransform<Matrix2d> 
for ColorContext {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view.get()
    }
}

impl
CanViewTransform<ColorContext, Matrix2d> 
for ColorContext {
    #[inline(always)]
    fn view_transform(
        &self, 
        value: Matrix2d
    ) -> ColorContext {
        ColorContext {
            view: Value(value),
            transform: Value(self.transform.get()),
            color: Value(self.color.get()),
        }
    }
}

impl
HasColor<Color> 
for ColorContext {
    #[inline(always)]
    fn get_color(&self) -> Color {
        self.color.get()
    }
}

impl 
CanColor<ColorContext, Color> 
for ColorContext {
    #[inline(always)]
    fn color(&self, value: Color) -> ColorContext {
        ColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            color: Value(value),
        }
    }
}

impl
AddRectangle<RectangleColorContext> 
for ColorContext {
    #[inline(always)]
    fn rect(
        &self, 
        x: Scalar, 
        y: Scalar, 
        w: Scalar, 
        h: Scalar
    ) -> RectangleColorContext {
        RectangleColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            color: Value(self.color.get()),
            rect: Value([x, y, w, h]),
        }
    }
}

#[test]
fn test_rect() {
    use {Context, AddColor};

    let c = Context::new();
    let color = c.rgba(1.0, 0.0, 0.0, 1.0);
    let color_rect = color.rect(0.0, 0.0, 100.0, 100.0);
    let rect_color = color_rect.rect.get();
    assert_eq!(rect_color[2], 100.0);
}

impl
AddEllipse<EllipseColorContext> 
for ColorContext {
    #[inline(always)]
    fn ellipse(
        &self, 
        x: Scalar, 
        y: Scalar, 
        w: Scalar, 
        h: Scalar
    ) -> EllipseColorContext {
        EllipseColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            color: Value(self.color.get()),
            rect: Value([x, y, w, h]),
        }
    }
}

impl<'b> 
AddPolygon<'b, PolygonColorContext<'b>> 
for ColorContext {
    #[inline(always)]
    fn polygon(
        &self, 
        polygon: Polygon<'b>
    ) -> PolygonColorContext<'b> {
        PolygonColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            color: Value(self.color.get()),
            polygon: Value(polygon),
        }
    }
}

impl
AddTween<LerpTweenColorContext> 
for ColorContext {
    #[inline(always)]
    fn lerp(
        &self, 
        tween_factor: Scalar
    ) -> LerpTweenColorContext {
        LerpTweenColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            color: Value(self.color.get()),
            tween_factor: Value(tween_factor),
        }
    }
}

impl<B: BackEnd<I>, I: ImageSize> 
Draw<B, I> 
for ColorContext {
    fn draw(&self, back_end: &mut B) {
        if back_end.supports_clear_rgba() {
            let color = self.color.get();
            back_end.clear_rgba(color[0], color[1], color[2], color[3]);
        }
    }
}

impl<'b, I: ImageSize> 
AddImage<'b, ImageColorContext<'b, I>, I> 
for ColorContext {
    #[inline(always)]
    fn image(&self, image: &'b I) -> ImageColorContext<'b, I> {
        let (w, h) = image.get_size();
        ImageColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            image: Value(image),
            source_rect: Value([0, 0, w as i32, h as i32]),
            color: Value(self.color.get()),
        }
    }
}

impl
AddLine<LineColorContext> 
for ColorContext {
    #[inline(always)]
    fn line(
        &self, 
        x1: Scalar, 
        y1: Scalar, 
        x2: Scalar, 
        y2: Scalar
    ) -> LineColorContext {
        LineColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            line: Value([x1, y1, x2, y2]),
            color: Value(self.color.get()),
        }
    }
}

