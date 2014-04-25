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

