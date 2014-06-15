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
    fn src_rect(&'a self, x: i32, y: i32, w: i32, h: i32) -> T;

    /// Moves to a relative source rectangle using the current source rectangle as tile.
    fn src_rel(&'a self, x: i32, y: i32) -> T;

    /// Flips the source rectangle horizontally.
    fn src_flip_h(&'a self) -> T;

    /// Flips the source rectangle vertically.
    fn src_flip_v(&'a self) -> T;

    /// Flips the source rectangle horizontally and vertically.
    fn src_flip_hv(&'a self) -> T;
}

impl<
    'a,
    T: HasSourceRectangle<'a, SourceRectangle> + CanSourceRectangle<'a, U, SourceRectangle>,
    U
> RelativeSourceRectangle<'a, U> for T {
    #[inline(always)]
    fn src_rel(&'a self, x: i32, y: i32) -> U {
        self.source_rectangle(relative_source_rectangle(*self.get_source_rectangle(), x, y))
    }

    #[inline(always)]
    fn src_rect(&'a self, x: i32, y: i32, w: i32, h: i32) -> U {
        self.source_rectangle([x, y, w, h])
    }

    #[inline(always)]
    fn src_flip_h(&'a self) -> U {
        let source_rect = self.get_source_rectangle();
        self.source_rectangle([
            source_rect[0] + source_rect[2],
            source_rect[1],
            -source_rect[2],
            source_rect[3]
        ])
    }

    #[inline(always)]
    fn src_flip_v(&'a self) -> U {
        let source_rect = self.get_source_rectangle();
        self.source_rectangle([
            source_rect[0],
            source_rect[1] + source_rect[3],
            source_rect[2],
            -source_rect[3]
        ])
    }

    #[inline(always)]
    fn src_flip_hv(&'a self) -> U {
        let source_rect = self.get_source_rectangle();
        self.source_rectangle([
            source_rect[0] + source_rect[2],
            source_rect[1] + source_rect[3],
            -source_rect[2],
            -source_rect[3]
        ])
    }
}

