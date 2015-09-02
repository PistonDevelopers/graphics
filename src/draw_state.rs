//! Draw state parameters.

/// Draw state parameters.
#[derive(Copy, Clone)]
pub struct DrawState {
    /// Blend effect.
    pub blend: Blend,
    /// Stencil effect.
    pub stencil: Option<Stencil>,
    /// Scissor rectangle.
    /// Limits the area of rendered pixels.
    /// Uses frame buffer coordinates.
    pub scissor: Option<[u32; 4]>,
}

impl DrawState {
    /// Clip draw state.
    pub fn clip(mut self) -> DrawState {
        self.stencil = Some(Stencil::Clip(1));
        self
    }

    /// Inside draw state.
    pub fn inside(mut self) -> DrawState {
        self.stencil = Some(Stencil::Inside(1));
        self
    }

    /// Outside draw state.
    pub fn outside(mut self) -> DrawState {
        self.stencil = Some(Stencil::Outside(1));
        self
    }

    /// Blend effect.
    pub fn blend(mut self, blend: Blend) -> DrawState {
        self.blend = blend;
        self
    }

    /// Stencil effect.
    pub fn stencil(mut self, stencil: Stencil) -> DrawState {
        self.stencil = Some(stencil);
        self
    }

    /// Scissor rectangle.
    pub fn scissor(mut self, scissor: [u32; 4]) -> DrawState {
        self.scissor = Some(scissor);
        self
    }
}

impl Default for DrawState {
    fn default() -> Self {
        DrawState {
            blend: Blend::Alpha,
            stencil: None,
            scissor: None,
        }
    }
}

#[derive(Copy, Clone)]
/// Blend effect.
///
/// This effect is applied after shaders.
/// Most hardware supports simple blending effects.
///
/// Simple blend effects are documented with their corresponding fixed
/// pipeline blending equation.
///
/// blend_equation: `source * (blend_parameter) <op> destination * (blend_parameter)`
pub enum Blend {
    /// Alpha blends colors.
    ///
    /// color: `source * (source_alpha) + destination * (1 - source_alpha)`
    ///
    /// alpha: `source * (1) + destination * (1)`
    Alpha,
    /// Adds color components.
    ///
    /// color: `source * (1) + destination * (1)`
    ///
    /// alpha: `source * (1) + destination * (1)`
    Add,
    /// Multiplies the color components.
    ///
    /// color: `source * (destination) + destination * (0)`
    ///
    /// alpha: `source * (destination_alpha) + destination * (0)`
    Multiply,
    /// Inverts the rendered color.
    ///
    /// color: `source * (1) - destination * (source)`
    ///
    /// alpha: `source * (0) + destination * (1)`.
    Invert,
}

#[derive(Copy, Clone)]
/// Stencil effect.
pub enum Stencil {
    /// Assign stencil value to pixels that are rendered invisible.
    /// This is used to define the area of clipping.
    Clip(u8),
    /// Renders pixels that has a stencil value.
    /// This is used to render inside an area of clipping.
    Inside(u8),
    /// Renders pixels that does not have a stencil value.
    /// This is used to render outside an area of clipping.
    Outside(u8),
}
