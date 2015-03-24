use draw_state::state::*;
use DrawState;

static DEFAULT_DRAW_STATE: &'static DrawState =
    &DrawState {
        primitive: Primitive {
            front_face: FrontFace::CounterClockwise,
            method: RasterMethod::Fill(
                CullFace::Nothing
            ),
            offset: None,
        },
        multi_sample: None,
        scissor: None,
        stencil: None,
        depth: None,
        blend: Some(Blend {
                color: BlendChannel {
                    equation: Equation::Add,
                    source: Factor(
                        InverseFlag::Normal,
                        BlendValue::SourceAlpha
                    ),
                    destination: Factor(
                        InverseFlag::Inverse,
                        BlendValue::SourceAlpha
                    ),
                },
                alpha: BlendChannel {
                    equation: Equation::Add,
                    source: Factor(
                        InverseFlag::Inverse,
                        BlendValue::Zero
                    ),
                    destination: Factor(
                        InverseFlag::Inverse,
                        BlendValue::Zero
                    ),
                },
                value: [0.0, 0.0, 0.0, 0.0],
            }),
        color_mask: MASK_ALL,
    };

/// Returns a default draw state that does additive blending and no culling.
pub fn default_draw_state() -> &'static DrawState {
    DEFAULT_DRAW_STATE
}
