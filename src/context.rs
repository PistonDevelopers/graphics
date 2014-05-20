
use {
    AddColor,
    AddEllipse,
    AddImage,
    AddLine,
    AddPolygon,
    AddRectangle,
    AddTween,
    Borrowed,
    ColorContext,
    EllipseContext,
    Field,
    Image,
    ImageRectangleContext,
    LineContext,
    PolygonContext,
    RectangleContext,
    TweenContext,
    Value,
};
use internal::{
    CanTransform,
    CanViewTransform,
    ColorComponent,
    HasTransform,
    HasViewTransform,
    Matrix2d,
    Polygon,
    Scalar,
};

/// Drawing 2d context.
pub struct Context<'a> {
    /// Base/original transformation.
    pub base: Field<'a, Matrix2d>,
    /// Current transformation.
    pub transform: Field<'a, Matrix2d>,
}

impl<'a> Clone for Context<'a> {
    #[inline(always)]
    fn clone(&self) -> Context<'static> {
        Context {
            base: Value(*self.base.get()),
            transform: Value(*self.transform.get()),
        }
    }
}

impl<'a> HasTransform<'a, Matrix2d> for Context<'a> {
    #[inline(always)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a> CanTransform<'a, Context<'a>, Matrix2d> for Context<'a> {
    #[inline(always)]
    fn transform(&'a self, value: Matrix2d) -> Context<'a> {
        Context {
            base: Borrowed(self.base.get()),
            transform: Value(value),
        }
    }
}

impl<'a> HasViewTransform<'a, Matrix2d> for Context<'a> {
    #[inline(always)]
    fn get_view_transform(&'a self) -> &'a Matrix2d {
        self.base.get()
    }
}

impl<'a> CanViewTransform<'a, Context<'a>, Matrix2d> for Context<'a> {
    #[inline(always)]
    fn view_transform(&'a self, value: Matrix2d) -> Context<'a> {
        Context {
            base: Value(value),
            transform: Borrowed(self.transform.get()),
        }
    }
}

impl<'a> Context<'a> {
    /// Creates a new drawing context.
    pub fn new() -> Context {
        Context {
            base:  Value(
                [1.0, 0.0, 0.0,
                 0.0, 1.0, 0.0]
            ),
            transform: Value(
                [1.0, 0.0, 0.0,
                 0.0, 1.0, 0.0]
            ),
        }
    }
}

#[test]
fn test_context() {
    use RelativeTransform2d;

    let c = Context::new();
    {
        let d = c.trans(20.0, 40.0);
        let d = d.trans(10.0, 10.0);
        let transform = d.transform.get();
        assert_eq!(transform[2], 30.0);
        assert_eq!(transform[5], 50.0);
    }
    
    let transform = c.transform.get();
    assert_eq!(transform[2], 0.0);
    assert_eq!(transform[5], 0.0);

    let c = c.rot_deg(90.0);
    let transform = c.transform.get();
    assert!((transform[0] - 0.0).abs() < 0.00001);
    assert!((transform[1] + 1.0).abs() < 0.00001);
}

#[test]
fn test_scale() {
    use RelativeTransform2d;

    let c = Context::new();
    let c = c.scale(2.0, 3.0);
    let transform = c.transform.get();
    assert!((transform[0] - 2.0).abs() < 0.00001);
    assert!((transform[4] - 3.0).abs() < 0.00001);
}

impl<'a> AddRectangle<'a, RectangleContext<'a>> for Context<'a> {
    #[inline(always)]
    fn rect(&'a self, x: Scalar, y: Scalar, w: Scalar, h: Scalar) -> RectangleContext<'a> {
        RectangleContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            rect: Value([x, y, w, h]),
        }
    }
}

#[test]
fn test_rect() {
    let c = Context::new();
    let d = c.rect(0.0, 0.0, 100.0, 50.0);
    let rect = d.rect.get();
    assert_eq!(rect[2], 100.0);
}

impl<'a> AddColor<'a, ColorContext<'a>> for Context<'a> {
    #[inline(always)]
    fn rgba(
        &'a self, 
        r: ColorComponent, 
        g: ColorComponent, 
        b: ColorComponent, 
        a: ColorComponent
    ) -> ColorContext<'a> {
        ColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            color: Value([r, g, b, a]),
        }
    }
}

#[test]
fn test_rgba() {
    let c = Context::new();
    let d: ColorContext = c.rgba(1.0, 0.0, 0.0, 1.0);
    let color = d.color.get();
    assert_eq!(color[0], 1.0);
}

impl<'a> AddEllipse<'a, EllipseContext<'a>> for Context<'a> {
    #[inline(always)]
    fn ellipse(&'a self, x: Scalar, y: Scalar, w: Scalar, h: Scalar) -> EllipseContext<'a> {
        EllipseContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            rect: Value([x, y, w, h]),
        }
    }
}

#[test]
fn test_ellipse() {
    let c = Context::new();
    let d: EllipseContext = c.ellipse(0.0, 0.0, 100.0, 100.0);
    let rect = d.rect.get();    
    assert_eq!(rect[2], 100.0);
}

impl<'a, 'b> AddPolygon<'a, PolygonContext<'a, 'b>> for Context<'a> {
    #[inline(always)]
    fn polygon(&'a self, polygon: Polygon<'b>) -> PolygonContext<'a, 'b> {
        PolygonContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            polygon: Value(polygon),
        }
    }
}

impl<'a> AddImage<'a, ImageRectangleContext<'a>> for Context<'a> {
    #[inline(always)]
    fn image(&'a self, image: Image) -> ImageRectangleContext<'a> {
        ImageRectangleContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            rect: Value([
                0.0, 
                0.0, 
                image.source_rect[2] as f64, 
                image.source_rect[3] as f64
            ]),
            image: Value(image),
        }
    }
}

impl<'a> AddTween<'a, TweenContext<'a>> for Context<'a> {
    #[inline(always)]
    fn lerp(&'a self, tween_factor: Scalar) -> TweenContext<'a> {
        TweenContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            tween_factor: Value(tween_factor),
        }
    }
}

impl<'a> AddLine<'a, LineContext<'a>> for Context<'a> {
    #[inline(always)]
    fn line(&'a self, x1: Scalar, y1: Scalar, x2: Scalar, y2: Scalar) -> LineContext<'a> {
        LineContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            line: Value([x1, y1, x2, y2]),
        }
    }
}

