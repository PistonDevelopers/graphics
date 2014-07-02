
use {
    Field,
    Value,
};
use triangulation::{
};
use internal::{
    CanColor,
    CanRectangle,
    CanTransform,
    CanViewTransform,
    Color,
    HasColor,
    HasRectangle,
    HasTransform,
    HasViewTransform,
    Matrix2d,
    Radius,
    Rectangle,
};

/// A round rectangle border color context.
pub struct RoundRectangleBorderColorContext {
    /// View transformation.
    pub view: Field<Matrix2d>,
    /// Current transformation.
    pub transform: Field<Matrix2d>,
    /// Current rectangle.
    pub rect: Field<Rectangle>,
    /// Current roundness radius.
    pub round_radius: Field<Radius>,
    /// Current color.
    pub color: Field<Color>,
    /// Current border.
    pub border: Field<Radius>,
}

impl Clone for RoundRectangleBorderColorContext {
    #[inline(always)]
    fn clone(&self) -> RoundRectangleBorderColorContext {
        RoundRectangleBorderColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            rect: Value(self.rect.get()),
            round_radius: Value(self.round_radius.get()),
            color: Value(self.color.get()),
            border: Value(self.border.get()),
        }
    }
}

impl HasTransform<Matrix2d> for RoundRectangleBorderColorContext {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform.get()
    }
}

impl CanTransform<RoundRectangleBorderColorContext, Matrix2d> 
for RoundRectangleBorderColorContext {
    #[inline(always)]
    fn transform(&self, value: Matrix2d) -> RoundRectangleBorderColorContext {
        RoundRectangleBorderColorContext {
            view: Value(self.view.get()),
            transform: Value(value),
            rect: Value(self.rect.get()),
            round_radius: Value(self.round_radius.get()),
            color: Value(self.color.get()),
            border: Value(self.border.get()),
        }
    }
}

impl HasViewTransform<Matrix2d> 
for RoundRectangleBorderColorContext {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view.get()
    }
}

impl CanViewTransform<RoundRectangleBorderColorContext, Matrix2d> 
for RoundRectangleBorderColorContext {
    #[inline(always)]
    fn view_transform(&self, value: Matrix2d) -> RoundRectangleBorderColorContext {
        RoundRectangleBorderColorContext {
            view: Value(value),
            transform: Value(self.transform.get()),
            rect: Value(self.rect.get()),
            round_radius: Value(self.round_radius.get()),
            color: Value(self.color.get()),
            border: Value(self.border.get()),
        }
    }
}

impl HasColor<Color> 
for RoundRectangleBorderColorContext {
    #[inline(always)]
    fn get_color(&self) -> Color {
        self.color.get()
    }
}

impl CanColor<RoundRectangleBorderColorContext, Color> 
for RoundRectangleBorderColorContext {
    #[inline(always)]
    fn color(&self, value: Color) -> RoundRectangleBorderColorContext {
        RoundRectangleBorderColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            color: Value(value),
            rect: Value(self.rect.get()),
            round_radius: Value(self.round_radius.get()),
            border: Value(self.border.get()),
        }
    }
}

impl HasRectangle<Rectangle> 
for RoundRectangleBorderColorContext {
    #[inline(always)]
    fn get_rectangle(&self) -> Rectangle {
        self.rect.get()
    }
}

impl CanRectangle<RoundRectangleBorderColorContext, Rectangle> 
for RoundRectangleBorderColorContext {
    #[inline(always)]
    fn rectangle(&self, rect: Rectangle) -> RoundRectangleBorderColorContext {
        RoundRectangleBorderColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            rect: Value(rect),
            round_radius: Value(self.round_radius.get()),
            color: Value(self.color.get()),
            border: Value(self.border.get()),
        }
    }
}

