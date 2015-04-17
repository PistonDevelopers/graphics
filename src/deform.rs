//! Least square deforming of a 2D grid.

use types::Rect;
use { Line, Graphics, DrawState };
use num::Float;
use triangulation::{ tx, ty };
use math::{ Matrix2d, Scalar, Vec2d };

/// Represents a deformed grid.
#[derive(Clone)]
pub struct DeformGrid {
    /// The number of columns in the grid.
    pub cols: usize,
    /// The number of rows in the grid.
    pub rows: usize,
    /// The grid undeformed, which is a plain rectangle.
    pub rect: [Scalar; 4],
    /// The vertices, deformed.
    pub vertices: Vec<Vec2d>,
    /// The triangle indices.
    pub indices: Vec<usize>,
    /// The texture coordinates.
    pub texture_coords: Vec<[f32; 2]>,
    /// Initial position of control points.
    pub ps: Vec<[Scalar; 2]>,
    /// The current position of control points.
    pub qs: Vec<[Scalar; 2]>,
    /// A weight computation buffer, one for each control point.
    pub wis: Vec<Scalar>
}

impl DeformGrid {
    /// Creates a new DeformGrid.
    pub fn new(rect: Rect, cols: usize, rows: usize) -> DeformGrid {
        let x = rect.pos.x; let y = rect.pos.y;
        let w = rect.size.w; let h = rect.size.h;
        let mut vertices = Vec::new();
        let mut texture_coords: Vec<[f32; 2]> = Vec::new();
        let units_h = w / cols as Scalar;
        let units_v = h / rows as Scalar;
        let nx = cols + 1;
        let ny = rows + 1;
        for iy in 0..ny {
            for ix in 0..nx {
                vertices.push([
                    x + ix as Scalar * units_h,
                    y + iy as Scalar * units_v
                ]);
                texture_coords.push([
                    ix as f32 * units_h as f32 / w as f32,
                    iy as f32 * units_v as f32 / h as f32
                ]);
            }
        }

        let mut indices = Vec::new();
        for iy in 0..ny - 1 {
            for ix in 0..nx - 1 {
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
            rect: rect.to_array(),
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
    pub fn set_current(&mut self, i: usize, pos: Vec2d) {
        self.qs[i] = pos;
    }

    /// Sets original control position.
    #[inline(always)]
    pub fn set_original(&mut self, i: usize, pos: Vec2d) {
        self.ps[i] = pos;
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
        let r = self.rect;
        let units_h = r[2] / cols as Scalar;
        let units_v = r[3] / rows as Scalar;
        let nx = cols + 1;
        let ny = rows + 1;
        for iy in 0..ny {
            for ix in 0..nx {
                self.vertices.push([
                    r[0] + ix as Scalar * units_h,
                    r[1] + iy as Scalar * units_v
                ]);
                self.texture_coords.push([
                    ix as f32 * units_h as f32 / r[2] as f32,
                    iy as f32 * units_v as f32 / r[3] as f32
                ]);
            }
        }
    }

    /// Finds original coordinate.
    /// If the deformed grid is overlapping itself, multiple hits might occur.
    /// Returns the first hit it finds.
    pub fn hit(&self, pos: Vec2d) -> Option<Vec2d> {
        use math::{ inside_triangle, to_barycentric, from_barycentric };
        let nx = self.cols + 1;
        let ny = self.rows + 1;
        for i in 0..nx - 1 {
            for j in 0..ny - 1 {
                let ip = i + j * nx;
                let p1 = self.vertices[ip];
                let ip = (i + 1) + j * nx;
                let p2 = self.vertices[ip];
                let ip = i + (j + 1) * nx;
                let p3 = self.vertices[ip];
                let ip = (i + 1) + (j + 1) * nx;
                let p4 = self.vertices[ip];
                let tri1 = [p1, p2, p3];
                let tri2 = [p3, p2, p4];
                if inside_triangle(tri1, [pos[0], pos[1]]) {
                    let b = to_barycentric(tri1, pos);
                    // Upper left triangle.
                    let tri = [
                        [i as Scalar, j as Scalar],
                        [(i + 1) as Scalar, j as Scalar],
                        [i as Scalar, (j + 1) as Scalar]
                    ];
                    let tri_pos = from_barycentric(tri, b);
                    let r = self.rect;
                    let units_h = r[2] / self.cols as Scalar;
                    let units_v = r[3] / self.rows as Scalar;
                    return Some([r[0] + tri_pos[0] * units_h, r[1] + tri_pos[1] * units_v]);
                } else if inside_triangle(tri2, [pos[0], pos[1]]) {
                    let b = to_barycentric(tri2, pos);
                    // Lower right triangle.
                    let tri = [
                        [i as Scalar, (j + 1) as Scalar],
                        [(i + 1) as Scalar, j as Scalar],
                        [(i + 1) as Scalar, (j + 1) as Scalar]
                    ];
                    let tri_pos = from_barycentric(tri, b);
                    let r = self.rect;
                    let units_h = r[2] / self.cols as Scalar;
                    let units_v = r[3] / self.rows as Scalar;
                    return Some([r[0] + tri_pos[0] * units_h, r[1] + tri_pos[1] * units_v]);
                }
            }
        }
        None
    }

    /// Draws deformed image.
    pub fn draw_image<G>(
        &self,
        texture: &<G as Graphics>::Texture,
        draw_state: &DrawState,
        transform: Matrix2d,
        g: &mut G,
    )
        where G: Graphics
    {
        let mat = transform;
        let color = [1.0; 4];
        let a = color[3];
        if a == 0.0 { return; }
        let buf_len = 360;
        let mut vertices: [f32; 720] = [0.0; 720];
        let mut uvs: [f32; 720] = [0.0; 720];
        let mut offset = 0;
        let vertex_align = 2;
        let uv_align = 2;
        for &ind in self.indices.iter() {
            if offset >= buf_len {
                g.tri_list_uv(
                    &draw_state,
                    &color,
                    texture,
                    |f| f(&vertices, &uvs)
                );
                offset = 0;
            }
            let vert = self.vertices[ind];
            let vert_ind = offset * vertex_align;
            vertices[vert_ind + 0] = tx(mat, vert[0], vert[1]);
            vertices[vert_ind + 1] = ty(mat, vert[0], vert[1]);
            let uv_ind = offset * uv_align;
            let uv = self.texture_coords[ind];
            uvs[uv_ind + 0] = uv[0];
            uvs[uv_ind + 1] = uv[1];
            offset += 1;
        }
        if offset > 0 {
            g.tri_list_uv(
                &draw_state,
                &color,
                texture,
                |f| f(
                    &vertices[..offset * vertex_align],
                    &uvs[..offset * uv_align]
                )
            );
        }
    }

    /// Adds a control point, in original coordinates.
    pub fn add_control_point(&mut self, pos: Vec2d) {
        self.ps.push(pos);
        self.qs.push(pos);
        self.wis.push(0.0);
    }

    /// Draw vertical grid lines.
    pub fn draw_vertical_lines<G>(
        &self,
        line: &Line,
        draw_state: &DrawState,
        transform: Matrix2d,
        g: &mut G,
    )
        where G: Graphics
    {
        let grid = self;
        let nx = grid.cols + 1;
        let ny = grid.rows + 1;
        for i in 0..nx {
            for j in 0..ny - 1 {
                let ip = i + j * nx;
                let x1 = grid.vertices[ip][0];
                let y1 = grid.vertices[ip][1];
                let ip = i + (j + 1) * nx;
                let x2 = grid.vertices[ip][0];
                let y2 = grid.vertices[ip][1];
                line.draw([x1, y1, x2, y2], draw_state, transform, g);
            }
        }
    }

    /// Draw horizontal grid lines.
    pub fn draw_horizontal_lines<G>(
        &self,
        line: &Line,
        draw_state: &DrawState,
        transform: Matrix2d,
        g: &mut G,
    )
        where G: Graphics
    {
        let grid = self;
        let nx = grid.cols + 1;
        let ny = grid.rows + 1;
        for i in 0..nx - 1 {
            for j in 0..ny {
                let ip = i + j * nx;
                let x1 = grid.vertices[ip][0];
                let y1 = grid.vertices[ip][1];
                let ip = (i + 1) + j * nx;
                let x2 = grid.vertices[ip][0];
                let y2 = grid.vertices[ip][1];
                line.draw([x1, y1, x2, y2], draw_state, transform, g);
            }
        }
    }

    /// Updates the grid, by deforming the vertices.
    pub fn update(&mut self) {
        use math::{ add, cross, dot, mul_scalar, perp, square_len, sub };

        let &mut DeformGrid {
            cols,
            rows,
            rect,
            ref mut ps,
            ref mut qs,
            ref mut vertices,
            ref mut wis,
            ..
        } = self;
        let ps = &mut ps[..];
        let qs = &mut qs[..];
        let wis = &mut wis[..];
        let vertices = &mut vertices[..];
        let x = rect[0]; let y = rect[1];
        let w = rect[2]; let h = rect[3];
        let units_h = w / cols as Scalar;
        let units_v = h / rows as Scalar;
        let num = ps.len();
        let eps = 0.00001;
        let nx = cols + 1;
        let ny = rows + 1;

        match ps.len() {
            0 => { return; },
            1 => {
                // Move all vertices same distance.
                let d = sub(qs[0], ps[0]);
                for iy in 0..ny {
                    for ix in 0..nx {
                        let ip = ix + iy * nx;
                        vertices[ip] = [
                            x + ix as Scalar * units_h + d[0],
                            y + iy as Scalar * units_v + d[1]
                        ];
                    }
                }
                return;
            }
            _ => {}
        }

        let zero = [0.0, 0.0];
        for m in 0..nx {
            for n in 0..ny {
                let ip = m + n * nx;
                let v = [m as Scalar * units_h + x,
                         n as Scalar * units_v + y];
                let mut sum_wi = 0.0;
                let mut p_star = zero;
                let mut q_star = zero;
                for i in 0..num {
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
                let mut fr = zero;
                let vp = perp(sub(v, p_star));
                for i in 0..num {
                    let pi = ps[i];
                    let qi = qs[i];
                    let pi_hat = sub(pi, p_star);
                    let qi_hat = sub(qi, q_star);
                    let ai11 = cross(pi, vp);
                    let ai21 = dot(pi_hat, vp);
                    let ai12 = -dot(pi, vp);
                    let ai22 = cross(pi_hat, vp);
                    fr[0] += wis[i] * (qi_hat[0] * ai11 + qi_hat[1] * ai21);
                    fr[1] += wis[i] * (qi_hat[0] * ai12 + qi_hat[1] * ai22);
                }

                let vl = square_len(vp);
                let fl = square_len(fr);
                let vl = if fl == 0.0 { 0.0 } else { (vl / fl).sqrt() };
                vertices[ip] = add(mul_scalar(fr, vl), q_star);
            }
        }
    }
}
