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

/// Streams an ellipse specified by a resolution.
#[inline(always)]
pub fn with_ellipse_tri_list_xy_rgba_f32(
    resolution: uint,
    m: &Matrix2d,
    rect: &Rectangle,
    color: [f32, ..4],
    f: |vertices: &[f32], colors: &[f32]|) {
    
    let (x, y, w, h) = (rect[0], rect[1], rect[2], rect[3]);
    let (cw, ch) = (0.5 * w, 0.5 * h);
    let (cx, cy) = (x + cw, y + ch);
    let n = resolution;
    let mut i = 0u;
    stream_polygon_tri_list_xy_rgba_f32(m, || {
        if i >= n { return None; }        

        let angle = i as f64 / n as f64;
        i += 1;
        Some([cx + angle.cos() * cw, cy + angle.sin() * ch])
    }, color, f);
}

/// Streams a polygon into tri list with color per vertex.
/// Uses buffers that fit inside L1 cache.
pub fn stream_polygon_tri_list_xy_rgba_f32(
    m: &Matrix2d,
    polygon: || -> Option<[f64, ..2]>,
    color: [f32, ..4],
    f: |vertices: &[f32], colors: &[f32]|) {
    
    let mut vertices: [f32, ..740] = [0.0, ..740];
    let mut colors: [f32, ..1480] = [0.0, ..1480];
    // Draw first triangle for testing.
    // Get the first point which will be used a lot.
    let fp = match polygon() { None => return, Some(val) => val };
    let (fx, fy) = (tx(m, fp[0], fp[1]), ty(m, fp[0], fp[1]));
    let mut i = 0;
    'read_vertices: loop {
        // Set the first point in triangle to use the beginning.
        let ind_out = i * 2 * 3;
        vertices[ind_out + 0] = fx;
        vertices[ind_out + 1] = fy;
        let ind_out = i * 4 * 3;
        vertices[ind_out + 0] = color[0];
        vertices[ind_out + 1] = color[1];
        vertices[ind_out + 2] = color[2];
        vertices[ind_out + 3] = color[3];

        for k in range(1u, 3) { 
            // Copy vertex.
            let ind_out = i * 2 * 3 + k * 2;
            let p = 
                match polygon() {
                    None => break 'read_vertices,
                    Some(val) => val,
                };
            vertices[ind_out + 0] = tx(m, p[0], p[1]);
            vertices[ind_out + 1] = ty(m, p[0], p[1]);
            let ind_out = i * 4 * 3 + k * 3;
            colors[ind_out + 0] = color[0];
            colors[ind_out + 1] = color[1];
            colors[ind_out + 2] = color[2];
            colors[ind_out + 3] = color[3];
        }

        i += 1;
    }

    f(vertices.slice(0, i * 2 * 3), 
        colors.slice(0, i * 4 * 3)); 
}

/// Splits polygon into convex segments with one color per vertex.
/// Create a buffer that fits into L1 cache with 1KB overhead.
pub fn with_polygon_tri_list_xy_rgba_f32(
    m: &Matrix2d,
    polygon: &[f64],
    color: [f32, ..4],
    f: |vertices: &[f32], colors: &[f32]|) {
    
    if polygon.len() < 2 { return; }
    let mut vertices: [f32, ..740] = [0.0, ..740];
    let mut colors: [f32, ..1480] = [0.0, ..1480];
    // Draw first triangle for testing.
    let triangles = (polygon.len() - 4) / 2;
    // Get the first point which will be used a lot.
    let fx = polygon[0];
    let fy = polygon[1];
    let (fx, fy) = (tx(m, fx, fy), ty(m, fx, fy));
    for i in range(0u, triangles) {
        // Set the first point in triangle to use the beginning.
        let ind_out = i * 2 * 3;
        vertices[ind_out + 0] = fx;
        vertices[ind_out + 1] = fy;
        for k in range(1u, 3) { 
            // Copy vertex.
            let ind_in = i * 2 + k * 2;
            let ind_out = i * 2 * 3 + k * 2;
            let (x, y) = (polygon[ind_in + 0], polygon[ind_in + 1]);
            vertices[ind_out + 0] = tx(m, x, y);
            vertices[ind_out + 1] = ty(m, x, y);
        }

        for k in range(0u, 3) {
            // Copy color.
            let ind_out = i * 4 * 3 + k * 4;
            colors[ind_out + 0] = color[0];
            colors[ind_out + 1] = color[1];
            colors[ind_out + 2] = color[2];
            colors[ind_out + 3] = color[3];
        }
    }

    f(vertices.slice(0, triangles * 2 * 3), 
        colors.slice(0, triangles * 4 * 3)); 
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

