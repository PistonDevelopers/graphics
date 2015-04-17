//! Transformation context

use draw_state::DrawState;
use default_draw_state;
use math::{
    abs_transform,
    identity,
    get_scale,
    multiply,
    scale,
    Matrix2d,
    Scalar,
};
use types::Size;
use Viewport;

/// Drawing 2d context.
#[derive(Copy, Clone)]
pub struct Context {
    /// Viewport information.
    pub viewport: Option<Viewport>,
    /// View transformation.
    pub view: Matrix2d,
    /// Current transformation.
    pub transform: Matrix2d,
    /// Current draw state settings.
    pub draw_state: DrawState,
}

impl Context {
    /// Creates a new drawing context.
    #[inline(always)]
    pub fn new() -> Context {
        Context {
            view: identity(),
            transform: identity(),
            draw_state: *default_draw_state(),
            viewport: None,
        }
    }

    /// Creates a new context with absolute transform in point coordinates.
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
    pub fn new_viewport(viewport: Viewport) -> Context {
        let sx = viewport.draw_size[0] as f64 / viewport.window_size[0] as f64;
        let sy = viewport.draw_size[1] as f64 / viewport.window_size[1] as f64;
        let mat = multiply(
                abs_transform(viewport.draw_size[0] as f64, viewport.draw_size[1] as f64),
                scale(sx, sy)
            );
        Context {
            view: mat,
            transform: mat,
            draw_state: *default_draw_state(),
            viewport: Some(viewport),
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
        let mat = abs_transform(w, h);
        Context {
            view: mat,
            transform: mat,
            draw_state: *default_draw_state(),
            viewport: None,
        }
    }

    /// Moves the current transform to the view coordinate system.
    ///
    /// This is usually [0.0, 0.0] in the upper left corner
    /// with the x axis pointing to the right
    /// and the y axis pointing down.
    #[inline(always)]
    pub fn view(mut self) -> Self {
        self.transform = self.view;
        self
    }

    /// Moves the current transform to the default coordinate system.
    ///
    /// This is usually [0.0, 0.0] in the center
    /// with the x axis pointing to the right
    /// and the y axis pointing up.
    #[inline(always)]
    pub fn reset(mut self) -> Self {
        self.transform = identity();
        self
    }

    /// Stores the current transform as new view.
    #[inline(always)]
    pub fn store_view(mut self) -> Self {
        self.view = self.transform;
        self
    }

    /// Computes the current view size.
    #[inline(always)]
    pub fn get_view_size(&self) -> Size {
        let scale = get_scale(self.view);
        [2.0 / scale[0], 2.0 / scale[1]].into()
    }
}

#[cfg(test)]
mod test {
    use super::Context;
    use num::Float;

    #[test]
    fn test_context() {
        use Transformed;

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
        use Transformed;

        let c = Context::new();
        let c = c.scale(2.0, 3.0);
        let transform = c.transform;
        assert!((transform[0][0] - 2.0).abs() < 0.00001);
        assert!((transform[1][1] - 3.0).abs() < 0.00001);
    }
}
