
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
    Matrix2d,
    PolygonContext,
    RectangleContext,
    TweenContext,
    Value,
    View,
};
use vecmath::{
    identity,
};
use internal::{
    CanTransform,
    HasTransform,
};

/// Drawing 2d context.
pub struct Context<'a> {
    /// Base/original transformation.
    pub base: Field<'a, Matrix2d>,
    /// Current transformation.
    pub transform: Field<'a, Matrix2d>,
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

impl<'a> Context<'a> {
    /// Creates a new drawing context.
    pub fn new() -> Context {
        Context {
            base:  Value([1.0, 0.0, 0.0,
                          0.0, 1.0, 0.0]),
            transform: Value([1.0, 0.0, 0.0,
                          0.0, 1.0, 0.0]),
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
        assert_eq!(d.transform.get()[2], 30.0);
        assert_eq!(d.transform.get()[5], 50.0);
    }
    assert_eq!(c.transform.get()[2], 0.0);
    assert_eq!(c.transform.get()[5], 0.0);

    let c = c.rot_deg(90.0);
    assert!((c.transform.get()[0] - 0.0).abs() < 0.00001);
    assert!((c.transform.get()[1] + 1.0).abs() < 0.00001);
}

#[test]
fn test_scale() {
    use RelativeTransform2d;

    let c = Context::new();
    let c = c.scale(2.0, 3.0);
    assert!((c.transform.get()[0] - 2.0).abs() < 0.00001);
    assert!((c.transform.get()[4] - 3.0).abs() < 0.00001);
}

impl<'a> AddRectangle<'a, RectangleContext<'a>> for Context<'a> {
    #[inline(always)]
    fn rect(&'a self, x: f64, y: f64, w: f64, h: f64) -> RectangleContext<'a> {
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
    assert_eq!(d.rect.get()[2], 100.0);
}

impl<'a> AddColor<'a, ColorContext<'a>> for Context<'a> {
    #[inline(always)]
    fn rgba(&'a self, r: f32, g: f32, b: f32, a: f32) -> ColorContext<'a> {
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
    assert_eq!(d.color.get()[0], 1.0);
}

impl<'a> AddEllipse<'a, EllipseContext<'a>> for Context<'a> {
    #[inline(always)]
    fn ellipse(&'a self, x: f64, y: f64, w: f64, h: f64) -> EllipseContext<'a> {
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
    assert_eq!(d.rect.get()[2], 100.0);
}

impl<'a> AddPolygon<'a, PolygonContext<'a>> for Context<'a> {
    #[inline(always)]
    fn polygon(&'a self, polygon: &'a [f64]) -> PolygonContext<'a> {
        PolygonContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            polygon: Value(polygon),
        }
    }
}

impl<'a> View<'a> for Context<'a> {
    #[inline(always)]
    fn view(&'a self) -> Context<'a> {
        Context {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.base.get()),
        }
    }

    #[inline(always)]
    fn reset(&'a self) -> Context<'a> {
        Context {
            base: Borrowed(self.base.get()),
            transform: Value(identity()),
        }
    }

    #[inline(always)]
    fn store_view(&'a self) -> Context<'a> {
        Context {
            base: Borrowed(self.transform.get()),
            transform: Borrowed(self.transform.get()),
        }
    }
}

impl<'a> AddImage<'a, ImageRectangleContext<'a>> for Context<'a> {
    #[inline(always)]
    fn image(&'a self, image: Image) -> ImageRectangleContext<'a> {
        ImageRectangleContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            rect: Value([0.0, 0.0, image.source_rect[2] as f64, image.source_rect[3] as f64]),
            image: Value(image),
        }
    }
}

impl<'a> AddTween<'a, TweenContext<'a>> for Context<'a> {
    #[inline(always)]
    fn lerp(&'a self, tween_factor: f64) -> TweenContext<'a> {
        TweenContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            tween_factor: Value(tween_factor),
        }
    }
}

impl<'a> AddLine<'a, LineContext<'a>> for LineContext<'a> {
    #[inline(always)]
    fn line(&'a self, x1: f64, y1: f64, x2: f64, y2: f64) -> LineContext<'a> {
        LineContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            line: Value([x1, y1, x2, y2]),
        }
    }
}

