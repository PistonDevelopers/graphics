
//! Various methods for computing with vectors.

use {Matrix2d, Ray, Vec2d, Rectangle, RoundRectangle};
use modular_index::{previous};

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

/// Shrinks a rectangle by a factor on all sides.
#[inline(always)]
pub fn margin_rectangle(rect: &Rectangle, m: f64) -> Rectangle {
    let w = rect[2] - 2.0 * m;
    let h = rect[3] - 2.0 * m;
    let (x, w) = if w < 0.0 { (rect[0] + 0.5 * rect[2], 0.0) } else { (rect[0] + m, w) };
    let (y, h) = if h < 0.0 { (rect[1] + 0.5 * rect[3], 0.0) } else { (rect[1] + m, h) };
    [x, y, w, h]
}

/// Shrinks a rectangle by a factor on all sides.
#[inline(always)]
pub fn margin_round_rectangle(rect: &RoundRectangle, m: f64) -> RoundRectangle {
    let w = rect[2] - 2.0 * m;
    let h = rect[3] - 2.0 * m;
    let (x, w) = if w < 0.0 { (rect[0] + 0.5 * rect[2], 0.0) } else { (rect[0] + m, w) };
    let (y, h) = if h < 0.0 { (rect[1] + 0.5 * rect[3], 0.0) } else { (rect[1] + m, h) };
    [x, y, w, h, rect[4]]
}

/// Computes a relative rectangle using the rectangle as a tile.
#[inline(always)]
pub fn relative_rectangle(rect: &Rectangle, x: f64, y: f64) -> Rectangle {
    [rect[0] + x * rect[2], rect[1] + y * rect[3], rect[2], rect[3]]
}

/// Computes a relative round rectangle using the round rectangle as a tile.
#[inline(always)]
pub fn relative_round_rectangle(rect: &RoundRectangle, x: f64, y: f64) -> RoundRectangle {
    [rect[0] + x * rect[2], rect[1] + y * rect[3], rect[2], rect[3], rect[4]]
}

/// Computes modular offset safely for numbers.
#[inline(always)]
pub fn modular_offset<T: Add<T, T> + Rem<T, T>>(n: &T, i: &T, off: &T) -> T {
    (*i + (*off % *n + *n)) % *n 
}

#[test]
fn test_modular_offset() {
    assert_eq!(modular_offset(&3.0_f64, &0.0_f64, &-1.0_f64), 2.0_f64);
    assert_eq!(modular_offset(&3.0_f64, &1.0_f64, &-1.0_f64), 0.0_f64);
    assert_eq!(modular_offset(&3.0_f64, &2.0_f64, &-1.0_f64), 1.0_f64);
    assert_eq!(modular_offset(&3.0_f64, &3.0_f64, &-1.0_f64), 2.0_f64);

    assert_eq!(modular_offset(&3.0_f64, &0.0_f64, &1.0_f64), 1.0_f64);
    assert_eq!(modular_offset(&3.0_f64, &1.0_f64, &1.0_f64), 2.0_f64);
    assert_eq!(modular_offset(&3.0_f64, &2.0_f64, &1.0_f64), 0.0_f64);
    assert_eq!(modular_offset(&3.0_f64, &3.0_f64, &1.0_f64), 1.0_f64);
}

/// Computes the area of a simple polygon.  
/// A simple polygon is one that does not intersect itself.
pub fn area(polygon: &[f64]) -> f64 {
    let n = polygon.len() / 2;
    let mut sum = 0.0_f64;
    for i in range(0, n) {
        let (qx, qy) = (polygon[i * 2], polygon[i * 2 + 1]);
        let p_i = previous(n, i);
        let (px, py) = (polygon[p_i * 2], polygon[p_i * 2 + 1]);
        sum += px * qy - qx * py;
    }

    0.5 * sum
}

