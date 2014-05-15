use {
    AddColor,
    Borrowed,
    Color,
    Field,
    Matrix2d,
    PolygonColorContext,
    Value,
    View
};
use vecmath::{
    identity,
};
use internal::{
    CanTransform,
    HasTransform,
};

/// A polygon context.
pub struct PolygonContext<'a, 'b> {
    /// Base/origin transform.
    pub base: Field<'a, Matrix2d>,
    /// Current transform.
    pub transform: Field<'a, Matrix2d>,
    /// Current polygon.
    pub polygon: Field<'a, &'b [f64]>
}

impl<'a, 'b> Clone for PolygonContext<'a, 'b> {
    #[inline(always)]
    fn clone(&self) -> PolygonContext<'static, 'b> {
        PolygonContext {
            base: self.base.clone(),
            transform: self.transform.clone(),
            polygon: Value(*self.polygon.get()),
        }
    }
}

impl<'a, 'b> HasTransform<'a, Matrix2d> for PolygonContext<'a, 'b> {
    #[inline(always)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a, 'b> CanTransform<'a, PolygonContext<'a, 'b>, Matrix2d> for PolygonContext<'a, 'b> {
    #[inline(always)]
    fn transform(&'a self, value: Matrix2d) -> PolygonContext<'a, 'b> {
        PolygonContext {
            base: Borrowed(self.base.get()),
            transform: Value(value),
            polygon: Borrowed(self.polygon.get()),
        }
    }
}

impl<'a, 'b> AddColor<'a, PolygonColorContext<'a, 'b>> for PolygonContext<'a, 'b> {
    #[inline(always)]
    fn rgba(&'a self, r: f32, g: f32, b: f32, a: f32) -> PolygonColorContext<'a, 'b> {
        PolygonColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            color: Value(Color([r, g, b, a])),
            polygon: Borrowed(self.polygon.get()),
        }
    }
}

impl<'a, 'b> View<'a> for PolygonContext<'a, 'b> {
    #[inline(always)]
    fn view(&'a self) -> PolygonContext<'a, 'b> {
        PolygonContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.base.get()),
            polygon: Borrowed(self.polygon.get()),
        }
    }

    #[inline(always)]
    fn reset(&'a self) -> PolygonContext<'a, 'b> {
        PolygonContext {
            base: Borrowed(self.base.get()),
            transform: Value(identity()),
            polygon: Borrowed(self.polygon.get()),
        }
    }

    #[inline(always)]
    fn store_view(&'a self) -> PolygonContext<'a, 'b> {
        PolygonContext {
            base: Borrowed(self.transform.get()),
            transform: Borrowed(self.transform.get()),
            polygon: Borrowed(self.polygon.get()),
        }
    }
}

