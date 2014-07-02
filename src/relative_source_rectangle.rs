use vecmath::{
    relative_source_rectangle,
};
use internal::{
    CanSourceRectangle,
    HasSourceRectangle,
    SourceRectangle,
};

/// Should be implemented by contexts that 
/// have source rectangle information.
pub trait RelativeSourceRectangle<T> {
    /// Adds a source rectangle.
    fn src_rect(&self, x: i32, y: i32, w: i32, h: i32) -> T;

    /// Moves to a relative source rectangle using 
    /// the current source rectangle as tile.
    fn src_rel(&self, x: i32, y: i32) -> T;

    /// Flips the source rectangle horizontally.
    fn src_flip_h(&self) -> T;

    /// Flips the source rectangle vertically.
    fn src_flip_v(&self) -> T;

    /// Flips the source rectangle horizontally and vertically.
    fn src_flip_hv(&self) -> T;
}

impl<T: HasSourceRectangle<SourceRectangle> 
        + CanSourceRectangle<U, SourceRectangle>,
    U
> RelativeSourceRectangle<U> for T {
    #[inline(always)]
    fn src_rel(&self, x: i32, y: i32) -> U {
        self.source_rectangle(
            relative_source_rectangle(self.get_source_rectangle(), x, y)
        )
    }

    #[inline(always)]
    fn src_rect(&self, x: i32, y: i32, w: i32, h: i32) -> U {
        self.source_rectangle([x, y, w, h])
    }

    #[inline(always)]
    fn src_flip_h(&self) -> U {
        let source_rect = self.get_source_rectangle();
        self.source_rectangle([
            source_rect[0] + source_rect[2],
            source_rect[1],
            -source_rect[2],
            source_rect[3]
        ])
    }

    #[inline(always)]
    fn src_flip_v(&self) -> U {
        let source_rect = self.get_source_rectangle();
        self.source_rectangle([
            source_rect[0],
            source_rect[1] + source_rect[3],
            source_rect[2],
            -source_rect[3]
        ])
    }

    #[inline(always)]
    fn src_flip_hv(&self) -> U {
        let source_rect = self.get_source_rectangle();
        self.source_rectangle([
            source_rect[0] + source_rect[2],
            source_rect[1] + source_rect[3],
            -source_rect[2],
            -source_rect[3]
        ])
    }
}

