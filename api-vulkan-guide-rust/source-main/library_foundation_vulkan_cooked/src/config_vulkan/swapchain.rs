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
}