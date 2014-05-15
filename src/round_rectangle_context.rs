use {
    AddColor,
    Borrowed,
    Color,
    Field,
    Matrix2d,
    Rectangle,
    RoundRectangleColorContext,
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

/// A round rectangle context.
pub struct RoundRectangleContext<'a> {
    /// Base/origin transform.
    pub base: Field<'a, Matrix2d>,
    /// Current transform.
    pub transform: Field<'a, Matrix2d>,
    /// Current rectangle.
    pub rect: Field<'a, Rectangle>,
    /// Current roundness radius.
    pub round_radius: Field<'a, f64>,
}

impl<'a> Clone for RoundRectangleContext<'a> {
    #[inline(always)]
    fn clone(&self) -> RoundRectangleContext<'static> {
        RoundRectangleContext {
            base: self.base.clone(),
            transform: self.transform.clone(),
            rect: self.rect.clone(),
            round_radius: self.round_radius.clone(),
        }
    }
}

impl<'a> HasTransform<'a, Matrix2d> for RoundRectangleContext<'a> {
    #[inline(always)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a> CanTransform<'a, RoundRectangleContext<'a>, Matrix2d> for RoundRectangleContext<'a> {
    #[inline(always)]
    fn transform(&'a self, value: Matrix2d) -> RoundRectangleContext<'a> {
        RoundRectangleContext {
            base: Borrowed(self.base.get()),
            transform: Value(value),
            rect: Borrowed(self.rect.get()),
            round_radius: Borrowed(self.round_radius.get()),
        }
    }
}

impl<'a> HasRectangle<'a, Rectangle> for RoundRectangleContext<'a> {
    #[inline(always)]
    fn get_rectangle(&'a self) -> &'a Rectangle {
        self.rect.get()
    }
}

impl<'a> CanRectangle<'a, RoundRectangleContext<'a>, Rectangle> for RoundRectangleContext<'a> {
    #[inline(always)]
    fn rectangle(&'a self, rect: Rectangle) -> RoundRectangleContext<'a> {
        RoundRectangleContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            rect: Value(rect),
            round_radius: Borrowed(self.round_radius.get()),
        }
    }
}

impl<'a> AddColor<'a, RoundRectangleColorContext<'a>> for RoundRectangleContext<'a> {
    /// Creates a RectangleColorContext.
    #[inline(always)]
    fn rgba(&'a self, r: f32, g: f32, b: f32, a: f32) -> RoundRectangleColorContext<'a> {
        RoundRectangleColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            color: Value(Color([r, g, b, a])),
            rect: Borrowed(self.rect.get()),
            round_radius: Borrowed(self.round_radius.get()),
        }
    }
}

impl<'a> View<'a> for RoundRectangleContext<'a> {
    #[inline(always)]
    fn view(&'a self) -> RoundRectangleContext<'a> {
        RoundRectangleContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.base.get()),
            rect: Borrowed(self.rect.get()),
            round_radius: Borrowed(self.round_radius.get()),
        }
    }

    #[inline(always)]
    fn reset(&'a self) -> RoundRectangleContext<'a> {
        RoundRectangleContext {
            base: Borrowed(self.base.get()),
            transform: Value(identity()),
            rect: Borrowed(self.rect.get()),
            round_radius: Borrowed(self.round_radius.get()),
        }
    }

    #[inline(always)]
    fn store_view(&'a self) -> RoundRectangleContext<'a> {
        RoundRectangleContext {
            base: Borrowed(self.transform.get()),
            transform: Borrowed(self.transform.get()),
            rect: Borrowed(self.rect.get()),
            round_radius: Borrowed(self.round_radius.get()),
        }
    }
}

