use ::vulkan::prelude::version1_2::*;
use ::vulkan::VulkanInstance;
use ::vulkan::VulkanDevicePhysical;
use ::vulkan::VulkanFormat;
use ::vulkan::VulkanImageTiling;
use ::vulkan::VulkanFormatFeatureFlagS;
use ::vulkan::VulkanExtentD2;
use ::vulkan::VulkanImageUsageFlagS;
use ::vulkan::VulkanMemoryPropertyFlagS;
use ::vulkan::VulkanImageAspectFlagS;
use ::vulkan::VulkanImage;
use ::vulkan::VulkanImageView;
use ::vulkan::VulkanDeviceMemory;
use ::vulkan::extend::VulkanMipLevel;
use ::vulkan::VulkanSampleCountFlagS;

use crate::termination::TerminationProcessMain;
use crate::application::vulkan_image::ApplicationVulkanImage;
use crate::application::vulkan_image::ApplicationVulkanImageView;


pub struct ApplicationVulkanDepth {}

impl ApplicationVulkanDepth {
    pub unsafe fn create_image_memory_view(
        vulkan_instance: &VulkanInstance,
        vulkan_physical_device: VulkanDevicePhysical,
        vulkan_logical_device: &VulkanDeviceLogical,
        vulkan_swapchain_extent: VulkanExtentD2,
        vulkan_anti_aliasing_multisampling_number: VulkanSampleCountFlagS)
     -> Result<(VulkanImage, VulkanDeviceMemory, VulkanImageView), TerminationProcessMain>
    {
        let selected_vulkan_depth_format =
            Self::get_format(vulkan_instance, vulkan_physical_device)?;
        let (vulkan_depth_image, vulkan_depth_image_memory) =
            ApplicationVulkanImage::create_with_memory(
                vulkan_instance,
                vulkan_physical_device,
                vulkan_logical_device,
                vulkan_swapchain_extent.width,
                vulkan_swapchain_extent.height,
                VulkanMipLevel::new(1),
                vulkan_anti_aliasing_multisampling_number,
                selected_vulkan_depth_format,
                VulkanImageTiling::OPTIMAL,
                VulkanImageUsageFlagS::DEPTH_STENCIL_ATTACHMENT,
                VulkanMemoryPropertyFlagS::DEVICE_LOCAL)?;
        let vulkan_depth_image_view =
            ApplicationVulkanImageView::create(
                vulkan_logical_device,
                vulkan_depth_image,
                selected_vulkan_depth_format,
                VulkanImageAspectFlagS::DEPTH,
                VulkanMipLevel::new(1))?;
        Ok((vulkan_depth_image, vulkan_depth_image_memory, vulkan_depth_image_view))
    }

    pub unsafe fn get_format(vulkan_instance: &VulkanInstance, vulkan_physical_device: VulkanDevicePhysical)
     -> Result<VulkanFormat, TerminationProcessMain>
    {
        let vulkan_format_candidate_s =
            &[VulkanFormat::D32_SFLOAT, VulkanFormat::D32_SFLOAT_S8_UINT, VulkanFormat::D24_UNORM_S8_UINT];
        Self::get_format_from_candidate_s(
            vulkan_instance, vulkan_physical_device,
            vulkan_format_candidate_s,
            VulkanImageTiling::OPTIMAL,
            VulkanFormatFeatureFlagS::DEPTH_STENCIL_ATTACHMENT)
    }

    unsafe fn get_format_from_candidate_s(
        vulkan_instance: &VulkanInstance,
        vulkan_physical_device: VulkanDevicePhysical,
        vulkan_format_candidate_s: &[VulkanFormat],
        vulkan_image_tiling: VulkanImageTiling,
        required_vulkan_format_feature_s: VulkanFormatFeatureFlagS)
     -> Result<VulkanFormat, TerminationProcessMain>
    {
        vulkan_format_candidate_s
        .iter()
        .cloned()
        .find(|f| {
            let vulkan_physical_device_format_property_s =
                vulkan_instance.get_physical_device_format_properties(vulkan_physical_device, *f);
            let available_vulkan_format_feature_flag_s =
                match vulkan_image_tiling {
                    VulkanImageTiling::LINEAR => vulkan_physical_device_format_property_s.linear_tiling_features,
                    VulkanImageTiling::OPTIMAL => vulkan_physical_device_format_property_s.optimal_tiling_features,
                    _ => VulkanFormatFeatureFlagS::empty(),
                };
            available_vulkan_format_feature_flag_s.contains(required_vulkan_format_feature_s)
        })
        .ok_or_else(|| TerminationProcessMain::InitializationVulkanFormatFeatureNotSupport)
    }
}