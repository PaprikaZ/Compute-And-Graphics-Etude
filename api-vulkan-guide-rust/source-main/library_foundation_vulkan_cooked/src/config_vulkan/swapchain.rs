use ::library_foundation_reintroduction::vulkan::VulkanFormat;
use ::library_foundation_reintroduction::vulkan::VulkanColorSpaceKhr;
use ::library_foundation_reintroduction::vulkan::VulkanPresentModeKhr;


#[derive(Debug, Clone)]
pub struct ConfigVulkanSwapchain {
    pub format_prioritized: VulkanFormat,
    pub color_space_prioritized: VulkanColorSpaceKhr,
    pub present_mode_prioritized: VulkanPresentModeKhr,
    pub present_mode_fallback: VulkanPresentModeKhr,
}

impl ConfigVulkanSwapchain {
    pub fn new(
        prioritized_format: VulkanFormat,
        prioritized_color_space: VulkanColorSpaceKhr,
        prioritized_present_mode: VulkanPresentModeKhr,
        fallback_present_mode: VulkanPresentModeKhr)
    -> Self
    {
        Self {
            format_prioritized: prioritized_format,
            color_space_prioritized: prioritized_color_space,
            present_mode_prioritized: prioritized_present_mode,
            present_mode_fallback: fallback_present_mode,
        }
    }
}