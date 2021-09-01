//! Various methods for computing with vectors.

use std::ops::{Add, Rem};

use vecmath::{self, traits::Float};
pub use vecmath::{
    mat2x3_inv as invert, row_mat2x3_mul as multiply, row_mat2x3_transform_pos2 as transform_pos,
    row_mat2x3_transform_vec2 as transform_vec, vec2_add as add, vec2_cast as cast,
    vec2_cross as cross, vec2_dot as dot, vec2_mul as mul, vec2_scale as mul_scalar,
    vec2_square_len as square_len, vec2_sub as sub,
};

use crate::{
    modular_index::previous,
    types::{Area, Color, Line, Polygon, Ray, Rectangle, SourceRectangle, Triangle},
};

/// The type used for scalars.
pub type Scalar = f64;

/// The type used for matrices.
pub type Matrix2d<T = Scalar> = vecmath::Matrix2x3<T>;

/// The type used for 2D vectors.
pub type Vec2d<T = Scalar> = vecmath::Vector2<T>;

/// The type used for 3D vectors.
pub type Vec3d<T = Scalar> = vecmath::Vector3<T>;

/// Creates a perpendicular vector.
#[inline(always)]
pub fn perp<T>(v: [T; 2]) -> [T; 2]
where
    T: Float,
{
    [-v[1], v[0]]
}

/// Transforms from normalized to absolute coordinates.
///
/// Computes absolute transform from width and height of viewport.
/// In absolute coordinates, the x axis points to the right,
/// and the y axis points down on the screen.
#[inline(always)]
pub fn abs_transform<T>(w: T, h: T) -> Matrix2d<T>
where
    T: Float,
{
    use vecmath::traits::{FromPrimitive, One, Zero};

    let _0: T = Zero::zero();
    let _1: T = One::one();
    let _2: T = FromPrimitive::from_f64(2.0);
    let sx = _2 / w;
    let sy = -_2 / h;
    [[sx, _0, -_1], [_0, sy, _1]]
}

/// Creates a translation matrix.
#[inline(always)]
pub fn translate<T>(v: Vec2d<T>) -> Matrix2d<T>
where
    T: Float,
{
    use vecmath::traits::{One, Zero};

    let _0: T = Zero::zero();
    let _1: T = One::one();
    [[_1, _0, v[0]], [_0, _1, v[1]]]
}

/// Creates a rotation matrix.
#[inline(always)]
pub fn rotate_radians<T>(angle: T) -> Matrix2d<T>
where
    T: Float,
{
    use vecmath::traits::Zero;

    let _0 = Zero::zero();
    let c = angle.cos();
    let s = angle.sin();
    [[c, -s, _0], [s, c, _0]]
}

/// Orients x axis to look at point.
///
/// Leaves x axis unchanged if the
/// point to look at is the origin.
#[inline(always)]
pub fn orient<T>(x: T, y: T) -> Matrix2d<T>
where
    T: Float,
{
    use vecmath::traits::Zero;

    let _0: T = Zero::zero();
    let len = x * x + y * y;
    if len == _0 {
        return identity();
    }

    let len = len.sqrt();
    let c = x / len;
    let s = y / len;
    [[c, -s, _0], [s, c, _0]]
}

/// Create a scale matrix.
#[inline(always)]
pub fn scale<T>(sx: T, sy: T) -> Matrix2d<T>
where
    T: Float,
{
    use vecmath::traits::Zero;

    let _0: T = Zero::zero();
    [[sx, _0, _0], [_0, sy, _0]]
}

/// Create a shear matrix.
#[inline(always)]
pub fn shear<T>(v: Vec2d<T>) -> Matrix2d<T>
where
    T: Float,
{
    use vecmath::traits::{One, Zero};

    let _0 = Zero::zero();
    let _1 = One::one();
    [[_1, v[0], _0], [v[1], _1, _0]]
}

/// Create an identity matrix.
#[inline(always)]
pub fn identity<T>() -> Matrix2d<T>
where
    T: Float,
{
    use vecmath::traits::{One, Zero};

    let _0: T = Zero::zero();
    let _1: T = One::one();
    [[_1, _0, _0], [_0, _1, _0]]
}

/// Extract scale information from matrix.
#[inline(always)]
pub fn get_scale<T>(m: Matrix2d<T>) -> Vec2d<T>
where
    T: Float,
{
    [
        (m[0][0] * m[0][0] + m[1][0] * m[1][0]).sqrt(),
        (m[0][1] * m[0][1] + m[1][1] * m[1][1]).sqrt(),
    ]
}

/// Compute the shortest vector from point to ray.
/// A ray stores starting point and directional vector.
#[inline(always)]
pub fn separation<T>(ray: Ray<T>, v: Vec2d<T>) -> Vec2d<T>
where
    T: Float,
{
    // Get the directional vector.
    let (dir_x, dir_y) = (ray[2], ray[3]);
    // Get displacement vector from point.
    let (dx, dy) = (ray[0] - v[0], ray[1] - v[1]);
    // Compute the component of position in ray direction.
    let dot = dir_x * v[0] + dir_y * v[1];
    // The directional vector multiplied with
    // the dot gives us a parallel vector.
    // When we subtract this from the displacement
    // we get a vector normal to the ray.
    // This is the shortest vector from the point to the ray.
    [dx - dot * dir_x, dy - dot * dir_y]
}

/// Returns the least separation out of four.
/// Each seperation can be computed using `separation` function.
/// The separation returned can be used
/// to solve collision of rectangles.
#[inline(always)]
pub fn least_separation_4<T>(
    sep1: Vec2d<T>,
    sep2: Vec2d<T>,
    sep3: Vec2d<T>,
    sep4: Vec2d<T>,
) -> Vec2d<T>
where
    T: Float,
{
    let dot1 = sep1[0] * sep1[0] + sep1[1] * sep1[1];
    let dot2 = sep2[0] * sep2[0] + sep2[1] * sep2[1];
    let dot3 = sep3[0] * sep3[0] + sep3[1] * sep3[1];
    let dot4 = sep4[0] * sep4[0] + sep4[1] * sep4[1];
    // Search for the smallest dot product.
    if dot1 < dot2 {
        if dot3 < dot4 {
            if dot1 < dot3 {
                sep1
            } else {
                sep3
            }
        } else {
            if dot1 < dot4 {
                sep1
            } else {
                sep4
            }
        }
    } else {
        if dot3 < dot4 {
            if dot2 < dot3 {
                sep2
            } else {
                sep3
            }
        } else {
            if dot2 < dot4 {
                sep2
            } else {
                sep4
            }
        }
    }
}

/// Shrinks a rectangle by a factor on all sides.
#[inline(always)]
pub fn margin_rectangle<T>(rect: Rectangle<T>, m: T) -> Rectangle<T>
where
    T: Float,
{
    use vecmath::traits::{FromPrimitive, Zero};

    let _0: T = Zero::zero();
    let _05: T = FromPrimitive::from_f64(0.5);
    let _2: T = FromPrimitive::from_f64(2.0);
    let w = rect[2] - _2 * m;
    let h = rect[3] - _2 * m;
    let (x, w) = if w < _0 {
        (rect[0] + _05 * rect[2], _0)
    } else {
        (rect[0] + m, w)
    };
    let (y, h) = if h < _0 {
        (rect[1] + _05 * rect[3], _0)
    } else {
        (rect[1] + m, h)
    };
    [x, y, w, h]
}

/// Computes a relative rectangle using the rectangle as a tile.
#[inline(always)]
pub fn relative_rectangle<T>(rect: Rectangle<T>, v: Vec2d<T>) -> Rectangle<T>
where
    T: Float,
{
    [
        rect[0] + v[0] * rect[2],
        rect[1] + v[1] * rect[3],
        rect[2],
        rect[3],
    ]
}

/// Computes overlap between two rectangles.
/// The area of the overlapping rectangle is positive.
/// A shared edge or corner is not considered overlap.
#[inline(always)]
pub fn overlap_rectangle<T>(a: Rectangle<T>, b: Rectangle<T>) -> Option<Rectangle<T>>
where
    T: Float,
{
    #[inline(always)]
    fn min<T: Float>(a: T, b: T) -> T {
        if a < b {
            a
        } else {
            b
        }
    }

    #[inline(always)]
    fn max<T: Float>(a: T, b: T) -> T {
        if a > b {
            a
        } else {
            b
        }
    }

    if a[0] < b[0] + b[2] && a[1] < b[1] + b[3] && b[0] < a[0] + a[2] && b[1] < a[1] + a[3] {
        let x = max(a[0], b[0]);
        let y = max(a[1], b[1]);
        let w = min(a[0] + a[2], b[0] + b[2]) - x;
        let h = min(a[1] + a[3], b[1] + b[3]) - y;
        Some([x, y, w, h])
    } else {
        None
    }
}

#[cfg(test)]
mod test_overlap {
    use super::overlap_rectangle;

    #[test]
    fn overlap() {
        let a = [0.0, 1.0, 100.0, 101.0];
        let b = [51.0, 52.0, 102.0, 103.0];
        let c = overlap_rectangle(a, b).unwrap();
        assert_eq!(c, [51.0, 52.0, 49.0, 50.0]);
        let d = overlap_rectangle(a, c).unwrap();
        assert_eq!(d, c);
        let e = overlap_rectangle(b, c).unwrap();
        assert_eq!(e, c);
    }

    #[test]
    fn edge() {
        let a = [0.0, 0.0, 100.0, 100.0];
        let b = [100.0, 0.0, 100.0, 100.0];
        let c = overlap_rectangle(a, b);
        assert_eq!(c, None);
    }
}

/// Computes a relative source rectangle using
/// the source rectangle as a tile.
#[inline(always)]
pub fn relative_source_rectangle<T>(rect: SourceRectangle<T>, x: T, y: T) -> SourceRectangle<T>
where
    T: Float,
{
    let (rx, ry, rw, rh) = (rect[0], rect[1], rect[2], rect[3]);
    let (x, y) = (rx + x * rw, ry + y * rh);
    [x, y, rw, rh]
}

/// Computes modular offset safely for numbers.
#[inline(always)]
pub fn modular_offset<T: Add<Output = T> + Rem<Output = T> + Copy>(n: &T, i: &T, off: &T) -> T {
    (*i + (*off % *n + *n)) % *n
}

#[cfg(test)]
mod test_modular_offset {
    use super::*;

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
}

/// Computes the area and centroid of a simple polygon.
///
/// A simple polygon is one that does not intersect itself.
/// Source: http://en.wikipedia.org/wiki/Polygon_area#Simple_polygons
pub fn area_centroid<T>(polygon: Polygon<'_, T>) -> (Area<T>, Vec2d<T>)
where
    T: Float,
{
    use vecmath::traits::{FromPrimitive, Zero};

    let _0: T = Zero::zero();
    let _05: T = FromPrimitive::from_f64(0.5);
    let _3: T = FromPrimitive::from_f64(3.0);
    let n = polygon.len();
    let mut sum = _0;
    let (mut cx, mut cy) = (_0, _0);
    for i in 0..n {
        let qx = polygon[i][0];
        let qy = polygon[i][1];
        let p_i = previous(n, i);
        let px = polygon[p_i][0];
        let py = polygon[p_i][1];
        let cross = px * qy - qx * py;
        cx += (px + qx) * cross;
        cy += (py + qy) * cross;
        sum += cross;
    }

    let area = _05 * sum;
    // 'cx / (6.0 * area)' = 'cx / (3.0 * sum)'
    let centroid = [cx / (_3 * sum), cy / (_3 * sum)];
    (area, centroid)
}

/// Computes area of a simple polygon.
///
/// A simple polygon is one that does not intersect itself.
#[inline(always)]
pub fn area<T>(polygon: Polygon<'_, T>) -> T
where
    T: Float,
{
    let (res, _) = area_centroid(polygon);
    res
}

/// Computes centroid of a simple polygon.
///
/// A simple polygon is one that does not intersect itself.
#[inline(always)]
pub fn centroid<T>(polygon: Polygon<'_, T>) -> Vec2d<T>
where
    T: Float,
{
    let (_, res) = area_centroid(polygon);
    res
}

/// Returns a number that tells which side it is relative to a line.
///
/// Computes the cross product of the vector that gives the line
/// with the vector between point and starting point of line.
/// One side of the line has opposite sign of the other.
#[inline(always)]
pub fn line_side<T>(line: Line<T>, v: Vec2d<T>) -> T
where
    T: Float,
{
    let (ax, ay) = (line[0], line[1]);
    let (bx, by) = (line[2], line[3]);
    (bx - ax) * (v[1] - ay) - (by - ay) * (v[0] - ax)
}

/// Returns true if point is inside triangle.
///
/// This is done by computing a `side` number for each edge.
/// If the number is inside if it is on the same side for all edges.
/// Might break for very small triangles.
pub fn inside_triangle<T>(triangle: Triangle<T>, v: Vec2d<T>) -> bool
where
    T: Float,
{
    use vecmath::traits::Zero;

    let _0: T = Zero::zero();

    let ax = triangle[0][0];
    let ay = triangle[0][1];
    let bx = triangle[1][0];
    let by = triangle[1][1];
    let cx = triangle[2][0];
    let cy = triangle[2][1];

    let ab_side = line_side([ax, ay, bx, by], v);
    let bc_side = line_side([bx, by, cx, cy], v);
    let ca_side = line_side([cx, cy, ax, ay], v);

    let ab_positive = ab_side >= _0;
    let bc_positive = bc_side >= _0;
    let ca_positive = ca_side >= _0;

    ab_positive == bc_positive && bc_positive == ca_positive
}

/// Returns true if triangle is clockwise.
///
/// This is done by computing which side the third vertex is relative to
/// the line starting from the first vertex to second vertex.
///
/// The triangle is considered clockwise if the third vertex is on the line
/// between the two first vertices.
#[inline(always)]
pub fn triangle_face<T>(triangle: Triangle<T>) -> bool
where
    T: Float,
{
    use vecmath::traits::Zero;

    let _0 = Zero::zero();

    let ax = triangle[0][0];
    let ay = triangle[0][1];
    let bx = triangle[1][0];
    let by = triangle[1][1];
    let cx = triangle[2][0];
    let cy = triangle[2][1];

    let ab_side = line_side([ax, ay, bx, by], [cx, cy]);

    ab_side <= _0
}

#[cfg(test)]
mod test_triangle {
    use super::*;

    #[test]
    fn test_triangle() {
        // Triangle counter clock-wise.
        let tri_1 = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0]];
        // Triangle clock-wise.
        let tri_2 = [[0.0, 0.0], [1.0, 1.0], [1.0, 0.0]];
        let (x, y) = (0.5, 0.25);
        assert!(inside_triangle(tri_1, [x, y]));
        assert!(inside_triangle(tri_2, [x, y]));
        assert_eq!(triangle_face(tri_1), false);
        assert!(triangle_face(tri_2));
    }
}

/// Transforms from cartesian coordinates to barycentric.
#[inline(always)]
pub fn to_barycentric<T>(triangle: Triangle<T>, pos: Vec2d<T>) -> Vec3d<T>
where
    T: Float,
{
    use vecmath::traits::One;

    let _1: T = One::one();
    let x = pos[0];
    let y = pos[1];
    let x1 = triangle[0][0];
    let y1 = triangle[0][1];
    let x2 = triangle[1][0];
    let y2 = triangle[1][1];
    let x3 = triangle[2][0];
    let y3 = triangle[2][1];
    let lambda1 = ((y2 - y3) * (x - x3) + (x3 - x2) * (y - y3))
        / ((y2 - y3) * (x1 - x3) + (x3 - x2) * (y1 - y3));
    let lambda2 = ((y3 - y1) * (x - x3) + (x1 - x3) * (y - y3))
        / ((y2 - y3) * (x1 - x3) + (x3 - x2) * (y1 - y3));
    let lambda3 = _1 - lambda1 - lambda2;
    [lambda1, lambda2, lambda3]
}

/// Transforms from barycentric coordinates to cartesian.
#[inline(always)]
pub fn from_barycentric<T>(triangle: Triangle<T>, lambda: Vec3d<T>) -> Vec2d<T>
where
    T: Float,
{
    let x1 = triangle[0][0];
    let y1 = triangle[0][1];
    let x2 = triangle[1][0];
    let y2 = triangle[1][1];
    let x3 = triangle[2][0];
    let y3 = triangle[2][1];
    [
        lambda[0] * x1 + lambda[1] * x2 + lambda[2] * x3,
        lambda[0] * y1 + lambda[1] * y2 + lambda[2] * y3,
    ]
}

#[cfg(test)]
mod test_barycentric {
    use super::*;

    #[test]
    fn test_barycentric() {
        let triangle = [[0.0, 0.0], [100.0, 0.0], [0.0, 50.0]];
        let old_pos = [10.0, 20.0];
        let b = to_barycentric(triangle, old_pos);
        let new_pos: Vec2d = from_barycentric(triangle, b);
        let eps = 0.00001;
        assert!((new_pos[0] - old_pos[0]).abs() < eps);
        assert!((new_pos[1] - old_pos[1]).abs() < eps);
    }
}

/// Transform color with hue, saturation and value.
///
/// Source: http://beesbuzz.biz/code/hsv_color_transforms.php
#[inline(always)]
pub fn hsv(color: Color, h_rad: f32, s: f32, v: f32) -> Color {
    let vsu = v * s * h_rad.cos();
    let vsw = v * s * h_rad.sin();
    [
        (0.299 * v + 0.701 * vsu + 0.168 * vsw) * color[0]
            + (0.587 * v - 0.587 * vsu + 0.330 * vsw) * color[1]
            + (0.114 * v - 0.114 * vsu - 0.497 * vsw) * color[2],
        (0.299 * v - 0.299 * vsu - 0.328 * vsw) * color[0]
            + (0.587 * v + 0.413 * vsu + 0.035 * vsw) * color[1]
            + (0.114 * v - 0.114 * vsu + 0.292 * vsw) * color[2],
        (0.299 * v - 0.3 * vsu + 1.25 * vsw) * color[0]
            + (0.587 * v - 0.588 * vsu - 1.05 * vsw) * color[1]
            + (0.114 * v + 0.886 * vsu - 0.203 * vsw) * color[2],
        color[3],
    ]
}
