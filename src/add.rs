use {
    ImageSize,
};
use internal::{
    Color,
    Polygon,
    Polygons,
    Radius,
    Width,
};
use vecmath::{
    Scalar
};

/// Implemented by contexts that can make a shape bevel.
pub trait AddBevel<T> {
    /// Bevels the shape of the current context.
    fn bevel(&self, radius: Radius) -> T;
}

/// Implemented by contexts that can add round border.
pub trait AddBevelBorder<T> {
    /// Adds a bevel border radius.
    fn bevel_border_radius(&self, radius: Radius) -> T;

    /// Adds a bevel border width.
    #[inline(always)]
    fn bevel_border_width(&self, width: Width) -> T {
        self.bevel_border_radius(0.5 * width)
    }
}

/// Implemented by contexts that can add border.
///
/// This is used in cases where border style is implicit of the shape.
/// For example, `RectangleContext` gives a `RectangleBorderContext `.
pub trait AddBorder<T> {
    /// Adds a border radius.
    fn border_radius(&self, radius: Radius) -> T;

    /// Adds a border width.
    #[inline(always)]
    fn border_width(&self, width: Width) -> T {
        self.border_radius(0.5 * width)
    }
}

/// Implemented by contexts who can add color.
pub trait AddColor<T> {
    /// Add color with alpha channel.
    fn rgba(&self, r: f32, g: f32, b: f32, a: f32) -> T;

    /// Adds color with alpha channel set to 1.0.
    #[inline(always)]
    fn rgb(&self, r: f32, g: f32, b: f32) -> T {
        self.rgba(r, g, b, 1.0)
    }

    /// Add color [r, g, b, a].
    #[inline(always)]
    fn color(&self, color: Color) -> T {
        self.rgba(color[0], color[1], color[2], color[3])
    }

    /// Adds a gray color.
    ///
    /// `0.0` is black and `1.0` is white.
    #[inline(always)]
    fn grey(&self, f: f32) -> T {
        self.rgba(f, f, f, 1.0)
    }

    /// Adds a white semi-transparent color.
    ///
    /// `0.0` is fully transparent and `1.0` is fully opaque.
    #[inline(always)]
    fn alpha(&self, f: f32) -> T {
        self.rgba(1.0, 1.0, 1.0, f)
    }

    /// Adds a hexadecimal color.
    /// Causes task panic if not valid format.
    #[inline(always)]
    fn hex(&self, hex: &str) -> T {
        use read_color::rgb_maybe_a;

        let (rgb, a) = rgb_maybe_a(&mut hex.chars()).unwrap();
        let color = match a {
            None => [rgb[0], rgb[1], rgb[2], 255],
            Some(a) => [rgb[0], rgb[1], rgb[2], a]
        };
        let inv_255 = 1.0f32 / 255.0f32;
        self.rgba(
            color[0] as f32 * inv_255,
            color[1] as f32 * inv_255,
            color[2] as f32 * inv_255,
            color[3] as f32 * inv_255
        )
    }
}


/// Implemented by all contexts that can add ellipse.
pub trait AddEllipse<T> {
    /// Adds an ellipse.
    fn ellipse(&self, x: Scalar, y: Scalar, w: Scalar, h: Scalar) -> T;

    /// Adds an ellipse with coordinates in the center.
    #[inline(always)]
    fn ellipse_centered(
        &self,
        center_x: Scalar,
        center_y: Scalar,
        radius_width: Radius,
        radius_height: Radius
    ) -> T {
        self.ellipse(
            center_x - radius_width,
            center_y - radius_height,
            2.0 * radius_width,
            2.0 * radius_height
       )
    }

    /// Adds a circle.
    #[inline(always)]
    fn circle(
        &self,
        center_x: Scalar,
        center_y: Scalar,
        radius: Radius
    ) -> T {
        self.ellipse(center_x - radius,
                  center_y - radius,
                  2.0 * radius,
                  2.0 * radius)
    }
}

/// Implemented by contexts that can add image.
pub trait AddImage<'b, T, I: ImageSize> {
    /// Add image to context.
    fn image(&self, image: &'b I) -> T;
}


/// Implemented by all contexts that can add rectangle.
pub trait AddLine<T> {
    /// Adds a line.
    fn line(
        &self,
        x1: Scalar,
        y1: Scalar,
        x2: Scalar,
        y2: Scalar
    ) -> T;
}

/// Implemented by contexts who can add polygon.
pub trait AddPolygon<'a, T> {
    /// Add polygon.
    fn polygon(&self, polygon: Polygon<'a>) -> T;
}

/// Implemented by tweening contexts that can add polygons.
pub trait AddPolygons<'a, T> {
    /// Add polygons.
    fn polygons(&self, polygons: Polygons<'a>) -> T;
}


/// Implemented by all contexts that can add rectangle.
pub trait AddRectangle<T> {
    /// Adds a rectangle.
    fn rect(&self, x: Scalar, y: Scalar, w: Scalar, h: Scalar) -> T;

    /// Adds a rectangle with coordinates in the center.
    #[inline(always)]
    fn rect_centered(
        &self,
        center_x: Scalar,
        center_y: Scalar,
        radius_width: Radius,
        radius_height: Radius
    ) -> T {
        self.rect(
            center_x - radius_width,
            center_y - radius_height,
            2.0 * radius_width,
            2.0 * radius_height
       )
    }

    /// Adds a square with coordinates of upper left corner.
    #[inline(always)]
    fn square(&self, x: Scalar, y: Scalar, w: Scalar) -> T {
        self.rect(x, y, w, w)
    }

    /// Adds a square with coordinates in the center.
    #[inline(always)]
    fn square_centered(
        &self,
        center_x: Scalar,
        center_y: Scalar,
        radius: Radius
    ) -> T {
        self.rect(center_x - radius,
                  center_y - radius,
                  2.0 * radius,
                  2.0 * radius)
    }
}

/// Implemented by contexts that can add round border.
pub trait AddRoundBorder<T> {
    /// Adds a round border radius.
    fn round_border_radius(&self, radius: Radius) -> T;

    /// Adds a round border width.
    #[inline(always)]
    fn round_border_width(&self, width: Width) -> T {
        self.round_border_radius(0.5 * width)
    }
}


/// Implemented by contexts that can make a shape rounder.
pub trait AddRound<T> {
    /// Rounds the shape of the current context.
    fn round(&self, radius: Radius) -> T;
}

/// Implemented by contexts that can add square border.
pub trait AddSquareBorder<T> {
    /// Adds a square border radius.
    fn square_border_radius(&self, radius: Radius) -> T;

    /// Adds a square border width.
    #[inline(always)]
    fn square_border_width(&self, width: Width) -> T {
        self.square_border_radius(0.5 * width)
    }
}


/// Implemented by contexts who can give an animated inbetweening context.
pub trait AddTween<T> {
    /// Do linear interpolation.
    fn lerp(&self, tween_factor: Scalar) -> T;
}
