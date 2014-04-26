
/// Implemented by contexts that can transform.
pub trait Transform2d<'a> {
    /// Translate x and y.
    fn trans(&'a self, x: f64, y: f64) -> Self;
  
    /// Translate x an y in local coordinates.
    fn trans_local(&'a self, x: f64, y: f64) -> Self;
 
    /// Rotates degrees.
    #[inline(always)]
    fn rot_deg(&'a self, angle: f64) -> Self {
        let pi: f64 = Float::pi();
        self.rot_rad(angle * pi / 180.0)
    }
    
    /// Rotate radians.
    fn rot_rad(&'a self, angle: f64) -> Self;

    /// Rotate radians in local coordinates.
    fn rot_rad_local(&'a self, angle: f64) -> Self;

    /// Scale.
    fn scale(&'a self, sx: f64, sy: f64) -> Self;

    /// Scales in local coordinates.
    fn scale_local(&'a self, sx: f64, sy: f64) -> Self;

    /// Scales in both directions.
    #[inline(always)]
    fn zoom(&'a self, s: f64) -> Self {
        self.scale(s, s)
    }

    /// Flips vertically.
    #[inline(always)]
    fn flip_v(&'a self) -> Self {
        self.scale(1.0, -1.0)
    }

    /// Flips horizontally.
    #[inline(always)]
    fn flip_h(&'a self) -> Self {
        self.scale(-1.0, 0.0)
    }

    /// Shear.
    fn shear(&'a self, sx: f64, sy: f64) -> Self;
}
