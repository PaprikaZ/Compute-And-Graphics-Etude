use std::ptr::copy_nonoverlapping;

use ::png::OutputInfo as FormatPngOutputInfomration;
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
use ::vulkan::VulkanExtentD3;
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
use ::vulkan::VulkanImageView;
use ::vulkan::VulkanSamplerCreateInformation;
use ::vulkan::VulkanFilter;
use ::vulkan::VulkanBorderColor;
use ::vulkan::VulkanCompareOperation;
use ::vulkan::VulkanSamplerMipmapMode;
use ::vulkan::VulkanSamplerAddressMode;
use ::vulkan::VulkanSampler;
use ::vulkan::VulkanDescriptorSetLayoutBinding;
use ::vulkan::VulkanDescriptorType;
use ::vulkan::VulkanShaderStageFlagS;
use ::vulkan::VulkanMipLevel;
use ::vulkan::VulkanSampleCountFlagS;

use crate::termination::TerminationProcessMain;
use crate::application::vulkan_buffer::ApplicationVulkanBuffer;
use crate::application::vulkan_image::ApplicationVulkanImage;
use crate::application::vulkan_command_buffer::ApplicationVulkanCommandBufferOneTime;
use crate::application::vulkan_image::ApplicationVulkanImageView;
use crate::application::vulkan_mipmap::ApplicationVulkanMipmap;


pub struct ApplicationVulkanTextureImage {}

impl ApplicationVulkanTextureImage {
    pub unsafe fn create_buffer_with_memory(
        vulkan_instance: &VulkanInstance,
        vulkan_physical_device: VulkanDevicePhysical,
        vulkan_logical_device: &VulkanDeviceLogical,
        vulkan_command_pool: VulkanCommandPool,
        vulkan_graphic_queue: VulkanQueue,
        texture_file_pixel_s: Vec<u8>,
        texture_image_information: FormatPngOutputInfomration,
        vulkan_mip_level: VulkanMipLevel)
     -> Result<(VulkanImage, VulkanDeviceMemory), TerminationProcessMain>
    {
        let texture_file_buffer_size = texture_image_information.buffer_size() as u64;
        let (vulkan_texture_staging_buffer, vulkan_texture_staging_buffer_memory) =
            ApplicationVulkanBuffer::create_with_memory(
                vulkan_instance,
                vulkan_physical_device,
                vulkan_logical_device,
                texture_file_buffer_size,
                VulkanBufferUsageFlagS::TRANSFER_SRC,
                VulkanMemoryPropertyFlagS::HOST_COHERENT | VulkanMemoryPropertyFlagS::HOST_VISIBLE)?;
        let map_texture_staging_memory_result =
            vulkan_logical_device.map_memory(
                vulkan_texture_staging_buffer_memory, 0, texture_file_buffer_size, VulkanMemoryMapFlagS::empty());
        let texture_staging_memory_address =
            match map_texture_staging_memory_result {
                Err(error) => {
                    let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                    return Err(TerminationProcessMain::InitializationVulkanMemoryMapFail(vulkan_error_code));
                },
                Ok(address) => address,
            };
        copy_nonoverlapping(texture_file_pixel_s.as_ptr(), texture_staging_memory_address.cast(), texture_file_pixel_s.len());
        vulkan_logical_device.unmap_memory(vulkan_texture_staging_buffer_memory);
        //
        let (vulkan_texture_image, vulkan_texture_image_memory) =
            ApplicationVulkanImage::create_with_memory(
                vulkan_instance,
                vulkan_physical_device,
                vulkan_logical_device,
                texture_image_information.width,
                texture_image_information.height,
                vulkan_mip_level,
                VulkanSampleCountFlagS::_1,
                VulkanFormat::R8G8B8A8_SRGB,
                VulkanImageTiling::OPTIMAL,
                VulkanImageUsageFlagS::SAMPLED
                | VulkanImageUsageFlagS::TRANSFER_DST
                | VulkanImageUsageFlagS::TRANSFER_SRC,
                VulkanMemoryPropertyFlagS::DEVICE_LOCAL)?;
        Self::transition_layout(
            vulkan_logical_device,
            vulkan_command_pool,
            vulkan_graphic_queue,
            vulkan_texture_image,
            VulkanFormat::R8G8B8A8_SRGB,
            VulkanImageLayout::UNDEFINED,
            VulkanImageLayout::TRANSFER_DST_OPTIMAL,
            vulkan_mip_level)?;
        Self::copy_from_buffer(
            vulkan_logical_device,
            vulkan_command_pool,
            vulkan_graphic_queue,
            vulkan_texture_staging_buffer,
            vulkan_texture_image,
            texture_image_information.width,
            texture_image_information.height)?;
        vulkan_logical_device.destroy_buffer(vulkan_texture_staging_buffer, None);
        vulkan_logical_device.free_memory(vulkan_texture_staging_buffer_memory, None);
        ApplicationVulkanMipmap::generate(
            vulkan_instance,
            vulkan_physical_device,
            vulkan_logical_device,
            vulkan_command_pool,
            vulkan_graphic_queue,
            vulkan_texture_image,
            VulkanFormat::R8G8B8A8_SRGB,
            texture_image_information.width,
            texture_image_information.height,
            vulkan_mip_level
        )?;

        Ok((vulkan_texture_image, vulkan_texture_image_memory))
    }

    unsafe fn transition_layout(
        vulkan_logical_device: &VulkanDeviceLogical,
        vulkan_command_pool: VulkanCommandPool,
        vulkan_graphic_queue: VulkanQueue,
        vulkan_texture_image: VulkanImage,
        _vulkan_texture_image_format: VulkanFormat,
        old_vulkan_texture_image_layout: VulkanImageLayout,
        new_vulkan_texture_image_layout: VulkanImageLayout,
        vulkan_mip_level: VulkanMipLevel)
     -> Result<(), TerminationProcessMain>
    {
        let (source_access_mask, destination_access_mask,
             source_stage_mask, destination_stage_mask) =
            match (old_vulkan_texture_image_layout, new_vulkan_texture_image_layout) {
                (VulkanImageLayout::UNDEFINED, VulkanImageLayout::TRANSFER_DST_OPTIMAL) => (
                    VulkanAccessFlagS::empty(),
                    VulkanAccessFlagS::TRANSFER_WRITE,
                    VulkanPipelineStageFlagS::TOP_OF_PIPE,
                    VulkanPipelineStageFlagS::TRANSFER),
                (VulkanImageLayout::TRANSFER_DST_OPTIMAL, VulkanImageLayout::SHADER_READ_ONLY_OPTIMAL) => (
                    VulkanAccessFlagS::TRANSFER_WRITE,
                    VulkanAccessFlagS::SHADER_READ,
                    VulkanPipelineStageFlagS::TRANSFER,
                    VulkanPipelineStageFlagS::FRAGMENT_SHADER),
                _ => return Err(TerminationProcessMain::InitializationVulkanTextureImageLayoutTransitionNotSupport),
            };
        let vulkan_command_buffer =
            ApplicationVulkanCommandBufferOneTime::create_and_begin(vulkan_logical_device, vulkan_command_pool)?;
        let vulkan_image_sub_resource_range =
            VulkanImageSubResourceRange::builder()
            .aspect_mask(VulkanImageAspectFlagS::COLOR)
            .base_mip_level(0)
            .level_count(vulkan_mip_level.as_raw())
            .base_array_layer(0)
            .layer_count(1);
        let vulkan_image_memory_barrier =
            VulkanImageMemoryBarrier::builder()
            .old_layout(old_vulkan_texture_image_layout)
            .new_layout(new_vulkan_texture_image_layout)
            .src_queue_family_index(VULKAN_QUEUE_FAMILY_IGNORED)
            .dst_queue_family_index(VULKAN_QUEUE_FAMILY_IGNORED)
            .image(vulkan_texture_image)
            .subresource_range(vulkan_image_sub_resource_range)
            .src_access_mask(source_access_mask)
            .dst_access_mask(destination_access_mask);
        vulkan_logical_device.cmd_pipeline_barrier(
            vulkan_command_buffer,
            source_stage_mask,
            destination_stage_mask,
            VulkanDependencyFlagS::empty(),
            &[] as &[VulkanMemoryBarrier],
            &[] as &[VulkanBufferMemoryBarrier],
            &[vulkan_image_memory_barrier]);
        ApplicationVulkanCommandBufferOneTime::end_submit_wait(
            vulkan_logical_device, vulkan_command_pool, vulkan_command_buffer, vulkan_graphic_queue)?;
        Ok(())
    }

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

    pub unsafe fn create_view(
        vulkan_logical_device: &VulkanDeviceLogical,
        vulkan_texture_image: VulkanImage,
        vulkan_mip_level: VulkanMipLevel)
     -> Result<VulkanImageView, TerminationProcessMain>
    {
        let vulkan_image_view =
            ApplicationVulkanImageView::create(
                vulkan_logical_device, vulkan_texture_image,
                VulkanFormat::R8G8B8A8_SRGB,
                VulkanImageAspectFlagS::COLOR,
                vulkan_mip_level)?;
        Ok(vulkan_image_view)
    }

    pub unsafe fn create_sampler(vulkan_logical_device: &VulkanDeviceLogical, vulkan_mip_level: VulkanMipLevel)
     -> Result<VulkanSampler, TerminationProcessMain>
    {
        let vulkan_sampler_create_information =
            VulkanSamplerCreateInformation::builder()
            .mag_filter(VulkanFilter::LINEAR)
            .min_filter(VulkanFilter::LINEAR)
            .address_mode_u(VulkanSamplerAddressMode::REPEAT)
            .address_mode_v(VulkanSamplerAddressMode::REPEAT)
            .address_mode_w(VulkanSamplerAddressMode::REPEAT)
            .anisotropy_enable(true)
            .max_anisotropy(16.0)
            .border_color(VulkanBorderColor::INT_OPAQUE_WHITE)
            .unnormalized_coordinates(false)
            .compare_enable(false)
            .compare_op(VulkanCompareOperation::ALWAYS)
            .mipmap_mode(VulkanSamplerMipmapMode::LINEAR)
            .min_lod(0.0)
            .max_lod(vulkan_mip_level.as_raw() as f32)
            .mip_lod_bias(0.0);
        let create_vulkan_sampler_result =
            vulkan_logical_device.create_sampler(&vulkan_sampler_create_information, None);
        match create_vulkan_sampler_result {
            Err(error) => {
                let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                Err(TerminationProcessMain::InitializationVulkanSamplerCreateFail(vulkan_error_code))
            },
            Ok(sampler) => Ok(sampler),
        }
    }

    pub unsafe fn create_sampler_descriptor_set_layout_binding()
     -> Result<VulkanDescriptorSetLayoutBinding, TerminationProcessMain>
    {
        let vulkan_descriptor_set_layout_binding =
            VulkanDescriptorSetLayoutBinding::builder()
            .binding(1)
            .descriptor_type(VulkanDescriptorType::COMBINED_IMAGE_SAMPLER)
            .descriptor_count(1)
            .stage_flags(VulkanShaderStageFlagS::FRAGMENT)
            .build();
        Ok(vulkan_descriptor_set_layout_binding)
    }
}
