use ::library_foundation_reintroduction::vulkan::VulkanLibraryLoader;
use ::library_foundation_reintroduction::vulkan::VulkanEntry;

use crate::error::foundation_vulkan_cooked::ErrorFoundationVulkanCookedOwn;
use crate::error::foundation_vulkan_cooked::ErrorFoundationVulkanCooked;


pub struct InitializationVulkanEntry {}

impl InitializationVulkanEntry {
    pub fn initialize(library_loader: VulkanLibraryLoader)
    -> Result<VulkanEntry, ErrorFoundationVulkanCooked>
    {
        match unsafe { VulkanEntry::new(library_loader) } {
            Err(_e) => Err(ErrorFoundationVulkanCookedOwn::VulkanEntryInitializeFail)?,
            Ok(e) => Ok(e),
        }
    }
}