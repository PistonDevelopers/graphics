
use can::{
    CanTransform,
    CanViewTransform,
};
use has::{
    HasTransform,
    HasViewTransform,
};
use vecmath::{
    identity,
    get_scale,
    Scalar,
};

/// Should be implemented by contexts that
/// draws something relative to view.
pub trait View:
    HasViewTransform + CanViewTransform
  + HasTransform + CanTransform
{
    /// Moves the current transform to the view coordinate system.
    ///
    /// This is usually [0.0, 0.0] in the upper left corner
    /// with the x axis pointing to the right
    /// and the y axis pointing down.
    #[inline(always)]
    fn view(&self) -> Self {
        self.transform(self.get_view_transform())
    }

    /// Moves the current transform to the default coordinate system.
    ///
    /// This is usually [0.0, 0.0] in the center
    /// with the x axis pointing to the right
    /// and the y axis pointing up.
    #[inline(always)]
    fn reset(&self) -> Self {
        self.transform(identity())
    }

    /// Stores the current transform as new view.
    #[inline(always)]
    fn store_view(&self) -> Self {
        self.view_transform(self.get_transform())
    }

    /// Computes the current view size.
    #[inline(always)]
    fn get_view_size(&self) -> (Scalar, Scalar) {
        let scale = get_scale(self.get_view_transform());
        (2.0 / scale[0], 2.0 / scale[1])
    }
}

impl<
    T: HasViewTransform
     + HasTransform
     + CanViewTransform
     + CanTransform
> View for T {}

