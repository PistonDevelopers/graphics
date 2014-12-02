use internal::ColorComponent;
use current::{ Get, Set };
use context::{ Transform, GetTransform, SetTransform };
use context::{ ViewTransform, GetViewTransform, SetViewTransform };
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
pub trait RelativeColor: Get<Color> + Set<Color> + Clone {
    /// Multiplies with red, green, blue and alpha values.
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
        let Color(val) = self.get();
        self.clone().set(Color(hsv(val, angle, 1.0, 1.0)))
    }
}

impl<T: Get<Color> + Set<Color> + Clone> RelativeColor for T {}

/// Should be implemented by contexts that have rectangle information.
pub trait RelativeRectangle: Get<Rect> + Set<Rect> + Clone {
    /// Shrinks the current rectangle equally by all sides.
    #[inline(always)]
    fn margin(&self, m: Scalar) -> Self {
        let Rect(val) = self.get();
        self.clone().set(Rect(margin_rectangle(val, m)))
    }

    /// Expands the current rectangle equally by all sides.
    #[inline(always)]
    fn expand(&self, m: Scalar) -> Self {
        self.margin(-m)
    }

    /// Moves to a relative rectangle using the current rectangle as tile.
    #[inline(always)]
    fn rel(&self, x: Scalar, y: Scalar) -> Self {
        let Rect(val) = self.get();
        self.clone().set(Rect(relative_rectangle(val, [x, y])))
    }
}

impl<T: Get<Rect> + Set<Rect> + Clone> RelativeRectangle for T {}

/// Should be implemented by contexts that
/// have source rectangle information.
pub trait RelativeSourceRectangle: Get<SrcRect> + Set<SrcRect> + Clone {
    /// Adds a source rectangle.
    #[inline(always)]
    fn src_rect(&self, x: i32, y: i32, w: i32, h: i32) -> Self {
        self.clone().set(SrcRect([x, y, w, h]))
    }

    /// Moves to a relative source rectangle using
    /// the current source rectangle as tile.
    #[inline(always)]
    fn src_rel(&self, x: i32, y: i32) -> Self {
        let SrcRect(val) = self.get();
        self.clone().set(SrcRect(
            relative_source_rectangle(val, x, y)
        ))
    }

    /// Flips the source rectangle horizontally.
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

    /// Flips the source rectangle vertically.
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

    /// Flips the source rectangle horizontally and vertically.
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

impl<T: Get<SrcRect>
      + Set<SrcRect>
      + Clone
> RelativeSourceRectangle for T {}

/// Implemented by contexts that can transform.
pub trait RelativeTransform: Get<Transform> + Set<Transform> + Clone {
    /// Appends transform to the current one.
    #[inline(always)]
    fn append_transform(&self, transform: Matrix2d) -> Self {
        let Transform(mat) = self.get_transform();
        self.clone().set(Transform(multiply(mat, transform)))
    }

    /// Prepends transform to the current one.
    #[inline(always)]
    fn prepend_transform(&self, transform: Matrix2d) -> Self {
        let Transform(mat) = self.get_transform();
        self.clone().set(Transform(multiply(transform, mat)))
    }

    /// Translate x an y in local coordinates.
    #[inline(always)]
    fn trans(&self, x: Scalar, y: Scalar) -> Self {
        let trans = translate([x, y]);
        let Transform(mat) = self.get_transform();
        self.clone().set(Transform(multiply(mat, trans)))
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
        let Transform(mat) = self.get_transform();
        self.clone().set(Transform(multiply(mat, rot)))
    }

    /// Orients x axis to look at point locally.
    ///
    /// Leaves x axis unchanged if the point to
    /// look at is the origin.
    #[inline(always)]
    fn orient(&self, x: Scalar, y: Scalar) -> Self {
        let orient = orient(x, y);
        let Transform(mat) = self.get_transform();
        self.clone().set(Transform(multiply(mat, orient)))
    }

    /// Scales in local coordinates.
    #[inline(always)]
    fn scale(&self, sx: Scalar, sy: Scalar) -> Self {
        let scale = scale(sx, sy);
        let Transform(mat) = self.get_transform();
        self.clone().set(Transform(multiply(mat, scale)))
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
        let Transform(mat) = self.get_transform();
        self.clone().set(Transform(multiply(mat, shear)))
    }
}

impl<T: Get<Transform> + Set<Transform> + Clone> RelativeTransform for T {}

/// Should be implemented by contexts that
/// draws something relative to view.
pub trait RelativeViewTransform:
    GetViewTransform + SetViewTransform
  + GetTransform + SetTransform
  + Clone
{
    /// Moves the current transform to the view coordinate system.
    ///
    /// This is usually [0.0, 0.0] in the upper left corner
    /// with the x axis pointing to the right
    /// and the y axis pointing down.
    #[inline(always)]
    fn view(&self) -> Self {
        let mut res = self.clone();
        let ViewTransform(mat) = self.get_view_transform();
        res.set_transform(Transform(mat));
        res
    }

    /// Moves the current transform to the default coordinate system.
    ///
    /// This is usually [0.0, 0.0] in the center
    /// with the x axis pointing to the right
    /// and the y axis pointing up.
    #[inline(always)]
    fn reset(&self) -> Self {
        let mut res = self.clone();
        res.set_transform(Transform(identity()));
        res
    }

    /// Stores the current transform as new view.
    #[inline(always)]
    fn store_view(&self) -> Self {
        let mut res = self.clone();
        let Transform(mat) = self.get_transform();
        res.set_view_transform(ViewTransform(mat));
        res
    }

    /// Computes the current view size.
    #[inline(always)]
    fn get_view_size(&self) -> (Scalar, Scalar) {
        let ViewTransform(mat) = self.get_view_transform();
        let scale = get_scale(mat);
        (2.0 / scale[0], 2.0 / scale[1])
    }
}

impl<
    T: GetViewTransform
     + GetTransform
     + SetViewTransform
     + SetTransform
     + Clone
> RelativeViewTransform for T {}

