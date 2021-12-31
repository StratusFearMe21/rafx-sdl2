use crate::render_features::{
    RenderFeature, RenderFeatureFlag, RenderFeatureFlagMask, RenderFeatureIndex, RenderFeatureMask,
    RenderPhase, RenderPhaseIndex, RenderPhaseMask,
};
use crate::visibility::ViewFrustumArc;
use glam::{Mat4, Vec3};
use rafx_visibility::{DepthRange, Projection};
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use std::sync::Arc;

pub type RenderViewIndex = u32;
pub type RenderViewCount = u32;

pub struct RenderViewSet {
    view_count: AtomicU32,
    frame_index: usize,
}

impl RenderViewSet {
    pub fn new(frame_index: usize) -> Self {
        RenderViewSet {
            view_count: Default::default(),
            frame_index,
        }
    }

    pub fn create_view(
        &self,
        view_frustum: ViewFrustumArc,
        eye_position: Vec3,
        view: Mat4,
        proj: Mat4,
        extents: (u32, u32),
        depth_range: RenderViewDepthRange,
        render_phase_mask: RenderPhaseMask,
        render_feature_mask: RenderFeatureMask,
        render_feature_flag_mask: RenderFeatureFlagMask,
        debug_name: String,
    ) -> RenderView {
        let view_index = self.view_count.fetch_add(1, Ordering::Release);
        RenderView::new(
            view_frustum,
            view_index,
            eye_position,
            view,
            proj,
            extents,
            depth_range,
            render_phase_mask,
            render_feature_mask,
            render_feature_flag_mask,
            debug_name,
            self.frame_index,
        )
    }

    pub fn view_count(&self) -> RenderViewCount {
        self.view_count.load(Ordering::Acquire)
    }
}

////////////////// Views //////////////////
pub struct RenderViewInner {
    view_frustum: ViewFrustumArc,
    eye_position: Vec3,
    view: Mat4,
    proj: Mat4,
    view_proj: Mat4,
    view_dir: Vec3,
    view_index: RenderViewIndex,
    // XY of the plane in screen coordinates, the size the framebuffer would be for 1:1
    extents: (u32, u32),
    depth_range: RenderViewDepthRange,
    render_phase_mask: RenderPhaseMask,
    render_feature_mask: RenderFeatureMask,
    render_feature_flag_mask: RenderFeatureFlagMask,
    debug_name: String,
    frame_index: usize,
}

#[derive(Clone)]
pub struct RenderViewDepthRange {
    pub near: f32,
    pub far: Option<f32>, // If none, it's an infinite projection
    pub reversed: bool,
}

impl RenderViewDepthRange {
    pub fn new(
        near: f32,
        far: f32,
    ) -> Self {
        RenderViewDepthRange {
            near,
            far: Some(far),
            reversed: false,
        }
    }

    pub fn from_projection(projection: &Projection) -> Self {
        let near = projection.near_distance();
        let far = projection.far_distance();
        match projection.depth_range() {
            DepthRange::Normal => RenderViewDepthRange::new(near, far),
            DepthRange::Infinite => RenderViewDepthRange::new_infinite(near),
            DepthRange::Reverse => RenderViewDepthRange::new_reverse(near, far),
            DepthRange::InfiniteReverse => RenderViewDepthRange::new_infinite_reverse(near),
        }
    }

    pub fn new_infinite(near: f32) -> Self {
        RenderViewDepthRange {
            near,
            far: None,
            reversed: false,
        }
    }

    pub fn new_reverse(
        near: f32,
        far: f32,
    ) -> Self {
        RenderViewDepthRange {
            near,
            far: Some(far),
            reversed: true,
        }
    }

    pub fn new_infinite_reverse(near: f32) -> Self {
        RenderViewDepthRange {
            near,
            far: None,
            reversed: true,
        }
    }

    // If the view is a reversed view, return the values reversed. This is handy for cases where
    // the math works nicely by flipping the near/far for a reverse.
    pub fn planes_after_reverse(&self) -> (Option<f32>, Option<f32>) {
        if self.reversed {
            (self.far, Some(self.near))
        } else {
            (Some(self.near), self.far)
        }
    }

    pub fn finite_planes_after_reverse(&self) -> Option<(f32, f32)> {
        if self.reversed {
            Some((self.far?, self.near))
        } else {
            Some((self.near, self.far?))
        }
    }

    pub fn planes_before_reverse(&self) -> (f32, Option<f32>) {
        (self.near, self.far)
    }
}

/// The `Renderer` processes `RenderView`s during the execution of the `RenderGraph`. Each
/// `RenderView` is associated with a specific `ViewFrustum` in the game world. The `RenderView`
/// may be registered for specific `RenderFeature`s by a `RenderFeatureMask` or `RenderPhase`s by
/// the `RenderPhaseMask`. If a `RenderView` is not registered for a `RenderFeature` or `RenderPhase`
/// then it will not be processed by that feature or phase.
#[derive(Clone)]
pub struct RenderView {
    inner: Arc<RenderViewInner>,
}

impl RenderView {
    // Exposed publically as it's used downstream and want to make sure the logic stays in sync
    pub fn view_mat4_to_view_dir(view: &glam::Mat4) -> glam::Vec3 {
        glam::Vec3::new(view.x_axis.z, view.y_axis.z, view.z_axis.z) * -1.0
    }

    pub fn new(
        view_frustum: ViewFrustumArc,
        view_index: RenderViewIndex,
        eye_position: Vec3,
        view: Mat4,
        proj: Mat4,
        extents: (u32, u32),
        depth_range: RenderViewDepthRange,
        render_phase_mask: RenderPhaseMask,
        render_feature_mask: RenderFeatureMask,
        render_feature_flag_mask: RenderFeatureFlagMask,
        debug_name: String,
        frame_index: usize,
    ) -> RenderView {
        let view_dir = Self::view_mat4_to_view_dir(&view);

        log::trace!("Allocate view {} {}", debug_name, view_index);
        let inner = RenderViewInner {
            view_frustum,
            eye_position,
            view,
            proj,
            view_proj: proj * view,
            view_dir,
            view_index,
            extents,
            depth_range,
            render_phase_mask,
            render_feature_mask,
            render_feature_flag_mask,
            debug_name,
            frame_index,
        };

        RenderView {
            inner: Arc::new(inner),
        }
    }

    pub fn frame_index(&self) -> usize {
        self.inner.frame_index
    }

    pub fn view_frustum(&self) -> ViewFrustumArc {
        self.inner.view_frustum.clone()
    }

    pub fn eye_position(&self) -> Vec3 {
        self.inner.eye_position
    }

    pub fn view_dir(&self) -> Vec3 {
        self.inner.view_dir
    }

    pub fn view_matrix(&self) -> Mat4 {
        self.inner.view
    }

    pub fn projection_matrix(&self) -> Mat4 {
        self.inner.proj
    }

    pub fn view_proj(&self) -> Mat4 {
        self.inner.view_proj
    }

    pub fn view_index(&self) -> RenderViewIndex {
        self.inner.view_index
    }

    pub fn extents(&self) -> (u32, u32) {
        self.inner.extents
    }

    pub fn extents_width(&self) -> u32 {
        self.inner.extents.0
    }

    pub fn extents_height(&self) -> u32 {
        self.inner.extents.1
    }

    pub fn depth_range(&self) -> &RenderViewDepthRange {
        &self.inner.depth_range
    }

    pub fn debug_name(&self) -> &str {
        &self.inner.debug_name
    }

    pub fn phase_is_relevant<RenderPhaseT: RenderPhase>(&self) -> bool {
        self.inner.render_phase_mask.is_included::<RenderPhaseT>()
    }

    pub fn phase_index_is_relevant(
        &self,
        phase_index: RenderPhaseIndex,
    ) -> bool {
        self.inner.render_phase_mask.is_included_index(phase_index)
    }

    pub fn render_phase_mask(&self) -> RenderPhaseMask {
        self.inner.render_phase_mask
    }

    pub fn feature_is_relevant<RenderFeatureT: RenderFeature>(&self) -> bool {
        self.inner
            .render_feature_mask
            .is_included::<RenderFeatureT>()
    }

    pub fn feature_index_is_relevant(
        &self,
        feature_index: RenderFeatureIndex,
    ) -> bool {
        self.inner
            .render_feature_mask
            .is_included_index(feature_index)
    }

    pub fn render_feature_mask(&self) -> RenderFeatureMask {
        self.inner.render_feature_mask
    }

    pub fn is_relevant<RenderPhaseT: RenderPhase, RenderFeatureT: RenderFeature>(&self) -> bool {
        self.phase_is_relevant::<RenderPhaseT>() && self.feature_is_relevant::<RenderFeatureT>()
    }

    pub fn index_is_relevant(
        &self,
        phase_index: RenderPhaseIndex,
        feature_index: RenderFeatureIndex,
    ) -> bool {
        self.phase_index_is_relevant(phase_index) && self.feature_index_is_relevant(feature_index)
    }

    pub fn feature_flag_is_relevant<RenderFeatureFlagT: RenderFeatureFlag>(&self) -> bool {
        self.inner
            .render_feature_flag_mask
            .is_included::<RenderFeatureFlagT>()
    }

    pub fn feature_flag_index_is_relevant(
        &self,
        feature_flag_index: RenderFeatureIndex,
    ) -> bool {
        self.inner
            .render_feature_flag_mask
            .is_included_index(feature_flag_index)
    }

    pub fn render_feature_flag_mask(&self) -> RenderFeatureFlagMask {
        self.inner.render_feature_flag_mask
    }
}
