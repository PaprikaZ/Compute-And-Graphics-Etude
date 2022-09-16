use std::cmp::max;

use ::vulkan::VULKAN_QUEUE_FAMILY_IGNORED;
use ::vulkan::prelude::version1_2::*;
use ::vulkan::VulkanInstance;
use ::vulkan::VulkanDevicePhysical;
use ::vulkan::VulkanImage;
use ::vulkan::VulkanFormat;
use ::vulkan::VulkanFormatFeatureFlagS;
use ::vulkan::VulkanCommandPool;
use ::vulkan::VulkanImageSubResourceRange;
use ::vulkan::VulkanImageMemoryBarrier;
use ::vulkan::VulkanImageLayout;
use ::vulkan::VulkanAccessFlagS;
use ::vulkan::VulkanPipelineStageFlagS;
use ::vulkan::VulkanDependencyFlagS;
use ::vulkan::VulkanMemoryBarrier;
use ::vulkan::VulkanBufferMemoryBarrier;
use ::vulkan::VulkanImageSubresourceLayerS;
use ::vulkan::VulkanImageAspectFlagS;
use ::vulkan::VulkanOffsetD3;
use ::vulkan::VulkanImageBlit;
use ::vulkan::VulkanFilter;
use ::vulkan::VulkanQueue;
use ::vulkan::VulkanMipLevel;

use crate::termination::TerminationProcessMain;
use crate::application::vulkan_command_buffer::ApplicationVulkanCommandBufferOneTime;


pub struct ApplicationVulkanMipmap {}

impl ApplicationVulkanMipmap {
    pub unsafe fn generate(
        vulkan_instance: &VulkanInstance,
        vulkan_physical_device: VulkanDevicePhysical,
        vulkan_logical_device: &VulkanDeviceLogical,
        vulkan_command_pool: VulkanCommandPool,
        vulkan_graphic_queue: VulkanQueue,
        vulkan_image: VulkanImage,
        vulkan_image_format: VulkanFormat,
        vulkan_image_width: u32,
        vulkan_image_height: u32,
        vulkan_mip_level: VulkanMipLevel)
     -> Result<(), TerminationProcessMain>
    {
        let be_physical_device_support_linear_blitting =
            Self::is_physical_device_support_linear_blitting(
                vulkan_instance, vulkan_physical_device, vulkan_image_format);
        if !be_physical_device_support_linear_blitting {
            return Err(TerminationProcessMain::InitializationVulkanDevicePhysicalSampledImageFilterLinearNotSupport);
        }
        let vulkan_command_buffer =
            ApplicationVulkanCommandBufferOneTime::create_and_begin(vulkan_logical_device, vulkan_command_pool)?;
        let mut vulkan_mip_width = vulkan_image_width;
        let mut vulkan_mip_height = vulkan_image_height;
        for iterating_mip_level in 1..vulkan_mip_level.as_raw() {
            let vulkan_image_sub_resource =
                VulkanImageSubResourceRange::builder()
                .aspect_mask(VulkanImageAspectFlagS::COLOR)
                .base_array_layer(0)
                .layer_count(1)
                .level_count(1)
                .base_mip_level(iterating_mip_level - 1);
            let vulkan_image_memory_barrier =
                VulkanImageMemoryBarrier::builder()
                .image(vulkan_image)
                .subresource_range(vulkan_image_sub_resource)
                .src_queue_family_index(VULKAN_QUEUE_FAMILY_IGNORED)
                .dst_queue_family_index(VULKAN_QUEUE_FAMILY_IGNORED)
                .old_layout(VulkanImageLayout::TRANSFER_DST_OPTIMAL)
                .new_layout(VulkanImageLayout::TRANSFER_SRC_OPTIMAL)
                .src_access_mask(VulkanAccessFlagS::TRANSFER_WRITE)
                .dst_access_mask(VulkanAccessFlagS::TRANSFER_READ);
            vulkan_logical_device.cmd_pipeline_barrier(
                vulkan_command_buffer,
                VulkanPipelineStageFlagS::TRANSFER,
                VulkanPipelineStageFlagS::TRANSFER,
                VulkanDependencyFlagS::empty(),
                &[] as &[VulkanMemoryBarrier],
                &[] as &[VulkanBufferMemoryBarrier],
                &[vulkan_image_memory_barrier]);
            let vulkan_blit_source_sub_resource =
                VulkanImageSubresourceLayerS::builder()
                .aspect_mask(VulkanImageAspectFlagS::COLOR)
                .mip_level(iterating_mip_level - 1)
                .base_array_layer(0)
                .layer_count(1);
            let vulkan_blit_destination_sub_resource =
                VulkanImageSubresourceLayerS::builder()
                .aspect_mask(VulkanImageAspectFlagS::COLOR)
                .mip_level(iterating_mip_level)
                .base_array_layer(0)
                .layer_count(1);
            let vulkan_image_blit =
                VulkanImageBlit::builder()
                .src_offsets([
                    VulkanOffsetD3 { x: 0, y: 0, z: 0 },
                    VulkanOffsetD3 {
                        x: vulkan_mip_width as i32,
                        y: vulkan_mip_height as i32,
                        z: 1,
                    },
                ])
                .src_subresource(vulkan_blit_source_sub_resource)
                .dst_offsets([
                    VulkanOffsetD3 { x: 0, y: 0, z: 0 },
                    VulkanOffsetD3 {
                        x: (if 1 < vulkan_mip_width { vulkan_mip_width / 2 } else { 1 }) as i32,
                        y: (if 1 < vulkan_mip_height { vulkan_mip_height / 2 } else { 1 }) as i32,
                        z: 1,
                    },
                ])
                .dst_subresource(vulkan_blit_destination_sub_resource);
            vulkan_logical_device.cmd_blit_image(
                vulkan_command_buffer,
                vulkan_image,
                VulkanImageLayout::TRANSFER_SRC_OPTIMAL,
                vulkan_image,
                VulkanImageLayout::TRANSFER_DST_OPTIMAL,
                &[vulkan_image_blit],
                VulkanFilter::LINEAR);
            let vulkan_image_memory_barrier =
                VulkanImageMemoryBarrier::builder()
                .image(vulkan_image)
                .subresource_range(vulkan_image_sub_resource)
                .src_queue_family_index(VULKAN_QUEUE_FAMILY_IGNORED)
                .dst_queue_family_index(VULKAN_QUEUE_FAMILY_IGNORED)
                .old_layout(VulkanImageLayout::TRANSFER_SRC_OPTIMAL)
                .new_layout(VulkanImageLayout::SHADER_READ_ONLY_OPTIMAL)
                .src_access_mask(VulkanAccessFlagS::TRANSFER_READ)
                .dst_access_mask(VulkanAccessFlagS::SHADER_READ);
            vulkan_logical_device.cmd_pipeline_barrier(
                vulkan_command_buffer,
                VulkanPipelineStageFlagS::TRANSFER,
                VulkanPipelineStageFlagS::FRAGMENT_SHADER,
                VulkanDependencyFlagS::empty(),
                &[] as &[VulkanMemoryBarrier],
                &[] as &[VulkanBufferMemoryBarrier],
                &[vulkan_image_memory_barrier]);
            if 1 < vulkan_mip_width { vulkan_mip_width /= 2; }
            if 1 < vulkan_mip_height { vulkan_mip_height /= 2; }
        }
        let vulkan_sub_resource =
            VulkanImageSubResourceRange::builder()
            .aspect_mask(VulkanImageAspectFlagS::COLOR)
            .base_array_layer(0)
            .layer_count(1)
            .level_count(1)
            .base_mip_level(vulkan_mip_level.as_raw() - 1);
        let vulkan_image_memory_barrier =
            VulkanImageMemoryBarrier::builder()
            .image(vulkan_image)
            .subresource_range(vulkan_sub_resource)
            .src_queue_family_index(VULKAN_QUEUE_FAMILY_IGNORED)
            .dst_queue_family_index(VULKAN_QUEUE_FAMILY_IGNORED)
            .old_layout(VulkanImageLayout::TRANSFER_DST_OPTIMAL)
            .new_layout(VulkanImageLayout::SHADER_READ_ONLY_OPTIMAL)
            .src_access_mask(VulkanAccessFlagS::TRANSFER_WRITE)
            .dst_access_mask(VulkanAccessFlagS::SHADER_READ);
        vulkan_logical_device.cmd_pipeline_barrier(
            vulkan_command_buffer,
            VulkanPipelineStageFlagS::TRANSFER,
            VulkanPipelineStageFlagS::FRAGMENT_SHADER,
            VulkanDependencyFlagS::empty(),
            &[] as &[VulkanMemoryBarrier],
            &[] as &[VulkanBufferMemoryBarrier],
            &[vulkan_image_memory_barrier]);
        ApplicationVulkanCommandBufferOneTime::end_submit_wait(
            vulkan_logical_device, vulkan_command_pool, vulkan_command_buffer, vulkan_graphic_queue)?;
        Ok(())
    }

    unsafe fn is_physical_device_support_linear_blitting(
        vulkan_instance: &VulkanInstance,
        vulkan_physical_device: VulkanDevicePhysical,
        vulkan_swapchain_image_format: VulkanFormat)
     -> bool
    {
        let vulkan_physical_device_format_property_s =
            vulkan_instance.get_physical_device_format_properties(vulkan_physical_device, vulkan_swapchain_image_format);
        vulkan_physical_device_format_property_s.optimal_tiling_features.contains(
            VulkanFormatFeatureFlagS::SAMPLED_IMAGE_FILTER_LINEAR)
    }

    pub fn calculate_level_max(width: u32, height: u32) -> VulkanMipLevel
    {
        let max_dimension = max(width, height) as f32;
        let max_level = (max_dimension.log2().floor() as u32) + 1;
        VulkanMipLevel::new(max_level)
    }
}
