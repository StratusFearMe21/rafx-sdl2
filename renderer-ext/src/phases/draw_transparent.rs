use renderer_base::RenderPhaseIndex;
use std::sync::atomic::Ordering;
use renderer_base::RenderPhase;
use std::sync::atomic::AtomicI32;
use std::convert::TryInto;

static DRAW_TRANSPARENT_RENDER_PHASE_INDEX: AtomicI32 = AtomicI32::new(-1);

pub struct DrawTransparentRenderPhase;

impl RenderPhase for DrawTransparentRenderPhase {
    fn set_render_phase_index(index: RenderPhaseIndex) {
        DRAW_TRANSPARENT_RENDER_PHASE_INDEX.store(index.try_into().unwrap(), Ordering::Release);
    }

    fn render_phase_index() -> RenderPhaseIndex {
        DRAW_TRANSPARENT_RENDER_PHASE_INDEX.load(Ordering::Acquire) as RenderPhaseIndex
    }

    fn sort_callback() {

    }
}