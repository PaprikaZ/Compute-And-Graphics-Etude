use ::vulkan::VulkanExtensionName;
use ::vulkan::VULKAN_SWAPCHAIN_EXTENSION_KHR;


pub const VULKAN_VALIDATION_BE_ENABLED: bool = cfg!(debug_assertions);
pub const VULKAN_VALIDATION_LAYER: VulkanExtensionName = VulkanExtensionName::from_bytes(b"VK_LAYER_KHRONOS_validation");
pub const VULKAN_DEVICE_PHYSICAL_EXTENSION_S: &[VulkanExtensionName] = &[VULKAN_SWAPCHAIN_EXTENSION_KHR.name];
pub const VULKAN_FRAME_IN_FLIGHT_MAX: u32 = 2;