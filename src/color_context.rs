
use {
    AddEllipse,
    AddImage,
    AddLine,
    AddPolygon,
    AddRectangle,
    AddTween,
    BackEnd,
    Borrowed,
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
pub struct ColorContext<'a> {
    /// View transformation.
    pub view: Field<'a, Matrix2d>,
    /// Current transformation.
    pub transform: Field<'a, Matrix2d>,
    /// Current color.
    pub color: Field<'a, Color>,
}

impl<'a> 
Clone 
for ColorContext<'a> {
    #[inline(always)]
    fn clone(&self) -> ColorContext<'static> {
        ColorContext {
            view: Value(*self.view.get()),
            transform: Value(*self.transform.get()),
            color: Value(*self.color.get()),
        }
    }
}

impl<'a> 
HasTransform<'a, Matrix2d> 
for ColorContext<'a> {
    #[inline(always)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a> 
CanTransform<'a, ColorContext<'a>, Matrix2d> 
for ColorContext<'a> {
    #[inline(always)]
    fn transform(&'a self, value: Matrix2d) -> ColorContext<'a> {
        ColorContext {
            view: Borrowed(self.view.get()),
            transform: Value(value),
            color: Borrowed(self.color.get()),
        }
    }
}

impl<'a> 
HasViewTransform<'a, Matrix2d> 
for ColorContext<'a> {
    #[inline(always)]
    fn get_view_transform(&'a self) -> &'a Matrix2d {
        self.view.get()
    }
}

impl<'a> 
CanViewTransform<'a, ColorContext<'a>, Matrix2d> 
for ColorContext<'a> {
    #[inline(always)]
    fn view_transform(
        &'a self, 
        value: Matrix2d
    ) -> ColorContext<'a> {
        ColorContext {
            view: Value(value),
            transform: Borrowed(self.transform.get()),
            color: Borrowed(self.color.get()),
        }
    }
}

impl<'a> 
HasColor<'a, Color> 
for ColorContext<'a> {
    #[inline(always)]
    fn get_color(&'a self) -> &'a Color {
        self.color.get()
    }
}

impl<'a> 
CanColor<'a, ColorContext<'a>, Color> 
for ColorContext<'a> {
    #[inline(always)]
    fn color(&'a self, value: Color) -> ColorContext<'a> {
        ColorContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            color: Value(value),
        }
    }
}

impl<'a> 
AddRectangle<'a, RectangleColorContext<'a>> 
for ColorContext<'a> {
    #[inline(always)]
    fn rect(
        &'a self, 
        x: Scalar, 
        y: Scalar, 
        w: Scalar, 
        h: Scalar
    ) -> RectangleColorContext<'a> {
        RectangleColorContext {
            view: Borrowed(self.view.get()),
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
    let color_rect = color.rect(0.0, 0.0, 100.0, 100.0);
    let rect_color = color_rect.rect.get();
    assert_eq!(rect_color[2], 100.0);
}

impl<'a> 
AddEllipse<'a, EllipseColorContext<'a>> 
for ColorContext<'a> {
    #[inline(always)]
    fn ellipse(
        &'a self, 
        x: Scalar, 
        y: Scalar, 
        w: Scalar, 
        h: Scalar
    ) -> EllipseColorContext<'a> {
        EllipseColorContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            color: Borrowed(self.color.get()),
            rect: Value([x, y, w, h]),
        }
    }
}

impl<'a, 'b> 
AddPolygon<'a, PolygonColorContext<'a, 'b>> 
for ColorContext<'a> {
    #[inline(always)]
    fn polygon(
        &'a self, 
        polygon: Polygon<'b>
    ) -> PolygonColorContext<'a, 'b> {
        PolygonColorContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            color: Borrowed(self.color.get()),
            polygon: Value(polygon),
        }
    }
}

impl<'a> 
AddTween<'a, LerpTweenColorContext<'a>> 
for ColorContext<'a> {
    #[inline(always)]
    fn lerp(
        &'a self, 
        tween_factor: Scalar
    ) -> LerpTweenColorContext<'a> {
        LerpTweenColorContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            color: Borrowed(self.color.get()),
            tween_factor: Value(tween_factor),
        }
    }
}

impl<'a, B: BackEnd<I>, I: ImageSize> 
Draw<'a, B, I> 
for ColorContext<'a> {
    fn draw(&self, back_end: &mut B) {
        if back_end.supports_clear_rgba() {
            let color = self.color.get();
            back_end.clear_rgba(color[0], color[1], color[2], color[3]);
        }
    }
}

impl<'a, 'b, I: ImageSize> 
AddImage<'a, 'b, ImageColorContext<'a, 'b, I>, I> 
for ColorContext<'a> {
    #[inline(always)]
    fn image(&'a self, image: &'b I) -> ImageColorContext<'a, 'b, I> {
        let (w, h) = image.get_size();
        ImageColorContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            image: Value(image),
            source_rect: Value([0, 0, w as i32, h as i32]),
            color: Borrowed(self.color.get()),
        }
    }
}

impl<'a> 
AddLine<'a, LineColorContext<'a>> 
for ColorContext<'a> {
    #[inline(always)]
    fn line(
        &'a self, 
        x1: Scalar, 
        y1: Scalar, 
        x2: Scalar, 
        y2: Scalar
    ) -> LineColorContext<'a> {
        LineColorContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            line: Value([x1, y1, x2, y2]),
            color: Borrowed(self.color.get()),
        }
    }
}

