//! Transformation context

use current::{ GetFrom, SetAt };
use vecmath::{
    identity,
    Matrix2d,
    Scalar
};

/// Transform property
#[deriving(Copy)]
pub struct Transform(pub Matrix2d);

/// View transform property
#[deriving(Copy)]
pub struct ViewTransform(pub Matrix2d);

/// Drawing 2d context.
#[deriving(Copy, Clone)]
pub struct Context {
    /// View transformation.
    pub view: Matrix2d,
    /// Current transformation.
    pub transform: Matrix2d,
}

impl SetAt<Context> for Transform {
    fn set_at(self, c: &mut Context) {
        let Transform(val) = self;
        c.transform = val;
    }
}

impl GetFrom<Context> for Transform {
    fn get_from(obj: &Context) -> Transform {
        Transform(obj.transform)
    }
}

impl SetAt<Context> for ViewTransform {
    fn set_at(self, c: &mut Context) {
        let ViewTransform(val) = self;
        c.view = val;
    }
}

impl GetFrom<Context> for ViewTransform {
    fn get_from(obj: &Context) -> ViewTransform {
        ViewTransform(obj.view)
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

