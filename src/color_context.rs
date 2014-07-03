
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
    ImageSize,
    ImageColorContext,
    LineColorContext,
    PolygonColorContext,
    RectangleColorContext,
    LerpTweenColorContext,
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
    pub view: Matrix2d,
    /// Current transformation.
    pub transform: Matrix2d,
    /// Current color.
    pub color: Color,
}

impl
Clone 
for ColorContext {
    #[inline(always)]
    fn clone(&self) -> ColorContext {
        ColorContext {
            view: self.view,
            transform: self.transform,
            color: self.color,
        }
    }
}

impl
HasTransform<Matrix2d> 
for ColorContext {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform
    }
}

impl
CanTransform<ColorContext, Matrix2d> 
for ColorContext {
    #[inline(always)]
    fn transform(&self, value: Matrix2d) -> ColorContext {
        ColorContext {
            view: self.view,
            transform: value,
            color: self.color,
        }
    }
}

impl
HasViewTransform<Matrix2d> 
for ColorContext {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view
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
            view: value,
            transform: self.transform,
            color: self.color,
        }
    }
}

impl
HasColor<Color> 
for ColorContext {
    #[inline(always)]
    fn get_color(&self) -> Color {
        self.color
    }
}

impl 
CanColor<ColorContext, Color> 
for ColorContext {
    #[inline(always)]
    fn color(&self, value: Color) -> ColorContext {
        ColorContext {
            view: self.view,
            transform: self.transform,
            color: value,
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
            view: self.view,
            transform: self.transform,
            color: self.color,
            rect: [x, y, w, h],
        }
    }
}

#[test]
fn test_rect() {
    use {Context, AddColor};

    let c = Context::new();
    let color = c.rgba(1.0, 0.0, 0.0, 1.0);
    let color_rect = color.rect(0.0, 0.0, 100.0, 100.0);
    let rect_color = color_rect.rect;
    assert_eq!(rect_color[2], 100.0);
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
            view: self.view,
            transform: self.transform,
            color: self.color,
            tween_factor: tween_factor,
        }
    }
}

impl<B: BackEnd<I>, I: ImageSize> 
Draw<B, I> 
for ColorContext {
    fn draw(&self, back_end: &mut B) {
        if back_end.supports_clear_rgba() {
            let color = self.color;
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
            view: self.view,
            transform: self.transform,
            image: image,
            source_rect: [0, 0, w as i32, h as i32],
            color: self.color,
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
            view: self.view,
            transform: self.transform,
            line: [x1, y1, x2, y2],
            color: self.color,
        }
    }
}

