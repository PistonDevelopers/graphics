//! Methods for converting shapes into triangles.

/// Creates triangle list vertices from rectangle.
pub fn rect_tri_list_xy_f32(rect: [f32, ..4]) -> [f32, ..12] {
    let (x, y, w, h) = (rect[0], rect[1], rect[2], rect[3]);
    let x2 = x + w;
    let y2 = y + h;
    [x, y, x2, y, x, y2,
     x2, y, x2, y2, x, y2]
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

