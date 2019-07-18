//! Graphics draw state.

/// Graphics draw state used for blending, clipping and stencil rendering.
#[derive(Copy, Clone, PartialEq, Debug, PartialOrd)]
pub struct DrawState {
    /// Scissor mask to use. If set, no pixel outside of this
    /// rectangle (in screen space) will be written to as a result of rendering.
    pub scissor: Option<[u32; 4]>,
    /// Stencil test to use. If None, no stencil testing is done.
    pub stencil: Option<Stencil>,
    /// Blend function to use. If None, blending is disabled.
    pub blend: Option<Blend>,
}

impl Default for DrawState {
    fn default() -> Self {
        DrawState::new_alpha()
    }
}

impl DrawState {
    /// Uses alpha blending.
    pub fn new_alpha() -> DrawState {
        DrawState {
            blend: Some(Blend::Alpha),
            stencil: None,
            scissor: None,
        }
    }

    /// Draws to stencil buffer with value 255.
    /// This can be used for clipping.
    pub fn new_clip() -> DrawState {
        DrawState {
            blend: Some(Blend::Alpha),
            stencil: Some(Stencil::Clip(255)),
            scissor: None,
        }
    }

    /// Tests against stencil buffer with value 255.
    /// Draws inside the shape defined by stencil buffer.
    pub fn new_inside() -> DrawState {
        DrawState {
            blend: Some(Blend::Alpha),
            stencil: Some(Stencil::Inside(255)),
            scissor: None,
        }
    }

    /// Tests against stencil buffer with value 255.
    /// Draws outside the shape defined by stencil buffer.
    pub fn new_outside() -> DrawState {
        DrawState {
            blend: Some(Blend::Alpha),
            stencil: Some(Stencil::Outside(255)),
            scissor: None,
        }
    }

    /// Sets blending.
    pub fn blend(mut self, blend: Blend) -> DrawState {
        self.blend = Some(blend);
        self
    }

    /// Sets scissor `[x, y, w, h]`.
    pub fn scissor(mut self, scissor: [u32; 4]) -> DrawState {
        self.scissor = Some(scissor);
        self
    }
}

/// The blend setting to use when drawing.
///
/// Using presets since some backends need one pipeline state object instance
/// per blending technique.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum Blend {
    /// Alpha blending (allows semi-transparent pixels).
    ///
    /// ```not_rust
    /// new_dest_color = src_color * src_alpha + dest_color * (1 - src_alpha)
    /// new_dest_alpha = src_alpha + dest_alpha
    /// ```
    Alpha,
    /// Additive blending.
    ///
    /// ```not_rust
    /// new_dest_color = src_color + dest_color
    /// new_dest_alpha = src_alpha + dest_alpha
    /// ```
    Add,
    /// Screen blending.
    ///
    /// ```not_rust
    /// new_dest_color = src_color * src_alpha + dest_color
    /// new_dest_alpha = src_alpha + dest_alpha
    /// ```
    Screen,
    /// Multiply color components.
    ///
    /// ```not_rust
    /// new_dest_color = src_color * dest_color
    /// new_dest_alpha = src_alpha * dest_alpha
    /// ```
    Multiply,
    /// Invert colors when rendering a white shape.
    ///
    /// ```not_rust
    /// new_dest_color = ref_color - src_color
    /// new_dest_alpha = dest_alpha
    /// ```
    ///
    /// When combining two fragments, subtract the destination color from a constant color
    /// using the source color as weight. Has an invert effect with the constant color
    /// as base and source color controlling displacement from the base color.
    /// A white source color and a white value results in plain invert.
    /// The output alpha is same as destination alpha.
    Invert,
}

/// Stencil buffer settings.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum Stencil {
    /// Draw to stencil buffer.
    Clip(u8),
    /// Draw pixels that have stencil value.
    Inside(u8),
    /// Draw pixels that does not have stencil value.
    Outside(u8),
}
