use {
    AddColor,
    Borrowed, 
    CanTransform,
    EllipseColorContext,
    Field, 
    HasTransform,
    Matrix2d, 
    Rectangle,
    RelativeRectangle, 
    Value,
    View,
};
use vecmath::{
    identity,
    margin_rectangle, 
    relative_rectangle, 
};

/// An ellipse context.
pub struct EllipseContext<'a> {
    /// Base/origin transform.
    pub base: Field<'a, Matrix2d>,
    /// Current transformation.
    pub transform: Field<'a, Matrix2d>,
    /// Current rectangle enclosing the ellipse.
    pub rect: Field<'a, Rectangle>,
}

impl<'a> HasTransform<'a, Matrix2d> for EllipseContext<'a> {
    #[inline(always)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a> CanTransform<'a, EllipseContext<'a>, Matrix2d> for EllipseContext<'a> {
    #[inline(always)]
    fn transform(&'a self, value: Matrix2d) -> EllipseContext<'a> {
        EllipseContext {
            base: Borrowed(self.base.get()),
            transform: Value(value),
            rect: Borrowed(self.rect.get()),
        }
    }
}

impl<'a> AddColor<'a, EllipseColorContext<'a>> for EllipseContext<'a> {
    #[inline(always)]
    fn rgba(&'a self, r: f32, g: f32, b: f32, a: f32) -> EllipseColorContext<'a> {
        EllipseColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            rect: Borrowed(self.rect.get()),
            color: Value([r, g, b, a]),
        }
    }
}

impl<'a> RelativeRectangle<'a> for EllipseContext<'a> {
    #[inline(always)]
    fn margin(&'a self, m: f64) -> EllipseContext<'a> {
        EllipseContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            rect: Value(margin_rectangle(self.rect.get(), m)),
        }
    }

    #[inline(always)]
    fn rel(&'a self, x: f64, y: f64) -> EllipseContext<'a> {
        EllipseContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            rect: Value(relative_rectangle(self.rect.get(), x, y)),
        }
    }
}

impl<'a> View<'a> for EllipseContext<'a> {
    #[inline(always)]
    fn view(&'a self) -> EllipseContext<'a> {
        EllipseContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.base.get()),
            rect: Borrowed(self.rect.get()),
        }
    }

    #[inline(always)]
    fn reset(&'a self) -> EllipseContext<'a> {
        EllipseContext {
            base: Borrowed(self.base.get()),
            transform: Value(identity()),
            rect: Borrowed(self.rect.get()),
        }
    }

    #[inline(always)]
    fn store_view(&'a self) -> EllipseContext<'a> {
        EllipseContext {
            base: Borrowed(self.transform.get()),
            transform: Borrowed(self.transform.get()),
            rect: Borrowed(self.rect.get()),
        }
    }
}


