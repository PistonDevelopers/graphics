
use {
    AddBevel,
    AddRound,
    BackEnd,
    BevelRectangleBorderColorContext,
    Draw,
    Field,
    ImageSize,
    RoundRectangleBorderColorContext,
    Value,
};
use triangulation::{
    rect_border_tri_list_rgba_f32,
    rect_border_tri_list_xy_f32,
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

/// A rectangle color context.
pub struct RectangleBorderColorContext {
    /// View transformation.
    pub view: Field<Matrix2d>,
    /// Current transformation.
    pub transform: Field<Matrix2d>,
    /// Current rectangle.
    pub rect: Field<Rectangle>,
    /// Current color.
    pub color: Field<Color>,
    /// Current border.
    pub border: Field<Radius>,
}

impl
Clone 
for RectangleBorderColorContext {
    #[inline(always)]
    fn clone(&self) -> RectangleBorderColorContext {
        RectangleBorderColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            rect: Value(self.rect.get()),
            color: Value(self.color.get()),
            border: Value(self.border.get()),
        }
    }
}

impl
HasTransform<Matrix2d> 
for RectangleBorderColorContext {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform.get()
    }
}

impl
CanTransform<RectangleBorderColorContext, Matrix2d> 
for RectangleBorderColorContext {
    #[inline(always)]
    fn transform(
        &self, 
        value: Matrix2d
    ) -> RectangleBorderColorContext {
        RectangleBorderColorContext {
            view: Value(self.view.get()),
            transform: Value(value),
            rect: Value(self.rect.get()),
            color: Value(self.color.get()),
            border: Value(self.border.get()),
        }
    }
}

impl
HasViewTransform<Matrix2d> 
for RectangleBorderColorContext {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view.get()
    }
}

impl
CanViewTransform<RectangleBorderColorContext, Matrix2d> 
for RectangleBorderColorContext {
    #[inline(always)]
    fn view_transform(
        &self, 
        value: Matrix2d
    ) -> RectangleBorderColorContext {
        RectangleBorderColorContext {
            view: Value(value),
            transform: Value(self.transform.get()),
            rect: Value(self.rect.get()),
            color: Value(self.color.get()),
            border: Value(self.border.get()),
        }
    }
}

impl
HasColor<Color> 
for RectangleBorderColorContext {
    #[inline(always)]
    fn get_color(&self) -> Color {
        self.color.get()
    }
}

impl
CanColor<RectangleBorderColorContext, Color> 
for RectangleBorderColorContext {
    #[inline(always)]
    fn color(
        &self, 
        value: Color
    ) -> RectangleBorderColorContext {
        RectangleBorderColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            color: Value(value),
            rect: Value(self.rect.get()),
            border: Value(self.border.get()),
        }
    }
}

impl
HasRectangle<Rectangle> 
for RectangleBorderColorContext {
    #[inline(always)]
    fn get_rectangle(&self) -> Rectangle {
        self.rect.get()
    }
}

impl
CanRectangle<RectangleBorderColorContext, Rectangle> 
for RectangleBorderColorContext {
    #[inline(always)]
    fn rectangle(
        &self, 
        rect: Rectangle
    ) -> RectangleBorderColorContext {
        RectangleBorderColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            rect: Value(rect),
            color: Value(self.color.get()),
            border: Value(self.border.get()),
        }
    }
}

impl
AddRound<RoundRectangleBorderColorContext> 
for RectangleBorderColorContext {
    #[inline(always)]
    fn round(
        &self, 
        radius: f64
    ) -> RoundRectangleBorderColorContext {
        RoundRectangleBorderColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            color: Value(self.color.get()),
            rect: Value(self.rect.get()),
            round_radius: Value(radius),
            border: Value(self.border.get()),
        }
    }
}

impl
AddBevel<BevelRectangleBorderColorContext> 
for RectangleBorderColorContext {
    #[inline(always)]
    fn bevel(
        &self, 
        radius: f64
    ) -> BevelRectangleBorderColorContext {
        BevelRectangleBorderColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            color: Value(self.color.get()),
            rect: Value(self.rect.get()),
            bevel_radius: Value(radius),
            border: Value(self.border.get()),
        }
    }
}

impl<B: BackEnd<I>, I: ImageSize> 
Draw<B, I> 
for RectangleBorderColorContext {
    #[inline(always)]
    fn draw(&self, back_end: &mut B) {
        if back_end.supports_tri_list_xy_f32_rgba_f32() {
            let rect = self.rect.get();
            let color = self.color.get();
            let border = self.border.get();
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely opaque.
            let needs_alpha = color[3] != 1.0;
            if needs_alpha { back_end.enable_alpha_blend(); }
            back_end.tri_list_xy_f32_rgba_f32(
                rect_border_tri_list_xy_f32(
                    self.transform.get(), rect, border),
                rect_border_tri_list_rgba_f32(color)
            );
            if needs_alpha { back_end.disable_alpha_blend(); }
        } else {
            unimplemented!();
        }
    }
}

