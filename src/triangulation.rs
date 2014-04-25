//! Methods for converting shapes into triangles.

use {Matrix2d, Rectangle};

#[inline(always)]
fn tx(m: &Matrix2d, x: f64, y: f64) -> f32 {
    (m[0] * x + m[1] * y + m[2]) as f32
}

#[inline(always)]
fn ty(m: &Matrix2d, x: f64, y: f64) -> f32 {
    (m[3] * x + m[4] * y + m[5]) as f32
}

/// Splits polygon into convex segments with one color per vertex.
/// Create a buffer that fits into L1 cache with 1KB overhead.
pub fn with_polygon_tri_list_xy_rgba_f32(
    m: &Matrix2d,
    polygon: &[f64],
    color: [f32, ..4],
    f: |vertices: &[f32], colors: &[f32]|) {
    let mut vertices: [f32, ..740] = [0.0, ..740];
    let mut colors: [f32, ..1480] = [0.0, ..1480];
    // Draw first triangle for testing.
    vertices[0] = tx(m, polygon[0], polygon[1]);
    vertices[1] = ty(m, polygon[0], polygon[1]);
    vertices[2] = tx(m, polygon[2], polygon[3]);
    vertices[3] = ty(m, polygon[2], polygon[3]);
    vertices[4] = tx(m, polygon[4], polygon[5]);
    vertices[5] = ty(m, polygon[4], polygon[5]);
    for i in range(0u, 3) {
        for (j, &c) in color.iter().enumerate() {
            colors[i * 4 + j] = c;
        }
    }

    f(vertices.slice(0, 6), colors.slice(0, 12)); 
}

/// Creates triangle list vertices from rectangle.
pub fn rect_tri_list_xy_f32(m: &Matrix2d, rect: &Rectangle) -> [f32, ..12] {
    let (x, y, w, h) = (rect[0], rect[1], rect[2], rect[3]);
    let (x2, y2) = (x + w, y + h);
    [tx(m,x,y), ty(m,x,y), tx(m,x2,y), ty(m,x2,y), tx(m,x,y2), ty(m,x,y2),
     tx(m,x2,y), ty(m,x2,y), tx(m,x2,y2), ty(m,x2,y2), tx(m,x,y2), ty(m,x,y2)]
}

/// Creates triangle list colors from rectangle.
pub fn rect_tri_list_rgba_f32(color: [f32, ..4]) -> [f32, ..48] {
    let (r, g, b, a) = (color[0], color[1], color[2], color[3]);
    [r, g, b, a, // 0
     r, g, b, a, // 1
     r, g, b, a, // 2
     r, g, b, a, // 3
     r, g, b, a, // 4
     r, g, b, a, // 5
     r, g, b, a, // 6
     r, g, b, a, // 7
     r, g, b, a, // 8
     r, g, b, a, // 9
     r, g, b, a, // 10
     r, g, b, a]
}

