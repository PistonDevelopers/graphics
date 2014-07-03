use {
    BackEnd,
    ImageSize,
    PolygonColorContext,
};
use triangulation::{
    with_polygon_tri_list_xy_f32_rgba_f32,
};

/// Implemented by contexts that can draws something using a back-end.
pub trait Draw<B: BackEnd<I>, I: ImageSize> {
    /// Draw using back-end.
    fn draw(&self, back_end: &mut B);
}

