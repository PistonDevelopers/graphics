
use vecmath::{
    multiply,
    orient,
    rotate_radians,
    scale,
    shear,
    translate,
};
use internal::{
    CanTransform,
    HasTransform,
    Matrix2d,
    Scalar,
};

/// Implemented by contexts that can transform.
pub trait RelativeTransform2d<'a> {
    /// Translate x and y.
    fn trans(&'a self, x: Scalar, y: Scalar) -> Self;

    /// Translate x an y in local coordinates.
    fn trans_local(&'a self, x: Scalar, y: Scalar) -> Self;

    /// Rotates degrees.
    #[inline(always)]
    fn rot_deg(&'a self, angle: Scalar) -> Self {
        let pi: Scalar = Float::pi();
        self.rot_rad(angle * pi / 180.0)
    }

    /// Rotates degrees in local coordinates.
    #[inline(always)]
    fn rot_deg_local(&'a self, angle: Scalar) -> Self {
        let pi: Scalar = Float::pi();
        self.rot_rad_local(angle * pi / 180.0)
    }

    /// Rotate radians.
    fn rot_rad(&'a self, angle: Scalar) -> Self;

    /// Rotate radians in local coordinates.
    fn rot_rad_local(&'a self, angle: Scalar) -> Self;

    /// Orients x axis to look at point.
    ///
    /// Leaves x axis unchanged if the point to look at is the origin.
    fn orient(&'a self, x: Scalar, y: Scalar) -> Self;

    /// Orients x axis to look at point locally.
    ///
    /// Leaves x axis unchanged if the point to look at is the origin.
    fn orient_local(&'a self, x: Scalar, y: Scalar) -> Self;

    /// Scale.
    fn scale(&'a self, sx: Scalar, sy: Scalar) -> Self;

    /// Scales in local coordinates.
    fn scale_local(&'a self, sx: Scalar, sy: Scalar) -> Self;

    /// Scales in both directions.
    #[inline(always)]
    fn zoom(&'a self, s: Scalar) -> Self {
        self.scale(s, s)
    }

    /// Scales in both directions in local coordinates.
    #[inline(always)]
    fn zoom_local(&'a self, s: Scalar) -> Self {
        self.scale_local(s, s)
    }

    /// Flips vertically.
    #[inline(always)]
    fn flip_v(&'a self) -> Self {
        self.scale(1.0, -1.0)
    }

    /// Flips vertically in local coordinates.
    #[inline(always)]
    fn flip_v_local(&'a self) -> Self {
        self.scale_local(1.0, -1.0)
    }

    /// Flips horizontally.
    #[inline(always)]
    fn flip_h(&'a self) -> Self {
        self.scale(-1.0, 0.0)
    }

    /// Flips horizontally in local coordinates.
    #[inline(always)]
    fn flip_h_local(&'a self) -> Self {
        self.scale_local(-1.0, 0.0)
    }

    /// Shear.
    fn shear(&'a self, sx: Scalar, sy: Scalar) -> Self;

    /// Shears in local coordinates.
    fn shear_local(&'a self, sx: Scalar, sy: Scalar) -> Self;
}

impl<
    'a,
    T: HasTransform<'a, Matrix2d> + CanTransform<'a, T, Matrix2d>
> RelativeTransform2d<'a> for T {
    #[inline(always)]
    fn trans(&'a self, x: Scalar, y: Scalar) -> T {
        let trans = translate(x, y);
        self.transform(multiply(trans, *self.get_transform()))
    }

    #[inline(always)]
    fn trans_local(&'a self, x: Scalar, y: Scalar) -> T {
        let trans = translate(x, y);
        self.transform(multiply(*self.get_transform(), trans))
    }

    #[inline(always)]
    fn rot_rad(&'a self, angle: Scalar) -> T {
        let rot = rotate_radians(angle);
        self.transform(multiply(rot, *self.get_transform()))
    }

    #[inline(always)]
    fn rot_rad_local(&'a self, angle: Scalar) -> T {
        let rot = rotate_radians(angle);
        self.transform(multiply(*self.get_transform(), rot))
    }

    #[inline(always)]
    fn orient(&'a self, x: Scalar, y: Scalar) -> T {
        let orient = orient(x, y);
        self.transform(multiply(orient, *self.get_transform()))
    }

    #[inline(always)]
    fn orient_local(&'a self, x: Scalar, y: Scalar) -> T {
        let orient = orient(x, y);
        self.transform(multiply(*self.get_transform(), orient))
    }

    #[inline(always)]
    fn scale(&'a self, sx: Scalar, sy: Scalar) -> T {
        let scale = scale(sx, sy);
        self.transform(multiply(scale, *self.get_transform()))
    }

    #[inline(always)]
    fn scale_local(&'a self, sx: Scalar, sy: Scalar) -> T {
        let scale = scale(sx, sy);
        self.transform(multiply(*self.get_transform(), scale))
    }

    #[inline(always)]
    fn shear(&'a self, sx: Scalar, sy: Scalar) -> T {
        let shear = shear(sx, sy);
        self.transform(multiply(shear, *self.get_transform()))
    }

    #[inline(always)]
    fn shear_local(&'a self, sx: Scalar, sy: Scalar) -> T {
        let shear = shear(sx, sy);
        self.transform(multiply(*self.get_transform(), shear))
    }
}

