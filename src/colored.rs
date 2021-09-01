use crate::{
    math::hsv,
    radians::Radians,
    types::{Color, ColorComponent},
    Ellipse, Line, Rectangle,
};

/// Implemented by contexts that contains color.
pub trait Colored: Sized {
    /// Multiplies with red, green, blue and alpha values.
    fn mul_rgba(
        self,
        r: ColorComponent,
        g: ColorComponent,
        b: ColorComponent,
        a: ColorComponent,
    ) -> Self;

    /// Mixes the current color with white.
    ///
    /// 0 is black and 1 is white.
    #[inline(always)]
    fn tint(self, f: ColorComponent) -> Self {
        self.mul_rgba(f, f, f, 1.0)
    }

    /// Mixes the current color with black.
    ///
    /// 0 is white and 1 is black.
    #[inline(always)]
    fn shade(self, f: ColorComponent) -> Self {
        let f = 1.0 - f;
        self.mul_rgba(f, f, f, 1.0)
    }

    /// Rotates hue by degrees.
    #[inline(always)]
    fn hue_deg(self, angle: ColorComponent) -> Self {
        let pi: ColorComponent = Radians::_180();
        self.hue_rad(angle * pi / 180.0)
    }

    /// Rotates hue by radians.
    fn hue_rad(self, angle: ColorComponent) -> Self;
}

impl Colored for Color {
    #[inline(always)]
    fn mul_rgba(
        self,
        r: ColorComponent,
        g: ColorComponent,
        b: ColorComponent,
        a: ColorComponent,
    ) -> Self {
        [self[0] * r, self[1] * g, self[2] * b, self[3] * a]
    }

    #[inline(always)]
    fn hue_rad(self, angle: ColorComponent) -> Self {
        hsv(self, angle, 1.0, 1.0)
    }
}

impl Colored for Line {
    #[inline(always)]
    fn mul_rgba(
        mut self,
        r: ColorComponent,
        g: ColorComponent,
        b: ColorComponent,
        a: ColorComponent,
    ) -> Self {
        self.color = self.color.mul_rgba(r, g, b, a);
        self
    }

    #[inline(always)]
    fn hue_rad(mut self, angle: ColorComponent) -> Self {
        self.color = self.color.hue_rad(angle);
        self
    }
}

impl Colored for Ellipse {
    #[inline(always)]
    fn mul_rgba(
        mut self,
        r: ColorComponent,
        g: ColorComponent,
        b: ColorComponent,
        a: ColorComponent,
    ) -> Self {
        self.color = self.color.mul_rgba(r, g, b, a);
        self
    }

    #[inline(always)]
    fn hue_rad(mut self, angle: ColorComponent) -> Self {
        self.color = self.color.hue_rad(angle);
        self
    }
}

impl Colored for Rectangle {
    #[inline(always)]
    fn mul_rgba(
        mut self,
        r: ColorComponent,
        g: ColorComponent,
        b: ColorComponent,
        a: ColorComponent,
    ) -> Self {
        self.color = self.color.mul_rgba(r, g, b, a);
        self
    }

    #[inline(always)]
    fn hue_rad(mut self, angle: ColorComponent) -> Self {
        self.color = self.color.hue_rad(angle);
        self
    }
}
