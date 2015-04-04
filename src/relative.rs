use internal::ColorComponent;
use quack::{ Get, Set };
use context::Context;
use vecmath::{
    get_scale,
    hsv,
    identity,
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
use Color;
use Rect;
use SrcRect;

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

impl<T: Get<Color> + Set<Color> + Clone> Colored for T {
    #[inline(always)]
    fn mul_rgba(
        &self,
        r: ColorComponent,
        g: ColorComponent,
        b: ColorComponent,
        a: ColorComponent
    ) -> Self {
        let Color(col) = self.get();
        self.clone().set(Color([col[0] * r, col[1] * g, col[2] * b, col[3] * a]))
    }

    #[inline(always)]
    fn hue_rad(&self, angle: ColorComponent) -> Self {
        let Color(val) = self.get();
        self.clone().set(Color(hsv(val, angle, 1.0, 1.0)))
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

/// Implemented by contexts that can transform.
pub trait Transformed: Sized {
    /// Appends transform to the current one.
    fn append_transform(self, transform: Matrix2d) -> Self;

    /// Prepends transform to the current one.
    fn prepend_transform(self, transform: Matrix2d) -> Self;

    /// Translate x an y in local coordinates.
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
    fn shear(self, v: Vec2d) -> Self;
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
    fn shear(self, v: Vec2d) -> Self {
        let shear = shear(v);
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
    fn shear(mut self, v: Vec2d) -> Self {
        self.transform = self.transform.shear(v);
        self
    }
}

/// Should be implemented by contexts that
/// draws something relative to view.
pub trait ViewTransformed {
    /// Moves the current transform to the view coordinate system.
    ///
    /// This is usually [0.0, 0.0] in the upper left corner
    /// with the x axis pointing to the right
    /// and the y axis pointing down.
    #[inline(always)]
    fn view(self) -> Self;

    /// Moves the current transform to the default coordinate system.
    ///
    /// This is usually [0.0, 0.0] in the center
    /// with the x axis pointing to the right
    /// and the y axis pointing up.
    fn reset(self) -> Self;

    /// Stores the current transform as new view.
    fn store_view(self) -> Self;

    /// Computes the current view size.
    fn get_view_size(&self) -> (Scalar, Scalar);
}

impl ViewTransformed for Context {
    #[inline(always)]
    fn view(mut self) -> Self {
        self.transform = self.view;
        self
    }

    #[inline(always)]
    fn reset(mut self) -> Self {
        self.transform = identity();
        self
    }

    #[inline(always)]
    fn store_view(mut self) -> Self {
        self.view = self.transform;
        self
    }

    #[inline(always)]
    fn get_view_size(&self) -> (Scalar, Scalar) {
        let scale = get_scale(self.view);
        (2.0 / scale[0], 2.0 / scale[1])
    }
}
