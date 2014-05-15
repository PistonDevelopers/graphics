use {
    AddColor,
    Borrowed,
    Color,
    Field,
    Matrix2d,
    Rectangle,
    BevelRectangleColorContext,
    Value,
    View,
};
use vecmath::{
    identity,
};
use internal::{
    CanRectangle,
    CanTransform,
    HasRectangle,
    HasTransform,
};

/// A bevel rectangle context.
pub struct BevelRectangleContext<'a> {
    /// Base/origin transform.
    pub base: Field<'a, Matrix2d>,
    /// Current transform.
    pub transform: Field<'a, Matrix2d>,
    /// Current rectangle.
    pub rect: Field<'a, Rectangle>,
    /// Current bevel radius.
    pub bevel_radius: Field<'a, f64>,
}

impl<'a> Clone for BevelRectangleContext<'a> {
    #[inline(always)]
    fn clone(&self) -> BevelRectangleContext<'static> {
        BevelRectangleContext {
            base: self.base.clone(),
            transform: self.transform.clone(),
            rect: self.rect.clone(),
            bevel_radius: self.bevel_radius.clone(),
        }
    }
}

impl<'a> HasTransform<'a, Matrix2d> for BevelRectangleContext<'a> {
    #[inline(always)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a> CanTransform<'a, BevelRectangleContext<'a>, Matrix2d> for BevelRectangleContext<'a> {
    #[inline(always)]
    fn transform(&'a self, value: Matrix2d) -> BevelRectangleContext<'a> {
        BevelRectangleContext {
            base: Borrowed(self.base.get()),
            transform: Value(value),
            rect: Borrowed(self.rect.get()),
            bevel_radius: Borrowed(self.bevel_radius.get()),
        }
    }
}

impl<'a> HasRectangle<'a, Rectangle> for BevelRectangleContext<'a> {
    #[inline(always)]
    fn get_rectangle(&'a self) -> &'a Rectangle {
        self.rect.get()
    }
}

impl<'a> CanRectangle<'a, BevelRectangleContext<'a>, Rectangle> for BevelRectangleContext<'a> {
    #[inline(always)]
    fn rectangle(&'a self, rect: Rectangle) -> BevelRectangleContext<'a> {
        BevelRectangleContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            rect: Value(rect),
            bevel_radius: Borrowed(self.bevel_radius.get()),
        }
    }
}

impl<'a> AddColor<'a, BevelRectangleColorContext<'a>> for BevelRectangleContext<'a> {
    /// Creates a RectangleColorContext.
    #[inline(always)]
    fn rgba(&'a self, r: f32, g: f32, b: f32, a: f32) -> BevelRectangleColorContext<'a> {
        BevelRectangleColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            color: Value(Color([r, g, b, a])),
            rect: Borrowed(self.rect.get()),
            bevel_radius: Borrowed(self.bevel_radius.get()),
        }
    }
}

impl<'a> View<'a> for BevelRectangleContext<'a> {
    #[inline(always)]
    fn view(&'a self) -> BevelRectangleContext<'a> {
        BevelRectangleContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.base.get()),
            rect: Borrowed(self.rect.get()),
            bevel_radius: Borrowed(self.bevel_radius.get()),
        }
    }

    #[inline(always)]
    fn reset(&'a self) -> BevelRectangleContext<'a> {
        BevelRectangleContext {
            base: Borrowed(self.base.get()),
            transform: Value(identity()),
            rect: Borrowed(self.rect.get()),
            bevel_radius: Borrowed(self.bevel_radius.get()),
        }
    }

    #[inline(always)]
    fn store_view(&'a self) -> BevelRectangleContext<'a> {
        BevelRectangleContext {
            base: Borrowed(self.transform.get()),
            transform: Borrowed(self.transform.get()),
            rect: Borrowed(self.rect.get()),
            bevel_radius: Borrowed(self.bevel_radius.get()),
        }
    }
}

