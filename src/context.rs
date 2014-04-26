
use {Field, Borrowed, Value};
use vecmath::{rotate_radians, multiply, translate, scale, shear};
use {Transform2d, Matrix2d};
use {ColorContext, EllipseContext, PolygonContext, RectangleContext};
use {AddColor, AddEllipse, AddPolygon, AddRectangle};

/// Drawing 2d context.
pub struct Context<'a> {
    /// Base/original transformation.
    pub base: Field<'a, Matrix2d>,
    /// Current transformation.
    pub transform: Field<'a, Matrix2d>,
}

impl<'a> Transform2d<'a> for Context<'a> { 
    #[inline(always)]
    fn trans(&'a self, x: f64, y: f64) -> Context<'a> {
        Context {
            base: Borrowed(self.base.get()),
            transform: Value({
                let trans = translate(x, y);
                 multiply(&trans, self.transform.get())
            }),
        }
    }

    #[inline(always)]
    fn trans_local(&'a self, x: f64, y: f64) -> Context<'a> {
        Context {
            base: Borrowed(self.base.get()),
            transform: Value({
                let trans = translate(x, y);
                 multiply(self.transform.get(), &trans)
            }),
        }
    }

    #[inline(always)]
    fn rot_rad(&'a self, angle: f64) -> Context<'a> {
        Context {
            base: Borrowed(self.base.get()),
            transform: Value({
                let rot = rotate_radians(angle);
                multiply(&rot, self.transform.get())
            }),
        }
    }

    #[inline(always)]
    fn rot_rad_local(&'a self, angle: f64) -> Context<'a> {
        Context {
            base: Borrowed(self.base.get()),
            transform: Value({
                let rot = rotate_radians(angle);
                multiply(self.transform.get(), &rot)
            }),
        }
    }

    #[inline(always)]
    fn scale(&'a self, sx: f64, sy: f64) -> Context<'a> {
        Context {
            base: Borrowed(self.base.get()),
            transform: Value({
                let scale = scale(sx, sy);
                multiply(&scale, self.transform.get())
            }),
        }
    }
    
    #[inline(always)]
    fn scale_local(&'a self, sx: f64, sy: f64) -> Context<'a> {
        Context {
            base: Borrowed(self.base.get()),
            transform: Value({
                let scale = scale(sx, sy);
                multiply(self.transform.get(), &scale)
            }),
        }
    }
    
    #[inline(always)]
    fn shear(&'a self, sx: f64, sy: f64) -> Context<'a> {
        Context {
            base: Borrowed(self.base.get()),
            transform: Value({
                let shear = shear(sx, sy);
                multiply(&shear, self.transform.get())
            }),
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
    assert!((c.transform.get()[1] - 1.0).abs() < 0.00001);
}

#[test]
fn test_scale() {
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

