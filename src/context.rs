//! Transformation context

use current::{ Get, Modifier, Set };
use vecmath::{
    identity,
    Matrix2d,
    Scalar
};

/// Transform property
#[deriving(Copy)]
pub struct Transform(pub Matrix2d);

/// Wrapper trait for `Get<Transform>`
pub trait GetTransform: Get<Transform> {
    /// Gets view transform
    #[inline(always)]
    fn get_transform(&self) -> Transform {
        self.get()
    }
}

impl<T: Get<Transform>> GetTransform for T {}

/// Wrapper trait for `Set<Transform>`
pub trait SetTransform: Set<Transform> {
    /// Sets view transform
    #[inline(always)]
    fn set_transform(&mut self, val: Transform) {
        self.set_mut(val);
    }
}

impl<T: Set<Transform>> SetTransform for T {}

/// View transform property
#[deriving(Copy)]
pub struct ViewTransform(pub Matrix2d);

/// Wrapper trait for `Get<ViewTransform>`
pub trait GetViewTransform: Get<ViewTransform> {
    /// Gets view transform
    #[inline(always)]
    fn get_view_transform(&self) -> ViewTransform {
        self.get()
    }
}

impl<T: Get<ViewTransform>> GetViewTransform for T {}

/// Wrapper trait for `Set<ViewTransform>`
pub trait SetViewTransform: Set<ViewTransform> {
    /// Sets view transform
    #[inline(always)]
    fn set_view_transform(&mut self, val: ViewTransform) {
        self.set_mut(val);
    }
}

impl<T: Set<ViewTransform>> SetViewTransform for T {}

/// Drawing 2d context.
#[deriving(Copy, Clone)]
pub struct Context {
    /// View transformation.
    pub view: Matrix2d,
    /// Current transformation.
    pub transform: Matrix2d,
}

impl Modifier<Context> for Transform {
    fn modify(self, c: &mut Context) {
        let Transform(val) = self;
        c.transform = val;
    }
}

impl Get<Transform> for Context {
    fn get(&self) -> Transform {
        Transform(self.transform)
    }
}

impl Modifier<Context> for ViewTransform {
    fn modify(self, c: &mut Context) {
        let ViewTransform(val) = self;
        c.view = val;
    }
}

impl Get<ViewTransform> for Context {
    fn get(&self) -> ViewTransform {
        ViewTransform(self.view)
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

#[cfg(test)]
mod test {
    use super::Context;
    use std::num::Float;

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
}

