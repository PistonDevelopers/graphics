
//! Various methods for computing with vectors.

use {Matrix2d, Ray, Vec2d};

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

/// Compute the shortest vector from point to ray.
/// A ray stores starting point and directional vector.
#[inline(always)]
pub fn separation(ray: Ray, x: f64, y: f64) -> Vec2d {
    // Get the directional vector.
    let (dir_x, dir_y) = (ray[2], ray[3]);
    // Get displacement vector from point.
    let (dx, dy) = (ray[0] - x, ray[1] - y);
    // Compute the component of position in ray direction.
    let dot = dir_x * x + dir_y * y;
    // The directional vector multiplied with the dot gives us a parallel vector.
    // When we subtract this from the displacement we get a vector normal to the ray.
    // This is the shortest vector from the point to the ray.
    [dx - dot * dir_x, dy - dot * dir_y]
}

/// Returns the least separation out of four.
/// Each seperation can be computed using `separation` function.
/// The separation returned can be used to solve collision of rectangles.
#[inline(always)]
pub fn least_separation_4(sep1: Vec2d, sep2: Vec2d, sep3: Vec2d, sep4: Vec2d) -> Vec2d {
    let dot1 = sep1[0] * sep1[0] + sep1[1] * sep1[1];
    let dot2 = sep2[0] * sep2[0] + sep2[1] * sep2[1];
    let dot3 = sep3[0] * sep3[0] + sep3[1] * sep3[1];
    let dot4 = sep4[0] * sep4[0] + sep4[1] * sep4[1];
    // Search for the smallest dot product.
    if dot1 < dot2 {
        if dot3 < dot4 {
            if dot1 < dot3 { sep1 } else { sep3 }
        } else {
            if dot1 < dot4 { sep1 } else { sep4 }
        }
    } else {
        if dot3 < dot4 {
            if dot2 < dot3 { sep2 } else { sep3 }
        } else {
            if dot2 < dot4 { sep2 } else { sep4 }
        }
    }
}
