use ::library_foundation_reintroduction::window_uniform::WindowUniformWindow;
use ::library_foundation_reintroduction::vulkan::VulkanBuilderHas;
use ::library_foundation_reintroduction::vulkan::VulkanSurfaceCapabilitySKhr;
use ::library_foundation_reintroduction::vulkan::VulkanExtentD2;
use ::library_foundation_reintroduction::vulkan::VulkanSharingMode;
use ::library_foundation_reintroduction::vulkan::VulkanPresentModeKhr;
use ::library_foundation_reintroduction::vulkan::queue::VulkanQueueFamilyIndexGraphic;
use ::library_foundation_reintroduction::vulkan::queue::VulkanQueueFamilyIndexPresent;
use ::library_foundation_reintroduction::vulkan::queue::VulkanQueueFamilyIndex;
use ::library_foundation_reintroduction::vulkan::swapchain::VulkanSwapchainImageNumber;


pub struct NegotiationVulkanSwapchain {}

impl NegotiationVulkanSwapchain {
    pub fn negotiate_present_mode(
        prioritized_vulkan_present_mode: VulkanPresentModeKhr,
        fallback_vulkan_present_mode: VulkanPresentModeKhr,
        vulkan_present_mode_s: &[VulkanPresentModeKhr])
    -> VulkanPresentModeKhr
    {
        vulkan_present_mode_s
        .iter()
        .find(|m| **m == prioritized_vulkan_present_mode)
        .map(|m| m.clone())
        .unwrap_or(fallback_vulkan_present_mode)
    }

    pub fn negotiate_extent(window: &WindowUniformWindow, vulkan_surface_capability_s: &VulkanSurfaceCapabilitySKhr)
    -> VulkanExtentD2
    {
        let clamp = |min: u32, max: u32, n: u32| min.max(max.min(n));
        if vulkan_surface_capability_s.current_extent.width == u32::max_value() {
            let window_inner_size = window.inner_size();
            VulkanExtentD2::builder()
            .width(clamp(
                vulkan_surface_capability_s.min_image_extent.width,
                vulkan_surface_capability_s.max_image_extent.width,
                window_inner_size.width
            ))
            .height(clamp(
                vulkan_surface_capability_s.min_image_extent.height,
                vulkan_surface_capability_s.max_image_extent.height,
                window_inner_size.height
            ))
            .build()
        } else {
            vulkan_surface_capability_s.current_extent
        }
    }

    pub fn negotiate_image_number(vulkan_surface_capability_s: &VulkanSurfaceCapabilitySKhr)
    -> VulkanSwapchainImageNumber
    {
        let preferred_vulkan_swapchain_image_number = vulkan_surface_capability_s.min_image_count;
        let negotiated_vulkan_swapchain_image_number =
            if vulkan_surface_capability_s.max_image_count != 0 {
                preferred_vulkan_swapchain_image_number.min(vulkan_surface_capability_s.max_image_count)
            } else {
                preferred_vulkan_swapchain_image_number
            };
        VulkanSwapchainImageNumber::new(negotiated_vulkan_swapchain_image_number)
    }

    pub fn negotiate_sharing_mode_and_queue_family_index_s_graphic_present(
        vulkan_graphic_queue_family_index: VulkanQueueFamilyIndexGraphic,
        vulkan_present_queue_family_index: VulkanQueueFamilyIndexPresent)
    -> (VulkanSharingMode, Vec<VulkanQueueFamilyIndex>)
    {
        let mut vulkan_queue_family_index_s = Vec::new();
        if vulkan_graphic_queue_family_index.as_raw() == vulkan_present_queue_family_index.as_raw() {
            (VulkanSharingMode::EXCLUSIVE, vulkan_queue_family_index_s)
        } else {
            vulkan_queue_family_index_s.push(vulkan_graphic_queue_family_index.into());
            vulkan_queue_family_index_s.push(vulkan_present_queue_family_index.into());
            (VulkanSharingMode::CONCURRENT, vulkan_queue_family_index_s)
        }
    }
}
