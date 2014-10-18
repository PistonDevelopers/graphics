//! Least square deforming of a 2D grid.

use {
    BackEnd,
    ImageSize,
    ColorContext,
    Context,
    AddLine,
    AddSquareBorder,
    Draw,
};
use triangulation::{tx, ty};

/// Represents a deformed grid.
pub struct DeformGrid {
    /// The number of columns in the grid.
    pub cols: uint,
    /// The number of rows in the grid.
    pub rows: uint,
    /// The grid undeformed, which is a plain rectangle.
    pub rect: [f64, ..4],
    /// The vertices, deformed.
    pub vertices: Vec<[f64, ..2]>,
    /// The triangle indices.
    pub indices: Vec<uint>,
    /// The texture coordinates.
    pub texture_coords: Vec<[f32, ..2]>,
    /// Initial position of control points.
    pub ps: Vec<[f64, ..2]>,
    /// The current position of control points.
    pub qs: Vec<[f64, ..2]>,
    /// A weight computation buffer, one for each control point.
    pub wis: Vec<f64>
}

impl DeformGrid {
    /// Creates a new DeformGrid.
    pub fn new(rect: [f64, ..4], cols: uint, rows: uint) -> DeformGrid {
        let x = rect[0]; let y = rect[1];
        let w = rect[2]; let h = rect[3];
        let mut vertices = Vec::new();
        let mut texture_coords: Vec<[f32, ..2]> = Vec::new();
        let units_h = w / cols as f64;
        let units_v = h / rows as f64;
        let nx = cols + 1;
        let ny = rows + 1;
        for iy in range(0, ny) {
            for ix in range(0, nx) {
                vertices.push([
                    x + ix as f64 * units_h, 
                    y + iy as f64 * units_v
                ]);
                texture_coords.push([
                    ix as f32 * units_h as f32 / w as f32, 
                    iy as f32 * units_v as f32 / h as f32
                ]);
            }
        }

        let mut indices = Vec::new();
        for iy in range(0, ny - 1) {
            for ix in range(0, nx - 1) {
                indices.push(ix + iy * nx);
                indices.push((ix + 1) + iy * nx);
                indices.push(ix + (iy + 1) * nx);

                indices.push(ix + (iy + 1) * nx);
                indices.push((ix + 1) + iy * nx);
                indices.push((ix + 1) + (iy + 1) * nx);
            }
        }

        DeformGrid {
            cols: cols,
            rows: rows,
            rect: rect,
            vertices: vertices,
            indices: indices,
            texture_coords: texture_coords,
            ps: Vec::new(),
            qs: Vec::new(),
            wis: Vec::new(),
        }
    }

    /// Sets current control position.
    #[inline(always)]
    pub fn set_current(&mut self, i: uint, pos: [f64, ..2]) {
        let ptr = self.qs.get_mut(i);
        *ptr = pos;
    }
    
    /// Sets original control position.
    #[inline(always)]
    pub fn set_original(&mut self, i: uint, pos: [f64, ..2]) {
        let ptr = self.ps.get_mut(i);
        *ptr = pos;
    }

    /// Removes all control points.
    pub fn reset_control_points(&mut self) {
        // These values don't need drop, so we can set length directly.
        unsafe {
            self.ps.set_len(0);
            self.qs.set_len(0);
            self.wis.set_len(0);
        }
    }

    /// Sets vertices and texture coords back to default.
    pub fn reset_vertices_and_texture_coords(&mut self) { 
        unsafe {
            self.vertices.set_len(0);
            self.texture_coords.set_len(0);
        }

        let cols = self.cols;
        let rows = self.rows;
        let (x, y, w, h) = self.get_rect();
        let units_h = w / cols as f64;
        let units_v = h / rows as f64;
        let nx = cols + 1;
        let ny = rows + 1;
        for iy in range(0, ny) {
            for ix in range(0, nx) {
                self.vertices.push([
                    x + ix as f64 * units_h, 
                    y + iy as f64 * units_v
                ]);
                self.texture_coords.push([
                    ix as f32 * units_h as f32 / w as f32, 
                    iy as f32 * units_v as f32 / h as f32
                ]);
            }
        }
    }

    /// Gets the original rectangle as a tuple.
    #[inline(always)]
    pub fn get_rect(&self) -> (f64, f64, f64, f64) {
        (self.rect[0], self.rect[1], self.rect[2], self.rect[3])
    }

    /// Finds original coordinate.
    /// If the deformed grid is overlapping itself, multiple hits might occur.
    /// Returns the first hit it finds.
    pub fn hit(&self, pos: [f64, ..2]) -> Option<[f64, ..2]> {
        use vecmath::{ inside_triangle, to_barycentric, from_barycentric };
        let nx = self.cols + 1;
        let ny = self.rows + 1;
        for i in range(0, nx - 1) {
            for j in range(0, ny - 1) {
                let ip = i + j * nx;
                let p1 = self.vertices[ip];
                let ip = (i + 1) + j * nx;
                let p2 = self.vertices[ip];
                let ip = i + (j + 1) * nx;
                let p3 = self.vertices[ip];
                let ip = (i + 1) + (j + 1) * nx;
                let p4 = self.vertices[ip];
                let tri1 = [p1[0], p1[1],  p2[0], p2[1],  p3[0], p3[1]];
                let tri2 = [p3[0], p3[1],  p2[0], p2[1],  p4[0], p4[1]];
                if inside_triangle(tri1, pos[0], pos[1]) {
                    let b = to_barycentric(tri1, pos);
                    // Upper left triangle.
                    let tri = [
                        i as f64, j as f64,
                        (i + 1) as f64, j as f64,
                        i as f64, (j + 1) as f64
                    ];
                    let tri_pos = from_barycentric(tri, b);
                    let (rx, ry, w, h) = self.get_rect();
                    let units_h = w / self.cols as f64;
                    let units_v = h / self.rows as f64;
                    return Some([rx + tri_pos[0] * units_h, ry + tri_pos[1] * units_v]);
                } else if inside_triangle(tri2, pos[0], pos[1]) {
                    let b = to_barycentric(tri2, pos);
                    // Lower right triangle.
                    let tri = [
                        i as f64, (j + 1) as f64,
                        (i + 1) as f64, j as f64,
                        (i + 1) as f64, (j + 1) as f64
                    ];
                    let tri_pos = from_barycentric(tri, b);
                    let (rx, ry, w, h) = self.get_rect();
                    let units_h = w / self.cols as f64;
                    let units_v = h / self.rows as f64;
                    return Some([rx + tri_pos[0] * units_h, ry + tri_pos[1] * units_v]);
                }
            }
        }
        None
    }

    /// Draws deformed image.
    pub fn draw_image<B: BackEnd<I>, I: ImageSize>(
        &self,
        c: &Context,
        back_end: &mut B,
        texture: &I
    ) {
        if !back_end.supports_single_texture() { return; }
        if !back_end.supports_tri_list_xy_f32_rgba_f32_uv_f32() { return; }
        let mat = c.transform;
        let (r, g, b, a) = (1.0, 1.0, 1.0, 1.0);
        let needs_alpha = a != 1.0
            || back_end.has_texture_alpha(texture);
        if needs_alpha { back_end.enable_alpha_blend(); }
        back_end.enable_single_texture(texture);
        let buf_len = 360;
        let mut vertices: [f32, ..720] = [0.0, ..720];
        let mut colors: [f32, ..1440] = [0.0, ..1440];
        let mut uvs: [f32, ..720] = [0.0, ..720];
        let mut offset = 0;
        let vertex_align = 2;
        let color_align = 4;
        let uv_align = 2;
        for &ind in self.indices.iter() {
            if offset >= buf_len {
                back_end.tri_list_xy_f32_rgba_f32_uv_f32(
                    vertices.as_slice(),
                    colors.as_slice(),
                    uvs.as_slice()
                );
                offset = 0;
            }
            let vert = self.vertices[ind];
            let vert_ind = offset * vertex_align;
            vertices[vert_ind + 0] = tx(mat, vert[0], vert[1]);
            vertices[vert_ind + 1] = ty(mat, vert[0], vert[1]);
            let color_ind = offset * color_align;
            colors[color_ind + 0] = r;
            colors[color_ind + 1] = g;
            colors[color_ind + 2] = b;
            colors[color_ind + 3] = a;
            let uv_ind = offset * uv_align;
            let uv = self.texture_coords[ind];
            uvs[uv_ind + 0] = uv[0];
            uvs[uv_ind + 1] = uv[1];
            offset += 1;
        }
        if offset > 0 {
            back_end.tri_list_xy_f32_rgba_f32_uv_f32(
                vertices.slice_to(offset * vertex_align),
                colors.slice_to(offset * color_align),
                uvs.slice_to(offset * uv_align)
            );
        }
        back_end.disable_single_texture();
        if needs_alpha { back_end.disable_alpha_blend(); }
    }

    /// Adds a control point, in original coordinates.
    pub fn add_control_point(&mut self, pos: [f64, ..2]) {
        self.ps.push(pos);
        self.qs.push(pos);
        self.wis.push(0.0);
    }

    /// Draw vertical grid lines.
    pub fn draw_vertical_lines<B: BackEnd<I>, I: ImageSize>(
        &self, 
        c: &ColorContext,
        back_end: &mut B,
        border_width: f64,
    ) {
        let grid = self;
        let nx = grid.cols + 1;
        let ny = grid.rows + 1;
        for i in range(0, nx) {
            for j in range(0, ny - 1) {
                let ip = i + j * nx;
                let x1 = grid.vertices[ip][0];
                let y1 = grid.vertices[ip][1];
                let ip = i + (j + 1) * nx;
                let x2 = grid.vertices[ip][0];
                let y2 = grid.vertices[ip][1];
                c.line(x1, y1, x2, y2)
                .square_border_width(border_width)
                .draw(back_end);
            }
        }
    }
    
    /// Draw horizontal grid lines.
    pub fn draw_horizontal_lines<B: BackEnd<I>, I: ImageSize>(
        &self, 
        c: &ColorContext,
        back_end: &mut B,
        border_width: f64,
    ) {
        let grid = self;
        let nx = grid.cols + 1;
        let ny = grid.rows + 1;
        for i in range(0, nx - 1) {
            for j in range(0, ny) {
                let ip = i + j * nx;
                let x1 = grid.vertices[ip][0];
                let y1 = grid.vertices[ip][1];
                let ip = (i + 1) + j * nx;
                let x2 = grid.vertices[ip][0];
                let y2 = grid.vertices[ip][1];
                c.line(x1, y1, x2, y2)
                .square_border_width(border_width)
                .draw(back_end);
            }
        }
    }

    /// Updates the grid, by deforming the vertices.
    pub fn update(&mut self) {
        use vecmath::{ add, mul_scalar, perp, square_len, sub };

        let &DeformGrid {
            cols,
            rows,
            rect,
            ref mut ps,
            ref mut qs,
            ref mut vertices,
            ref mut wis,
            ..
        } = self;
        let ps = ps.as_mut_slice();
        let qs = qs.as_mut_slice();
        let wis = wis.as_mut_slice();
        let vertices = vertices.as_mut_slice();
        let x = rect[0]; let y = rect[1];
        let w = rect[2]; let h = rect[3];
        let units_h = w / cols as f64;
        let units_v = h / rows as f64;
        let num = ps.len();
        let eps = 0.00001; 
        let nx = cols + 1;
        let ny = rows + 1;

        match ps.len() {
            0 => { return; },
            1 => {
                // Move all vertices same distance.
                let dx = qs[0][0] - ps[0][0];
                let dy = qs[0][1] - ps[0][1];
                for iy in range(0, ny) {
                    for ix in range(0, nx) {
                        let ip = ix + iy * nx;
                        vertices[ip] = [
                            x + ix as f64 * units_h + dx, 
                            y + iy as f64 * units_v + dy
                        ];
                    }
                }
                return;
            }
            _ => {}
        }

        for m in range(0, nx) {
            for n in range(0, ny) {
                let ip = m + n * nx;
                let v = [m as f64 * units_h + x,
                         n as f64 * units_v + y];
                let mut sum_wi = 0.0;
                let mut p_star = [0.0, 0.0];
                let mut q_star = [0.0, 0.0];
                for i in range(0, num) {
                    let pi = ps[i];
                    let vl = square_len(sub(pi, v));
                    let w = if vl < eps && vl > -eps { 1.0 / eps } else { 1.0 / vl };
                    sum_wi += w;
                    p_star = add(p_star, mul_scalar(pi, w));
                    q_star = add(q_star, mul_scalar(qs[i], w));
                    wis[i] = w;
                }

                let inv_sum_wi = 1.0 / sum_wi;
                p_star = mul_scalar(p_star, inv_sum_wi);
                q_star = mul_scalar(q_star, inv_sum_wi);
                let mut fr = [0.0, 0.0];
                let vp = perp(sub(v, p_star));
                for i in range(0, num) {
                    let pi = ps[i];
                    let qi = qs[i];
                    let pi_hat_x = pi[0] - p_star[0]; let pi_hat_y = pi[1] - p_star[1];
                    let qi_hat_x = qi[0] - q_star[0]; let qi_hat_y = qi[1] - q_star[1];
                    let ai11 = pi[0] * vp[1] - pi[1] * vp[0];
                    let ai21 = pi_hat_y * vp[1] + pi_hat_x * vp[0];
                    let ai12 = pi[0] * (-vp[0]) - pi[1] * vp[1];
                    let ai22 = pi_hat_y * (-vp[0]) + pi_hat_x * vp[1];
                    fr[0] += wis[i] * (qi_hat_x * ai11 + qi_hat_y * ai21);
                    fr[1] += wis[i] * (qi_hat_x * ai12 + qi_hat_y * ai22);
                }

                let vl = square_len(vp);
                let fl = square_len(fr);
                let vl = if fl == 0.0 { 0.0 } else { (vl / fl).sqrt() };
                vertices[ip] = add(mul_scalar(fr, vl), q_star);
            }
        }
    }
}

