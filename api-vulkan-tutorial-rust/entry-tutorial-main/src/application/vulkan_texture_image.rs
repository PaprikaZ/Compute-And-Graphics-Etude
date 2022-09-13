use std::fs::File;
use std::path::Path;
use std::ptr::copy_nonoverlapping;

use ::png;
use ::vulkan::VULKAN_QUEUE_FAMILY_IGNORED;
use ::vulkan::prelude::version1_2::*;
use ::vulkan::VulkanErrorCode;
use ::vulkan::VulkanInstance;
use ::vulkan::VulkanDevicePhysical;
use ::vulkan::VulkanBufferUsageFlagS;
use ::vulkan::VulkanMemoryPropertyFlagS;
use ::vulkan::VulkanMemoryMapFlagS;
use ::vulkan::VulkanFormat;
use ::vulkan::VulkanImageTiling;
use ::vulkan::VulkanImageUsageFlagS;
use ::vulkan::VulkanImageLayout;
use ::vulkan::VulkanImageCreateInformation;
use ::vulkan::VulkanImageType;
use ::vulkan::VulkanExtentD3;
use ::vulkan::VulkanSampleCountFlagS;
use ::vulkan::VulkanSharingMode;
use ::vulkan::VulkanMemoryAllocateInformation;
use ::vulkan::VulkanAccessFlagS;
use ::vulkan::VulkanPipelineStageFlagS;
use ::vulkan::VulkanImageSubResourceRange;
use ::vulkan::VulkanImageAspectFlagS;
use ::vulkan::VulkanImageMemoryBarrier;
use ::vulkan::VulkanMemoryBarrier;
use ::vulkan::VulkanImageSubresourceLayerS;
use ::vulkan::VulkanBufferImageCopy;
use ::vulkan::VulkanOffsetD3;
use ::vulkan::VulkanImage;
use ::vulkan::VulkanDeviceMemory;
use ::vulkan::VulkanCommandPool;
use ::vulkan::VulkanDependencyFlagS;
use ::vulkan::VulkanBufferMemoryBarrier;
use ::vulkan::VulkanQueue;
use ::vulkan::VulkanBuffer;

use crate::termination::TerminationProcessMain;
use crate::application::vulkan_buffer::ApplicationVulkanBuffer;
use crate::application::vulkan_memory::ApplicationVulkanMemory;

use super::vulkan_command_buffer::ApplicationVulkanCommandBufferOneTime;


pub struct ApplicationVulkanTextureImage {}

impl ApplicationVulkanTextureImage {
    unsafe fn copy_from_buffer(
        vulkan_logical_device: &VulkanDeviceLogical,
        vulkan_command_pool: VulkanCommandPool,
        vulkan_graphic_queue: VulkanQueue,
        vulkan_source_buffer: VulkanBuffer,
        vulkan_texture_image: VulkanImage,
        vulkan_texture_image_width: u32,
        vulkan_texture_image_height: u32)
     -> Result<(), TerminationProcessMain>
    {
        let vulkan_command_buffer =
            ApplicationVulkanCommandBufferOneTime::create_and_begin(vulkan_logical_device, vulkan_command_pool)?;
        let vulkan_image_sub_resource =
            VulkanImageSubresourceLayerS::builder()
            .aspect_mask(VulkanImageAspectFlagS::COLOR)
            .mip_level(0)
            .base_array_layer(0)
            .layer_count(1);
        let vulkan_buffer_image_copy =
            VulkanBufferImageCopy::builder()
            .buffer_offset(0)
            .buffer_row_length(0)
            .buffer_image_height(0)
            .image_subresource(vulkan_image_sub_resource)
            .image_offset(VulkanOffsetD3 { x: 0, y: 0, z: 0 })
            .image_extent(VulkanExtentD3 {
                width: vulkan_texture_image_width,
                height: vulkan_texture_image_height,
                depth: 1,
            });
        vulkan_logical_device.cmd_copy_buffer_to_image(
            vulkan_command_buffer,
            vulkan_source_buffer,
            vulkan_texture_image,
            VulkanImageLayout::TRANSFER_DST_OPTIMAL,
            &[vulkan_buffer_image_copy]);
        ApplicationVulkanCommandBufferOneTime::end_submit_wait(
            vulkan_logical_device, vulkan_command_pool, vulkan_command_buffer, vulkan_graphic_queue)?;
        Ok(())
    }
}
