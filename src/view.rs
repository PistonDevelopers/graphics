
use internal::{
    CanTransform,
    CanViewTransform,
    HasTransform,
    HasViewTransform,
    Matrix2d,
    Scalar,
};
use vecmath::{
    identity,
    get_scale,
};

/// Should be implemented by contexts that
/// draws something relative to view.
pub trait View {
    /// Moves the current transform to the view coordinate system.
    ///
    /// This is usually [0.0, 0.0] in the upper left corner
    /// with the x axis pointing to the right
    /// and the y axis pointing down.
    fn view(&self) -> Self;

    /// Moves the current transform to the default coordinate system.
    ///
    /// This is usually [0.0, 0.0] in the center
    /// with the x axis pointing to the right
    /// and the y axis pointing up.
    fn reset(&self) -> Self;

    /// Stores the current transform as new view.
    fn store_view(&self) -> Self;

    /// Computes the current view size.
    fn get_view_size(&self) -> (Scalar, Scalar);
}

impl<
    T: HasViewTransform<Matrix2d> 
        + HasTransform<Matrix2d>
        + CanViewTransform<T, Matrix2d>
        + CanTransform<T, Matrix2d>
> View for T {
    #[inline(always)]
    fn view(&self) -> T {
        self.transform(self.get_view_transform())
    }

    #[inline(always)]
    fn reset(&self) -> T {
        self.transform(identity())
    }

    #[inline(always)]
    fn store_view(&self) -> T {
        self.view_transform(self.get_transform())
    }

    #[inline(always)]
    fn get_view_size(&self) -> (Scalar, Scalar) {
        let scale = get_scale(self.get_view_transform());
        (2.0 / scale[0], 2.0 / scale[1])
    }
}

