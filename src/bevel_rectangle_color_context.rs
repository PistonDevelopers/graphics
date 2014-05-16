use {
    BackEnd,
    Borrowed,
    Clear,
    Color,
    Field,
    Fill,
    Matrix2d,
    Rectangle,
    Value,
    View,
};
use vecmath::{
    identity,
};
use triangulation::{
    with_round_rectangle_tri_list_xy_f32_rgba_f32
};
use internal::{
    CanColor,
    CanRectangle,
    CanTransform,
    HasColor,
    HasRectangle,
    HasTransform,
};

/// A rectangle color context.
pub struct BevelRectangleColorContext<'a> {
    /// Base/original transformation.
    pub base: Field<'a, Matrix2d>,
    /// Current transformation.
    pub transform: Field<'a, Matrix2d>,
    /// Current rectangle.
    pub rect: Field<'a, Rectangle>,
    /// Current bevel radius.
    pub bevel_radius: Field<'a, f64>,
    /// Current color.
    pub color: Field<'a, Color>,
}

impl<'a> Clone for BevelRectangleColorContext<'a> {
    #[inline(always)]
    fn clone(&self) -> BevelRectangleColorContext<'static> {
        BevelRectangleColorContext {
            base: self.base.clone(),
            transform: self.transform.clone(),
            rect: self.rect.clone(),
            bevel_radius: self.bevel_radius.clone(),
            color: self.color.clone(),
        }
    }
}

impl<'a> HasTransform<'a, Matrix2d> for BevelRectangleColorContext<'a> {
    #[inline(always)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a> CanTransform<'a, BevelRectangleColorContext<'a>, Matrix2d> for BevelRectangleColorContext<'a> {
    #[inline(always)]
    fn transform(&'a self, value: Matrix2d) -> BevelRectangleColorContext<'a> {
        BevelRectangleColorContext {
            base: Borrowed(self.base.get()),
            transform: Value(value),
            rect: Borrowed(self.rect.get()),
            bevel_radius: Borrowed(self.bevel_radius.get()),
            color: Borrowed(self.color.get()),
        }
    }
}

impl<'a> HasColor<'a, Color> for BevelRectangleColorContext<'a> {
    #[inline(always)]
    fn get_color(&'a self) -> &'a Color {
        self.color.get()
    }
}

impl<'a> CanColor<'a, BevelRectangleColorContext<'a>, Color> for BevelRectangleColorContext<'a> {
    #[inline(always)]
    fn color(&'a self, value: Color) -> BevelRectangleColorContext<'a> {
        BevelRectangleColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            color: Value(value),
            rect: Borrowed(self.rect.get()),
            bevel_radius: Borrowed(self.bevel_radius.get()),
        }
    }
}

impl<'a> HasRectangle<'a, Rectangle> for BevelRectangleColorContext<'a> {
    #[inline(always)]
    fn get_rectangle(&'a self) -> &'a Rectangle {
        self.rect.get()
    }
}

impl<'a> CanRectangle<'a, BevelRectangleColorContext<'a>, Rectangle> for BevelRectangleColorContext<'a> {
    #[inline(always)]
    fn rectangle(&'a self, rect: Rectangle) -> BevelRectangleColorContext<'a> {
        BevelRectangleColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            rect: Value(rect),
            bevel_radius: Borrowed(self.bevel_radius.get()),
            color: Borrowed(self.color.get()),
        }
    }
}

impl<'a> Clear for BevelRectangleColorContext<'a> {
    #[inline(always)]
    fn clear<B: BackEnd>(&self, back_end: &mut B) {
        if back_end.supports_clear_rgba() {
            let &Color(color) = self.color.get();
            back_end.clear_rgba(color[0], color[1], color[2], color[3]);
        } else {
            unimplemented!();
        }
    }
}

impl<'a> Fill<'a> for BevelRectangleColorContext<'a> {
    #[inline(always)]
    fn fill<B: BackEnd>(&'a self, back_end: &mut B) {
        if back_end.supports_tri_list_xy_f32_rgba_f32() {
            let rect = self.rect.get();
            let bevel_radius = self.bevel_radius.get();
            let &Color(color) = self.color.get();
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely opaque.
            let needs_alpha = color[3] != 1.0;
            if needs_alpha { back_end.enable_alpha_blend(); }
            with_round_rectangle_tri_list_xy_f32_rgba_f32(
                2,
                self.transform.get(),
                rect,
                bevel_radius,
                &Color(color),
                |vertices, colors| {
                    back_end.tri_list_xy_f32_rgba_f32(vertices, colors)
                }
            );
            if needs_alpha { back_end.disable_alpha_blend(); }
        } else {
            unimplemented!();
        }
    }
}

impl<'a> View<'a> for BevelRectangleColorContext<'a> {
    #[inline(always)]
    fn view(&'a self) -> BevelRectangleColorContext<'a> {
        BevelRectangleColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.base.get()),
            rect: Borrowed(self.rect.get()),
            bevel_radius: Borrowed(self.bevel_radius.get()),
            color: Borrowed(self.color.get()),
        }
    }

    #[inline(always)]
    fn reset(&'a self) -> BevelRectangleColorContext<'a> {
        BevelRectangleColorContext {
            base: Borrowed(self.base.get()),
            transform: Value(identity()),
            rect: Borrowed(self.rect.get()),
            bevel_radius: Borrowed(self.bevel_radius.get()),
            color: Borrowed(self.color.get()),
        }
    }

    #[inline(always)]
    fn store_view(&'a self) -> BevelRectangleColorContext<'a> {
        BevelRectangleColorContext {
            base: Borrowed(self.transform.get()),
            transform: Borrowed(self.transform.get()),
            rect: Borrowed(self.rect.get()),
            bevel_radius: Borrowed(self.bevel_radius.get()),
            color: Borrowed(self.color.get()),
        }
    }
}

