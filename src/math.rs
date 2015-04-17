//! Various methods for computing with vectors.

pub use vecmath_lib::row_mat2x3_mul as multiply;
pub use vecmath_lib::vec2_dot as dot;
pub use vecmath_lib::vec2_cross as cross;
pub use vecmath_lib::vec2_add as add;
pub use vecmath_lib::vec2_sub as sub;
pub use vecmath_lib::vec2_cast as cast;
pub use vecmath_lib::vec2_mul as mul;
pub use vecmath_lib::vec2_scale as mul_scalar;
pub use vecmath_lib::vec2_square_len as square_len;
pub use vecmath_lib::row_mat2x3_transform_pos2 as transform_pos;
pub use vecmath_lib::row_mat2x3_transform_vec2 as transform_vec;

use std::ops::{ Add, Rem };
use vecmath_lib;
use types::{
    Area,
    Color,
    Line,
    Polygon,
    Ray,
    Rectangle,
    SourceRectangle,
    Triangle,
};
use num::Float;
use modular_index::previous;

/// The type used for scalars.
pub type Scalar = f64;

/// The type used for matrices.
pub type Matrix2d = vecmath_lib::Matrix2x3<Scalar>;

/// The type used for 2D vectors.
pub type Vec2d = vecmath_lib::Vector2<Scalar>;

/// The type used for 3D vectors.
pub type Vec3d = vecmath_lib::Vector3<Scalar>;

/// Creates a perpendicular vector.
#[inline(always)]
pub fn perp(v: [Scalar; 2]) -> [Scalar; 2] {
    [-v[1], v[0]]
}

/// Transforms from normalized to absolute coordinates.
///
/// Computes absolute transform from width and height of viewport.
/// In absolute coordinates, the x axis points to the right,
/// and the y axis points down on the screen.
#[inline(always)]
pub fn abs_transform(w: Scalar, h: Scalar) -> Matrix2d {
    let sx = 2.0 / w;
    let sy = -2.0 / h;
    [[ sx, 0.0, -1.0 ],
     [ 0.0,  sy, 1.0 ]]
}

/// Creates a translation matrix.
#[inline(always)]
pub fn translate(v: Vec2d) -> Matrix2d {
    [[1.0, 0.0, v[0]],
     [0.0, 1.0, v[1]]]
}

/// Creates a rotation matrix.
#[inline(always)]
pub fn rotate_radians(angle: Scalar) -> Matrix2d {
    let c = angle.cos();
    let s = angle.sin();
    [[c, -s, 0.0],
     [s,  c, 0.0]]
}

/// Orients x axis to look at point.
///
/// Leaves x axis unchanged if the
/// point to look at is the origin.
#[inline(always)]
pub fn orient(x: Scalar, y: Scalar) -> Matrix2d {
    let len = x * x + y * y;
    if len == 0.0 {
        return identity()
    }

    let len = len.sqrt();
    let c = x / len;
    let s = y / len;
    [[c, -s, 0.0],
     [s,  c, 0.0]]
}

/// Create a scale matrix.
#[inline(always)]
pub fn scale(sx: Scalar, sy: Scalar) -> Matrix2d {
    [[sx, 0.0, 0.0],
     [0.0, sy, 0.0]]
}

/// Create a shear matrix.
#[inline(always)]
pub fn shear(v: Vec2d) -> Matrix2d {
    [[1.0, v[0], 0.0],
     [v[1], 1.0, 0.0]]
}

/// Create an identity matrix.
#[inline(always)]
pub fn identity() -> Matrix2d {
    [[1.0, 0.0, 0.0],
     [0.0, 1.0, 0.0]]
}

/// Extract scale information from matrix.
#[inline(always)]
pub fn get_scale(m: Matrix2d) -> Vec2d {
    [
        (m[0][0] * m[0][0] + m[1][0] * m[1][0]).sqrt(),
        (m[0][1] * m[0][1] + m[1][1] * m[1][1]).sqrt()
    ]
}

/// Compute the shortest vector from point to ray.
/// A ray stores starting point and directional vector.
#[inline(always)]
pub fn separation(ray: Ray, v: Vec2d) -> Vec2d {
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
pub fn least_separation_4(
    sep1: Vec2d,
    sep2: Vec2d,
    sep3: Vec2d,
    sep4: Vec2d
) -> Vec2d {
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
pub fn margin_rectangle(rect: Rectangle, m: Scalar) -> Rectangle {
    let w = rect[2] - 2.0 * m;
    let h = rect[3] - 2.0 * m;
    let (x, w)
        =   if w < 0.0 {
                (rect[0] + 0.5 * rect[2], 0.0)
            } else {
                (rect[0] + m, w)
            };
    let (y, h)
        =   if h < 0.0 {
                (rect[1] + 0.5 * rect[3], 0.0)
            } else {
                (rect[1] + m, h)
            };
    [x, y, w, h]
}

/// Computes a relative rectangle using the rectangle as a tile.
#[inline(always)]
pub fn relative_rectangle(rect: Rectangle, v: Vec2d) -> Rectangle {
    [
        rect[0] + v[0] * rect[2],
        rect[1] + v[1] * rect[3],
        rect[2],
        rect[3]
    ]
}

/// Computes a relative source rectangle using
/// the source rectangle as a tile.
#[inline(always)]
pub fn relative_source_rectangle(
    rect: SourceRectangle,
    x: i32,
    y: i32
) -> SourceRectangle {
    let (rx, ry, rw, rh) = (rect[0], rect[1], rect[2], rect[3]);
    let (x, y) = (rx + x * rw, ry + y * rh);
    [x, y, rw, rh]
}

/// Computes modular offset safely for numbers.
#[inline(always)]
pub fn modular_offset<T: Add<Output = T> + Rem<Output = T> + Copy>(
    n: &T,
    i: &T,
    off: &T
) -> T {
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
pub fn area_centroid(polygon: Polygon) -> (Area, Vec2d) {
    let n = polygon.len();
    let mut sum = 0.0;
    let (mut cx, mut cy) = (0.0, 0.0);
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

    let area = 0.5 * sum;
    // 'cx / (6.0 * area)' = 'cx / (3.0 * sum)'
    let centroid = [cx / (3.0 * sum), cy / (3.0 * sum)];
    (area, centroid)
}

/// Computes area of a simple polygon.
///
/// A simple polygon is one that does not intersect itself.
#[inline(always)]
pub fn area(polygon: Polygon) -> Scalar {
    let (res, _) = area_centroid(polygon);
    res
}

/// Computes centroid of a simple polygon.
///
/// A simple polygon is one that does not intersect itself.
#[inline(always)]
pub fn centroid(polygon: Polygon) -> Vec2d {
    let (_, res) = area_centroid(polygon);
    res
}

/// Returns a number that tells which side it is relative to a line.
///
/// Computes the cross product of the vector that gives the line
/// with the vector between point and starting point of line.
/// One side of the line has opposite sign of the other.
#[inline(always)]
pub fn line_side<L: Into<Line>>(line: L, v: Vec2d) -> Scalar {
    let line: Line = line.into();
    let (ax, ay) = (line.s.x, line.s.y);
    let (bx, by) = (line.e.x, line.e.y);
    (bx - ax) * (v[1] - ay) - (by - ay) * (v[0] - ax)
}

/// Returns true if point is inside triangle.
///
/// This is done by computing a `side` number for each edge.
/// If the number is inside if it is on the same side for all edges.
/// Might break for very small triangles.
pub fn inside_triangle(triangle: Triangle, v: Vec2d) -> bool {
    let ax = triangle[0][0];
    let ay = triangle[0][1];
    let bx = triangle[1][0];
    let by = triangle[1][1];
    let cx = triangle[2][0];
    let cy = triangle[2][1];

    let ab_side = line_side([ax, ay, bx, by], v);
    let bc_side = line_side([bx, by, cx, cy], v);
    let ca_side = line_side([cx, cy, ax, ay], v);

    let ab_positive = ab_side.is_sign_positive();
    let bc_positive = bc_side.is_sign_positive();
    let ca_positive = ca_side.is_sign_positive();

    ab_positive == bc_positive
    && bc_positive == ca_positive
}

/// Returns true if triangle is clockwise.
///
/// This is done by computing which side the third vertex is relative to
/// the line starting from the first vertex to second vertex.
#[inline(always)]
pub fn triangle_face(
    triangle: Triangle
) -> bool {
    let ax = triangle[0][0];
    let ay = triangle[0][1];
    let bx = triangle[1][0];
    let by = triangle[1][1];
    let cx = triangle[2][0];
    let cy = triangle[2][1];

    let ab_side = line_side([ax, ay, bx, by], [cx, cy]);

    ab_side.is_sign_negative()
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
        assert_eq!(inside_triangle(tri_1, [x, y]), true);
        assert_eq!(inside_triangle(tri_2, [x, y]), true);
        assert_eq!(triangle_face(tri_1), false);
        assert_eq!(triangle_face(tri_2), true);
    }
}

/// Transforms from cartesian coordinates to barycentric.
#[inline(always)]
pub fn to_barycentric(triangle: Triangle, pos: Vec2d) -> Vec3d {
    let x = pos[0]; let y = pos[1];
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
    let lambda3 = 1.0 - lambda1 - lambda2;
    [lambda1, lambda2, lambda3]
}

/// Transforms from barycentric coordinates to cartesian.
#[inline(always)]
pub fn from_barycentric(triangle: Triangle, lambda: Vec3d) -> Vec2d {
    let x1 = triangle[0][0];
    let y1 = triangle[0][1];
    let x2 = triangle[1][0];
    let y2 = triangle[1][1];
    let x3 = triangle[2][0];
    let y3 = triangle[2][1];
    [lambda[0] * x1 + lambda[1] * x2 + lambda[2] * x3,
     lambda[0] * y1 + lambda[1] * y2 + lambda[2] * y3]
}

#[cfg(test)]
mod test_barycentric {
    use num::Float;
    use super::*;

    #[test]
    fn test_barycentric() {
        let triangle = [[0.0, 0.0], [100.0, 0.0], [0.0, 50.0]];
        let old_pos = [10.0, 20.0];
        let b = to_barycentric(triangle, old_pos);
        let new_pos = from_barycentric(triangle, b);
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
