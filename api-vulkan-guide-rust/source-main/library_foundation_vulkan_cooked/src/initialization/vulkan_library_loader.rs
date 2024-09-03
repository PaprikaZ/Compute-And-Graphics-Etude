use ::library_foundation_reintroduction::vulkan::VULKAN_LIBRARY_FILE_NAME;
use ::library_foundation_reintroduction::vulkan::VulkanLibraryLoader;

use crate::error::foundation_vulkan_cooked::ErrorFoundationVulkanCookedOwn;
use crate::error::foundation_vulkan_cooked::ErrorFoundationVulkanCooked;


pub struct InitializationVulkanLibraryLoader {}

impl InitializationVulkanLibraryLoader {
    pub fn initialize() -> Result<VulkanLibraryLoader, ErrorFoundationVulkanCooked>
    {
        match unsafe { VulkanLibraryLoader::new(VULKAN_LIBRARY_FILE_NAME) } {
            Err(_e) => Err(ErrorFoundationVulkanCookedOwn::VulkanLibraryLoaderInitializeFail)?,
            Ok(l) => Ok(l),
        }
    }
}