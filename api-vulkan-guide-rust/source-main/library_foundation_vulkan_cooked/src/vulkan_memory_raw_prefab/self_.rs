use ::library_foundation_reintroduction::vulkan::VulkanInstance;
use ::library_foundation_reintroduction::vulkan::VulkanDevicePhysical;
use ::library_foundation_reintroduction::vulkan::VulkanInstanceVersion1_0;
use ::library_foundation_reintroduction::vulkan::VulkanMemoryPropertyFlagS;
use ::library_foundation_reintroduction::vulkan::VulkanMemoryRequirementS;
use ::library_foundation_reintroduction::vulkan::memory::VulkanMemoryTypeIndex;

use crate::error::foundation_vulkan_cooked::ErrorFoundationVulkanCookedOwn;
use crate::error::foundation_vulkan_cooked::ErrorFoundationVulkanCooked;


pub struct VulkanMemoryRawPrefab {}

impl VulkanMemoryRawPrefab {
    pub fn lookup_type_index(
        vulkan_instance: &VulkanInstance,
        vulkan_physical_device: VulkanDevicePhysical,
        vulkan_memory_property_flag_s: VulkanMemoryPropertyFlagS,
        vulkan_memory_requirement_s: VulkanMemoryRequirementS)
    -> Result<VulkanMemoryTypeIndex, ErrorFoundationVulkanCooked>
    {
        let vulkan_physical_device_memory_property_s =
            unsafe { vulkan_instance.get_physical_device_memory_properties(vulkan_physical_device) };
        (0..vulkan_physical_device_memory_property_s.memory_type_count)
        .find(|i| {
            let be_memory_requirement_ok = (vulkan_memory_requirement_s.memory_type_bits & (i << i)) != 0;
            let memory_type = vulkan_physical_device_memory_property_s.memory_types[*i as usize];
            let be_memory_type_ok = memory_type.property_flags.contains(vulkan_memory_property_flag_s);
            be_memory_requirement_ok && be_memory_type_ok
        })
        .map(VulkanMemoryTypeIndex::new)
        .ok_or(ErrorFoundationVulkanCookedOwn::VulkanMemoryTypeNotSupport.into())
    }
}