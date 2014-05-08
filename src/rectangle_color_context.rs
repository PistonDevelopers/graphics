
use {
    AddRound, 
    BackEnd,
    Borrowed, 
    CanColor,
    CanTransform,
    Clear, 
    Color,
    Field, 
    Fill, 
    HasColor,
    HasTransform,
    Matrix2d, 
    Rectangle, 
    RelativeRectangle,
    RoundRectangleColorContext,
    Value,
    View
};
use vecmath::{
    identity,
    margin_rectangle, 
    relative_rectangle, 
};
use triangulation::{
    rect_tri_list_xy_f32, 
    rect_tri_list_rgba_f32,
};

/// A rectangle color context.
pub struct RectangleColorContext<'a> {
    /// Base/original transformation.
    pub base: Field<'a, Matrix2d>,
    /// Current transformation.
    pub transform: Field<'a, Matrix2d>,
    /// Current rectangle.
    pub rect: Field<'a, Rectangle>,
    /// Current color.
    pub color: Field<'a, Color>,
}

impl<'a> HasTransform<'a, Matrix2d> for RectangleColorContext<'a> {
    #[inline(always)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a> CanTransform<'a, RectangleColorContext<'a>, Matrix2d> for RectangleColorContext<'a> {
    #[inline(always)]
    fn transform(&'a self, value: Matrix2d) -> RectangleColorContext<'a> {
        RectangleColorContext {
            base: Borrowed(self.base.get()),
            transform: Value(value),
            rect: Borrowed(self.rect.get()),
            color: Borrowed(self.color.get()),
        }
    }
}

impl<'a> HasColor<'a, Color> for RectangleColorContext<'a> {
    #[inline(always)]
    fn get_color(&'a self) -> &'a Color {
        self.color.get()
    }
}

impl<'a> CanColor<'a, RectangleColorContext<'a>, Color> for RectangleColorContext<'a> {
    #[inline(always)]
    fn color(&'a self, value: Color) -> RectangleColorContext<'a> {
        RectangleColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            color: Value(value),
            rect: Borrowed(self.rect.get()),
        }
    }
}

impl<'a> Fill<'a> for RectangleColorContext<'a> {
    fn fill<B: BackEnd>(&'a self, back_end: &mut B) {
        if back_end.supports_tri_list_xy_f32_rgba_f32() {
            let rect = self.rect.get();
            let color = self.color.get();
            let color: [f32, ..4] = [color[0], color[1], color[2], color[3]];
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely opaque.
            let needs_alpha = color[3] != 1.0;
            if needs_alpha { back_end.enable_alpha_blend(); }
            back_end.tri_list_xy_f32_rgba_f32(
                rect_tri_list_xy_f32(self.transform.get(), rect),
                rect_tri_list_rgba_f32(color)
            );
            if needs_alpha { back_end.disable_alpha_blend(); }
        } else {
            unimplemented!();
        }
    }
}

impl<'a> Clear for RectangleColorContext<'a> {
    fn clear<B: BackEnd>(&self, back_end: &mut B) {
        if back_end.supports_clear_rgba() {
            let color = self.color.get();
            back_end.clear_rgba(color[0], color[1], color[2], color[3]);
        } else {
            unimplemented!();
        }
    }
}

impl<'a> RelativeRectangle<'a> for RectangleColorContext<'a> {
    #[inline(always)]
    fn margin(&'a self, m: f64) -> RectangleColorContext<'a> {
        RectangleColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            color: Borrowed(self.color.get()),
            rect: Value(margin_rectangle(self.rect.get(), m)),
        }
    }

    #[inline(always)]
    fn rel(&'a self, x: f64, y: f64) -> RectangleColorContext<'a> {
        RectangleColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            color: Borrowed(self.color.get()),
            rect: Value(relative_rectangle(self.rect.get(), x, y)),
        }
    }
}

impl<'a> AddRound<'a, RoundRectangleColorContext<'a>> for RectangleColorContext<'a> {
    #[inline(always)]
    fn round(&'a self, radius: f64) -> RoundRectangleColorContext<'a> {
        RoundRectangleColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            color: Borrowed(self.color.get()),
            round_rect: {
                let rect = self.rect.get();
                Value([rect[0], rect[1], rect[2], rect[3], radius])
            },
        }
    }
}

impl<'a> View<'a> for RectangleColorContext<'a> {
    #[inline(always)]
    fn view(&'a self) -> RectangleColorContext<'a> {
        RectangleColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.base.get()),
            rect: Borrowed(self.rect.get()),
            color: Borrowed(self.color.get()),
        }
    }

    #[inline(always)]
    fn reset(&'a self) -> RectangleColorContext<'a> {
        RectangleColorContext {
            base: Borrowed(self.base.get()),
            transform: Value(identity()),
            rect: Borrowed(self.rect.get()),
            color: Borrowed(self.color.get()),
        }
    }

    #[inline(always)]
    fn store_view(&'a self) -> RectangleColorContext<'a> {
        RectangleColorContext {
            base: Borrowed(self.transform.get()),
            transform: Borrowed(self.transform.get()),
            rect: Borrowed(self.rect.get()),
            color: Borrowed(self.color.get()),
        }
    }
}
