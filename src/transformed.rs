use crate::{
    math::{multiply, orient, rotate_radians, scale, shear, translate, Matrix2d, Scalar, Vec2d},
    radians::Radians,
    Context,
};

/// Implemented by contexts that can transform.
pub trait Transformed: Sized {
    /// Appends transform to the current one.
    fn append_transform(self, transform: Matrix2d) -> Self;

    /// Prepends transform to the current one.
    fn prepend_transform(self, transform: Matrix2d) -> Self;

    /// Translate x and y in local coordinates.
    fn trans(self, x: Scalar, y: Scalar) -> Self;

    /// Rotates degrees in local coordinates.
    #[inline(always)]
    fn rot_deg(self, angle: Scalar) -> Self {
        let pi: Scalar = Radians::_180();
        self.rot_rad(angle * pi / 180.0)
    }

    /// Rotate radians in local coordinates.
    fn rot_rad(self, angle: Scalar) -> Self;

    /// Orients x axis to look at point locally.
    ///
    /// Leaves x axis unchanged if the point to
    /// look at is the origin.
    fn orient(self, x: Scalar, y: Scalar) -> Self;

    /// Scales in local coordinates.
    fn scale(self, sx: Scalar, sy: Scalar) -> Self;

    /// Translate position in local coordinates.
    #[inline(always)]
    fn trans_pos<P: Into<Vec2d>>(self, pos: P) -> Self {
        let pos = pos.into();
        self.trans(pos[0], pos[1])
    }

    /// Orients x axis to look at point locally.
    #[inline(always)]
    fn orient_pos<P: Into<Vec2d>>(self, pos: P) -> Self {
        let pos = pos.into();
        self.orient(pos[0], pos[1])
    }

    /// Scales in local coordinates.
    fn scale_pos<P: Into<Vec2d>>(self, pos: P) -> Self {
        let pos = pos.into();
        self.scale(pos[0], pos[1])
    }

    /// Scales in both directions in local coordinates.
    #[inline(always)]
    fn zoom(self, s: Scalar) -> Self {
        self.scale(s, s)
    }

    /// Flips vertically in local coordinates.
    #[inline(always)]
    fn flip_v(self) -> Self {
        self.scale(1.0, -1.0)
    }

    /// Flips horizontally in local coordinates.
    #[inline(always)]
    fn flip_h(self) -> Self {
        self.scale(-1.0, 1.0)
    }

    /// Flips horizontally and vertically in local coordinates.
    #[inline(always)]
    fn flip_hv(self) -> Self {
        self.scale(-1.0, -1.0)
    }

    /// Shears in local coordinates.
    fn shear(self, x: Scalar, y: Scalar) -> Self;

    /// Shears in local coordinates.
    #[inline(always)]
    fn shear_pos<P: Into<Vec2d>>(self, pos: P) -> Self {
        let pos = pos.into();
        self.shear(pos[0], pos[1])
    }
}

impl Transformed for Matrix2d {
    #[inline(always)]
    fn append_transform(self, transform: Matrix2d) -> Self {
        multiply(self, transform)
    }

    #[inline(always)]
    fn prepend_transform(self, transform: Matrix2d) -> Self {
        multiply(transform, self)
    }

    #[inline(always)]
    fn trans(self, x: Scalar, y: Scalar) -> Self {
        let trans = translate([x, y]);
        multiply(self, trans)
    }

    #[inline(always)]
    fn rot_rad(self, angle: Scalar) -> Self {
        let rot = rotate_radians(angle);
        multiply(self, rot)
    }

    #[inline(always)]
    fn orient(self, x: Scalar, y: Scalar) -> Self {
        let orient = orient(x, y);
        multiply(self, orient)
    }

    #[inline(always)]
    fn scale(self, sx: Scalar, sy: Scalar) -> Self {
        let scale = scale(sx, sy);
        multiply(self, scale)
    }

    #[inline(always)]
    fn shear(self, x: Scalar, y: Scalar) -> Self {
        let shear = shear([x, y]);
        multiply(self, shear)
    }
}

impl Transformed for Context {
    #[inline(always)]
    fn append_transform(mut self, transform: Matrix2d) -> Self {
        self.transform = self.transform.append_transform(transform);
        self
    }

    #[inline(always)]
    fn prepend_transform(mut self, transform: Matrix2d) -> Self {
        self.transform = self.transform.prepend_transform(transform);
        self
    }

    #[inline(always)]
    fn trans(mut self, x: Scalar, y: Scalar) -> Self {
        self.transform = self.transform.trans(x, y);
        self
    }

    #[inline(always)]
    fn rot_rad(mut self, angle: Scalar) -> Self {
        self.transform = self.transform.rot_rad(angle);
        self
    }

    #[inline(always)]
    fn orient(mut self, x: Scalar, y: Scalar) -> Self {
        self.transform = self.transform.orient(x, y);
        self
    }

    #[inline(always)]
    fn scale(mut self, sx: Scalar, sy: Scalar) -> Self {
        self.transform = self.transform.scale(sx, sy);
        self
    }

    #[inline(always)]
    fn shear(mut self, x: Scalar, y: Scalar) -> Self {
        self.transform = self.transform.shear(x, y);
        self
    }
}
