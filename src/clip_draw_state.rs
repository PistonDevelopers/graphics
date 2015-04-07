use draw_state::state::*;
use DrawState;

static CLIP_DRAW_STATE: &'static DrawState =
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
        stencil: Some(Stencil {
                front: StencilSide {
                    fun: Comparison::Never,
                    value: 1,
                    mask_read: 255,
                    mask_write: 255,
                    op_fail: StencilOp::Replace,
                    op_depth_fail: StencilOp::Keep,
                    op_pass: StencilOp::Keep,
                },
                back: StencilSide {
                    fun: Comparison::Never,
                    value: 1,
                    mask_read: 255,
                    mask_write: 255,
                    op_fail: StencilOp::Replace,
                    op_depth_fail: StencilOp::Keep,
                    op_pass: StencilOp::Keep,
                },
            }),
        depth: None,
        blend: Some(Blend {
                color: BlendChannel {
                    equation: Equation::Add,
                    source: Factor::ZeroPlus(BlendValue::SourceAlpha),
                    destination: Factor::OneMinus(BlendValue::SourceAlpha),
                },
                alpha: BlendChannel {
                    equation: Equation::Add,
                    source: Factor::One,
                    destination: Factor::One,
                },
                value: [0.0, 0.0, 0.0, 0.0],
            }),
        color_mask: MASK_NONE,
    };

/// Returns a default draw state that does additive blending and no culling.
pub fn clip_draw_state() -> &'static DrawState {
    CLIP_DRAW_STATE
}
