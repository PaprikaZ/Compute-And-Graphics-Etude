use std::cmp::max;

use ::vulkan::prelude::version1_2::*;
use ::vulkan::VulkanInstance;
use ::vulkan::VulkanDevicePhysical;
use ::vulkan::VulkanSampleCountFlagS;
use ::vulkan::VulkanExtentD2;
use ::vulkan::VulkanFormat;
use ::vulkan::VulkanMipLevel;
use ::vulkan::VulkanImageTiling;
use ::vulkan::VulkanImageUsageFlagS;
use ::vulkan::VulkanMemoryPropertyFlagS;
use ::vulkan::VulkanImageAspectFlagS;
use ::vulkan::VulkanImage;
use ::vulkan::VulkanImageView;
use ::vulkan::VulkanDeviceMemory;

use crate::termination::TerminationProcessMain;
use crate::application::vulkan_image::ApplicationVulkanImage;
use crate::application::vulkan_image::ApplicationVulkanImageView;


pub struct ApplicationVulkanAntiAliasingMultiSampling {}

impl ApplicationVulkanAntiAliasingMultiSampling {
    pub unsafe fn get_sample_count_max(
        vulkan_instance: &VulkanInstance,
        vulkan_physical_device: VulkanDevicePhysical)
     -> VulkanSampleCountFlagS
    {
        let vulkan_physical_device_property_s =
            vulkan_instance.get_physical_device_properties(vulkan_physical_device);
        let frame_buffer_sample_count_flag_s = max(
            vulkan_physical_device_property_s.limits.framebuffer_color_sample_counts,
            vulkan_physical_device_property_s.limits.framebuffer_depth_sample_counts);
        [VulkanSampleCountFlagS::_64,
         VulkanSampleCountFlagS::_32,
         VulkanSampleCountFlagS::_16,
         VulkanSampleCountFlagS::_8,
         VulkanSampleCountFlagS::_4,
         VulkanSampleCountFlagS::_2]
        .iter()
        .cloned()
        .find(|c| frame_buffer_sample_count_flag_s.contains(*c))
        .unwrap_or(VulkanSampleCountFlagS::_1)
    }
}