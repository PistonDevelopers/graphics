use internal::{
    CanColor,
    CanRectangle,
    CanSourceRectangle,
    CanTransform,
    Color,
    ColorComponent,
    HasColor,
    HasRectangle,
    HasSourceRectangle,
    HasTransform,
    Rectangle,
    SourceRectangle,
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
};

/// Implemented by contexts that contains color.
pub trait RelativeColor<T> {
    /// Multiplies with red, green, blue and alpha values.
    fn mul_rgba(
        &self, 
        r: ColorComponent, 
        g: ColorComponent, 
        b: ColorComponent, 
        a: ColorComponent
    ) -> T;

    /// Mixes the current color with white.
    ///
    /// 0 is black and 1 is white.
    #[inline(always)]
    fn tint(&self, f: ColorComponent) -> T {
        self.mul_rgba(f, f, f, 1.0)
    }

    /// Mixes the current color with black.
    ///
    /// 0 is white and 1 is black.
    #[inline(always)]
    fn shade(&self, f: ColorComponent) -> T {
        let f = 1.0 - f;
        self.mul_rgba(f, f, f, 1.0)
    }
    
    /// Rotates hue by degrees.
    #[inline(always)]
    fn hue_deg(&self, angle: ColorComponent) -> T {
        let pi: ColorComponent = Float::pi();
        self.hue_rad(angle * pi / 180.0)
    }

    /// Rotates hue by radians.
    fn hue_rad(&self, angle: ColorComponent) -> T;
}

impl<
    T: HasColor<Color> + CanColor<U, Color>,
    U
> RelativeColor<U> for T {
    #[inline(always)]
    fn mul_rgba(
        &self, 
        r: ColorComponent, 
        g: ColorComponent, 
        b: ColorComponent, 
        a: ColorComponent
    ) -> U {
        let color = self.get_color();
        self.color([color[0] * r, color[1] * g, color[2] * b, color[3] * a])
    }
    
    #[inline(always)]
    fn hue_rad(&self, angle: ColorComponent) -> U {
        self.color(hsv(self.get_color(), angle, 1.0, 1.0))
    }
}

/// Should be implemented by contexts that have rectangle information.
pub trait RelativeRectangle<T> {
    /// Shrinks the current rectangle equally by all sides.
    fn margin(&self, m: f64) -> T;

    /// Expands the current rectangle equally by all sides.
    #[inline(always)]
    fn expand(&self, m: f64) -> T {
        self.margin(-m)
    }

    /// Moves to a relative rectangle using the current rectangle as tile.
    fn rel(&self, x: f64, y: f64) -> T;
}

impl<
    T: HasRectangle<Rectangle> + CanRectangle<U, Rectangle>,
    U
> RelativeRectangle<U> for T {
    #[inline(always)]
    fn margin(&self, m: f64) -> U {
        self.rectangle(margin_rectangle(self.get_rectangle(), m))
    }

    #[inline(always)]
    fn rel(&self, x: f64, y: f64) -> U {
        self.rectangle(relative_rectangle(self.get_rectangle(), x, y))
    }
}

/// Should be implemented by contexts that 
/// have source rectangle information.
pub trait RelativeSourceRectangle<T> {
    /// Adds a source rectangle.
    fn src_rect(&self, x: i32, y: i32, w: i32, h: i32) -> T;

    /// Moves to a relative source rectangle using 
    /// the current source rectangle as tile.
    fn src_rel(&self, x: i32, y: i32) -> T;

    /// Flips the source rectangle horizontally.
    fn src_flip_h(&self) -> T;

    /// Flips the source rectangle vertically.
    fn src_flip_v(&self) -> T;

    /// Flips the source rectangle horizontally and vertically.
    fn src_flip_hv(&self) -> T;
}

impl<T: HasSourceRectangle<SourceRectangle> 
        + CanSourceRectangle<U, SourceRectangle>,
    U
> RelativeSourceRectangle<U> for T {
    #[inline(always)]
    fn src_rel(&self, x: i32, y: i32) -> U {
        self.source_rectangle(
            relative_source_rectangle(self.get_source_rectangle(), x, y)
        )
    }

    #[inline(always)]
    fn src_rect(&self, x: i32, y: i32, w: i32, h: i32) -> U {
        self.source_rectangle([x, y, w, h])
    }

    #[inline(always)]
    fn src_flip_h(&self) -> U {
        let source_rect = self.get_source_rectangle();
        self.source_rectangle([
            source_rect[0] + source_rect[2],
            source_rect[1],
            -source_rect[2],
            source_rect[3]
        ])
    }

    #[inline(always)]
    fn src_flip_v(&self) -> U {
        let source_rect = self.get_source_rectangle();
        self.source_rectangle([
            source_rect[0],
            source_rect[1] + source_rect[3],
            source_rect[2],
            -source_rect[3]
        ])
    }

    #[inline(always)]
    fn src_flip_hv(&self) -> U {
        let source_rect = self.get_source_rectangle();
        self.source_rectangle([
            source_rect[0] + source_rect[2],
            source_rect[1] + source_rect[3],
            -source_rect[2],
            -source_rect[3]
        ])
    }
}

/// Implemented by contexts that can transform.
pub trait RelativeTransform2d {
    /// Translate x an y in local coordinates.
    fn trans(&self, x: Scalar, y: Scalar) -> Self;
    
    /// Rotates degrees in local coordinates.
    #[inline(always)]
    fn rot_deg(&self, angle: Scalar) -> Self {
        let pi: Scalar = Float::pi();
        self.rot_rad(angle * pi / 180.0)
    }

    /// Rotate radians in local coordinates.
    fn rot_rad(&self, angle: Scalar) -> Self;

    /// Orients x axis to look at point locally.
    ///
    /// Leaves x axis unchanged if the point to 
    /// look at is the origin.
    fn orient(&self, x: Scalar, y: Scalar) -> Self;

    /// Scales in local coordinates.
    fn scale(&self, sx: Scalar, sy: Scalar) -> Self;

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
        self.scale(-1.0, 0.0)
    }

    /// Flips horizontally and vertically in local coordinates.
    #[inline(always)]
    fn flip_hv(&self) -> Self {
        self.scale(-1.0, -1.0)
    }

    /// Shears in local coordinates.
    fn shear(&self, sx: Scalar, sy: Scalar) -> Self;
}

impl<T: HasTransform<Matrix2d> + CanTransform<T, Matrix2d>
> RelativeTransform2d for T {
    #[inline(always)]
    fn trans(&self, x: Scalar, y: Scalar) -> T {
        let trans = translate(x, y);
        self.transform(multiply(self.get_transform(), trans))
    }

    #[inline(always)]
    fn rot_rad(&self, angle: Scalar) -> T {
        let rot = rotate_radians(angle);
        self.transform(multiply(self.get_transform(), rot))
    }

    #[inline(always)]
    fn orient(&self, x: Scalar, y: Scalar) -> T {
        let orient = orient(x, y);
        self.transform(multiply(self.get_transform(), orient))
    }

    #[inline(always)]
    fn scale(&self, sx: Scalar, sy: Scalar) -> T {
        let scale = scale(sx, sy);
        self.transform(multiply(self.get_transform(), scale))
    }

    #[inline(always)]
    fn shear(&self, sx: Scalar, sy: Scalar) -> T {
        let shear = shear(sx, sy);
        self.transform(multiply(self.get_transform(), shear))
    }
}

