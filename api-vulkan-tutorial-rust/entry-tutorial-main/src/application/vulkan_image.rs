use ::vulkan::prelude::version1_2::*;
use ::vulkan::VulkanErrorCode;
use ::vulkan::VulkanImage;
use ::vulkan::VulkanFormat;
use ::vulkan::VulkanImageView;
use ::vulkan::VulkanImageSubResourceRange;
use ::vulkan::VulkanImageAspectFlagS;
use ::vulkan::VulkanImageViewType;
use ::vulkan::VulkanImageViewCreateInformation;

use crate::termination::TerminationProcessMain;


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
