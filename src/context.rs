
use add;
use {
    BackEnd,
    Draw,
    ImageSize,
};
use std::num::Float;
use triangulation;
use internal::{
    Color,
    ColorComponent,
    Polygon,
    Polygons,
    Radius,
    Rectangle,
    SourceRectangle,
};
use can::{
    CanColor,
    CanRectangle,
    CanSourceRectangle,
    CanTransform,
    CanViewTransform,
};
use has::{
    HasColor,
    HasRectangle,
    HasSourceRectangle,
    HasTransform,
    HasViewTransform,
};
use vecmath::{
    identity,
    Matrix2d,
    Scalar
};
use shape;

/// Drawing 2d context.
#[deriving(Copy, Clone)]
pub struct Context {
    /// View transformation.
    pub view: Matrix2d,
    /// Current transformation.
    pub transform: Matrix2d,
}

impl HasTransform for Context {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform
    }
}

impl CanTransform for Context {
    #[inline(always)]
    fn transform(&self, value: Matrix2d) -> Context {
        Context {
            transform: value,
            ..*self
        }
    }
}

impl HasViewTransform for Context {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view
    }
}

impl CanViewTransform for Context {
    #[inline(always)]
    fn view_transform(&self, value: Matrix2d) -> Context {
        Context { view: value, ..*self }
    }
}

impl Context {
    /// Creates a new drawing context.
    #[inline(always)]
    pub fn new() -> Context {
        Context {
            view: identity(),
            transform: identity(),
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
    pub fn abs(w: Scalar, h: Scalar) -> Context {
        let sx = 2.0 / w;
        let sy = -2.0 / h;
        let mat = [[ sx, 0.0, -1.0 ],
                   [ 0.0,  sy, 1.0 ]];
        Context {
            view: mat,
            transform: mat,
        }
    }
}

#[test]
fn test_context() {
    use RelativeTransform;

    let c = Context::new();
    {
        let d = c.trans(20.0, 40.0);
        let d = d.trans(10.0, 10.0);
        let transform = d.transform;
        assert_eq!(transform[0][2], 30.0);
        assert_eq!(transform[1][2], 50.0);
    }

    let transform = c.transform;
    assert_eq!(transform[0][2], 0.0);
    assert_eq!(transform[1][2], 0.0);

    let c = c.rot_deg(90.0);
    let transform = c.transform;
    assert!((transform[0][0] - 0.0).abs() < 0.00001);
    assert!((transform[0][1] + 1.0).abs() < 0.00001);
}

#[test]
fn test_scale() {
    use RelativeTransform;

    let c = Context::new();
    let c = c.scale(2.0, 3.0);
    let transform = c.transform;
    assert!((transform[0][0] - 2.0).abs() < 0.00001);
    assert!((transform[1][1] - 3.0).abs() < 0.00001);
}

#[test]
fn test_rect() {
    use add::AddRectangle;

    let c = Context::new();
    let d = c.rect(0.0, 0.0, 100.0, 50.0);
    let rect = d.shape.variant.rect;
    println!("{}", rect[2]);
    assert_eq!(rect[2], 100.0);
}

#[test]
fn test_ellipse() {
    use add::AddEllipse;

    let c = Context::new();
    let d = c.ellipse(0.0, 0.0, 100.0, 100.0);
    let rect = d.shape.variant.rect;
    assert_eq!(rect[2], 100.0);
}

#[test]
fn test_rgba_1() {
    use add::AddRectangle;
    use add::AddColor;

    let c = Context::new();
    let d = c.rect(0.0, 0.0, 100.0, 100.0);
    let e = d.rgba(1.0, 0.0, 0.0, 1.0);
    let color = e.color;
    assert_eq!(color[0], 1.0);
}

