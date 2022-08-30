use ::vulkan::VulkanExtensionName;


pub const VULKAN_VALIDATION_BE_ENABLED: bool = cfg!(debug_assertions);
pub const VULKAN_VALIDATION_LAYER: VulkanExtensionName = VulkanExtensionName::from_bytes(b"VK_LAYER_KHRONOS_validation");