use ::library_foundation_reintroduction::vulkan::VulkanBuilderHas;
use ::library_foundation_reintroduction::vulkan::VulkanInstanceVersion1_0;
use ::library_foundation_reintroduction::vulkan::VulkanInstanceVersion1_1;
use ::library_foundation_reintroduction::vulkan::VulkanInstance;
use ::library_foundation_reintroduction::vulkan::VulkanDevicePhysical;
use ::library_foundation_reintroduction::vulkan::VulkanVersionVanilla;
use ::library_foundation_reintroduction::vulkan::version::VulkanVersionApi;

use crate::error::foundation_vulkan_cooked::ErrorFoundationVulkanCooked;
use crate::vulkan_requirement::version::VulkanRequirementVersionApiLeast;


pub struct VulkanRequirementDevicePhysical {}

impl VulkanRequirementDevicePhysical {
    pub fn fulfill_version(
        vulkan_instance: &VulkanInstance,
        vulkan_physical_device: VulkanDevicePhysical,
        least_vulkan_api_version_requirement: &VulkanRequirementVersionApiLeast)
    -> Result<(), ErrorFoundationVulkanCooked>
    {
        let property_s = unsafe { vulkan_instance.get_physical_device_properties(vulkan_physical_device) };
        let api_version = VulkanVersionApi::from(VulkanVersionVanilla::from(property_s.api_version));
        least_vulkan_api_version_requirement.fulfill_device_physical(&api_version)?;
        Ok(())
    }
}