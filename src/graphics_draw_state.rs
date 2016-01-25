use draw_state_lib::state::{
    Blend,
    ColorMask,
    Depth,
    MultiSample,
    Rasterizer,
    Stencil
};
use draw_state_lib::target::Rect;

/// Graphics draw state used for blending, clipping and stencil rendering.
#[derive(Copy, Clone, PartialEq, Debug, PartialOrd)]
pub struct DrawState {
    /// How to rasterize geometric primitives.
    pub rasterizer: Rasterizer,
    /// Multi-sampling mode
    pub multi_sample: Option<MultiSample>,
    /// Scissor mask to use. If set, no pixel outside of this rectangle (in screen space) will be
    /// written to as a result of rendering.
    pub scissor: Option<Rect>,
    /// Stencil test to use. If None, no stencil testing is done.
    pub stencil: Option<Stencil>,
    /// Depth test to use. If None, no depth testing is done.
    pub depth: Option<Depth>,
    /// Blend function to use. If None, no blending is done.
    pub blend: Option<Blend>,
    /// Color mask to use. Each flag indicates that the given color channel can be written to, and
    /// they can be OR'd together.
    pub color_mask: ColorMask,
}
