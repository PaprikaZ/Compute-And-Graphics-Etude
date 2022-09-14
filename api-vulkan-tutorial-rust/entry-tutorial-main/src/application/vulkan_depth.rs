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

use crate::termination::TerminationProcessMain;
use crate::application::vulkan_image::ApplicationVulkanImage;
use crate::application::vulkan_image::ApplicationVulkanImageView;


pub struct ApplicationVulkanDepth {}

impl ApplicationVulkanDepth {
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