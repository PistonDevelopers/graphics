use vecmath::{
    relative_source_rectangle,
};
use internal::{
    CanSourceRectangle,
    HasSourceRectangle,
    SourceRectangle,
};

/// Should be implemented by contexts that have source rectangle information.
pub trait RelativeSourceRectangle<'a, T> {
    /// Adds a source rectangle.
    fn src_rect(&'a self, x: u32, y: u32, w: u32, h: u32) -> T;

    /// Moves to a relative source rectangle using the current source rectangle as tile.
    fn src_rel(&'a self, x: i64, y: i64) -> T;
}

impl<
    'a,
    T: HasSourceRectangle<'a, SourceRectangle> + CanSourceRectangle<'a, U, SourceRectangle>,
    U
> RelativeSourceRectangle<'a, U> for T {
    #[inline(always)]
    fn src_rel(&'a self, x: i64, y: i64) -> U {
        self.source_rectangle(relative_source_rectangle(*self.get_source_rectangle(), x, y))
    }

    #[inline(always)]
    fn src_rect(&'a self, x: u32, y: u32, w: u32, h: u32) -> U {
        self.source_rectangle([x, y, w, h])
    }
}

