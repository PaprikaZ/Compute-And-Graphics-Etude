use ::library_foundation_reintroduction::vulkan::make_version;
use ::library_foundation_reintroduction::vulkan::version_major;
use ::library_foundation_reintroduction::vulkan::version_minor;
use ::library_foundation_reintroduction::vulkan::version_patch;
use ::library_foundation_reintroduction::vulkan::version::VulkanVersionApi;

use crate::error::foundation_vulkan_cooked::ErrorFoundationVulkanCookedOwn;
use crate::error::foundation_vulkan_cooked::ErrorFoundationVulkanCooked;


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VulkanRequirementVersionApiLeast(u32);

impl VulkanRequirementVersionApiLeast {
    pub fn new(major_version: u32, minor_version: u32, patch_version: u32) -> Self {
        Self(make_version(major_version, minor_version, patch_version))
    }

    pub fn as_raw(self) -> u32 {
        self.0
    }

    pub fn get_major(&self) -> u32 {
        version_major(self.0)
    }

    pub fn get_minor(&self) -> u32 {
        version_minor(self.0)
    }

    pub fn get_patch(&self) -> u32 {
        version_patch(self.0)
    }

    pub fn is_fulfilled_instance(&self, instance_api_version: &VulkanVersionApi) -> bool {
        let raw_instance_api_version = instance_api_version.unwrap();
        self.0 <= raw_instance_api_version
    }

    pub fn is_fulfilled_device_physical(&self, physical_device_api_version: &VulkanVersionApi) -> bool {
        let raw_physical_device_api_version = physical_device_api_version.unwrap();
        self.0 <= raw_physical_device_api_version
    }

    pub fn fulfill_instance(&self, instance_api_version: &VulkanVersionApi)
    -> Result<(), ErrorFoundationVulkanCooked>
    {
        if self.is_fulfilled_instance(instance_api_version) {
            Ok(())
        } else {
            Err(ErrorFoundationVulkanCookedOwn::VulkanRequirementVersionApiLeastInstanceNotFulfilled)?
        }
    }

    pub fn fulfill_device_physical(&self, physical_device_api_version: &VulkanVersionApi)
    -> Result<(), ErrorFoundationVulkanCooked>
    {
        if self.is_fulfilled_device_physical(physical_device_api_version) {
            Ok(())
        } else {
            Err(ErrorFoundationVulkanCookedOwn::VulkanRequirementVersionApiLeastDevicePhysicalNotFulfilled)?
        }
    }
}

