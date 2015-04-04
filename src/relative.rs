use internal::{ Color, ColorComponent };
use math::{
    hsv,
    margin_rectangle,
    relative_rectangle,
    relative_source_rectangle,
    Scalar,
};
use radians::Radians;

/// Implemented by contexts that contains color.
pub trait Colored: Sized {
    /// Multiplies with red, green, blue and alpha values.
    fn mul_rgba(
        &self,
        r: ColorComponent,
        g: ColorComponent,
        b: ColorComponent,
        a: ColorComponent
    ) -> Self;

    /// Mixes the current color with white.
    ///
    /// 0 is black and 1 is white.
    #[inline(always)]
    fn tint(&self, f: ColorComponent) -> Self {
        self.mul_rgba(f, f, f, 1.0)
    }

    /// Mixes the current color with black.
    ///
    /// 0 is white and 1 is black.
    #[inline(always)]
    fn shade(&self, f: ColorComponent) -> Self {
        let f = 1.0 - f;
        self.mul_rgba(f, f, f, 1.0)
    }

    /// Rotates hue by degrees.
    #[inline(always)]
    fn hue_deg(&self, angle: ColorComponent) -> Self {
        let pi: ColorComponent = Radians::_180();
        self.hue_rad(angle * pi / 180.0)
    }

    /// Rotates hue by radians.
    fn hue_rad(&self, angle: ColorComponent) -> Self;
}

impl Colored for Color {
    #[inline(always)]
    fn mul_rgba(
        &self,
        r: ColorComponent,
        g: ColorComponent,
        b: ColorComponent,
        a: ColorComponent
    ) -> Self {
        [self[0] * r, self[1] * g, self[2] * b, self[3] * a]
    }

    #[inline(always)]
    fn hue_rad(&self, angle: ColorComponent) -> Self {
        hsv(*self, angle, 1.0, 1.0)
    }
}

/// Should be implemented by contexts that have rectangle information.
pub trait Rectangled: Sized {
    /// Shrinks the current rectangle equally by all sides.
    fn margin(&self, m: Scalar) -> Self;

    /// Expands the current rectangle equally by all sides.
    #[inline(always)]
    fn expand(&self, m: Scalar) -> Self {
        self.margin(-m)
    }

    /// Moves to a relative rectangle using the current rectangle as tile.
    fn rel(&self, x: Scalar, y: Scalar) -> Self;
}

/*
impl<T: Get<Rect> + Set<Rect> + Clone> Rectangled for T {
    #[inline(always)]
    fn margin(&self, m: Scalar) -> Self {
        let Rect(val) = self.get();
        self.clone().set(Rect(margin_rectangle(val, m)))
    }

    #[inline(always)]
    fn rel(&self, x: Scalar, y: Scalar) -> Self {
        let Rect(val) = self.get();
        self.clone().set(Rect(relative_rectangle(val, [x, y])))
    }
}
*/

/// Should be implemented by contexts that
/// have source rectangle information.
pub trait SourceRectangled {
    /// Adds a source rectangle.
    fn src_rect(&self, x: i32, y: i32, w: i32, h: i32) -> Self;

    /// Moves to a relative source rectangle using
    /// the current source rectangle as tile.
    fn src_rel(&self, x: i32, y: i32) -> Self;

    /// Flips the source rectangle horizontally.
    fn src_flip_h(&self) -> Self;

    /// Flips the source rectangle vertically.
    fn src_flip_v(&self) -> Self;

    /// Flips the source rectangle horizontally and vertically.
    fn src_flip_hv(&self) -> Self;
}

/*
impl<T: Get<SrcRect>
      + Set<SrcRect>
      + Clone
> SourceRectangled for T {
    #[inline(always)]
    fn src_rect(&self, x: i32, y: i32, w: i32, h: i32) -> Self {
        self.clone().set(SrcRect([x, y, w, h]))
    }

    #[inline(always)]
    fn src_rel(&self, x: i32, y: i32) -> Self {
        let SrcRect(val) = self.get();
        self.clone().set(SrcRect(
            relative_source_rectangle(val, x, y)
        ))
    }

    #[inline(always)]
    fn src_flip_h(&self) -> Self {
        let SrcRect(source_rect) = self.get();
        self.clone().set(SrcRect([
            source_rect[0] + source_rect[2],
            source_rect[1],
            -source_rect[2],
            source_rect[3]
        ]))
    }

    #[inline(always)]
    fn src_flip_v(&self) -> Self {
        let SrcRect(source_rect) = self.get();
        self.clone().set(SrcRect([
            source_rect[0],
            source_rect[1] + source_rect[3],
            source_rect[2],
            -source_rect[3]
        ]))
    }

    #[inline(always)]
    fn src_flip_hv(&self) -> Self {
        let SrcRect(source_rect) = self.get();
        self.clone().set(SrcRect([
            source_rect[0] + source_rect[2],
            source_rect[1] + source_rect[3],
            -source_rect[2],
            -source_rect[3]
        ]))
    }
}
*/
