use crate::resources::descriptor_sets::DescriptorSetBindingKey;
use fnv::FnvHashMap;
use rafx_api::{
    RafxBuffer, RafxBufferDef, RafxDeviceContext, RafxMemoryUsage, RafxQueueType, RafxResourceType,
    RafxResult,
};

//
// Metadata about a buffer for a particular descriptor in a descriptor layout
//
#[derive(Clone)]
pub(super) struct DescriptorSetPoolRequiredBufferInfo {
    pub(super) dst_element: DescriptorSetBindingKey,
    pub(super) descriptor_type: RafxResourceType,
    pub(super) per_descriptor_size: u32,
    pub(super) per_descriptor_stride: u32,
}

//
// Creates and manages the internal buffers for a single binding within a descriptor pool chunk
//
pub(super) struct DescriptorBindingBufferSet {
    pub(super) buffer: RafxBuffer,
    pub(super) buffer_info: DescriptorSetPoolRequiredBufferInfo,
}

impl DescriptorBindingBufferSet {
    fn new(
        device_context: &RafxDeviceContext,
        descriptor_set_count: u32,
        buffer_info: &DescriptorSetPoolRequiredBufferInfo,
    ) -> RafxResult<Self> {
        //This is the only one we support right now
        assert!(buffer_info.descriptor_type == RafxResourceType::UNIFORM_BUFFER);

        let buffer = device_context.create_buffer(&RafxBufferDef {
            size: (buffer_info.per_descriptor_stride * descriptor_set_count) as u64,
            memory_usage: RafxMemoryUsage::CpuToGpu,
            resource_type: RafxResourceType::UNIFORM_BUFFER,
            queue_type: RafxQueueType::Graphics,
            ..Default::default()
        })?;

        Ok(DescriptorBindingBufferSet {
            buffer,
            buffer_info: buffer_info.clone(),
        })
    }
}

//
// Creates and manages the internal buffers for a descriptor pool chunk
//
pub(super) struct DescriptorLayoutBufferSet {
    pub(super) buffer_sets: FnvHashMap<DescriptorSetBindingKey, DescriptorBindingBufferSet>,
}

impl DescriptorLayoutBufferSet {
    pub(super) fn new(
        device_context: &RafxDeviceContext,
        buffer_infos: &[DescriptorSetPoolRequiredBufferInfo],
        descriptor_set_count: u32,
    ) -> RafxResult<Self> {
        let mut buffer_sets: FnvHashMap<DescriptorSetBindingKey, DescriptorBindingBufferSet> =
            Default::default();
        for buffer_info in buffer_infos {
            let buffer = DescriptorBindingBufferSet::new(
                device_context,
                descriptor_set_count,
                &buffer_info,
            )?;
            buffer_sets.insert(buffer_info.dst_element, buffer);
        }

        Ok(DescriptorLayoutBufferSet { buffer_sets })
    }
}
