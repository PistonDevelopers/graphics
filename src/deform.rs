//! Least square deforming of a 2D grid.

/// Represents a deformed grid.
pub struct DeformGrid {
    /// The grid undeformed, which is a plain rectangle.
    pub rect: [f64, ..4],
    /// The units to perform deform resolution.
    pub units: f64,
    /// The vertices, deformed.
    pub vertices: Vec<[f64, ..2]>,
    /// Initial position of control points.
    pub ps: Vec<[f64, ..2]>,
    /// The current position of control points.
    pub qs: Vec<[f64, ..2]>,
    /// A weight computation buffer, one for each control point.
    pub wis: Vec<f64>
}

impl DeformGrid {
    /// Updates the grid, by deforming the vertices.
    pub fn update(&mut self) {
        let &DeformGrid {
            rect,
            ref mut ps,
            ref mut qs,
            ref mut vertices,
            ref mut wis,
            units,
        } = self;
        let ps = ps.as_mut_slice();
        let qs = qs.as_mut_slice();
        let wis = wis.as_mut_slice();
        let fr = vertices.as_mut_slice();
        let x = rect[0]; let y = rect[1];
        let w = rect[2]; let h = rect[3];
        let grid_width = (w / units + 1.0).ceil() as uint;
        let grid_height = (h / units + 1.0).ceil() as uint;
        let num = ps.len();
        let eps = 0.00001; 

        for m in range(0, grid_width) {
            for n in range(0, grid_height) {
                let ip = m + n * grid_width;
                let vx = m as f64 * units + x;
                let vy = n as f64 * units + y;
                let mut sum_wi = 0.0;
                let mut p_star_x = 0.0; let mut p_star_y = 0.0;
                let mut q_star_x = 0.0; let mut q_star_y = 0.0;
                for i in range(0, num) {
                    let pix = ps[i][0]; let piy = ps[i][1];
                    let qix = qs[i][0]; let qiy = qs[i][1];
                    let vl = (pix - vx) * (pix - vx) + (piy - vy) * (piy - vy);
               
                    let w = if vl < eps && vl > -eps { 1.0 / eps } else { 1.0 / vl };
                    sum_wi += w;
                    p_star_x += w * pix; p_star_y += w * piy;
                    q_star_x += w * qix; q_star_y += w * qiy;
                    wis[i] = w;
                }

                p_star_x /= sum_wi; p_star_y /= sum_wi;
                q_star_x /= sum_wi; q_star_y /= sum_wi;
                fr[ip] = [0.0, 0.0];
                let vpx = -(vy - p_star_y); let vpy = vx - p_star_x;
                for i in range(0, num) {
                    let pix = ps[i][0]; let piy = ps[i][1];
                    let qix = qs[i][0]; let qiy = qs[i][1];
                    let pi_hat_x = pix - p_star_x; let pi_hat_y = piy - p_star_y;
                    let qi_hat_x = qix - q_star_x; let qi_hat_y = qiy - q_star_y;
                    let ai11 = pix * (vx - p_star_x) + piy * (vy - p_star_y);
                    let ai21 = pi_hat_y * (vx - p_star_x) - pi_hat_x * (vy - p_star_y);
                    let ai12 = pix * (-vpx) + piy * (-vpy);
                    let ai22 = pi_hat_y * (-vpx) - pi_hat_x * (-vpy);
                    fr[ip][0] += wis[i] * (qi_hat_x * ai11 + qi_hat_y * ai21);
                    fr[ip + 1][1] += wis[i] * (qi_hat_x * ai12 + qi_hat_y * ai22);
                }

                let vl = (vx - p_star_x) * (vx - p_star_x) + (vy - p_star_y) * (vy - p_star_y);
                let fl = fr[ip][0] * fr[ip][0] + fr[ip][1] * fr[ip][1];
                let vl = if fl == 0.0 { 0.0 } else { (vl / fl).sqrt() };
                fr[ip][0] = fr[ip][0] * vl + q_star_x;
                fr[ip][1] = fr[ip][1] * vl + q_star_y;
            }
        }
    }
}

