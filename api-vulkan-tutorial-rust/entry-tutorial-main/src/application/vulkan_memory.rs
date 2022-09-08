use ::vulkan::prelude::version1_2::*;
use ::vulkan::VulkanMemoryPropertyFlagS;
use ::vulkan::VulkanDevicePhysical;
use ::vulkan::VulkanMemoryRequirementS;
use ::vulkan::VulkanMemoryTypeIndex;

use crate::termination::TerminationProcessMain;


pub struct ApplicationVulkanMemory {}

impl ApplicationVulkanMemory {
    pub unsafe fn get_type_index(
        vulkan_instance: &VulkanInstance,
        vulkan_physical_device: VulkanDevicePhysical,
        vulkan_memory_property_flag_s: VulkanMemoryPropertyFlagS,
        vulkan_memory_requirement_s: VulkanMemoryRequirementS,
    ) -> Result<VulkanMemoryTypeIndex, TerminationProcessMain> {
        let vulkan_physical_device_memory_property_s =
            vulkan_instance.get_physical_device_memory_properties(vulkan_physical_device);
        (0..vulkan_physical_device_memory_property_s.memory_type_count)
        .find(|i| {
            let be_memory_requirement_ok = (vulkan_memory_requirement_s.memory_type_bits & (1 << i)) != 0;
            let memory_type = vulkan_physical_device_memory_property_s.memory_types[*i as usize];
            let be_memory_type_ok = memory_type.property_flags.contains(vulkan_memory_property_flag_s);
            be_memory_requirement_ok && be_memory_type_ok
        })
        .map(|i| VulkanMemoryTypeIndex::new(i))
        .ok_or_else(|| TerminationProcessMain::InitializationVulkanMemoryTypeNotSupport)
    }
}