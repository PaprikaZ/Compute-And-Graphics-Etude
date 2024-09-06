use crate::vulkan::version::VulkanVersionEntry;

//Vulkan SDK version which required portability subset extension targeting MacOS
pub static VULKAN_PORTABILITY_VERSION_ENTRY_MACOS_MIN: VulkanVersionEntry = VulkanVersionEntry::new(1, 3, 216);