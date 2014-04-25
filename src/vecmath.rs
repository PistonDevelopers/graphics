
//! Various methods for computing with vectors.

use {Matrix2d, InfiniteLine, Vec2d};

/// Multiplies two matrices.
#[inline(always)]
pub fn multiply(m: &[f64, ..6], b: &[f64, ..6]) -> Matrix2d {
    [m[0]*b[0]+m[1]*b[3]+m[2]*0.0,  m[0]*b[1]+m[1]*b[4]+m[2]*0.0,  m[0]*b[2]+m[1]*b[5]+m[2]*1.0,
     m[3]*b[0]+m[4]*b[3]+m[5]*0.0,  m[3]*b[1]+m[4]*b[4]+m[5]*0.0,  m[3]*b[2]+m[4]*b[5]+m[5]*1.0]
}

/// Creates a translation matrix.
#[inline(always)]
pub fn translate(x: f64, y: f64) -> Matrix2d {
    [1.0, 0.0, x,
     0.0, 1.0, y]
}

/// Creates a rotation matrix.
#[inline(always)]
pub fn rotate_radians(angle: f64) -> Matrix2d {
    let c = angle.cos();
    let s = angle.sin();
    [c, s, 0.0,
    -s, c, 0.0]
}

/// Create a scale matrix.
#[inline(always)]
pub fn scale(sx: f64, sy: f64) -> Matrix2d {
    [sx, 0.0, 0.0,
     0.0, sy, 0.0]
}

/// Create a shear matrix.
#[inline(always)]
pub fn shear(sx: f64, sy: f64) -> Matrix2d {
    [1.0, sx, 0.0,
     sy, 1.0, 0.0]
}

/// Compute the shortest vector from a point to an infinite line.
/// An infinite line stores a point on the line and a normal vector.
#[inline(always)]
pub fn separation(line: InfiniteLine, x: f64, y: f64) -> Vec2d {
    // Get the normal vector.
    let (nx, ny) = (line[2], line[3]);
    // Get displacement vector from point.
    let (dx, dy) = (line[0] - x, line[1] - y);
    // Compute the component of position in direction of normal vector.
    let dot = nx * x + ny * y;
    // The normal vector multiplied with the dot gives us coordinates
    // parallel to the line.
    // When we subtract this from the displacement we get a vector normal to the line.
    // This is the shortest vector from the point to the infinite line.
    [dx - dot * nx, dy - dot * ny]
}

