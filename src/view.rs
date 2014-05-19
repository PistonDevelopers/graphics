
use internal::{
    CanTransform,
    CanViewTransform,
    HasTransform,
    HasViewTransform,
    Matrix2d,
};
use vecmath::{
    identity,
};

/// Should be implemented by contexts that draws something relative to view.
pub trait View<'a> {
    /// Moves the current transform to the view coordinate system.
    ///
    /// This is usually [0.0, 0.0] in the upper left corner
    /// with the x axis pointing to the right
    /// and the y axis pointing down.
    fn view(&'a self) -> Self;

    /// Moves the current transform to the default coordinate system.
    ///
    /// This is usually [0.0, 0.0] in the center
    /// with the x axis pointing to the right
    /// and the y axis pointing up.
    fn reset(&'a self) -> Self;

    /// Stores the current transform as new view.
    fn store_view(&'a self) -> Self;
}

impl<
    'a,
    T: 
        HasViewTransform<'a, Matrix2d> 
        + HasTransform<'a, Matrix2d>
        + CanViewTransform<'a, T, Matrix2d>
        + CanTransform<'a, T, Matrix2d>
> View<'a> for T {
    #[inline(always)]
    fn view(&'a self) -> T {
        self.transform(*self.get_view_transform())
    }

    #[inline(always)]
    fn reset(&'a self) -> T {
        self.transform(identity())
    }

    #[inline(always)]
    fn store_view(&'a self) -> T {
        self.view_transform(*self.get_transform())
    }
}

