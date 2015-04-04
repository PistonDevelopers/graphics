use math::{
    margin_rectangle,
    relative_rectangle,
    relative_source_rectangle,
    Scalar,
};
use internal::Rectangle;

/// Should be implemented by contexts that have rectangle information.
pub trait Rectangled: Sized {
    /// Shrinks the current rectangle equally by all sides.
    fn margin(self, m: Scalar) -> Self;

    /// Expands the current rectangle equally by all sides.
    #[inline(always)]
    fn expand(self, m: Scalar) -> Self {
        self.margin(-m)
    }

    /// Moves to a relative rectangle using the current rectangle as tile.
    fn rel(self, x: Scalar, y: Scalar) -> Self;
}

/*
impl<T: Get<Rect> + Set<Rect> + Clone> Rectangled for T {
    #[inline(always)]
    fn margin(&self, m: Scalar) -> Self {
        let Rect(val) = self.get();
        self.clone().set(Rect(margin_rectangle(val, m)))
    }

    #[inline(always)]
    fn rel(&self, x: Scalar, y: Scalar) -> Self {
        let Rect(val) = self.get();
        self.clone().set(Rect(relative_rectangle(val, [x, y])))
    }
}
*/

/// Should be implemented by contexts that
/// have source rectangle information.
pub trait SourceRectangled {
    /// Adds a source rectangle.
    fn src_rect(&self, x: i32, y: i32, w: i32, h: i32) -> Self;

    /// Moves to a relative source rectangle using
    /// the current source rectangle as tile.
    fn src_rel(&self, x: i32, y: i32) -> Self;

    /// Flips the source rectangle horizontally.
    fn src_flip_h(&self) -> Self;

    /// Flips the source rectangle vertically.
    fn src_flip_v(&self) -> Self;

    /// Flips the source rectangle horizontally and vertically.
    fn src_flip_hv(&self) -> Self;
}

/*
impl<T: Get<SrcRect>
      + Set<SrcRect>
      + Clone
> SourceRectangled for T {
    #[inline(always)]
    fn src_rect(&self, x: i32, y: i32, w: i32, h: i32) -> Self {
        self.clone().set(SrcRect([x, y, w, h]))
    }

    #[inline(always)]
    fn src_rel(&self, x: i32, y: i32) -> Self {
        let SrcRect(val) = self.get();
        self.clone().set(SrcRect(
            relative_source_rectangle(val, x, y)
        ))
    }

    #[inline(always)]
    fn src_flip_h(&self) -> Self {
        let SrcRect(source_rect) = self.get();
        self.clone().set(SrcRect([
            source_rect[0] + source_rect[2],
            source_rect[1],
            -source_rect[2],
            source_rect[3]
        ]))
    }

    #[inline(always)]
    fn src_flip_v(&self) -> Self {
        let SrcRect(source_rect) = self.get();
        self.clone().set(SrcRect([
            source_rect[0],
            source_rect[1] + source_rect[3],
            source_rect[2],
            -source_rect[3]
        ]))
    }

    #[inline(always)]
    fn src_flip_hv(&self) -> Self {
        let SrcRect(source_rect) = self.get();
        self.clone().set(SrcRect([
            source_rect[0] + source_rect[2],
            source_rect[1] + source_rect[3],
            -source_rect[2],
            -source_rect[3]
        ]))
    }
}
*/
