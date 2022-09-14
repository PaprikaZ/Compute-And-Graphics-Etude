use ::vulkan::prelude::version1_2::*;
use ::vulkan::VulkanErrorCode;
use ::vulkan::VulkanDevicePhysical;
use ::vulkan::VulkanInstance;
use ::vulkan::VulkanFormat;
use ::vulkan::VulkanImageTiling;
use ::vulkan::VulkanImageUsageFlagS;
use ::vulkan::VulkanImage;
use ::vulkan::VulkanImageView;
use ::vulkan::VulkanImageSubResourceRange;
use ::vulkan::VulkanImageAspectFlagS;
use ::vulkan::VulkanImageViewType;
use ::vulkan::VulkanImageViewCreateInformation;
use ::vulkan::VulkanMemoryPropertyFlagS;
use ::vulkan::VulkanImageCreateInformation;
use ::vulkan::VulkanImageType;
use ::vulkan::VulkanDeviceMemory;
use ::vulkan::VulkanExtentD3;
use ::vulkan::VulkanImageLayout;
use ::vulkan::VulkanSampleCountFlagS;
use ::vulkan::VulkanSharingMode;
use ::vulkan::VulkanMemoryAllocateInformation;

use crate::termination::TerminationProcessMain;
use crate::application::vulkan_memory::ApplicationVulkanMemory;


pub struct ApplicationVulkanImage {}

impl ApplicationVulkanImage {
    pub unsafe fn create_with_memory(
        vulkan_instance: &VulkanInstance,
        vulkan_physical_device: VulkanDevicePhysical,
        vulkan_logical_device: &VulkanDeviceLogical,
        vulkan_image_width: u32,
        vulkan_image_height: u32,
        vulkan_image_format: VulkanFormat,
        vulkan_image_tiling: VulkanImageTiling,
        vulkan_image_usage: VulkanImageUsageFlagS,
        required_vulkan_memory_property_s: VulkanMemoryPropertyFlagS)
     -> Result<(VulkanImage, VulkanDeviceMemory), TerminationProcessMain>
    {
        let vulkan_texture_image_create_information =
            VulkanImageCreateInformation::builder()
            .image_type(VulkanImageType::_2D)
            .extent(VulkanExtentD3 { width: vulkan_image_width, height: vulkan_image_height, depth: 1 })
            .mip_levels(1)
            .array_layers(1)
            .format(vulkan_image_format)
            .tiling(vulkan_image_tiling)
            .initial_layout(VulkanImageLayout::UNDEFINED)
            .usage(vulkan_image_usage)
            .samples(VulkanSampleCountFlagS::_1)
            .sharing_mode(VulkanSharingMode::EXCLUSIVE);
        let create_vulkan_texture_image_result =
            vulkan_logical_device.create_image(&vulkan_texture_image_create_information, None);
        let vulkan_texture_image =
            match create_vulkan_texture_image_result {
                Err(error) => {
                    let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                    return Err(TerminationProcessMain::InitializationVulkanImageCreateFail(vulkan_error_code));
                },
                Ok(image) => image,
            };
        let vulkan_texture_image_memory_requirement_s =
            vulkan_logical_device.get_image_memory_requirements(vulkan_texture_image);
        let vulkan_texture_image_memory_type_index =
            ApplicationVulkanMemory::get_type_index(
                vulkan_instance,
                vulkan_physical_device,
                required_vulkan_memory_property_s,
                vulkan_texture_image_memory_requirement_s)?;
        let vulkan_texture_image_memory_allocate_information =
            VulkanMemoryAllocateInformation::builder()
            .allocation_size(vulkan_texture_image_memory_requirement_s.size)
            .memory_type_index(vulkan_texture_image_memory_type_index.as_raw());
        let allocate_vulkan_texture_image_memory_result =
            vulkan_logical_device.allocate_memory(&vulkan_texture_image_memory_allocate_information, None);
        let vulkan_texture_image_memory =
            match allocate_vulkan_texture_image_memory_result {
                Err(error) => {
                    let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                    return Err(TerminationProcessMain::InitializationVulkanMemoryAllocateFail(vulkan_error_code));
                },
                Ok(memory) => memory,
            };
        let bind_vulkan_texture_image_memory_result =
            vulkan_logical_device.bind_image_memory(vulkan_texture_image, vulkan_texture_image_memory, 0);
        match bind_vulkan_texture_image_memory_result {
            Err(error) => {
                let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                return Err(TerminationProcessMain::InitializationVulkanMemoryBufferBindFail(vulkan_error_code));
            },
            Ok(()) => (),
        };
        Ok((vulkan_texture_image, vulkan_texture_image_memory))
    }
}

pub struct ApplicationVulkanImageView {}

impl ApplicationVulkanImageView {
    pub unsafe fn create(
        vulkan_logical_device: &VulkanDeviceLogical,
        vulkan_image: VulkanImage,
        vulkan_format: VulkanFormat)
     -> Result<VulkanImageView, TerminationProcessMain>
    {
        let vulkan_image_sub_resource_range =
            VulkanImageSubResourceRange::builder()
            .aspect_mask(VulkanImageAspectFlagS::COLOR)
            .base_mip_level(0)
            .level_count(1)
            .base_array_layer(0)
            .layer_count(1);
        let vulkan_image_view_create_information =
            VulkanImageViewCreateInformation::builder()
            .image(vulkan_image)
            .view_type(VulkanImageViewType::_2D)
            .format(vulkan_format)
            .subresource_range(vulkan_image_sub_resource_range);
        let create_vulkan_image_view_result =
            vulkan_logical_device.create_image_view(&vulkan_image_view_create_information, None);
        match create_vulkan_image_view_result {
            Err(error) => {
                let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                return Err(TerminationProcessMain::InitializationVulkanImageViewCreateFail(vulkan_error_code));
            },
            Ok(image_view) => Ok(image_view),
        }
    }
}
