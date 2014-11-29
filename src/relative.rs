use internal::ColorComponent;
use can::{
    CanColor,
    CanRectangle,
    CanSourceRectangle,
    CanTransform,
};
use has::{
    HasColor,
    HasRectangle,
    HasSourceRectangle,
    HasTransform,
};
use vecmath::{
    hsv,
    margin_rectangle,
    multiply,
    orient,
    relative_rectangle,
    relative_source_rectangle,
    rotate_radians,
    scale,
    shear,
    translate,
    Matrix2d,
    Scalar,
    Vec2d,
};
use radians::Radians;

/// Implemented by contexts that contains color.
pub trait RelativeColor: HasColor + CanColor {
    /// Multiplies with red, green, blue and alpha values.
    #[inline(always)]
    fn mul_rgba(
        &self,
        r: ColorComponent,
        g: ColorComponent,
        b: ColorComponent,
        a: ColorComponent
    ) -> Self {
        let color = self.get_color();
        self.color([color[0] * r, color[1] * g, color[2] * b, color[3] * a])
    }

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
    #[inline(always)]
    fn hue_rad(&self, angle: ColorComponent) -> Self {
        self.color(hsv(self.get_color(), angle, 1.0, 1.0))
    }
}

impl<T: HasColor + CanColor> RelativeColor for T {}

/// Should be implemented by contexts that have rectangle information.
pub trait RelativeRectangle: HasRectangle + CanRectangle {
    /// Shrinks the current rectangle equally by all sides.
    #[inline(always)]
    fn margin(&self, m: Scalar) -> Self {
        self.rectangle(margin_rectangle(self.get_rectangle(), m))
    }

    /// Expands the current rectangle equally by all sides.
    #[inline(always)]
    fn expand(&self, m: Scalar) -> Self {
        self.margin(-m)
    }

    /// Moves to a relative rectangle using the current rectangle as tile.
    #[inline(always)]
    fn rel(&self, x: Scalar, y: Scalar) -> Self {
        self.rectangle(relative_rectangle(self.get_rectangle(), [x, y]))
    }
}

impl<T: HasRectangle + CanRectangle> RelativeRectangle for T {}

/// Should be implemented by contexts that
/// have source rectangle information.
pub trait RelativeSourceRectangle: HasSourceRectangle + CanSourceRectangle {
    /// Adds a source rectangle.
    #[inline(always)]
    fn src_rect(&self, x: i32, y: i32, w: i32, h: i32) -> Self {
        self.source_rectangle([x, y, w, h])
    }

    /// Moves to a relative source rectangle using
    /// the current source rectangle as tile.
    #[inline(always)]
    fn src_rel(&self, x: i32, y: i32) -> Self {
        self.source_rectangle(
            relative_source_rectangle(self.get_source_rectangle(), x, y)
        )
    }

    /// Flips the source rectangle horizontally.
    #[inline(always)]
    fn src_flip_h(&self) -> Self {
        let source_rect = self.get_source_rectangle();
        self.source_rectangle([
            source_rect[0] + source_rect[2],
            source_rect[1],
            -source_rect[2],
            source_rect[3]
        ])
    }

    /// Flips the source rectangle vertically.
    #[inline(always)]
    fn src_flip_v(&self) -> Self {
        let source_rect = self.get_source_rectangle();
        self.source_rectangle([
            source_rect[0],
            source_rect[1] + source_rect[3],
            source_rect[2],
            -source_rect[3]
        ])
    }

    /// Flips the source rectangle horizontally and vertically.
    #[inline(always)]
    fn src_flip_hv(&self) -> Self {
        let source_rect = self.get_source_rectangle();
        self.source_rectangle([
            source_rect[0] + source_rect[2],
            source_rect[1] + source_rect[3],
            -source_rect[2],
            -source_rect[3]
        ])
    }
}

impl<T: HasSourceRectangle
      + CanSourceRectangle,
> RelativeSourceRectangle for T {}

/// Implemented by contexts that can transform.
pub trait RelativeTransform: HasTransform + CanTransform {
    /// Appends transform to the current one.
    #[inline(always)]
    fn append_transform(&self, transform: Matrix2d) -> Self {
        self.transform(multiply(self.get_transform(), transform))
    }

    /// Prepends transform to the current one.
    #[inline(always)]
    fn prepend_transform(&self, transform: Matrix2d) -> Self {
        self.transform(multiply(transform, self.get_transform()))
    }

    /// Translate x an y in local coordinates.
    #[inline(always)]
    fn trans(&self, x: Scalar, y: Scalar) -> Self {
        let trans = translate([x, y]);
        self.transform(multiply(self.get_transform(), trans))
    }

    /// Rotates degrees in local coordinates.
    #[inline(always)]
    fn rot_deg(&self, angle: Scalar) -> Self {
        let pi: Scalar = Radians::_180();
        self.rot_rad(angle * pi / 180.0)
    }

    /// Rotate radians in local coordinates.
    #[inline(always)]
    fn rot_rad(&self, angle: Scalar) -> Self {
        let rot = rotate_radians(angle);
        self.transform(multiply(self.get_transform(), rot))
    }

    /// Orients x axis to look at point locally.
    ///
    /// Leaves x axis unchanged if the point to
    /// look at is the origin.
    #[inline(always)]
    fn orient(&self, x: Scalar, y: Scalar) -> Self {
        let orient = orient(x, y);
        self.transform(multiply(self.get_transform(), orient))
    }

    /// Scales in local coordinates.
    #[inline(always)]
    fn scale(&self, sx: Scalar, sy: Scalar) -> Self {
        let scale = scale(sx, sy);
        self.transform(multiply(self.get_transform(), scale))
    }

    /// Scales in both directions in local coordinates.
    #[inline(always)]
    fn zoom(&self, s: Scalar) -> Self {
        self.scale(s, s)
    }

    /// Flips vertically in local coordinates.
    #[inline(always)]
    fn flip_v(&self) -> Self {
        self.scale(1.0, -1.0)
    }

    /// Flips horizontally in local coordinates.
    #[inline(always)]
    fn flip_h(&self) -> Self {
        self.scale(-1.0, 1.0)
    }

    /// Flips horizontally and vertically in local coordinates.
    #[inline(always)]
    fn flip_hv(&self) -> Self {
        self.scale(-1.0, -1.0)
    }

    /// Shears in local coordinates.
    #[inline(always)]
    fn shear(&self, v: Vec2d) -> Self {
        let shear = shear(v);
        self.transform(multiply(self.get_transform(), shear))
    }
}

impl<T: HasTransform + CanTransform> RelativeTransform for T {}

