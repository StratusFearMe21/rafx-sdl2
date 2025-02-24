// This code is auto-generated by the shader processor.

#[allow(unused_imports)]
use rafx::RafxResult;

#[allow(unused_imports)]
use rafx::framework::{
    DescriptorSetAllocator, DescriptorSetArc, DescriptorSetBindings, DescriptorSetInitializer,
    DescriptorSetWriter, DescriptorSetWriterContext, DynDescriptorSet, ImageViewResource,
    ResourceArc,
};

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct ShadowMap2DDataStd140 {
    pub shadow_map_view_proj: [[f32; 4]; 4], // +0 (size: 64)
    pub shadow_map_light_dir: [f32; 3],      // +64 (size: 12)
    pub _padding0: [u8; 4],                  // +76 (size: 4)
} // 80 bytes

impl Default for ShadowMap2DDataStd140 {
    fn default() -> Self {
        ShadowMap2DDataStd140 {
            shadow_map_view_proj: <[[f32; 4]; 4]>::default(),
            shadow_map_light_dir: <[f32; 3]>::default(),
            _padding0: [u8::default(); 4],
        }
    }
}

pub type ShadowMap2DDataUniform = ShadowMap2DDataStd140;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct MaterialDataStd140 {
    pub base_color_factor: [f32; 4],         // +0 (size: 16)
    pub emissive_factor: [f32; 3],           // +16 (size: 12)
    pub metallic_factor: f32,                // +28 (size: 4)
    pub roughness_factor: f32,               // +32 (size: 4)
    pub normal_texture_scale: f32,           // +36 (size: 4)
    pub occlusion_texture_strength: f32,     // +40 (size: 4)
    pub alpha_cutoff: f32,                   // +44 (size: 4)
    pub has_base_color_texture: u32,         // +48 (size: 4)
    pub has_metallic_roughness_texture: u32, // +52 (size: 4)
    pub has_normal_texture: u32,             // +56 (size: 4)
    pub has_occlusion_texture: u32,          // +60 (size: 4)
    pub has_emissive_texture: u32,           // +64 (size: 4)
    pub _padding0: [u8; 12],                 // +68 (size: 12)
} // 80 bytes

impl Default for MaterialDataStd140 {
    fn default() -> Self {
        MaterialDataStd140 {
            base_color_factor: <[f32; 4]>::default(),
            emissive_factor: <[f32; 3]>::default(),
            metallic_factor: <f32>::default(),
            roughness_factor: <f32>::default(),
            normal_texture_scale: <f32>::default(),
            occlusion_texture_strength: <f32>::default(),
            alpha_cutoff: <f32>::default(),
            has_base_color_texture: <u32>::default(),
            has_metallic_roughness_texture: <u32>::default(),
            has_normal_texture: <u32>::default(),
            has_occlusion_texture: <u32>::default(),
            has_emissive_texture: <u32>::default(),
            _padding0: [u8::default(); 12],
        }
    }
}

pub type MaterialDataUniform = MaterialDataStd140;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct PerViewDataStd140 {
    pub view: [[f32; 4]; 4],                                 // +0 (size: 64)
    pub view_proj: [[f32; 4]; 4],                            // +64 (size: 64)
    pub ambient_light: [f32; 4],                             // +128 (size: 16)
    pub point_light_count: u32,                              // +144 (size: 4)
    pub directional_light_count: u32,                        // +148 (size: 4)
    pub spot_light_count: u32,                               // +152 (size: 4)
    pub _padding0: [u8; 4],                                  // +156 (size: 4)
    pub point_lights: [PointLightStd140; 16],                // +160 (size: 1024)
    pub directional_lights: [DirectionalLightStd140; 16],    // +1184 (size: 1024)
    pub spot_lights: [SpotLightStd140; 16],                  // +2208 (size: 1536)
    pub shadow_map_2d_data: [ShadowMap2DDataStd140; 32],     // +3744 (size: 2560)
    pub shadow_map_cube_data: [ShadowMapCubeDataStd140; 16], // +6304 (size: 256)
} // 6560 bytes

impl Default for PerViewDataStd140 {
    fn default() -> Self {
        PerViewDataStd140 {
            view: <[[f32; 4]; 4]>::default(),
            view_proj: <[[f32; 4]; 4]>::default(),
            ambient_light: <[f32; 4]>::default(),
            point_light_count: <u32>::default(),
            directional_light_count: <u32>::default(),
            spot_light_count: <u32>::default(),
            _padding0: [u8::default(); 4],
            point_lights: [<PointLightStd140>::default(); 16],
            directional_lights: [<DirectionalLightStd140>::default(); 16],
            spot_lights: [<SpotLightStd140>::default(); 16],
            shadow_map_2d_data: [<ShadowMap2DDataStd140>::default(); 32],
            shadow_map_cube_data: [<ShadowMapCubeDataStd140>::default(); 16],
        }
    }
}

pub type PerViewDataUniform = PerViewDataStd140;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct SpotLightStd140 {
    pub position_ws: [f32; 3],     // +0 (size: 12)
    pub _padding0: [u8; 4],        // +12 (size: 4)
    pub direction_ws: [f32; 3],    // +16 (size: 12)
    pub _padding1: [u8; 4],        // +28 (size: 4)
    pub position_vs: [f32; 3],     // +32 (size: 12)
    pub _padding2: [u8; 4],        // +44 (size: 4)
    pub direction_vs: [f32; 3],    // +48 (size: 12)
    pub _padding3: [u8; 4],        // +60 (size: 4)
    pub color: [f32; 4],           // +64 (size: 16)
    pub spotlight_half_angle: f32, // +80 (size: 4)
    pub range: f32,                // +84 (size: 4)
    pub intensity: f32,            // +88 (size: 4)
    pub shadow_map: i32,           // +92 (size: 4)
} // 96 bytes

impl Default for SpotLightStd140 {
    fn default() -> Self {
        SpotLightStd140 {
            position_ws: <[f32; 3]>::default(),
            _padding0: [u8::default(); 4],
            direction_ws: <[f32; 3]>::default(),
            _padding1: [u8::default(); 4],
            position_vs: <[f32; 3]>::default(),
            _padding2: [u8::default(); 4],
            direction_vs: <[f32; 3]>::default(),
            _padding3: [u8::default(); 4],
            color: <[f32; 4]>::default(),
            spotlight_half_angle: <f32>::default(),
            range: <f32>::default(),
            intensity: <f32>::default(),
            shadow_map: <i32>::default(),
        }
    }
}

pub type SpotLightUniform = SpotLightStd140;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct DirectionalLightStd140 {
    pub direction_ws: [f32; 3], // +0 (size: 12)
    pub _padding0: [u8; 4],     // +12 (size: 4)
    pub direction_vs: [f32; 3], // +16 (size: 12)
    pub _padding1: [u8; 4],     // +28 (size: 4)
    pub color: [f32; 4],        // +32 (size: 16)
    pub intensity: f32,         // +48 (size: 4)
    pub shadow_map: i32,        // +52 (size: 4)
    pub _padding2: [u8; 8],     // +56 (size: 8)
} // 64 bytes

impl Default for DirectionalLightStd140 {
    fn default() -> Self {
        DirectionalLightStd140 {
            direction_ws: <[f32; 3]>::default(),
            _padding0: [u8::default(); 4],
            direction_vs: <[f32; 3]>::default(),
            _padding1: [u8::default(); 4],
            color: <[f32; 4]>::default(),
            intensity: <f32>::default(),
            shadow_map: <i32>::default(),
            _padding2: [u8::default(); 8],
        }
    }
}

pub type DirectionalLightUniform = DirectionalLightStd140;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct PointLightStd140 {
    pub position_ws: [f32; 3], // +0 (size: 12)
    pub _padding0: [u8; 4],    // +12 (size: 4)
    pub position_vs: [f32; 3], // +16 (size: 12)
    pub _padding1: [u8; 4],    // +28 (size: 4)
    pub color: [f32; 4],       // +32 (size: 16)
    pub range: f32,            // +48 (size: 4)
    pub intensity: f32,        // +52 (size: 4)
    pub shadow_map: i32,       // +56 (size: 4)
    pub _padding2: [u8; 4],    // +60 (size: 4)
} // 64 bytes

impl Default for PointLightStd140 {
    fn default() -> Self {
        PointLightStd140 {
            position_ws: <[f32; 3]>::default(),
            _padding0: [u8::default(); 4],
            position_vs: <[f32; 3]>::default(),
            _padding1: [u8::default(); 4],
            color: <[f32; 4]>::default(),
            range: <f32>::default(),
            intensity: <f32>::default(),
            shadow_map: <i32>::default(),
            _padding2: [u8::default(); 4],
        }
    }
}

pub type PointLightUniform = PointLightStd140;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct ShadowMapCubeDataStd140 {
    pub cube_map_projection_near_z: f32, // +0 (size: 4)
    pub cube_map_projection_far_z: f32,  // +4 (size: 4)
    pub _padding0: [u8; 8],              // +8 (size: 8)
} // 16 bytes

impl Default for ShadowMapCubeDataStd140 {
    fn default() -> Self {
        ShadowMapCubeDataStd140 {
            cube_map_projection_near_z: <f32>::default(),
            cube_map_projection_far_z: <f32>::default(),
            _padding0: [u8::default(); 8],
        }
    }
}

pub type ShadowMapCubeDataUniform = ShadowMapCubeDataStd140;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct MaterialDataUboStd140 {
    pub data: MaterialDataStd140, // +0 (size: 80)
} // 80 bytes

impl Default for MaterialDataUboStd140 {
    fn default() -> Self {
        MaterialDataUboStd140 {
            data: <MaterialDataStd140>::default(),
        }
    }
}

pub type MaterialDataUboUniform = MaterialDataUboStd140;

pub const PER_VIEW_DATA_DESCRIPTOR_SET_INDEX: usize = 0;
pub const PER_VIEW_DATA_DESCRIPTOR_BINDING_INDEX: usize = 0;
pub const SMP_DESCRIPTOR_SET_INDEX: usize = 0;
pub const SMP_DESCRIPTOR_BINDING_INDEX: usize = 1;
pub const SMP_DEPTH_DESCRIPTOR_SET_INDEX: usize = 0;
pub const SMP_DEPTH_DESCRIPTOR_BINDING_INDEX: usize = 2;
pub const SHADOW_MAP_IMAGES_DESCRIPTOR_SET_INDEX: usize = 0;
pub const SHADOW_MAP_IMAGES_DESCRIPTOR_BINDING_INDEX: usize = 3;
pub const SHADOW_MAP_IMAGES_CUBE_DESCRIPTOR_SET_INDEX: usize = 0;
pub const SHADOW_MAP_IMAGES_CUBE_DESCRIPTOR_BINDING_INDEX: usize = 4;
pub const PER_MATERIAL_DATA_DESCRIPTOR_SET_INDEX: usize = 1;
pub const PER_MATERIAL_DATA_DESCRIPTOR_BINDING_INDEX: usize = 0;
pub const BASE_COLOR_TEXTURE_DESCRIPTOR_SET_INDEX: usize = 1;
pub const BASE_COLOR_TEXTURE_DESCRIPTOR_BINDING_INDEX: usize = 1;
pub const METALLIC_ROUGHNESS_TEXTURE_DESCRIPTOR_SET_INDEX: usize = 1;
pub const METALLIC_ROUGHNESS_TEXTURE_DESCRIPTOR_BINDING_INDEX: usize = 2;
pub const NORMAL_TEXTURE_DESCRIPTOR_SET_INDEX: usize = 1;
pub const NORMAL_TEXTURE_DESCRIPTOR_BINDING_INDEX: usize = 3;
pub const OCCLUSION_TEXTURE_DESCRIPTOR_SET_INDEX: usize = 1;
pub const OCCLUSION_TEXTURE_DESCRIPTOR_BINDING_INDEX: usize = 4;
pub const EMISSIVE_TEXTURE_DESCRIPTOR_SET_INDEX: usize = 1;
pub const EMISSIVE_TEXTURE_DESCRIPTOR_BINDING_INDEX: usize = 5;

pub struct DescriptorSet0Args<'a> {
    pub per_view_data: &'a PerViewDataUniform,
    pub shadow_map_images: &'a [Option<&'a ResourceArc<ImageViewResource>>; 32],
    pub shadow_map_images_cube: &'a [Option<&'a ResourceArc<ImageViewResource>>; 16],
}

impl<'a> DescriptorSetInitializer<'a> for DescriptorSet0Args<'a> {
    type Output = DescriptorSet0;

    fn create_dyn_descriptor_set(
        descriptor_set: DynDescriptorSet,
        args: Self,
    ) -> Self::Output {
        let mut descriptor = DescriptorSet0(descriptor_set);
        descriptor.set_args(args);
        descriptor
    }

    fn create_descriptor_set(
        descriptor_set_allocator: &mut DescriptorSetAllocator,
        descriptor_set: DynDescriptorSet,
        args: Self,
    ) -> RafxResult<DescriptorSetArc> {
        let mut descriptor = Self::create_dyn_descriptor_set(descriptor_set, args);
        descriptor.0.flush(descriptor_set_allocator)?;
        Ok(descriptor.0.descriptor_set().clone())
    }
}

impl<'a> DescriptorSetWriter<'a> for DescriptorSet0Args<'a> {
    fn write_to(
        descriptor_set: &mut DescriptorSetWriterContext,
        args: Self,
    ) {
        descriptor_set.set_buffer_data(
            PER_VIEW_DATA_DESCRIPTOR_BINDING_INDEX as u32,
            args.per_view_data,
        );
        descriptor_set.set_images(
            SHADOW_MAP_IMAGES_DESCRIPTOR_BINDING_INDEX as u32,
            args.shadow_map_images,
        );
        descriptor_set.set_images(
            SHADOW_MAP_IMAGES_CUBE_DESCRIPTOR_BINDING_INDEX as u32,
            args.shadow_map_images_cube,
        );
    }
}

pub struct DescriptorSet0(pub DynDescriptorSet);

impl DescriptorSet0 {
    pub fn set_args_static(
        descriptor_set: &mut DynDescriptorSet,
        args: DescriptorSet0Args,
    ) {
        descriptor_set.set_buffer_data(
            PER_VIEW_DATA_DESCRIPTOR_BINDING_INDEX as u32,
            args.per_view_data,
        );
        descriptor_set.set_images(
            SHADOW_MAP_IMAGES_DESCRIPTOR_BINDING_INDEX as u32,
            args.shadow_map_images,
        );
        descriptor_set.set_images(
            SHADOW_MAP_IMAGES_CUBE_DESCRIPTOR_BINDING_INDEX as u32,
            args.shadow_map_images_cube,
        );
    }

    pub fn set_args(
        &mut self,
        args: DescriptorSet0Args,
    ) {
        self.set_per_view_data(args.per_view_data);
        self.set_shadow_map_images(args.shadow_map_images);
        self.set_shadow_map_images_cube(args.shadow_map_images_cube);
    }

    pub fn set_per_view_data(
        &mut self,
        per_view_data: &PerViewDataUniform,
    ) {
        self.0
            .set_buffer_data(PER_VIEW_DATA_DESCRIPTOR_BINDING_INDEX as u32, per_view_data);
    }

    pub fn set_shadow_map_images(
        &mut self,
        shadow_map_images: &[Option<&ResourceArc<ImageViewResource>>; 32],
    ) {
        self.0.set_images(
            SHADOW_MAP_IMAGES_DESCRIPTOR_BINDING_INDEX as u32,
            shadow_map_images,
        );
    }

    pub fn set_shadow_map_images_element(
        &mut self,
        index: usize,
        element: &ResourceArc<ImageViewResource>,
    ) {
        self.0.set_image_at_index(
            SHADOW_MAP_IMAGES_DESCRIPTOR_BINDING_INDEX as u32,
            index,
            element,
        );
    }

    pub fn set_shadow_map_images_cube(
        &mut self,
        shadow_map_images_cube: &[Option<&ResourceArc<ImageViewResource>>; 16],
    ) {
        self.0.set_images(
            SHADOW_MAP_IMAGES_CUBE_DESCRIPTOR_BINDING_INDEX as u32,
            shadow_map_images_cube,
        );
    }

    pub fn set_shadow_map_images_cube_element(
        &mut self,
        index: usize,
        element: &ResourceArc<ImageViewResource>,
    ) {
        self.0.set_image_at_index(
            SHADOW_MAP_IMAGES_CUBE_DESCRIPTOR_BINDING_INDEX as u32,
            index,
            element,
        );
    }

    pub fn flush(
        &mut self,
        descriptor_set_allocator: &mut DescriptorSetAllocator,
    ) -> RafxResult<()> {
        self.0.flush(descriptor_set_allocator)
    }
}

pub struct DescriptorSet1Args<'a> {
    pub per_material_data: &'a MaterialDataUboUniform,
    pub base_color_texture: &'a ResourceArc<ImageViewResource>,
    pub metallic_roughness_texture: &'a ResourceArc<ImageViewResource>,
    pub normal_texture: &'a ResourceArc<ImageViewResource>,
    pub occlusion_texture: &'a ResourceArc<ImageViewResource>,
    pub emissive_texture: &'a ResourceArc<ImageViewResource>,
}

impl<'a> DescriptorSetInitializer<'a> for DescriptorSet1Args<'a> {
    type Output = DescriptorSet1;

    fn create_dyn_descriptor_set(
        descriptor_set: DynDescriptorSet,
        args: Self,
    ) -> Self::Output {
        let mut descriptor = DescriptorSet1(descriptor_set);
        descriptor.set_args(args);
        descriptor
    }

    fn create_descriptor_set(
        descriptor_set_allocator: &mut DescriptorSetAllocator,
        descriptor_set: DynDescriptorSet,
        args: Self,
    ) -> RafxResult<DescriptorSetArc> {
        let mut descriptor = Self::create_dyn_descriptor_set(descriptor_set, args);
        descriptor.0.flush(descriptor_set_allocator)?;
        Ok(descriptor.0.descriptor_set().clone())
    }
}

impl<'a> DescriptorSetWriter<'a> for DescriptorSet1Args<'a> {
    fn write_to(
        descriptor_set: &mut DescriptorSetWriterContext,
        args: Self,
    ) {
        descriptor_set.set_buffer_data(
            PER_MATERIAL_DATA_DESCRIPTOR_BINDING_INDEX as u32,
            args.per_material_data,
        );
        descriptor_set.set_image(
            BASE_COLOR_TEXTURE_DESCRIPTOR_BINDING_INDEX as u32,
            args.base_color_texture,
        );
        descriptor_set.set_image(
            METALLIC_ROUGHNESS_TEXTURE_DESCRIPTOR_BINDING_INDEX as u32,
            args.metallic_roughness_texture,
        );
        descriptor_set.set_image(
            NORMAL_TEXTURE_DESCRIPTOR_BINDING_INDEX as u32,
            args.normal_texture,
        );
        descriptor_set.set_image(
            OCCLUSION_TEXTURE_DESCRIPTOR_BINDING_INDEX as u32,
            args.occlusion_texture,
        );
        descriptor_set.set_image(
            EMISSIVE_TEXTURE_DESCRIPTOR_BINDING_INDEX as u32,
            args.emissive_texture,
        );
    }
}

pub struct DescriptorSet1(pub DynDescriptorSet);

impl DescriptorSet1 {
    pub fn set_args_static(
        descriptor_set: &mut DynDescriptorSet,
        args: DescriptorSet1Args,
    ) {
        descriptor_set.set_buffer_data(
            PER_MATERIAL_DATA_DESCRIPTOR_BINDING_INDEX as u32,
            args.per_material_data,
        );
        descriptor_set.set_image(
            BASE_COLOR_TEXTURE_DESCRIPTOR_BINDING_INDEX as u32,
            args.base_color_texture,
        );
        descriptor_set.set_image(
            METALLIC_ROUGHNESS_TEXTURE_DESCRIPTOR_BINDING_INDEX as u32,
            args.metallic_roughness_texture,
        );
        descriptor_set.set_image(
            NORMAL_TEXTURE_DESCRIPTOR_BINDING_INDEX as u32,
            args.normal_texture,
        );
        descriptor_set.set_image(
            OCCLUSION_TEXTURE_DESCRIPTOR_BINDING_INDEX as u32,
            args.occlusion_texture,
        );
        descriptor_set.set_image(
            EMISSIVE_TEXTURE_DESCRIPTOR_BINDING_INDEX as u32,
            args.emissive_texture,
        );
    }

    pub fn set_args(
        &mut self,
        args: DescriptorSet1Args,
    ) {
        self.set_per_material_data(args.per_material_data);
        self.set_base_color_texture(args.base_color_texture);
        self.set_metallic_roughness_texture(args.metallic_roughness_texture);
        self.set_normal_texture(args.normal_texture);
        self.set_occlusion_texture(args.occlusion_texture);
        self.set_emissive_texture(args.emissive_texture);
    }

    pub fn set_per_material_data(
        &mut self,
        per_material_data: &MaterialDataUboUniform,
    ) {
        self.0.set_buffer_data(
            PER_MATERIAL_DATA_DESCRIPTOR_BINDING_INDEX as u32,
            per_material_data,
        );
    }

    pub fn set_base_color_texture(
        &mut self,
        base_color_texture: &ResourceArc<ImageViewResource>,
    ) {
        self.0.set_image(
            BASE_COLOR_TEXTURE_DESCRIPTOR_BINDING_INDEX as u32,
            base_color_texture,
        );
    }

    pub fn set_metallic_roughness_texture(
        &mut self,
        metallic_roughness_texture: &ResourceArc<ImageViewResource>,
    ) {
        self.0.set_image(
            METALLIC_ROUGHNESS_TEXTURE_DESCRIPTOR_BINDING_INDEX as u32,
            metallic_roughness_texture,
        );
    }

    pub fn set_normal_texture(
        &mut self,
        normal_texture: &ResourceArc<ImageViewResource>,
    ) {
        self.0.set_image(
            NORMAL_TEXTURE_DESCRIPTOR_BINDING_INDEX as u32,
            normal_texture,
        );
    }

    pub fn set_occlusion_texture(
        &mut self,
        occlusion_texture: &ResourceArc<ImageViewResource>,
    ) {
        self.0.set_image(
            OCCLUSION_TEXTURE_DESCRIPTOR_BINDING_INDEX as u32,
            occlusion_texture,
        );
    }

    pub fn set_emissive_texture(
        &mut self,
        emissive_texture: &ResourceArc<ImageViewResource>,
    ) {
        self.0.set_image(
            EMISSIVE_TEXTURE_DESCRIPTOR_BINDING_INDEX as u32,
            emissive_texture,
        );
    }

    pub fn flush(
        &mut self,
        descriptor_set_allocator: &mut DescriptorSetAllocator,
    ) -> RafxResult<()> {
        self.0.flush(descriptor_set_allocator)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_struct_shadow_map2_d_data_std140() {
        assert_eq!(std::mem::size_of::<ShadowMap2DDataStd140>(), 80);
        assert_eq!(std::mem::size_of::<[[f32; 4]; 4]>(), 64);
        assert_eq!(std::mem::align_of::<[[f32; 4]; 4]>(), 4);
        assert_eq!(
            memoffset::offset_of!(ShadowMap2DDataStd140, shadow_map_view_proj),
            0
        );
        assert_eq!(std::mem::size_of::<[f32; 3]>(), 12);
        assert_eq!(std::mem::align_of::<[f32; 3]>(), 4);
        assert_eq!(
            memoffset::offset_of!(ShadowMap2DDataStd140, shadow_map_light_dir),
            64
        );
        assert_eq!(std::mem::size_of::<[u8; 4]>(), 4);
        assert_eq!(std::mem::align_of::<[u8; 4]>(), 1);
        assert_eq!(memoffset::offset_of!(ShadowMap2DDataStd140, _padding0), 76);
    }

    #[test]
    fn test_struct_material_data_std140() {
        assert_eq!(std::mem::size_of::<MaterialDataStd140>(), 80);
        assert_eq!(std::mem::size_of::<[f32; 4]>(), 16);
        assert_eq!(std::mem::align_of::<[f32; 4]>(), 4);
        assert_eq!(
            memoffset::offset_of!(MaterialDataStd140, base_color_factor),
            0
        );
        assert_eq!(std::mem::size_of::<[f32; 3]>(), 12);
        assert_eq!(std::mem::align_of::<[f32; 3]>(), 4);
        assert_eq!(
            memoffset::offset_of!(MaterialDataStd140, emissive_factor),
            16
        );
        assert_eq!(std::mem::size_of::<f32>(), 4);
        assert_eq!(std::mem::align_of::<f32>(), 4);
        assert_eq!(
            memoffset::offset_of!(MaterialDataStd140, metallic_factor),
            28
        );
        assert_eq!(std::mem::size_of::<f32>(), 4);
        assert_eq!(std::mem::align_of::<f32>(), 4);
        assert_eq!(
            memoffset::offset_of!(MaterialDataStd140, roughness_factor),
            32
        );
        assert_eq!(std::mem::size_of::<f32>(), 4);
        assert_eq!(std::mem::align_of::<f32>(), 4);
        assert_eq!(
            memoffset::offset_of!(MaterialDataStd140, normal_texture_scale),
            36
        );
        assert_eq!(std::mem::size_of::<f32>(), 4);
        assert_eq!(std::mem::align_of::<f32>(), 4);
        assert_eq!(
            memoffset::offset_of!(MaterialDataStd140, occlusion_texture_strength),
            40
        );
        assert_eq!(std::mem::size_of::<f32>(), 4);
        assert_eq!(std::mem::align_of::<f32>(), 4);
        assert_eq!(memoffset::offset_of!(MaterialDataStd140, alpha_cutoff), 44);
        assert_eq!(std::mem::size_of::<u32>(), 4);
        assert_eq!(std::mem::align_of::<u32>(), 4);
        assert_eq!(
            memoffset::offset_of!(MaterialDataStd140, has_base_color_texture),
            48
        );
        assert_eq!(std::mem::size_of::<u32>(), 4);
        assert_eq!(std::mem::align_of::<u32>(), 4);
        assert_eq!(
            memoffset::offset_of!(MaterialDataStd140, has_metallic_roughness_texture),
            52
        );
        assert_eq!(std::mem::size_of::<u32>(), 4);
        assert_eq!(std::mem::align_of::<u32>(), 4);
        assert_eq!(
            memoffset::offset_of!(MaterialDataStd140, has_normal_texture),
            56
        );
        assert_eq!(std::mem::size_of::<u32>(), 4);
        assert_eq!(std::mem::align_of::<u32>(), 4);
        assert_eq!(
            memoffset::offset_of!(MaterialDataStd140, has_occlusion_texture),
            60
        );
        assert_eq!(std::mem::size_of::<u32>(), 4);
        assert_eq!(std::mem::align_of::<u32>(), 4);
        assert_eq!(
            memoffset::offset_of!(MaterialDataStd140, has_emissive_texture),
            64
        );
        assert_eq!(std::mem::size_of::<[u8; 12]>(), 12);
        assert_eq!(std::mem::align_of::<[u8; 12]>(), 1);
        assert_eq!(memoffset::offset_of!(MaterialDataStd140, _padding0), 68);
    }

    #[test]
    fn test_struct_per_view_data_std140() {
        assert_eq!(std::mem::size_of::<PerViewDataStd140>(), 6560);
        assert_eq!(std::mem::size_of::<[[f32; 4]; 4]>(), 64);
        assert_eq!(std::mem::align_of::<[[f32; 4]; 4]>(), 4);
        assert_eq!(memoffset::offset_of!(PerViewDataStd140, view), 0);
        assert_eq!(std::mem::size_of::<[[f32; 4]; 4]>(), 64);
        assert_eq!(std::mem::align_of::<[[f32; 4]; 4]>(), 4);
        assert_eq!(memoffset::offset_of!(PerViewDataStd140, view_proj), 64);
        assert_eq!(std::mem::size_of::<[f32; 4]>(), 16);
        assert_eq!(std::mem::align_of::<[f32; 4]>(), 4);
        assert_eq!(memoffset::offset_of!(PerViewDataStd140, ambient_light), 128);
        assert_eq!(std::mem::size_of::<u32>(), 4);
        assert_eq!(std::mem::align_of::<u32>(), 4);
        assert_eq!(
            memoffset::offset_of!(PerViewDataStd140, point_light_count),
            144
        );
        assert_eq!(std::mem::size_of::<u32>(), 4);
        assert_eq!(std::mem::align_of::<u32>(), 4);
        assert_eq!(
            memoffset::offset_of!(PerViewDataStd140, directional_light_count),
            148
        );
        assert_eq!(std::mem::size_of::<u32>(), 4);
        assert_eq!(std::mem::align_of::<u32>(), 4);
        assert_eq!(
            memoffset::offset_of!(PerViewDataStd140, spot_light_count),
            152
        );
        assert_eq!(std::mem::size_of::<[u8; 4]>(), 4);
        assert_eq!(std::mem::align_of::<[u8; 4]>(), 1);
        assert_eq!(memoffset::offset_of!(PerViewDataStd140, _padding0), 156);
        assert_eq!(std::mem::size_of::<[PointLightStd140; 16]>(), 1024);
        assert_eq!(std::mem::align_of::<[PointLightStd140; 16]>(), 4);
        assert_eq!(memoffset::offset_of!(PerViewDataStd140, point_lights), 160);
        assert_eq!(std::mem::size_of::<[DirectionalLightStd140; 16]>(), 1024);
        assert_eq!(std::mem::align_of::<[DirectionalLightStd140; 16]>(), 4);
        assert_eq!(
            memoffset::offset_of!(PerViewDataStd140, directional_lights),
            1184
        );
        assert_eq!(std::mem::size_of::<[SpotLightStd140; 16]>(), 1536);
        assert_eq!(std::mem::align_of::<[SpotLightStd140; 16]>(), 4);
        assert_eq!(memoffset::offset_of!(PerViewDataStd140, spot_lights), 2208);
        assert_eq!(std::mem::size_of::<[ShadowMap2DDataStd140; 32]>(), 2560);
        assert_eq!(std::mem::align_of::<[ShadowMap2DDataStd140; 32]>(), 4);
        assert_eq!(
            memoffset::offset_of!(PerViewDataStd140, shadow_map_2d_data),
            3744
        );
        assert_eq!(std::mem::size_of::<[ShadowMapCubeDataStd140; 16]>(), 256);
        assert_eq!(std::mem::align_of::<[ShadowMapCubeDataStd140; 16]>(), 4);
        assert_eq!(
            memoffset::offset_of!(PerViewDataStd140, shadow_map_cube_data),
            6304
        );
    }

    #[test]
    fn test_struct_spot_light_std140() {
        assert_eq!(std::mem::size_of::<SpotLightStd140>(), 96);
        assert_eq!(std::mem::size_of::<[f32; 3]>(), 12);
        assert_eq!(std::mem::align_of::<[f32; 3]>(), 4);
        assert_eq!(memoffset::offset_of!(SpotLightStd140, position_ws), 0);
        assert_eq!(std::mem::size_of::<[u8; 4]>(), 4);
        assert_eq!(std::mem::align_of::<[u8; 4]>(), 1);
        assert_eq!(memoffset::offset_of!(SpotLightStd140, _padding0), 12);
        assert_eq!(std::mem::size_of::<[f32; 3]>(), 12);
        assert_eq!(std::mem::align_of::<[f32; 3]>(), 4);
        assert_eq!(memoffset::offset_of!(SpotLightStd140, direction_ws), 16);
        assert_eq!(std::mem::size_of::<[u8; 4]>(), 4);
        assert_eq!(std::mem::align_of::<[u8; 4]>(), 1);
        assert_eq!(memoffset::offset_of!(SpotLightStd140, _padding1), 28);
        assert_eq!(std::mem::size_of::<[f32; 3]>(), 12);
        assert_eq!(std::mem::align_of::<[f32; 3]>(), 4);
        assert_eq!(memoffset::offset_of!(SpotLightStd140, position_vs), 32);
        assert_eq!(std::mem::size_of::<[u8; 4]>(), 4);
        assert_eq!(std::mem::align_of::<[u8; 4]>(), 1);
        assert_eq!(memoffset::offset_of!(SpotLightStd140, _padding2), 44);
        assert_eq!(std::mem::size_of::<[f32; 3]>(), 12);
        assert_eq!(std::mem::align_of::<[f32; 3]>(), 4);
        assert_eq!(memoffset::offset_of!(SpotLightStd140, direction_vs), 48);
        assert_eq!(std::mem::size_of::<[u8; 4]>(), 4);
        assert_eq!(std::mem::align_of::<[u8; 4]>(), 1);
        assert_eq!(memoffset::offset_of!(SpotLightStd140, _padding3), 60);
        assert_eq!(std::mem::size_of::<[f32; 4]>(), 16);
        assert_eq!(std::mem::align_of::<[f32; 4]>(), 4);
        assert_eq!(memoffset::offset_of!(SpotLightStd140, color), 64);
        assert_eq!(std::mem::size_of::<f32>(), 4);
        assert_eq!(std::mem::align_of::<f32>(), 4);
        assert_eq!(
            memoffset::offset_of!(SpotLightStd140, spotlight_half_angle),
            80
        );
        assert_eq!(std::mem::size_of::<f32>(), 4);
        assert_eq!(std::mem::align_of::<f32>(), 4);
        assert_eq!(memoffset::offset_of!(SpotLightStd140, range), 84);
        assert_eq!(std::mem::size_of::<f32>(), 4);
        assert_eq!(std::mem::align_of::<f32>(), 4);
        assert_eq!(memoffset::offset_of!(SpotLightStd140, intensity), 88);
        assert_eq!(std::mem::size_of::<i32>(), 4);
        assert_eq!(std::mem::align_of::<i32>(), 4);
        assert_eq!(memoffset::offset_of!(SpotLightStd140, shadow_map), 92);
    }

    #[test]
    fn test_struct_directional_light_std140() {
        assert_eq!(std::mem::size_of::<DirectionalLightStd140>(), 64);
        assert_eq!(std::mem::size_of::<[f32; 3]>(), 12);
        assert_eq!(std::mem::align_of::<[f32; 3]>(), 4);
        assert_eq!(
            memoffset::offset_of!(DirectionalLightStd140, direction_ws),
            0
        );
        assert_eq!(std::mem::size_of::<[u8; 4]>(), 4);
        assert_eq!(std::mem::align_of::<[u8; 4]>(), 1);
        assert_eq!(memoffset::offset_of!(DirectionalLightStd140, _padding0), 12);
        assert_eq!(std::mem::size_of::<[f32; 3]>(), 12);
        assert_eq!(std::mem::align_of::<[f32; 3]>(), 4);
        assert_eq!(
            memoffset::offset_of!(DirectionalLightStd140, direction_vs),
            16
        );
        assert_eq!(std::mem::size_of::<[u8; 4]>(), 4);
        assert_eq!(std::mem::align_of::<[u8; 4]>(), 1);
        assert_eq!(memoffset::offset_of!(DirectionalLightStd140, _padding1), 28);
        assert_eq!(std::mem::size_of::<[f32; 4]>(), 16);
        assert_eq!(std::mem::align_of::<[f32; 4]>(), 4);
        assert_eq!(memoffset::offset_of!(DirectionalLightStd140, color), 32);
        assert_eq!(std::mem::size_of::<f32>(), 4);
        assert_eq!(std::mem::align_of::<f32>(), 4);
        assert_eq!(memoffset::offset_of!(DirectionalLightStd140, intensity), 48);
        assert_eq!(std::mem::size_of::<i32>(), 4);
        assert_eq!(std::mem::align_of::<i32>(), 4);
        assert_eq!(
            memoffset::offset_of!(DirectionalLightStd140, shadow_map),
            52
        );
        assert_eq!(std::mem::size_of::<[u8; 8]>(), 8);
        assert_eq!(std::mem::align_of::<[u8; 8]>(), 1);
        assert_eq!(memoffset::offset_of!(DirectionalLightStd140, _padding2), 56);
    }

    #[test]
    fn test_struct_point_light_std140() {
        assert_eq!(std::mem::size_of::<PointLightStd140>(), 64);
        assert_eq!(std::mem::size_of::<[f32; 3]>(), 12);
        assert_eq!(std::mem::align_of::<[f32; 3]>(), 4);
        assert_eq!(memoffset::offset_of!(PointLightStd140, position_ws), 0);
        assert_eq!(std::mem::size_of::<[u8; 4]>(), 4);
        assert_eq!(std::mem::align_of::<[u8; 4]>(), 1);
        assert_eq!(memoffset::offset_of!(PointLightStd140, _padding0), 12);
        assert_eq!(std::mem::size_of::<[f32; 3]>(), 12);
        assert_eq!(std::mem::align_of::<[f32; 3]>(), 4);
        assert_eq!(memoffset::offset_of!(PointLightStd140, position_vs), 16);
        assert_eq!(std::mem::size_of::<[u8; 4]>(), 4);
        assert_eq!(std::mem::align_of::<[u8; 4]>(), 1);
        assert_eq!(memoffset::offset_of!(PointLightStd140, _padding1), 28);
        assert_eq!(std::mem::size_of::<[f32; 4]>(), 16);
        assert_eq!(std::mem::align_of::<[f32; 4]>(), 4);
        assert_eq!(memoffset::offset_of!(PointLightStd140, color), 32);
        assert_eq!(std::mem::size_of::<f32>(), 4);
        assert_eq!(std::mem::align_of::<f32>(), 4);
        assert_eq!(memoffset::offset_of!(PointLightStd140, range), 48);
        assert_eq!(std::mem::size_of::<f32>(), 4);
        assert_eq!(std::mem::align_of::<f32>(), 4);
        assert_eq!(memoffset::offset_of!(PointLightStd140, intensity), 52);
        assert_eq!(std::mem::size_of::<i32>(), 4);
        assert_eq!(std::mem::align_of::<i32>(), 4);
        assert_eq!(memoffset::offset_of!(PointLightStd140, shadow_map), 56);
        assert_eq!(std::mem::size_of::<[u8; 4]>(), 4);
        assert_eq!(std::mem::align_of::<[u8; 4]>(), 1);
        assert_eq!(memoffset::offset_of!(PointLightStd140, _padding2), 60);
    }

    #[test]
    fn test_struct_shadow_map_cube_data_std140() {
        assert_eq!(std::mem::size_of::<ShadowMapCubeDataStd140>(), 16);
        assert_eq!(std::mem::size_of::<f32>(), 4);
        assert_eq!(std::mem::align_of::<f32>(), 4);
        assert_eq!(
            memoffset::offset_of!(ShadowMapCubeDataStd140, cube_map_projection_near_z),
            0
        );
        assert_eq!(std::mem::size_of::<f32>(), 4);
        assert_eq!(std::mem::align_of::<f32>(), 4);
        assert_eq!(
            memoffset::offset_of!(ShadowMapCubeDataStd140, cube_map_projection_far_z),
            4
        );
        assert_eq!(std::mem::size_of::<[u8; 8]>(), 8);
        assert_eq!(std::mem::align_of::<[u8; 8]>(), 1);
        assert_eq!(memoffset::offset_of!(ShadowMapCubeDataStd140, _padding0), 8);
    }

    #[test]
    fn test_struct_material_data_ubo_std140() {
        assert_eq!(std::mem::size_of::<MaterialDataUboStd140>(), 80);
        assert_eq!(std::mem::size_of::<MaterialDataStd140>(), 80);
        assert_eq!(std::mem::align_of::<MaterialDataStd140>(), 4);
        assert_eq!(memoffset::offset_of!(MaterialDataUboStd140, data), 0);
    }
}
