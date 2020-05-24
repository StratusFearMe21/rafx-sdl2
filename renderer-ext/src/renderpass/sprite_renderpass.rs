use std::mem;
use ash::vk;
use ash::prelude::VkResult;
use std::ffi::CString;
use std::mem::ManuallyDrop;

use ash::version::DeviceV1_0;

use renderer_shell_vulkan::{VkDevice, VkDeviceContext};
use renderer_shell_vulkan::VkSwapchain;
use renderer_shell_vulkan::offset_of;
use renderer_shell_vulkan::SwapchainInfo;
use renderer_shell_vulkan::VkQueueFamilyIndices;
use renderer_shell_vulkan::VkBuffer;
use renderer_shell_vulkan::util;

use renderer_shell_vulkan::VkImage;
use image::error::ImageError::Decoding;
use std::process::exit;
use image::{GenericImageView, ImageFormat};
use ash::vk::ShaderStageFlags;

use crate::time::TimeState;
//use crate::resource_managers::{SpriteResourceManager, PipelineInfo};
//use crate::pipeline_manager::PipelineManager;
use crate::pipeline_description::{AttachmentReference, SwapchainSurfaceInfo};
use crate::asset_resource::AssetResource;
use std::hint::unreachable_unchecked;
use crate::resource_managers::PipelineSwapchainInfo;

struct SpriteRenderpassStats {
    draw_call_count: u32,
}

/// Per-pass "global" data
#[derive(Clone, Debug, Copy)]
struct SpriteUniformBufferObject {
    // View and projection matrices
    view_proj: [[f32; 4]; 4],
}

/// Vertex format for vertices sent to the GPU
#[derive(Clone, Debug, Copy)]
#[repr(C)]
pub struct SpriteVertex {
    pub pos: [f32; 2],
    pub tex_coord: [f32; 2],
    //color: [u8; 4],
}

/// Used as static data to represent a quad
#[derive(Clone, Debug, Copy)]
struct QuadVertex {
    pos: [f32; 3],
    tex_coord: [f32; 2],
}

/// Static data the represents a "unit" quad
const QUAD_VERTEX_LIST: [QuadVertex; 4] = [
    QuadVertex {
        pos: [-0.5, -0.5, 0.0],
        tex_coord: [1.0, 0.0],
    },
    QuadVertex {
        pos: [0.5, -0.5, 0.0],
        tex_coord: [0.0, 0.0],
    },
    QuadVertex {
        pos: [0.5, 0.5, 0.0],
        tex_coord: [0.0, 1.0],
    },
    QuadVertex {
        pos: [-0.5, 0.5, 0.0],
        tex_coord: [1.0, 1.0],
    },
];

/// Draw order of QUAD_VERTEX_LIST
const QUAD_INDEX_LIST: [u16; 6] = [0, 1, 2, 2, 3, 0];

/// Draws sprites
pub struct VkSpriteRenderPass {
    pub device_context: VkDeviceContext,
    pub swapchain_info: SwapchainInfo,

    pipeline_info: PipelineSwapchainInfo,

    // This contains bindings for the UBO containing a view/proj matrix and a sampler
    //pub descriptor_set_layout_per_pass: vk::DescriptorSetLayout,
    pub descriptor_pool_per_pass: vk::DescriptorPool,

    // One per present index
    pub descriptor_sets_per_pass: Vec<vk::DescriptorSet>,

    // Static resources for the renderpass, including a frame buffer per present index
    // pub pipeline_layout: vk::PipelineLayout,
    // pub renderpass: vk::RenderPass,
    // pub pipeline: vk::Pipeline,
    pub frame_buffers: Vec<vk::Framebuffer>,

    // Command pool and list of command buffers, one per present index
    pub command_pool: vk::CommandPool,
    pub command_buffers: Vec<vk::CommandBuffer>,

    // Buffers for sending data to be rendered to the GPU
    // Indexed by present index, then vertex buffer.
    pub vertex_buffers: Vec<Vec<ManuallyDrop<VkBuffer>>>,
    pub index_buffers: Vec<Vec<ManuallyDrop<VkBuffer>>>,

    // Sends the view/proj matrix to the GPU. One per present index.
    pub uniform_buffers: Vec<ManuallyDrop<VkBuffer>>,

    // The sampler used to draw a sprite
    //pub image_sampler: vk::Sampler,
}

impl VkSpriteRenderPass {
    pub fn new(
        device_context: &VkDeviceContext,
        swapchain: &VkSwapchain,
        pipeline_info: PipelineSwapchainInfo,
        swapchain_surface_info: &SwapchainSurfaceInfo,
    ) -> VkResult<Self> {
        //
        // Command Buffers
        //
        let command_pool = Self::create_command_pool(
            &device_context.device(),
            &device_context.queue_family_indices(),
        )?;

        //
        // Static resources used by GPU
        //
        //let image_sampler = Self::create_texture_image_sampler(&device_context.device());

        let mut uniform_buffers = Vec::with_capacity(swapchain.swapchain_info.image_count);
        for _ in 0..swapchain.swapchain_info.image_count {
            uniform_buffers.push(Self::create_uniform_buffer(&device_context)?)
        }

        //
        // Descriptors
        //
        let descriptor_set_layout_per_pass = pipeline_info.descriptor_set_layouts[0].get_raw();
        let descriptor_set_layout_per_sprite = pipeline_info.descriptor_set_layouts[1].get_raw();

        let descriptor_pool_per_pass = Self::create_descriptor_pool_per_pass(
            &device_context.device(),
            swapchain.swapchain_info.image_count as u32,
        )?;

        let descriptor_sets_per_pass = Self::create_descriptor_sets_per_pass(
            &device_context.device(),
            &descriptor_pool_per_pass,
            descriptor_set_layout_per_pass.descriptor_set_layout,
            swapchain.swapchain_info.image_count,
            &uniform_buffers,
            //&image_sampler,
        )?;

        let descriptor_set_layouts = [
            descriptor_set_layout_per_pass,
            descriptor_set_layout_per_sprite,
        ];

        //
        // Renderpass Resources
        //
        let frame_buffers = Self::create_framebuffers(
            &device_context.device(),
            &swapchain.swapchain_image_views,
            &swapchain.swapchain_info,
            &pipeline_info.renderpass.get_raw(),
        );

        let command_buffers = Self::create_command_buffers(
            &device_context.device(),
            &swapchain.swapchain_info,
            &command_pool,
        )?;

        //
        // Containers for dynamically-allocated resources
        //
        let mut vertex_buffers = Vec::with_capacity(swapchain.swapchain_info.image_count);
        let mut index_buffers = Vec::with_capacity(swapchain.swapchain_info.image_count);
        for _ in 0..swapchain.swapchain_info.image_count {
            vertex_buffers.push(vec![]);
            index_buffers.push(vec![]);
        }

        Ok(VkSpriteRenderPass {
            device_context: device_context.clone(),
            swapchain_info: swapchain.swapchain_info.clone(),
            pipeline_info,
            frame_buffers,
            command_pool,
            command_buffers,
            vertex_buffers,
            index_buffers,
            uniform_buffers,
            descriptor_pool_per_pass,
            descriptor_sets_per_pass,
            //image_sampler,
        })
    }

    fn create_command_pool(
        logical_device: &ash::Device,
        queue_family_indices: &VkQueueFamilyIndices,
    ) -> VkResult<vk::CommandPool> {
        log::info!(
            "Creating command pool with queue family index {}",
            queue_family_indices.graphics_queue_family_index
        );
        let pool_create_info = vk::CommandPoolCreateInfo::builder()
            .flags(
                vk::CommandPoolCreateFlags::TRANSIENT
                    | vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER,
            )
            .queue_family_index(queue_family_indices.graphics_queue_family_index);

        unsafe { logical_device.create_command_pool(&pool_create_info, None) }
    }

    fn create_uniform_buffer(device_context: &VkDeviceContext) -> VkResult<ManuallyDrop<VkBuffer>> {
        let buffer = VkBuffer::new(
            device_context,
            vk_mem::MemoryUsage::CpuToGpu,
            vk::BufferUsageFlags::UNIFORM_BUFFER,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            mem::size_of::<SpriteUniformBufferObject>() as u64,
        );

        Ok(ManuallyDrop::new(buffer?))
    }

    // pub fn create_texture_image_sampler(logical_device: &ash::Device) -> vk::Sampler {
    //     let sampler_info = vk::SamplerCreateInfo::builder()
    //         .mag_filter(vk::Filter::LINEAR)
    //         .min_filter(vk::Filter::LINEAR)
    //         .address_mode_u(vk::SamplerAddressMode::REPEAT)
    //         .address_mode_v(vk::SamplerAddressMode::REPEAT)
    //         .address_mode_w(vk::SamplerAddressMode::REPEAT)
    //         .anisotropy_enable(false)
    //         .max_anisotropy(1.0)
    //         .border_color(vk::BorderColor::INT_OPAQUE_BLACK)
    //         .unnormalized_coordinates(false)
    //         .compare_enable(false)
    //         .compare_op(vk::CompareOp::ALWAYS)
    //         .mipmap_mode(vk::SamplerMipmapMode::LINEAR)
    //         .mip_lod_bias(0.0)
    //         .min_lod(0.0)
    //         .max_lod(0.0);
    //
    //     unsafe { logical_device.create_sampler(&sampler_info, None).unwrap() }
    // }

    fn create_descriptor_pool_per_pass(
        logical_device: &ash::Device,
        swapchain_image_count: u32,
    ) -> VkResult<vk::DescriptorPool> {
        let pool_sizes = [
            vk::DescriptorPoolSize::builder()
                .ty(vk::DescriptorType::UNIFORM_BUFFER)
                .descriptor_count(3)
                .build(),
            vk::DescriptorPoolSize::builder()
                .ty(vk::DescriptorType::SAMPLER)
                .descriptor_count(3)
                .build(),
        ];

        let descriptor_pool_info = vk::DescriptorPoolCreateInfo::builder()
            .pool_sizes(&pool_sizes)
            .max_sets(3);

        unsafe { logical_device.create_descriptor_pool(&descriptor_pool_info, None) }
    }

    fn create_descriptor_sets_per_pass(
        logical_device: &ash::Device,
        descriptor_pool: &vk::DescriptorPool,
        descriptor_set_layout: vk::DescriptorSetLayout,
        swapchain_image_count: usize,
        uniform_buffers: &Vec<ManuallyDrop<VkBuffer>>,
//        image_sampler: &vk::Sampler,
    ) -> VkResult<Vec<vk::DescriptorSet>> {
        // DescriptorSetAllocateInfo expects an array with an element per set
        let descriptor_set_layouts = vec![descriptor_set_layout; swapchain_image_count];

        let alloc_info = vk::DescriptorSetAllocateInfo::builder()
            .descriptor_pool(*descriptor_pool)
            .set_layouts(descriptor_set_layouts.as_slice());

        let descriptor_sets = unsafe { logical_device.allocate_descriptor_sets(&alloc_info) }?;

        for i in 0..swapchain_image_count {
            let descriptor_buffer_infos = [vk::DescriptorBufferInfo::builder()
                .buffer(uniform_buffers[i as usize].buffer())
                .offset(0)
                .range(mem::size_of::<SpriteUniformBufferObject>() as u64)
                .build()];

            // let sampler_descriptor_image_infos = [vk::DescriptorImageInfo::builder()
            //     .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
            //     .sampler(*image_sampler)
            //     .build()];

            let descriptor_writes = [
                vk::WriteDescriptorSet::builder()
                    .dst_set(descriptor_sets[i])
                    .dst_binding(0)
                    .dst_array_element(0)
                    .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
                    .buffer_info(&descriptor_buffer_infos)
                    .build(),
                // vk::WriteDescriptorSet::builder()
                //     .dst_set(descriptor_sets[i])
                //     .dst_binding(1)
                //     .dst_array_element(0)
                //     .descriptor_type(vk::DescriptorType::SAMPLER)
                //     .image_info(&sampler_descriptor_image_infos)
                //     .build(),
            ];

            unsafe {
                logical_device.update_descriptor_sets(&descriptor_writes, &[]);
            }
        }

        Ok(descriptor_sets)
    }

    fn create_framebuffers(
        logical_device: &ash::Device,
        swapchain_image_views: &Vec<vk::ImageView>,
        swapchain_info: &SwapchainInfo,
        renderpass: &vk::RenderPass,
    ) -> Vec<vk::Framebuffer> {
        swapchain_image_views
            .iter()
            .map(|&swapchain_image_view| {
                let framebuffer_attachments = [swapchain_image_view];
                let frame_buffer_create_info = vk::FramebufferCreateInfo::builder()
                    .render_pass(*renderpass)
                    .attachments(&framebuffer_attachments)
                    .width(swapchain_info.extents.width)
                    .height(swapchain_info.extents.height)
                    .layers(1);

                unsafe {
                    //TODO: Pass this error up
                    logical_device
                        .create_framebuffer(&frame_buffer_create_info, None)
                        .unwrap()
                }
            })
            .collect()
    }

    fn create_command_buffers(
        logical_device: &ash::Device,
        swapchain_info: &SwapchainInfo,
        command_pool: &vk::CommandPool,
    ) -> VkResult<Vec<vk::CommandBuffer>> {
        let command_buffer_allocate_info = vk::CommandBufferAllocateInfo::builder()
            .command_buffer_count(swapchain_info.image_count as u32)
            .command_pool(*command_pool)
            .level(vk::CommandBufferLevel::PRIMARY);

        unsafe { logical_device.allocate_command_buffers(&command_buffer_allocate_info) }
    }

    fn update_command_buffer(
        device_context: &VkDeviceContext,
        swapchain_info: &SwapchainInfo,
        renderpass: &vk::RenderPass,
        framebuffer: &vk::Framebuffer,
        pipeline: &vk::Pipeline,
        pipeline_layout: &vk::PipelineLayout,
        command_buffer: &vk::CommandBuffer,
        vertex_buffers: &mut Vec<ManuallyDrop<VkBuffer>>,
        index_buffers: &mut Vec<ManuallyDrop<VkBuffer>>,
        descriptor_set_per_pass: &vk::DescriptorSet,
        descriptor_set_per_texture: &[vk::DescriptorSet],
        time_state: &TimeState,
    ) -> VkResult<()> {
        let command_buffer_begin_info = vk::CommandBufferBeginInfo::builder();

        let clear_values = [vk::ClearValue {
            color: vk::ClearColorValue {
                float32: [0.0, 0.0, 0.0, 1.0],
            },
        }];

        fn drop_old_buffers(buffers: &mut Vec<ManuallyDrop<VkBuffer>>) {
            for b in buffers.iter_mut() {
                unsafe {
                    ManuallyDrop::drop(b);
                }
            }

            buffers.clear();
        }

        drop_old_buffers(vertex_buffers);
        drop_old_buffers(index_buffers);

        #[derive(Debug)]
        struct Sprite {
            position: glam::Vec3,
            texture_size: glam::Vec2,
            scale: f32,
            rotation: f32,
            texture_descriptor_index: u32,
        }

        #[derive(Debug)]
        struct DrawCall {
            index_buffer_first_element: u16,
            index_buffer_count: u16,
            texture_descriptor_index: u32,
        }

        const SPRITE_COUNT: usize = 50;
        let mut sprites = Vec::with_capacity(SPRITE_COUNT);

        if !descriptor_set_per_texture.is_empty() {
            let mut rng: pcg_rand::Pcg32 = rand::SeedableRng::seed_from_u64(42);
            use rand::Rng;
            use std::f32::consts::PI;

            for i in (0..SPRITE_COUNT) {
                sprites.push(Sprite {
                    position: glam::Vec3::new(
                        rng.gen_range(-400.0, 400.0),
                        rng.gen_range(-300.0, 300.0),
                        0.0,
                    ),
                    //texture_size: glam::Vec2::new(800.0, 450.0),
                    texture_size: glam::Vec2::new(850.0, 400.0),
                    //scale: rng.gen_range(0.1, 0.2),
                    scale: rng.gen_range(0.1, 0.2),
                    rotation: rng.gen_range(-180.0, 180.0)
                        + (rng.gen_range(-0.5, 0.5)
                            * (time_state.total_time().as_secs_f32() * 180.0)),
                    texture_descriptor_index: rng
                        .gen_range(0, descriptor_set_per_texture.len() as u32),
                });
            }
        }

        // let sprites = [
        //     Sprite {
        //         position: glam::Vec3::new(0.0, 0.0, 0.0),
        //         texture_size: glam::Vec2::new(800.0, 450.0),
        //         scale: 1.0,
        //         rotation: 0.0,
        //         texture_descriptor_index: 0
        //     },
        //     Sprite {
        //         position: glam::Vec3::new(-200.0, 0.0, 0.0),
        //         texture_size: glam::Vec2::new(800.0, 450.0),
        //         scale: 0.5,
        //         rotation: 30.0,
        //         texture_descriptor_index: 1
        //     },
        // ];

        let mut draw_calls = Vec::with_capacity(sprites.len());

        let mut vertex_list: Vec<SpriteVertex> =
            Vec::with_capacity(QUAD_VERTEX_LIST.len() * sprites.len());
        let mut index_list: Vec<u16> = Vec::with_capacity(QUAD_INDEX_LIST.len() * sprites.len());
        for sprite in &sprites {
            let draw_call = DrawCall {
                index_buffer_first_element: 0,
                index_buffer_count: 4,
                texture_descriptor_index: sprite.texture_descriptor_index,
            };

            const DEG_TO_RAD: f32 = std::f32::consts::PI / 180.0;

            let matrix = glam::Mat4::from_translation(sprite.position)
                * glam::Mat4::from_rotation_z(sprite.rotation * DEG_TO_RAD)
                * glam::Mat4::from_scale(glam::Vec3::new(
                    sprite.texture_size.x() * sprite.scale,
                    sprite.texture_size.y() * sprite.scale,
                    1.0,
                ));

            let vertex_buffer_first_element = vertex_list.len() as u16;

            for vertex in &QUAD_VERTEX_LIST {
                //let pos = vertex.pos;
                let transformed_pos = matrix.transform_point3(vertex.pos.into());

                vertex_list.push(SpriteVertex {
                    pos: transformed_pos.truncate().into(),
                    tex_coord: vertex.tex_coord,
                    //color: [255, 255, 255, 255]
                });
            }

            let index_buffer_first_element = index_list.len() as u16;
            for index in &QUAD_INDEX_LIST {
                index_list.push((*index + vertex_buffer_first_element));
            }

            let draw_call = DrawCall {
                index_buffer_first_element,
                index_buffer_count: QUAD_INDEX_LIST.len() as u16,
                texture_descriptor_index: sprite.texture_descriptor_index,
            };

            draw_calls.push(draw_call);
        }

        // //const QUADS_TO_DRAW : usize = 1;
        // let mut vertex_list : Vec<Vertex> = Vec::with_capacity(QUAD_VERTEX_LIST.len() * QUADS_TO_DRAW);
        // let mut index_list : Vec<u16> = Vec::with_capacity(QUAD_INDEX_LIST.len() * QUADS_TO_DRAW);
        //
        // {
        //     //let scoped_timer = crate::time::ScopeTimer::new("build buffer data");
        //     for quad_index in 0..QUADS_TO_DRAW {
        //         for vertex in &QUAD_VERTEX_LIST {
        //             vertex_list.push(*vertex);
        //         }
        //
        //         for index in &QUAD_INDEX_LIST {
        //             index_list.push(*index + (QUAD_VERTEX_LIST.len() * quad_index) as u16);
        //         }
        //     }
        // }

        let mut draw_list_count = 0;
        if !sprites.is_empty() {
            let vertex_buffer = {
                let vertex_buffer_size =
                    vertex_list.len() as u64 * std::mem::size_of::<SpriteVertex>() as u64;
                let mut vertex_buffer = VkBuffer::new(
                    device_context,
                    vk_mem::MemoryUsage::CpuToGpu,
                    vk::BufferUsageFlags::VERTEX_BUFFER,
                    vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
                    vertex_buffer_size,
                )?;

                vertex_buffer.write_to_host_visible_buffer(vertex_list.as_slice())?;
                vertex_buffer
            };

            //TODO: Duplicated code here
            let index_buffer = {
                let index_buffer_size = index_list.len() as u64 * std::mem::size_of::<u16>() as u64;
                let mut index_buffer = VkBuffer::new(
                    device_context,
                    vk_mem::MemoryUsage::CpuToGpu,
                    vk::BufferUsageFlags::INDEX_BUFFER,
                    vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
                    index_buffer_size,
                )?;

                index_buffer.write_to_host_visible_buffer(index_list.as_slice())?;
                index_buffer
            };

            vertex_buffers.push(ManuallyDrop::new(vertex_buffer));
            index_buffers.push(ManuallyDrop::new(index_buffer));
            draw_list_count += 1;
        }

        let render_pass_begin_info = vk::RenderPassBeginInfo::builder()
            .render_pass(*renderpass)
            .framebuffer(*framebuffer)
            .render_area(vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: swapchain_info.extents.clone(),
            })
            .clear_values(&clear_values);

        // Implicitly resets the command buffer
        unsafe {
            let logical_device = device_context.device();
            logical_device.begin_command_buffer(*command_buffer, &command_buffer_begin_info)?;

            logical_device.cmd_begin_render_pass(
                *command_buffer,
                &render_pass_begin_info,
                vk::SubpassContents::INLINE,
            );

            if !sprites.is_empty() {
                logical_device.cmd_bind_pipeline(
                    *command_buffer,
                    vk::PipelineBindPoint::GRAPHICS,
                    *pipeline,
                );

                // Bind per-pass data (UBO with view/proj matrix, sampler)
                logical_device.cmd_bind_descriptor_sets(
                    *command_buffer,
                    vk::PipelineBindPoint::GRAPHICS,
                    *pipeline_layout,
                    0,
                    &[*descriptor_set_per_pass],
                    &[],
                );

                logical_device.cmd_bind_vertex_buffers(
                    *command_buffer,
                    0, // first binding
                    &[vertex_buffers[0].buffer()],
                    &[0], // offsets
                );

                logical_device.cmd_bind_index_buffer(
                    *command_buffer,
                    index_buffers[0].buffer(),
                    0, // offset
                    vk::IndexType::UINT16,
                );

                for draw_call in draw_calls {
                    // Bind per-draw-call data (i.e. texture)
                    logical_device.cmd_bind_descriptor_sets(
                        *command_buffer,
                        vk::PipelineBindPoint::GRAPHICS,
                        *pipeline_layout,
                        1,
                        &[descriptor_set_per_texture[draw_call.texture_descriptor_index as usize]],
                        &[],
                    );

                    logical_device.cmd_draw_indexed(
                        *command_buffer,
                        draw_call.index_buffer_count as u32,
                        1,
                        draw_call.index_buffer_first_element as u32,
                        0,
                        0,
                    );
                }
            }

            logical_device.cmd_end_render_pass(*command_buffer);
            logical_device.end_command_buffer(*command_buffer)
        }
    }

    //TODO: This is doing a GPU idle wait, would be better to integrate it into the command
    // buffer
    pub fn update_uniform_buffer(
        &mut self,
        swapchain_image_index: usize,
        extents: vk::Extent2D,
        hidpi_factor: f64,
    ) -> VkResult<()> {
        // // Pixel-perfect rendering...
        // let half_width = (extents.width as f64 / hidpi_factor) as f32;
        // let half_height = (extents.height as f64 / hidpi_factor) as f32;

        // Resolution-independent rendering...
        let fov = extents.width as f32 / extents.height as f32;
        let half_width = 400.0;
        let half_height = 400.0 / fov;

        let proj = orthographic_rh_gl(
            -half_width,
            half_width,
            -half_height,
            half_height,
            -100.0,
            100.0,
        );

        let ubo = SpriteUniformBufferObject { view_proj: proj };

        self.uniform_buffers[swapchain_image_index].write_to_host_visible_buffer(&[ubo])
    }

    pub fn update(
        &mut self,
        present_index: usize,
        hidpi_factor: f64,
        //sprite_resource_manager: &SpriteResourceManager,
        descriptor_set_per_texture: &[vk::DescriptorSet],
        time_state: &TimeState,
    ) -> VkResult<()> {
        //TODO: Integrate this into the command buffer we create below
        self.update_uniform_buffer(present_index, self.swapchain_info.extents, hidpi_factor)?;

        Self::update_command_buffer(
            &self.device_context,
            &self.swapchain_info,
            //&self.renderpass,
            &self.pipeline_info.renderpass.get_raw(),
            &self.frame_buffers[present_index],
            // &self.pipeline,
            // &self.pipeline_layout,
            &self.pipeline_info.pipeline.get_raw().pipeline,
            &self.pipeline_info.pipeline_layout.get_raw().pipeline_layout,
            &self.command_buffers[present_index],
            &mut self.vertex_buffers[present_index],
            &mut self.index_buffers[present_index],
            &self.descriptor_sets_per_pass[present_index],
            //sprite_resource_manager.descriptor_sets(),
            descriptor_set_per_texture,
            //&vec![],
            time_state,
        )
    }
}

impl Drop for VkSpriteRenderPass {
    fn drop(&mut self) {
        log::trace!("destroying VkSpriteRenderPass");

        fn drop_all_buffer_lists(buffer_list: &mut Vec<Vec<ManuallyDrop<VkBuffer>>>) {
            for buffers in buffer_list {
                for mut b in &mut *buffers {
                    unsafe {
                        ManuallyDrop::drop(&mut b);
                    }
                }
            }
        }

        unsafe {
            let device = self.device_context.device();
            //device.destroy_sampler(self.image_sampler, None);

            for uniform_buffer in &mut self.uniform_buffers {
                ManuallyDrop::drop(uniform_buffer);
            }

            drop_all_buffer_lists(&mut self.vertex_buffers);
            drop_all_buffer_lists(&mut self.index_buffers);

            device.destroy_command_pool(self.command_pool, None);

            for frame_buffer in &self.frame_buffers {
                device.destroy_framebuffer(*frame_buffer, None);
            }

            // device.destroy_pipeline(self.pipeline, None);
            // device.destroy_pipeline_layout(self.pipeline_layout, None);
            // device.destroy_render_pass(self.renderpass, None);

            device.destroy_descriptor_pool(self.descriptor_pool_per_pass, None);
            //device.destroy_descriptor_set_layout(self.descriptor_set_layout_per_pass, None);
        }

        log::trace!("destroyed VkSpriteRenderPass");
    }
}

// This is almost copy-pasted from glam. I wanted to avoid pulling in the entire library for a
// single function
pub fn orthographic_rh_gl(
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    near: f32,
    far: f32,
) -> [[f32; 4]; 4] {
    let a = 2.0 / (right - left);
    let b = 2.0 / (top - bottom);
    let c = -2.0 / (far - near);
    let tx = -(right + left) / (right - left);
    let ty = -(top + bottom) / (top - bottom);
    let tz = -(far + near) / (far - near);

    [
        [a, 0.0, 0.0, 0.0],
        [0.0, b, 0.0, 0.0],
        [0.0, 0.0, c, 0.0],
        [tx, ty, tz, 1.0],
    ]
}