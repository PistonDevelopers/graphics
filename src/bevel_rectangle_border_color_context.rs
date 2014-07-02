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

/// A bevel rectangle border color context.
pub struct BevelRectangleBorderColorContext {
    /// View transformation.
    pub view: Field<Matrix2d>,
    /// Current transformation.
    pub transform: Field<Matrix2d>,
    /// Current rectangle.
    pub rect: Field<Rectangle>,
    /// Current bevel radius.
    pub bevel_radius: Field<Radius>,
    /// Current color.
    pub color: Field<Color>,
    /// Current border.
    pub border: Field<Radius>,
}

impl
Clone 
for BevelRectangleBorderColorContext {
    #[inline(always)]
    fn clone(&self) -> BevelRectangleBorderColorContext {
        BevelRectangleBorderColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            rect: Value(self.rect.get()),
            bevel_radius: Value(self.bevel_radius.get()),
            color: Value(self.color.get()),
            border: Value(self.border.get()),
        }
    }
}

impl
HasTransform<Matrix2d> 
for BevelRectangleBorderColorContext {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform.get()
    }
}

impl
CanTransform<BevelRectangleBorderColorContext, Matrix2d> 
for BevelRectangleBorderColorContext {
    #[inline(always)]
    fn transform(
        &self, 
        value: Matrix2d
    ) -> BevelRectangleBorderColorContext {
        BevelRectangleBorderColorContext {
            view: Value(self.view.get()),
            transform: Value(value),
            rect: Value(self.rect.get()),
            bevel_radius: Value(self.bevel_radius.get()),
            color: Value(self.color.get()),
            border: Value(self.border.get()),
        }
    }
}

impl
HasViewTransform<Matrix2d> 
for BevelRectangleBorderColorContext {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view.get()
    }
}

impl
CanViewTransform<BevelRectangleBorderColorContext, Matrix2d> 
for BevelRectangleBorderColorContext {
    #[inline(always)]
    fn view_transform(
        &self, 
        value: Matrix2d
    ) -> BevelRectangleBorderColorContext {
        BevelRectangleBorderColorContext {
            view: Value(value),
            transform: Value(self.transform.get()),
            rect: Value(self.rect.get()),
            bevel_radius: Value(self.bevel_radius.get()),
            color: Value(self.color.get()),
            border: Value(self.border.get()),
        }
    }
}

impl
HasColor<Color> 
for BevelRectangleBorderColorContext {
    #[inline(always)]
    fn get_color(&self) -> Color {
        self.color.get()
    }
}

impl
CanColor<BevelRectangleBorderColorContext, Color> 
for BevelRectangleBorderColorContext {
    #[inline(always)]
    fn color(
        &self, 
        value: Color
    ) -> BevelRectangleBorderColorContext {
        BevelRectangleBorderColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            color: Value(value),
            rect: Value(self.rect.get()),
            bevel_radius: Value(self.bevel_radius.get()),
            border: Value(self.border.get()),
        }
    }
}

impl
HasRectangle<Rectangle> 
for BevelRectangleBorderColorContext {
    #[inline(always)]
    fn get_rectangle(&self) -> Rectangle {
        self.rect.get()
    }
}

impl
CanRectangle<BevelRectangleBorderColorContext, Rectangle> 
for BevelRectangleBorderColorContext {
    #[inline(always)]
    fn rectangle(
        &self, 
        rect: Rectangle
    ) -> BevelRectangleBorderColorContext {
        BevelRectangleBorderColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            rect: Value(rect),
            bevel_radius: Value(self.bevel_radius.get()),
            color: Value(self.color.get()),
            border: Value(self.border.get()),
        }
    }
}

