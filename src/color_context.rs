
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


