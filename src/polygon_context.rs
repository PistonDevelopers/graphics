use {
    AddColor,
    Borrowed,
    Field,
    PolygonColorContext,
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
};

/// A polygon context.
pub struct PolygonContext<'a, 'b> {
    /// View transform.
    pub view: Field<'a, Matrix2d>,
    /// Current transform.
    pub transform: Field<'a, Matrix2d>,
    /// Current polygon.
    pub polygon: Field<'a, Polygon<'b>>
}

impl<'a, 'b> Clone for PolygonContext<'a, 'b> {
    #[inline(always)]
    fn clone(&self) -> PolygonContext<'static, 'b> {
        PolygonContext {
            view: Value(*self.view.get()),
            transform: Value(*self.transform.get()),
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
            view: Borrowed(self.view.get()),
            transform: Value(value),
            polygon: Borrowed(self.polygon.get()),
        }
    }
}

impl<'a, 'b> HasViewTransform<'a, Matrix2d> for PolygonContext<'a, 'b> {
    #[inline(always)]
    fn get_view_transform(&'a self) -> &'a Matrix2d {
        self.view.get()
    }
}

impl<'a, 'b> CanViewTransform<'a, PolygonContext<'a, 'b>, Matrix2d> 
for PolygonContext<'a, 'b> {
    #[inline(always)]
    fn view_transform(&'a self, value: Matrix2d) -> PolygonContext<'a, 'b> {
        PolygonContext {
            view: Value(value),
            transform: Borrowed(self.transform.get()),
            polygon: Borrowed(self.polygon.get()),
        }
    }
}

impl<'a, 'b> AddColor<'a, PolygonColorContext<'a, 'b>> for PolygonContext<'a, 'b> {
    #[inline(always)]
    fn rgba(
        &'a self, 
        r: ColorComponent, 
        g: ColorComponent, 
        b: ColorComponent, 
        a: ColorComponent
    ) -> PolygonColorContext<'a, 'b> {
        PolygonColorContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            color: Value([r, g, b, a]),
            polygon: Borrowed(self.polygon.get()),
        }
    }
}

