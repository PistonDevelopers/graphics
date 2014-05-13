//! Methods for converting shapes into triangles.

use std;
use {
    Image,
    Matrix2d,
    Rectangle,
};
use interpolation::{lerp};

#[inline(always)]
fn tx(m: &Matrix2d, x: f64, y: f64) -> f32 {
    (m[0] * x + m[1] * y + m[2]) as f32
}

#[inline(always)]
fn ty(m: &Matrix2d, x: f64, y: f64) -> f32 {
    (m[3] * x + m[4] * y + m[5]) as f32
}

/// Streams tweened polygons using linear interpolation.
#[inline(always)]
pub fn with_lerp_polygons_tri_list_xy_f32_rgba_f32(
    m: &Matrix2d,
    polygons: &[&[f64]],
    tween_factor: f64,
    color: [f32, ..4],
    f: |vertices: &[f32], colors: &[f32]|) {

    let poly_len = polygons.len() as f64;
    // Map to interval between 0 and 1.
    let tw = tween_factor % 1.0;
    // Map negative values to positive.
    let tw = if tw < 0.0 { tw + 1.0 } else { tw };
    // Map to frame.
    let tw = tw * poly_len;
    // Get the current frame.
    let frame = tw as uint;
    // Get the next frame.
    let next_frame = (frame + 1) % polygons.len();
    let p0 = polygons[frame];
    let p1 = polygons[next_frame];
    // Get factor between frames.
    let tw = tw - frame as f64;
    let n = polygons[0].len();
    let mut i = 0u;
    stream_polygon_tri_list_xy_f32_rgba_f32(m, || {
        if i >= n { return None; }

        let j = i;
        i += 2;
        let (x0, y0) = (p0[j], p0[j+1]);
        let (x1, y1) = (p1[j], p1[j+1]);
        Some([lerp(&x0, &x1, &tw), lerp(&y0, &y1, &tw)])
    }, color, f);
}

/// Streams an ellipse specified by a resolution.
#[inline(always)]
pub fn with_ellipse_tri_list_xy_f32_rgba_f32(
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
    stream_polygon_tri_list_xy_f32_rgba_f32(m, || {
        if i >= n { return None; }

        let angle = i as f64 / n as f64 * std::f64::consts::PI_2;
        i += 1;
        Some([cx + angle.cos() * cw, cy + angle.sin() * ch])
    }, color, f);
}

/// Streams a round rectangle.
#[inline(always)]
pub fn with_round_rectangle_tri_list_xy_f32_rgba_f32(
    resolution_corner: uint,
    m: &Matrix2d,
    rect: &Rectangle,
    round_radius: &f64,
    color: [f32, ..4],
    f: |vertices: &[f32], colors: &[f32]|) {

    let (x, y, w, h) = (rect[0], rect[1], rect[2], rect[3]);
    let radius = *round_radius;
    let n = resolution_corner * 4 + 4;
    let mut i = 0u;
    stream_polygon_tri_list_xy_f32_rgba_f32(m, || {
        if i >= n { return None; }

        let j = i;
        i += 1;
        match j {
            j if j >= resolution_corner * 3 => {
                let angle = j as f64 / (n - 3) as f64 * std::f64::consts::PI_2;
                let (cx, cy) = (x + w - radius, y + radius);
                Some([cx + angle.cos() * radius, cy + angle.sin() * radius])
            },
            j if j >= resolution_corner * 2 => {
                let angle = j as f64 / (n - 2) as f64 * std::f64::consts::PI_2;
                let (cx, cy) = (x + radius, y + radius);
                Some([cx + angle.cos() * radius, cy + angle.sin() * radius])
            },
            j if j >= resolution_corner * 1 => {
                let angle = j as f64 / (n - 1) as f64 * std::f64::consts::PI_2;
                let (cx, cy) = (x + radius, y + h - radius);
                Some([cx + angle.cos() * radius, cy + angle.sin() * radius])
            },
            j => {
                let angle = j as f64 / (n - 0) as f64 * std::f64::consts::PI_2;
                let (cx, cy) = (x + w - radius, y + h - radius);
                Some([cx + angle.cos() * radius, cy + angle.sin() * radius])
            },
        }
    }, color, f);
}

/// Streams a polygon into tri list with color per vertex.
/// Uses buffers that fit inside L1 cache.
pub fn stream_polygon_tri_list_xy_f32_rgba_f32(
    m: &Matrix2d,
    polygon: || -> Option<[f64, ..2]>,
    color: [f32, ..4],
    f: |vertices: &[f32], colors: &[f32]|) {

    let mut vertices: [f32, ..740] = [0.0, ..740];
    let mut colors: [f32, ..1480] = [0.0, ..1480];
    // Get the first point which will be used a lot.
    let fp = match polygon() { None => return, Some(val) => val };
    let (fx, fy) = (tx(m, fp[0], fp[1]), ty(m, fp[0], fp[1]));
    let gp = match polygon() { None => return, Some(val) => val };
    let (gx, gy) = (tx(m, gp[0], gp[1]), ty(m, gp[0], gp[1]));
    let mut gx = gx;
    let mut gy = gy;
    let mut i = 0;
    'read_vertices: loop {
        let ind_out = i * 2 * 3;
        vertices[ind_out + 0] = fx;
        vertices[ind_out + 1] = fy;
        let ind_out = i * 4 * 3;
        colors[ind_out + 0] = color[0];
        colors[ind_out + 1] = color[1];
        colors[ind_out + 2] = color[2];
        colors[ind_out + 3] = color[3];

        // Copy vertex.
        let ind_out = i * 2 * 3 + 2;
        let p =
            match polygon() {
                None => break 'read_vertices,
                Some(val) => val,
            };
        let x = tx(m, p[0], p[1]);
        let y = ty(m, p[0], p[1]);

        vertices[ind_out + 0] = gx;
        vertices[ind_out + 1] = gy;
        vertices[ind_out + 2] = x;
        vertices[ind_out + 3] = y;
        gx = x;
        gy = y;
        let ind_out = i * 4 * 3 + 4;
        colors[ind_out + 0] = color[0];
        colors[ind_out + 1] = color[1];
        colors[ind_out + 2] = color[2];
        colors[ind_out + 3] = color[3];
        colors[ind_out + 4] = color[0];
        colors[ind_out + 5] = color[1];
        colors[ind_out + 6] = color[2];
        colors[ind_out + 7] = color[3];

        i += 1;
        // Buffer is full.
        if i * 2 * 3 + 2 == vertices.len() {
            // Send chunk and start over.
            f(vertices.slice(0, i * 2 * 3),
                colors.slice(0, i * 4 * 3));
            i = 0;
        }
    }

    if i > 0 {
        f(vertices.slice(0, i * 2 * 3),
            colors.slice(0, i * 4 * 3));
    }
}

/// Splits polygon into convex segments with one color per vertex.
/// Create a buffer that fits into L1 cache with 1KB overhead.
pub fn with_polygon_tri_list_xy_f32_rgba_f32(
    m: &Matrix2d,
    polygon: &[f64],
    color: [f32, ..4],
    f: |vertices: &[f32], colors: &[f32]|) {

    let n = polygon.len();
    let mut i = 0;
    stream_polygon_tri_list_xy_f32_rgba_f32(
        m, || {
            if i >= n { return None; }

            let j = i;
            i += 2;
            Some([polygon[j], polygon[j+1]])
        }, color, f);
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

/// Creates triangle list texture coords from image.
pub fn rect_tri_list_uv_f32(image: &Image) -> [f32, ..12] {
    let x1 = image.source_rect[0] as f32 / image.texture_width as f32;
    let y1 = image.source_rect[1] as f32 / image.texture_height as f32;
    let x2 = (image.source_rect[0] + image.source_rect[2]) as f32 / image.texture_width as f32;
    let y2 = (image.source_rect[1] + image.source_rect[3]) as f32 / image.texture_height as f32;
    [x1, y1, x2, y1, x1, y2,
     x2, y1, x2, y2, x1, y2]
}

