use ::library_foundation_reintroduction::vulkan::VulkanSurfaceFormatKhr;
use ::library_foundation_reintroduction::vulkan::VulkanFormat;
use ::library_foundation_reintroduction::vulkan::VulkanColorSpaceKhr;


pub struct ApplicationNegotiationVulkanSwapchain {}

impl ApplicationNegotiationVulkanSwapchain {
    pub fn negotiate_surface_format(
        prioritized_vulkan_format: VulkanFormat,
        prioritized_vulkan_color_space: VulkanColorSpaceKhr,
        vulkan_surface_format_s: &[VulkanSurfaceFormatKhr]
    )
    -> VulkanSurfaceFormatKhr
    {
        vulkan_surface_format_s
        .iter()
        .find(|f|
            f.format == prioritized_vulkan_format &&
            f.color_space == prioritized_vulkan_color_space)
        .map(|f| f.clone())
        .unwrap_or_else(|| vulkan_surface_format_s[0])
    }
}