use crate::vulkan::VulkanLayerName;


#[derive(Debug, Clone, Copy)]
pub struct VulkanLayer {
    pub name: VulkanLayerName,
}

impl VulkanLayer {
}

//
pub const VULKAN_LAYER_SWITCHABLE_GRAPHICS: VulkanLayer = VulkanLayer {
    name: VulkanLayerName::from_bytes(b"VK_LAYER_AMD_switchable_graphics"),
};
pub const VULKAN_LAYER_KHRONOS_PROFILES: VulkanLayer = VulkanLayer {
    name: VulkanLayerName::from_bytes(b"VK_LAYER_KHRONOS_profiles"),
};
pub const VULKAN_LAYER_KHRONOS_SYNCHRONIZATION2: VulkanLayer = VulkanLayer {
    name: VulkanLayerName::from_bytes(b"VK_LAYER_KHRONOS_synchronization2"),
};
pub const VULKAN_LAYER_KHRONOS_VALIDATION: VulkanLayer = VulkanLayer {
    name: VulkanLayerName::from_bytes(b"VK_LAYER_KHRONOS_validation"),
};
//
pub const VULKAN_LAYER_LUNARG_API_DUMP: VulkanLayer = VulkanLayer {
    name: VulkanLayerName::from_bytes(b"VULKAN_LAYER_LUNARG_api_dump"),
};
pub const VULKAN_LAYER_LUNARG_GFX_RECONSTRUCT: VulkanLayer = VulkanLayer {
    name: VulkanLayerName::from_bytes(b"VULKAN_LAYER_LUNARG_gfxreconstruct"),
};
pub const VULKAN_LAYER_LUNARG_MONITOR: VulkanLayer = VulkanLayer {
    name: VulkanLayerName::from_bytes(b"VULKAN_LAYER_LUNARG_monitor"),
};
pub const VULKAN_LAYER_LUNARG_SCREENSHOT: VulkanLayer = VulkanLayer {
    name: VulkanLayerName::from_bytes(b"VULKAN_LAYER_LUNARG_screenshot"),
};