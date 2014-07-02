use {
    AddColor,
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
pub struct PolygonContext<'b> {
    /// View transform.
    pub view: Field<Matrix2d>,
    /// Current transform.
    pub transform: Field<Matrix2d>,
    /// Current polygon.
    pub polygon: Field<Polygon<'b>>
}

impl<'b> 
Clone 
for PolygonContext<'b> {
    #[inline(always)]
    fn clone(&self) -> PolygonContext<'b> {
        PolygonContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            polygon: Value(self.polygon.get()),
        }
    }
}

impl<'b> 
HasTransform<Matrix2d> 
for PolygonContext<'b> {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform.get()
    }
}

impl<'b> 
CanTransform<PolygonContext<'b>, Matrix2d> 
for PolygonContext<'b> {
    #[inline(always)]
    fn transform(
        &self, 
        value: Matrix2d
    ) -> PolygonContext<'b> {
        PolygonContext {
            view: Value(self.view.get()),
            transform: Value(value),
            polygon: Value(self.polygon.get()),
        }
    }
}

impl<'b> 
HasViewTransform<Matrix2d> 
for PolygonContext<'b> {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view.get()
    }
}

impl<'b> 
CanViewTransform<PolygonContext<'b>, Matrix2d> 
for PolygonContext<'b> {
    #[inline(always)]
    fn view_transform(
        &self, 
        value: Matrix2d
    ) -> PolygonContext<'b> {
        PolygonContext {
            view: Value(value),
            transform: Value(self.transform.get()),
            polygon: Value(self.polygon.get()),
        }
    }
}

impl<'b> 
AddColor<PolygonColorContext<'b>> 
for PolygonContext<'b> {
    #[inline(always)]
    fn rgba(
        &self, 
        r: ColorComponent, 
        g: ColorComponent, 
        b: ColorComponent, 
        a: ColorComponent
    ) -> PolygonColorContext<'b> {
        PolygonColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            color: Value([r, g, b, a]),
            polygon: Value(self.polygon.get()),
        }
    }
}

