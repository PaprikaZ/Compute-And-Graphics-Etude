use ::vulkan::prelude::version1_2::*;
use ::vulkan::VulkanErrorCode;
use ::vulkan::VulkanImage;
use ::vulkan::VulkanFormat;
use ::vulkan::VulkanComponentMapping;
use ::vulkan::VulkanComponentSwizzle;
use ::vulkan::VulkanImageSubResourceRange;
use ::vulkan::VulkanImageAspectFlagS;
use ::vulkan::VulkanImageView;
use ::vulkan::VulkanImageViewCreateInformation;
use ::vulkan::VulkanImageViewType;

use crate::termination::TerminationProcessMain;
use crate::application::vulkan_image::ApplicationVulkanImageView;


pub struct ApplicationInstanceSwapchainImageView {}

impl ApplicationInstanceSwapchainImageView {
    pub unsafe fn create_all(
        vulkan_logical_device: &VulkanDeviceLogical,
        vulkan_format: VulkanFormat,
        vulkan_image_s: &Vec<VulkanImage>)
     -> Result<Vec<VulkanImageView>, TerminationProcessMain>
    {
        let vulkan_swapchain_image_view_s =
            vulkan_image_s
            .iter()
            .map(|i| ApplicationVulkanImageView::create(vulkan_logical_device, *i, vulkan_format))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(vulkan_swapchain_image_view_s)
    }

    unsafe fn _create(
        vulkan_logical_device: &VulkanDeviceLogical,
        vulkan_format: VulkanFormat,
        vulkan_image: &VulkanImage,
    ) -> Result<VulkanImageView, VulkanErrorCode> {
        let vulkan_component_mapping_s =
            VulkanComponentMapping
            ::builder()
            .r(VulkanComponentSwizzle::IDENTITY)
            .g(VulkanComponentSwizzle::IDENTITY)
            .b(VulkanComponentSwizzle::IDENTITY)
            .a(VulkanComponentSwizzle::IDENTITY);
        let vulkan_image_sub_resource_range =
            VulkanImageSubResourceRange
            ::builder()
            .aspect_mask(VulkanImageAspectFlagS::COLOR)
            .base_mip_level(0)
            .level_count(1)
            .base_array_layer(0)
            .layer_count(1);
        let vulkan_image_view_create_information =
            VulkanImageViewCreateInformation
            ::builder()
            .image(*vulkan_image)
            .view_type(VulkanImageViewType::_2D)
            .format(vulkan_format)
            .components(vulkan_component_mapping_s)
            .subresource_range(vulkan_image_sub_resource_range);
        vulkan_logical_device
        .create_image_view(&vulkan_image_view_create_information, None)
        .map_err(|e| VulkanErrorCode::new(e.as_raw()))
    }
}