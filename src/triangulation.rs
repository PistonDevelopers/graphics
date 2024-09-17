//! Methods for converting shapes into triangles.
use interpolation::lerp;

use crate::{
    math::{multiply, orient, translate, Matrix2d, Scalar, Vec2d},
    radians::Radians,
    types::{Line, Polygon, Polygons, Radius, Rectangle, Resolution, SourceRectangle},
    ImageSize, BACK_END_MAX_VERTEX_COUNT as BUFFER_SIZE,
};

/// Transformed x coordinate as f32.
#[inline(always)]
pub fn tx(m: Matrix2d, x: Scalar, y: Scalar) -> f32 {
    (m[0][0] * x + m[0][1] * y + m[0][2]) as f32
}

/// Transformed y coordinate as f32.
#[inline(always)]
pub fn ty(m: Matrix2d, x: Scalar, y: Scalar) -> f32 {
    (m[1][0] * x + m[1][1] * y + m[1][2]) as f32
}

/// Streams tweened polygons using linear interpolation.
#[inline(always)]
pub fn with_lerp_polygons_tri_list<F>(
    m: Matrix2d,
    polygons: Polygons<'_>,
    tween_factor: Scalar,
    f: F,
) where
    F: FnMut(&[[f32; 2]]),
{
    let poly_len = polygons.len() as Scalar;
    // Map to interval between 0 and 1.
    let tw = tween_factor % 1.0;
    // Map negative values to positive.
    let tw = if tw < 0.0 { tw + 1.0 } else { tw };
    // Map to frame.
    let tw = tw * poly_len;
    // Get the current frame.
    let frame = tw as usize;
    // Get the next frame.
    let next_frame = (frame + 1) % polygons.len();
    let p0 = polygons[frame];
    let p1 = polygons[next_frame];
    // Get factor between frames.
    let tw = tw - frame as Scalar;
    let n = polygons[0].len();
    stream_polygon_tri_list(m, (0..n).map(|j| lerp(&p0[j], &p1[j], &tw)), f);
}

/// Streams an ellipse specified by a resolution.
#[inline(always)]
pub fn with_ellipse_tri_list<F>(resolution: Resolution, m: Matrix2d, rect: Rectangle, f: F)
where
    F: FnMut(&[[f32; 2]]),
{
    let (x, y, w, h) = (rect[0], rect[1], rect[2], rect[3]);
    let (cw, ch) = (0.5 * w, 0.5 * h);
    let (cx, cy) = (x + cw, y + ch);
    let n = resolution;
    stream_polygon_tri_list(
        m,
        (0..n).map(|i| {
            let angle = i as Scalar / n as Scalar * <Scalar as Radians>::_360();
            [cx + angle.cos() * cw, cy + angle.sin() * ch]
        }),
        f,
    );
}

/// Streams a round border line.
#[inline(always)]
pub fn with_round_border_line_tri_list<F>(
    resolution_cap: Resolution,
    m: Matrix2d,
    line: Line,
    round_border_radius: Radius,
    f: F,
) where
    F: FnMut(&[[f32; 2]]),
{
    let radius = round_border_radius;
    let (x1, y1, x2, y2) = (line[0], line[1], line[2], line[3]);
    let (dx, dy) = (x2 - x1, y2 - y1);
    let w = (dx * dx + dy * dy).sqrt();
    let m = multiply(m, translate([x1, y1]));
    let m = multiply(m, orient(dx, dy));
    let n = resolution_cap * 2;
    stream_polygon_tri_list(
        m,
        (0..n).map(|j| {
            // Detect the half circle from index.
            // There is one half circle at each end of the line.
            // Together they form a full circle if
            // the length of the line is zero.
            match j {
                j if j >= resolution_cap => {
                    // Compute the angle to match start and end
                    // point of half circle.
                    // This requires an angle offset since
                    // the other end of line is the first half circle.
                    let angle = (j - resolution_cap) as Scalar / (resolution_cap - 1) as Scalar
                        * <Scalar as Radians>::_180()
                        + <Scalar as Radians>::_180();
                    // Rotate 90 degrees since the line is horizontal.
                    let angle = angle + <Scalar as Radians>::_90();
                    [w + angle.cos() * radius, angle.sin() * radius]
                }
                j => {
                    // Compute the angle to match start and end
                    // point of half circle.
                    let angle =
                        j as Scalar / (resolution_cap - 1) as Scalar * <Scalar as Radians>::_180();
                    // Rotate 90 degrees since the line is horizontal.
                    let angle = angle + <Scalar as Radians>::_90();
                    [angle.cos() * radius, angle.sin() * radius]
                }
            }
        }),
        f,
    );
}

/// Streams a round rectangle.
#[allow(clippy::identity_op)] // Identity operations are used for readibility.
#[inline(always)]
pub fn with_round_rectangle_tri_list<F>(
    resolution_corner: Resolution,
    m: Matrix2d,
    rect: Rectangle,
    round_radius: Radius,
    f: F,
) where
    F: FnMut(&[[f32; 2]]),
{
    use vecmath::traits::FromPrimitive;

    let (x, y, w, h) = (rect[0], rect[1], rect[2], rect[3]);
    let radius = round_radius;
    let n = resolution_corner * 4;
    stream_polygon_tri_list(
        m,
        (0..n).map(|j| {
            // Detect quarter circle from index.
            // There is one quarter circle at each corner.
            // Together they form a full circle if
            // each side of rectangle is 2 times the radius.
            match j {
                j if j >= resolution_corner * 3 => {
                    // Compute the angle to match start and end
                    // point of quarter circle.
                    // This requires an angle offset since this
                    // is the last quarter.
                    let angle: Scalar = (j - resolution_corner * 3) as Scalar
                        / (resolution_corner - 1) as Scalar
                        * <Scalar as Radians>::_90()
                        + <Scalar as FromPrimitive>::from_f64(3.0) * <Scalar as Radians>::_90();
                    // Set center of the circle to the last corner.
                    let (cx, cy) = (x + w - radius, y + radius);
                    [cx + angle.cos() * radius, cy + angle.sin() * radius]
                }
                j if j >= resolution_corner * 2 => {
                    // Compute the angle to match start and end
                    // point of quarter circle.
                    // This requires an angle offset since
                    // this is the second last quarter.
                    let angle = (j - resolution_corner * 2) as Scalar
                        / (resolution_corner - 1) as Scalar
                        * <Scalar as Radians>::_90()
                        + <Scalar as Radians>::_180();
                    // Set center of the circle to the second last corner.
                    let (cx, cy) = (x + radius, y + radius);
                    [cx + angle.cos() * radius, cy + angle.sin() * radius]
                }
                j if j >= resolution_corner * 1 => {
                    // Compute the angle to match start and end
                    // point of quarter circle.
                    // This requires an angle offset since
                    // this is the second quarter.
                    let angle = (j - resolution_corner) as Scalar
                        / (resolution_corner - 1) as Scalar
                        * <Scalar as Radians>::_90()
                        + <Scalar as Radians>::_90();
                    // Set center of the circle to the second corner.
                    let (cx, cy) = (x + radius, y + h - radius);
                    [cx + angle.cos() * radius, cy + angle.sin() * radius]
                }
                j => {
                    // Compute the angle to match start and end
                    // point of quarter circle.
                    let angle = j as Scalar / (resolution_corner - 1) as Scalar
                        * <Scalar as Radians>::_90();
                    // Set center of the circle to the first corner.
                    let (cx, cy) = (x + w - radius, y + h - radius);
                    [cx + angle.cos() * radius, cy + angle.sin() * radius]
                }
            }
        }),
        f,
    );
}

/// Streams a polygon into tri list.
/// Uses buffers that fit inside L1 cache.
///
/// `polygon` is a function that provides the vertices that comprise the polygon. Each
/// call to E will return a new vertex until there are none left.
///
/// `f` is a function that consumes the tri list constructed by the output of `polygon`,
/// one chunk (buffer) at a time.
///
/// Each chunk (buffer) is a fixed size array) of the format:
///
/// ```
/// //     [[x0, y0], [x1, y1], [x2, y2], [x3, y3], ... [y4, y5], ...]
/// //      ^--------------------------^  ^--------------------^
/// //        3 Points of triangle   3 points of second triangle,
/// ```
///
/// Together all the chunks comprise the full tri list. Each time the buffer size is
/// reached, that chunk is fed to `f`, then this function proceeds using a new buffer
/// until a call to `polygon` returns `None`, indicating there are no points left in
/// the polygon. (in which case the last partially filled buffer is sent to `f`)
pub fn stream_polygon_tri_list<E, F>(m: Matrix2d, mut polygon: E, mut f: F)
where
    E: Iterator<Item = Vec2d>,
    F: FnMut(&[[f32; 2]]),
{
    let mut vertices: [[f32; 2]; BUFFER_SIZE] = [[0.0; 2]; BUFFER_SIZE];
    // Get the first point which will be used a lot.
    let fp = match polygon.next() {
        None => return,
        Some(val) => val,
    };
    let f1 = [tx(m, fp[0], fp[1]), ty(m, fp[0], fp[1])];
    let gp = match polygon.next() {
        None => return,
        Some(val) => val,
    };
    let mut g1 = [tx(m, gp[0], gp[1]), ty(m, gp[0], gp[1])];
    let mut i = 0;
    let vertices_per_triangle = 3;
    let align_vertices = vertices_per_triangle;
    'read_vertices: loop {
        let ind_out = i * align_vertices;
        vertices[ind_out] = f1;

        // Copy vertex.
        let p = match polygon.next() {
            None => break 'read_vertices,
            Some(val) => val,
        };
        let pos = [tx(m, p[0], p[1]), ty(m, p[0], p[1])];

        vertices[ind_out + 1] = g1;
        vertices[ind_out + 2] = pos;
        g1 = pos;

        i += 1;
        // Buffer is full.
        if (i + 1) * align_vertices > BUFFER_SIZE {
            // Send chunk and start over.
            f(&vertices[0..i * align_vertices]);
            i = 0;
        }
    }

    if i > 0 {
        f(&vertices[0..i * align_vertices]);
    }
}

/// Streams an ellipse border specified by a resolution.
#[inline(always)]
pub fn with_ellipse_border_tri_list<F>(
    resolution: Resolution,
    m: Matrix2d,
    rect: Rectangle,
    border_radius: Radius,
    f: F,
) where
    F: FnMut(&[[f32; 2]]),
{
    let (x, y, w, h) = (rect[0], rect[1], rect[2], rect[3]);
    let (cw, ch) = (0.5 * w, 0.5 * h);
    let (cw1, ch1) = (cw + border_radius, ch + border_radius);
    let (cw2, ch2) = (cw - border_radius, ch - border_radius);
    let (cx, cy) = (x + cw, y + ch);
    let n = resolution;
    let mut i = 0;
    stream_quad_tri_list(
        m,
        || {
            if i > n {
                return None;
            }

            let angle = i as Scalar / n as Scalar * <Scalar as Radians>::_360();
            let cos = angle.cos();
            let sin = angle.sin();
            i += 1;
            Some((
                [cx + cos * cw1, cy + sin * ch1],
                [cx + cos * cw2, cy + sin * ch2],
            ))
        },
        f,
    );
}

/// Streams an arc between the two radian boundaries.
#[inline(always)]
pub fn with_arc_tri_list<F>(
    start_radians: Scalar,
    end_radians: Scalar,
    resolution: Resolution,
    m: Matrix2d,
    rect: Rectangle,
    border_radius: Radius,
    f: F,
) where
    F: FnMut(&[[f32; 2]]),
{
    let (x, y, w, h) = (rect[0], rect[1], rect[2], rect[3]);
    let (cw, ch) = (0.5 * w, 0.5 * h);
    let (cw1, ch1) = (cw + border_radius, ch + border_radius);
    let (cw2, ch2) = (cw - border_radius, ch - border_radius);
    let (cx, cy) = (x + cw, y + ch);
    let mut i = 0;

    let twopi = <Scalar as Radians>::_360();
    let max_seg_size = twopi / resolution as Scalar;

    let (start_radians, delta) = if (end_radians - start_radians).abs() >= twopi {
        // Remove overlap.
        (0.0, twopi)
    } else {
        // Take true modulus by 2pi.
        (
            start_radians,
            (((end_radians - start_radians) % twopi) + twopi) % twopi,
        )
    };

    // Taking ceiling here implies that the resolution parameter provides a
    // lower bound on the drawn resolution.
    let n_quads = (delta / max_seg_size).ceil() as u64;

    // n_quads * seg_size exactly spans the included angle.
    let seg_size = delta / n_quads as Scalar;
    stream_quad_tri_list(
        m,
        || {
            if i > n_quads {
                return None;
            }

            let angle = start_radians + (i as Scalar * seg_size);

            let cos = angle.cos();
            let sin = angle.sin();
            i += 1;
            Some((
                [cx + cos * cw1, cy + sin * ch1],
                [cx + cos * cw2, cy + sin * ch2],
            ))
        },
        f,
    );
}

/// Streams a round rectangle border.
#[allow(clippy::identity_op)] // Identity operations are used for readibility.
#[inline(always)]
pub fn with_round_rectangle_border_tri_list<F>(
    resolution_corner: Resolution,
    m: Matrix2d,
    rect: Rectangle,
    round_radius: Radius,
    border_radius: Radius,
    f: F,
) where
    F: FnMut(&[[f32; 2]]),
{
    use vecmath::traits::FromPrimitive;

    let (x, y, w, h) = (rect[0], rect[1], rect[2], rect[3]);
    let radius = round_radius;
    let radius1 = round_radius + border_radius;
    let radius2 = round_radius - border_radius;
    let n = resolution_corner * 4;
    let mut i = 0;
    stream_quad_tri_list(
        m,
        || {
            if i > n {
                return None;
            }

            let j = i;
            i += 1;
            // Detect quarter circle from index.
            // There is one quarter circle at each corner.
            // Together they form a full circle if
            // each side of rectangle is 2 times the radius.
            match j {
                j if j == n => {
                    let (cx, cy) = (x + w - radius, y + h - radius);
                    Some(([cx + radius1, cy], [cx + radius2, cy]))
                }
                j if j >= resolution_corner * 3 => {
                    // Compute the angle to match start and end
                    // point of quarter circle.
                    // This requires an angle offset since this
                    // is the last quarter.
                    let angle: Scalar = (j - resolution_corner * 3) as Scalar
                        / (resolution_corner - 1) as Scalar
                        * <Scalar as Radians>::_90()
                        + <Scalar as FromPrimitive>::from_f64(3.0) * <Scalar as Radians>::_90();
                    // Set center of the circle to the last corner.
                    let (cx, cy) = (x + w - radius, y + radius);
                    let cos = angle.cos();
                    let sin = angle.sin();
                    Some((
                        [cx + cos * radius1, cy + sin * radius1],
                        [cx + cos * radius2, cy + sin * radius2],
                    ))
                }
                j if j >= resolution_corner * 2 => {
                    // Compute the angle to match start and end
                    // point of quarter circle.
                    // This requires an angle offset since
                    // this is the second last quarter.
                    let angle = (j - resolution_corner * 2) as Scalar
                        / (resolution_corner - 1) as Scalar
                        * <Scalar as Radians>::_90()
                        + <Scalar as Radians>::_180();
                    // Set center of the circle to the second last corner.
                    let (cx, cy) = (x + radius, y + radius);
                    let cos = angle.cos();
                    let sin = angle.sin();
                    Some((
                        [cx + cos * radius1, cy + sin * radius1],
                        [cx + cos * radius2, cy + sin * radius2],
                    ))
                }
                j if j >= resolution_corner * 1 => {
                    // Compute the angle to match start and end
                    // point of quarter circle.
                    // This requires an angle offset since
                    // this is the second quarter.
                    let angle = (j - resolution_corner) as Scalar
                        / (resolution_corner - 1) as Scalar
                        * <Scalar as Radians>::_90()
                        + <Scalar as Radians>::_90();
                    // Set center of the circle to the second corner.
                    let (cx, cy) = (x + radius, y + h - radius);
                    let cos = angle.cos();
                    let sin = angle.sin();
                    Some((
                        [cx + cos * radius1, cy + sin * radius1],
                        [cx + cos * radius2, cy + sin * radius2],
                    ))
                }
                j => {
                    // Compute the angle to match start and end
                    // point of quarter circle.
                    let angle = j as Scalar / (resolution_corner - 1) as Scalar
                        * <Scalar as Radians>::_90();
                    // Set center of the circle to the first corner.
                    let (cx, cy) = (x + w - radius, y + h - radius);
                    let cos = angle.cos();
                    let sin = angle.sin();
                    Some((
                        [cx + cos * radius1, cy + sin * radius1],
                        [cx + cos * radius2, cy + sin * radius2],
                    ))
                }
            }
        },
        f,
    );
}

/// Streams a quad into tri list.
///
/// Uses buffers that fit inside L1 cache.
/// The 'quad_edge' stream returns two points
/// defining the next edge.
///
/// `quad_edge` is a function that returns two vertices, which together comprise
/// one edge of a quad
///
///
/// `f` is a function that consumes the tri list constructed by the output of
/// `quad_edge`, one chunk (buffer) at a time
///
/// The tri list is series of buffers (fixed size array) of the format:
///
/// ```
/// //     [[x0, y0], [x1, y1], [x2, y2], [x3, y3], ... [y4, y5], ...]
/// //      ^--------------------------^  ^--------------------^
/// //        3 Points of triangle   3 points of second triangle,
/// //      ^------------------------------------^          __
/// //         Two triangles together form a single quad |\\ 2|
/// //                                                   |1\\ |
/// //                                                   |__\\|
/// ```
/// Together all the chunks comprise the full tri list. Each time the buffer size is
/// reached, that chunk is fed to `f`, then this function proceeds using a new buffer
/// until a call to `quad_edge` returns `None`, indicating there are no more edges left.
/// (in which case the last partially filled buffer is sent to `f`)
#[allow(clippy::identity_op)] // Identity operations are used for readibility.
pub fn stream_quad_tri_list<E, F>(m: Matrix2d, mut quad_edge: E, mut f: F)
where
    E: FnMut() -> Option<(Vec2d, Vec2d)>,
    F: FnMut(&[[f32; 2]]),
{
    let mut vertices: [[f32; 2]; BUFFER_SIZE] = [[0.0; 2]; BUFFER_SIZE];
    // Get the two points .
    let (fp1, fp2) = match quad_edge() {
        None => return,
        Some((val1, val2)) => (val1, val2),
    };
    // Transform the points using the matrix.
    let mut f1 = [tx(m, fp1[0], fp1[1]), ty(m, fp1[0], fp1[1])];
    let mut f2 = [tx(m, fp2[0], fp2[1]), ty(m, fp2[0], fp2[1])];
    // Counts the quads.
    let mut i = 0;
    let triangles_per_quad = 2;
    let vertices_per_triangle = 3;
    let align_vertices = triangles_per_quad * vertices_per_triangle;
    loop {
        // Read two more points.
        let (gp1, gp2) = match quad_edge() {
            None => break,
            Some((val1, val2)) => (val1, val2),
        };
        // Transform the points using the matrix.
        let g1 = [tx(m, gp1[0], gp1[1]), ty(m, gp1[0], gp1[1])];
        let g2 = [tx(m, gp2[0], gp2[1]), ty(m, gp2[0], gp2[1])];
        let ind_out = i * align_vertices;

        // First triangle.
        vertices[ind_out + 0] = f1;
        vertices[ind_out + 1] = f2;
        vertices[ind_out + 2] = g1;

        // Second triangle.
        vertices[ind_out + 3] = f2;
        vertices[ind_out + 4] = g1;
        vertices[ind_out + 5] = g2;

        // Next quad.
        i += 1;

        // Set next current edge.
        f1 = g1;
        f2 = g2;

        // Buffer is full.
        if (i + 1) * align_vertices > BUFFER_SIZE {
            // Send chunk and start over.
            f(&vertices[0..i * align_vertices]);
            i = 0;
        }
    }

    if i > 0 {
        f(&vertices[0..i * align_vertices]);
    }
}

/// Splits polygon into convex segments.
/// Create a buffer that fits into L1 cache with 1KB overhead.
///
/// See stream_polygon_tri_list docs for detailed explanation.
pub fn with_polygon_tri_list<F>(m: Matrix2d, polygon: Polygon<'_>, f: F)
where
    F: FnMut(&[[f32; 2]]),
{
    stream_polygon_tri_list(m, (0..polygon.len()).map(|i| polygon[i]), f);
}

/// Creates triangle list vertices from rectangle.
#[inline(always)]
pub fn rect_tri_list_xy(m: Matrix2d, rect: Rectangle) -> [[f32; 2]; 6] {
    let (x, y, w, h) = (rect[0], rect[1], rect[2], rect[3]);
    let (x2, y2) = (x + w, y + h);
    [
        [tx(m, x, y), ty(m, x, y)],
        [tx(m, x2, y), ty(m, x2, y)],
        [tx(m, x, y2), ty(m, x, y2)],
        [tx(m, x2, y), ty(m, x2, y)],
        [tx(m, x2, y2), ty(m, x2, y2)],
        [tx(m, x, y2), ty(m, x, y2)],
    ]
}

/// Creates triangle list vertices from rectangle.
#[inline(always)]
pub fn rect_border_tri_list_xy(
    m: Matrix2d,
    rect: Rectangle,
    border_radius: Radius,
) -> [[f32; 2]; 24] {
    let (x, y, w, h) = (rect[0], rect[1], rect[2], rect[3]);
    let (w1, h1) = (w + border_radius, h + border_radius);
    let (w2, h2) = (w - border_radius, h - border_radius);
    let (x11, y11) = (x - border_radius, y - border_radius);
    let (x21, y21) = (x + border_radius, y + border_radius);
    let (x12, y12) = (x + w1, y + h1);
    let (x22, y22) = (x + w2, y + h2);
    [
        [tx(m, x11, y11), ty(m, x11, y11)],
        [tx(m, x12, y11), ty(m, x12, y11)],
        [tx(m, x21, y21), ty(m, x21, y21)],
        [tx(m, x21, y21), ty(m, x21, y21)],
        [tx(m, x12, y11), ty(m, x12, y11)],
        [tx(m, x22, y21), ty(m, x22, y21)],
        [tx(m, x22, y21), ty(m, x22, y21)],
        [tx(m, x12, y11), ty(m, x12, y11)],
        [tx(m, x12, y12), ty(m, x12, y12)],
        [tx(m, x22, y21), ty(m, x22, y21)],
        [tx(m, x12, y12), ty(m, x12, y12)],
        [tx(m, x22, y22), ty(m, x22, y22)],
        [tx(m, x12, y12), ty(m, x12, y12)],
        [tx(m, x22, y22), ty(m, x22, y22)],
        [tx(m, x11, y12), ty(m, x11, y12)],
        [tx(m, x22, y22), ty(m, x22, y22)],
        [tx(m, x11, y12), ty(m, x11, y12)],
        [tx(m, x21, y22), ty(m, x21, y22)],
        [tx(m, x11, y12), ty(m, x11, y12)],
        [tx(m, x21, y21), ty(m, x21, y21)],
        [tx(m, x21, y22), ty(m, x21, y22)],
        [tx(m, x11, y12), ty(m, x11, y12)],
        [tx(m, x11, y11), ty(m, x11, y11)],
        [tx(m, x21, y21), ty(m, x21, y21)],
    ]
}

/// Creates triangle list texture coords from image.
#[inline(always)]
pub fn rect_tri_list_uv<I: ImageSize>(image: &I, source_rect: SourceRectangle) -> [[f32; 2]; 6] {
    let (w, h) = image.get_size();
    let (src_x, src_y, src_w, src_h) = (
        source_rect[0],
        source_rect[1],
        source_rect[2],
        source_rect[3],
    );

    let x1 = src_x as f32 / w as f32;
    let y1 = src_y as f32 / h as f32;
    let x2 = (src_w + src_x) as f32 / w as f32;
    let y2 = (src_h + src_y) as f32 / h as f32;
    [[x1, y1], [x2, y1], [x1, y2], [x2, y1], [x2, y2], [x1, y2]]
}
