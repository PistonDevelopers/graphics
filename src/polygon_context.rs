use {
    AddColor,
    Borrowed,
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
pub struct PolygonContext<'a> {
    /// Base/origin transform.
    pub base: Field<'a, Matrix2d>,
    /// Current transform.
    pub transform: Field<'a, Matrix2d>,
    /// Current polygon.
    pub polygon: Field<'a, &'a [f64]>
}

impl<'a> HasTransform<'a, Matrix2d> for PolygonContext<'a> {
    #[inline(always)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a> CanTransform<'a, PolygonContext<'a>, Matrix2d> for PolygonContext<'a> {
    #[inline(always)]
    fn transform(&'a self, value: Matrix2d) -> PolygonContext<'a> {
        PolygonContext {
            base: Borrowed(self.base.get()),
            transform: Value(value),
            polygon: Borrowed(self.polygon.get()),
        }
    }
}

impl<'a> AddColor<'a, PolygonColorContext<'a>> for PolygonContext<'a> {
    #[inline(always)]
    fn rgba(&'a self, r: f32, g: f32, b: f32, a: f32) -> PolygonColorContext<'a> {
        PolygonColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            color: Value([r, g, b, a]),
            polygon: Borrowed(self.polygon.get()),
        }
    }
}

impl<'a> View<'a> for PolygonContext<'a> {
    #[inline(always)]
    fn view(&'a self) -> PolygonContext<'a> {
        PolygonContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.base.get()),
            polygon: Borrowed(self.polygon.get()),
        }
    }

    #[inline(always)]
    fn reset(&'a self) -> PolygonContext<'a> {
        PolygonContext {
            base: Borrowed(self.base.get()),
            transform: Value(identity()),
            polygon: Borrowed(self.polygon.get()),
        }
    }

    #[inline(always)]
    fn store_view(&'a self) -> PolygonContext<'a> {
        PolygonContext {
            base: Borrowed(self.transform.get()),
            transform: Borrowed(self.transform.get()),
            polygon: Borrowed(self.polygon.get()),
        }
    }
}

