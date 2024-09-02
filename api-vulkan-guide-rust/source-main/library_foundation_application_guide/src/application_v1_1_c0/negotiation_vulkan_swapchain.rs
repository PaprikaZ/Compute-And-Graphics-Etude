use ::library_foundation_reintroduction::window_uniform::WindowUniformWindow;
use ::library_foundation_reintroduction::vulkan::VulkanExtentD2;
use ::library_foundation_reintroduction::vulkan::VulkanSurfaceFormatKhr;
use ::library_foundation_reintroduction::vulkan::VulkanPresentModeKhr;
use ::library_foundation_reintroduction::vulkan::VulkanSurfaceCapabilitySKhr;
use ::library_foundation_reintroduction::vulkan::VulkanSharingMode;
use ::library_foundation_reintroduction::vulkan::swapchain::VulkanSwapchainImageNumber;
use ::library_foundation_reintroduction::vulkan::queue::VulkanQueueFamilyIndexGraphic;
use ::library_foundation_reintroduction::vulkan::queue::VulkanQueueFamilyIndexPresent;


pub struct ApplicationNegotiationVulkanSwapchain {}

impl ApplicationNegotiationVulkanSwapchain {
    pub fn negotiate_surface_format(_vulkan_surface_format_s: &[VulkanSurfaceFormatKhr])
    -> VulkanSurfaceFormatKhr
    {
        todo!()
    }

    pub fn negotiate_present_mode(_vulkan_present_mode_s: &[VulkanPresentModeKhr])
    -> VulkanPresentModeKhr
    {
        todo!()
    }

    pub fn negotiate_extent(_window: &WindowUniformWindow, _vulkan_surface_capability_s: VulkanSurfaceCapabilitySKhr)
    -> VulkanExtentD2
    {
        todo!()
    }

    pub fn negotiate_image_number(_vulkan_surface_capability_s: VulkanSurfaceCapabilitySKhr)
    -> VulkanSwapchainImageNumber
    {
        todo!()
    }

    pub fn negotiate_image_sharing_mode_and_queue_family_index_s(
        _vulkan_graphic_queue_family_index: VulkanQueueFamilyIndexGraphic,
        _vulkan_present_queue_family_index: VulkanQueueFamilyIndexPresent)
    -> (VulkanSharingMode, Vec<u32>)
    {
        todo!()
    }
}
