use {
    AddColor,
    PolygonColorContext,
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
    pub view: Matrix2d,
    /// Current transform.
    pub transform: Matrix2d,
    /// Current polygon.
    pub polygon: Polygon<'b>
}

impl<'b> 
Clone 
for PolygonContext<'b> {
    #[inline(always)]
    fn clone(&self) -> PolygonContext<'b> {
        PolygonContext {
            view: self.view,
            transform: self.transform,
            polygon: self.polygon,
        }
    }
}

impl<'b> 
HasTransform<Matrix2d> 
for PolygonContext<'b> {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform
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
            view: self.view,
            transform: value,
            polygon: self.polygon,
        }
    }
}

impl<'b> 
HasViewTransform<Matrix2d> 
for PolygonContext<'b> {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view
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
            view: value,
            transform: self.transform,
            polygon: self.polygon,
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
            view: self.view,
            transform: self.transform,
            color: [r, g, b, a],
            polygon: self.polygon,
        }
    }
}

