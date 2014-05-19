
//! Various methods for computing with vectors.

use {
    Line,
    Matrix2d,
    Rectangle,
};
use modular_index::{previous};

/// Multiplies two matrices.
#[inline(always)]
pub fn multiply(
    &Matrix2d(m): &Matrix2d,
    &Matrix2d(b): &Matrix2d
) -> Matrix2d {
    Matrix2d(
        [m[0]*b[0]+m[1]*b[3]+m[2]*0.0,  m[0]*b[1]+m[1]*b[4]+m[2]*0.0,  m[0]*b[2]+m[1]*b[5]+m[2]*1.0,
         m[3]*b[0]+m[4]*b[3]+m[5]*0.0,  m[3]*b[1]+m[4]*b[4]+m[5]*0.0,  m[3]*b[2]+m[4]*b[5]+m[5]*1.0]
    )
}

/// Creates a translation matrix.
#[inline(always)]
pub fn translate(x: f64, y: f64) -> Matrix2d {
    Matrix2d(
        [1.0, 0.0, x,
         0.0, 1.0, y]
    )
}

/// Creates a rotation matrix.
#[inline(always)]
pub fn rotate_radians(angle: f64) -> Matrix2d {
    let c = angle.cos();
    let s = angle.sin();
    Matrix2d(
        [c, -s, 0.0,
         s,  c, 0.0]
    )
}

/// Orients x axis to look at point.
///
/// Leaves x axis unchanged if the point to look at is the origin.
#[inline(always)]
pub fn orient(x: f64, y: f64) -> Matrix2d {
    let len = x * x + y * y;
    if len == 0.0 {
         return Matrix2d(
            [1.0, 0.0, 0.0,
             0.0, 1.0, 0.0]
         );
    }

    let len = len.sqrt();
    let c = x / len;
    let s = y / len;
    Matrix2d(
        [c, -s, 0.0,
         s,  c, 0.0]
    )
}

/// Create a scale matrix.
#[inline(always)]
pub fn scale(sx: f64, sy: f64) -> Matrix2d {
    Matrix2d(
        [sx, 0.0, 0.0,
         0.0, sy, 0.0]
    )
}

/// Create a shear matrix.
#[inline(always)]
pub fn shear(sx: f64, sy: f64) -> Matrix2d {
    Matrix2d(
        [1.0, sx, 0.0,
         sy, 1.0, 0.0]
    )
}

/// Create an identity matrix.
#[inline(always)]
pub fn identity() -> Matrix2d {
    Matrix2d(
        [1.0, 0.0, 0.0,
         0.0, 1.0, 0.0]
    )
}

/// Compute the shortest vector from point to ray.
/// A ray stores starting point and directional vector.
#[inline(always)]
pub fn separation(
    &ray: &[f64, ..4],
    x: f64,
    y: f64) -> [f64, ..2] {
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
pub fn least_separation_4(
    &sep1: &[f64, ..2],
    &sep2: &[f64, ..2],
    &sep3: &[f64, ..2],
    &sep4: &[f64, ..2]
) -> [f64, ..2] {
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
pub fn margin_rectangle(
    &Rectangle(rect): &Rectangle,
    m: f64
) -> Rectangle {
    let w = rect[2] - 2.0 * m;
    let h = rect[3] - 2.0 * m;
    let (x, w) = if w < 0.0 { (rect[0] + 0.5 * rect[2], 0.0) } else { (rect[0] + m, w) };
    let (y, h) = if h < 0.0 { (rect[1] + 0.5 * rect[3], 0.0) } else { (rect[1] + m, h) };
    Rectangle([x, y, w, h])
}

/// Computes a relative rectangle using the rectangle as a tile.
#[inline(always)]
pub fn relative_rectangle(
    &Rectangle(rect): &Rectangle,
    x: f64,
    y: f64
) -> Rectangle {
    Rectangle([rect[0] + x * rect[2], rect[1] + y * rect[3], rect[2], rect[3]])
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

/// Computes the area and centroid of a simple polygon.
///
/// A simple polygon is one that does not intersect itself.
/// Source: http://en.wikipedia.org/wiki/Polygon_area#Simple_polygons
pub fn area_centroid(polygon: &[f64]) -> (f64, [f64, ..2]) {
    let n = polygon.len() / 2;
    let mut sum = 0.0_f64;
    let (mut cx, mut cy) = (0.0_f64, 0.0_f64);
    for i in range(0, n) {
        let (qx, qy) = (polygon[i * 2], polygon[i * 2 + 1]);
        let p_i = previous(n, i);
        let (px, py) = (polygon[p_i * 2], polygon[p_i * 2 + 1]);
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
pub fn area(polygon: &[f64]) -> f64 {
    let (res, _) = area_centroid(polygon);
    res
}

/// Computes centroid of a simple polygon.
///
/// A simple polygon is one that does not intersect itself.
#[inline(always)]
pub fn centroid(polygon: &[f64]) -> [f64, ..2] {
    let (_, res) = area_centroid(polygon);
    res
}

/// Returns a number that tells which side it is relative to a line.
///
/// Computes the cross product of the vector that gives the line
/// with the vector between point and starting point of line.
/// One side of the line has opposite sign of the other.
#[inline(always)]
pub fn line_side(
    &Line(line): &Line,
    x: f64,
    y: f64
) -> f64 {
    let (ax, ay) = (line[0], line[1]);
    let (bx, by) = (line[2], line[3]);
    (bx - ax) * (y - ay) - (by - ay) * (x - ax)
}

/// Returns true if point is inside triangle.
///
/// This is done by computing a `side` number for each edge.
/// If the number is inside if it is on the same side for all edges.
/// Might break for very small triangles.
pub fn inside_triangle(
    &triangle: &[f64, ..6],
    x: f64,
    y: f64
) -> bool {
    let (ax, ay) = (triangle[0], triangle[1]);
    let (bx, by) = (triangle[2], triangle[3]);
    let (cx, cy) = (triangle[4], triangle[5]);

    let ab_side = line_side(&Line([ax, ay, bx, by]), x, y);
    let bc_side = line_side(&Line([bx, by, cx, cy]), x, y);
    let ca_side = line_side(&Line([cx, cy, ax, ay]), x, y);

    let ab_positive = ab_side.is_positive();
    let bc_positive = bc_side.is_positive();
    let ca_positive = ca_side.is_positive();

    ab_positive == bc_positive
    && bc_positive == ca_positive
}

/// Returns true if triangle is clockwise.
///
/// This is done by computing which side the third vertex is relative to
/// the line starting from the first vertex to second vertex.
pub fn triangle_face(
    &triangle: &[f64, ..6]
) -> bool {
    let (ax, ay) = (triangle[0], triangle[1]);
    let (bx, by) = (triangle[2], triangle[3]);
    let (cx, cy) = (triangle[4], triangle[5]);

    let ab_side = line_side(&Line([ax, ay, bx, by]), cx, cy);

    ab_side.is_negative()
}

#[test]
fn test_triangle() {
    // Triangle counter clock-wise.
    let tri_1 = [0.0, 0.0, 1.0, 0.0, 1.0, 1.0];
    // Triangle clock-wise.
    let tri_2 = [0.0, 0.0, 1.0, 1.0, 1.0, 0.0];
    let (x, y) = (0.5, 0.25);
    assert_eq!(inside_triangle(&tri_1, x, y), true);
    assert_eq!(inside_triangle(&tri_2, x, y), true);
    assert_eq!(triangle_face(&tri_1), false);
    assert_eq!(triangle_face(&tri_2), true);
}
