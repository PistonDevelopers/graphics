use {
    AddColor, 
    Borrowed, 
    CanTransform,
    Field, 
    HasTransform,
    Matrix2d, 
    RelativeRectangle,
    RoundRectangle,
    RoundRectangleColorContext,
    Value, 
    View,
};
use vecmath::{
    identity,
    margin_round_rectangle, 
    relative_round_rectangle, 
};

/// A round rectangle context.
pub struct RoundRectangleContext<'a> {
    /// Base/origin transform.
    pub base: Field<'a, Matrix2d>,
    /// Current transform.
    pub transform: Field<'a, Matrix2d>,
    /// Current round rectangle.
    pub round_rect: Field<'a, RoundRectangle>,
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
            round_rect: Borrowed(self.round_rect.get()),
        }
    }
}

impl<'a> RelativeRectangle<'a> for RoundRectangleContext<'a> {
    #[inline(always)]
    fn margin(&'a self, m: f64) -> RoundRectangleContext<'a> {
        RoundRectangleContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            round_rect: Value(margin_round_rectangle(self.round_rect.get(), m)),
        }
    }

    #[inline(always)]
    fn rel(&'a self, x: f64, y: f64) -> RoundRectangleContext<'a> {
        RoundRectangleContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            round_rect: Value(relative_round_rectangle(self.round_rect.get(), x, y)),
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
            color: Value([r, g, b, a]),
            round_rect: Borrowed(self.round_rect.get()),
        }
    }
}

impl<'a> View<'a> for RoundRectangleContext<'a> {
    #[inline(always)]
    fn view(&'a self) -> RoundRectangleContext<'a> {
        RoundRectangleContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.base.get()),
            round_rect: Borrowed(self.round_rect.get()),
        }
    }

    #[inline(always)]
    fn reset(&'a self) -> RoundRectangleContext<'a> {
        RoundRectangleContext {
            base: Borrowed(self.base.get()),
            transform: Value(identity()),
            round_rect: Borrowed(self.round_rect.get()),
        }
    }

    #[inline(always)]
    fn store_view(&'a self) -> RoundRectangleContext<'a> {
        RoundRectangleContext {
            base: Borrowed(self.transform.get()),
            transform: Borrowed(self.transform.get()),
            round_rect: Borrowed(self.round_rect.get()),
        }
    }
}

