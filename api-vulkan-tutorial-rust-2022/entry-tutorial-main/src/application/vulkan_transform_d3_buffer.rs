use std::mem::size_of;

use ::vulkan::prelude::version1_2::*;
use ::vulkan::VulkanInstance;
use ::vulkan::VulkanBuffer;
use ::vulkan::VulkanDeviceMemory;
use ::vulkan::VulkanImage;
use ::vulkan::VulkanDevicePhysical;
use ::vulkan::VulkanBufferUsageFlagS;
use ::vulkan::VulkanMemoryPropertyFlagS;

use crate::termination::TerminationProcessMain;
use crate::lib::transform_d3_model_view_projection::TransformD3ModelViewProjection;
use crate::application::vulkan_buffer::ApplicationVulkanBuffer;


pub struct ApplicationVulkanTransformD3Buffer {}

impl ApplicationVulkanTransformD3Buffer {
    pub unsafe fn create_main_all(
        vulkan_instance: &VulkanInstance,
        vulkan_physical_device: VulkanDevicePhysical,
        vulkan_logical_device: &VulkanDeviceLogical,
        vulkan_swapchain_image_s: &Vec<VulkanImage>)
     -> Result<(Vec<VulkanBuffer>, Vec<VulkanDeviceMemory>), TerminationProcessMain>
    {
        let mut vulkan_main_3d_transform_buffer_s = Vec::new();
        let mut vulkan_main_3d_transform_buffer_memory_s = Vec::new();
        for _ in 0..vulkan_swapchain_image_s.len() {
            let (buffer, memory) =
                ApplicationVulkanBuffer::create_with_memory(
                    &vulkan_instance,
                    vulkan_physical_device,
                    vulkan_logical_device,
                    size_of::<TransformD3ModelViewProjection>() as u64,
                    VulkanBufferUsageFlagS::UNIFORM_BUFFER,
                    VulkanMemoryPropertyFlagS::HOST_COHERENT | VulkanMemoryPropertyFlagS::HOST_VISIBLE)?;
            vulkan_main_3d_transform_buffer_s.push(buffer);
            vulkan_main_3d_transform_buffer_memory_s.push(memory);
        }
        Ok((vulkan_main_3d_transform_buffer_s, vulkan_main_3d_transform_buffer_memory_s))
    }
}