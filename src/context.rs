
use {
    AddColor,
    AddEllipse,
    AddImage,
    AddLine,
    AddPolygon,
    AddRectangle,
    AddTween,
    ColorContext,
    EllipseContext,
    Field,
    ImageSize,
    ImageContext,
    LineContext,
    PolygonContext,
    RectangleContext,
    LerpTweenContext,
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
pub struct Context {
    /// View transformation.
    pub view: Field<Matrix2d>,
    /// Current transformation.
    pub transform: Field<Matrix2d>,
}

impl
Clone 
for Context {
    #[inline(always)]
    fn clone(&self) -> Context {
        Context {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
        }
    }
}

impl
HasTransform<Matrix2d> 
for Context {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform.get()
    }
}

impl
CanTransform<Context, Matrix2d> 
for Context {
    #[inline(always)]
    fn transform(&self, value: Matrix2d) -> Context {
        Context {
            view: Value(self.view.get()),
            transform: Value(value),
        }
    }
}

impl
HasViewTransform<Matrix2d> 
for Context {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view.get()
    }
}

impl
CanViewTransform<Context, Matrix2d> 
for Context {
    #[inline(always)]
    fn view_transform(&self, value: Matrix2d) -> Context {
        Context {
            view: Value(value),
            transform: Value(self.transform.get()),
        }
    }
}

impl
Context {
    /// Creates a new drawing context.
    #[inline(always)]
    pub fn new() -> Context {
        Context {
            view:  Value(
                [1.0, 0.0, 0.0,
                 0.0, 1.0, 0.0]
            ),
            transform: Value(
                [1.0, 0.0, 0.0,
                 0.0, 1.0, 0.0]
            ),
        }
    }

    /// Creates a new drawing context in absolute coordinates.
    ///
    /// This function assumes the default coordinate system
    /// being centered with x axis pointing to the right
    /// and y axis pointing up.
    ///
    /// Returns a drawing context
    /// with origin in the upper left corner
    /// and x axis pointing to the right
    /// and y axis pointing down.
    #[inline(always)]
    pub fn abs(w: f64, h: f64) -> Context {
        let sx = 2.0 / w;
        let sy = -2.0 / h;
        let mat = [ sx, 0.0, -1.0,
                   0.0,  sy, 1.0 ];
        Context {
            view: Value(mat),
            transform: Value(mat),
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

impl
AddRectangle<RectangleContext> 
for Context {
    #[inline(always)]
    fn rect(
        &self, 
        x: Scalar, 
        y: Scalar, 
        w: Scalar, 
        h: Scalar
    ) -> RectangleContext {
        RectangleContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
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

impl
AddColor<ColorContext> 
for Context {
    #[inline(always)]
    fn rgba(
        &self, 
        r: ColorComponent, 
        g: ColorComponent, 
        b: ColorComponent, 
        a: ColorComponent
    ) -> ColorContext {
        ColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
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

impl
AddEllipse<EllipseContext> 
for Context {
    #[inline(always)]
    fn ellipse(
        &self, 
        x: Scalar, 
        y: Scalar, 
        w: Scalar, 
        h: Scalar
    ) -> EllipseContext {
        EllipseContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
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

impl<'b> 
AddPolygon<'b, PolygonContext<'b>> 
for Context {
    #[inline(always)]
    fn polygon<'b>(
        &self, 
        polygon: Polygon<'b>
    ) -> PolygonContext<'b> {
        PolygonContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            polygon: Value(polygon),
        }
    }
}

impl<'b, I: ImageSize> 
AddImage<'b, ImageContext<'b, I>, I> 
for Context {
    #[inline(always)]
    fn image(&self, image: &'b I) -> ImageContext<'b, I> {
        let (w, h) = image.get_size();
        ImageContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            image: Value(image),
            source_rect: Value([0, 0, w as i32, h as i32]),
        }
    }
}

impl
AddTween<LerpTweenContext> 
for Context {
    #[inline(always)]
    fn lerp(&self, tween_factor: Scalar) -> LerpTweenContext {
        LerpTweenContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            tween_factor: Value(tween_factor),
        }
    }
}

impl
AddLine<LineContext> 
for Context {
    #[inline(always)]
    fn line(
        &self, 
        x1: Scalar, 
        y1: Scalar, 
        x2: Scalar, 
        y2: Scalar
    ) -> LineContext {
        LineContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            line: Value([x1, y1, x2, y2]),
        }
    }
}

