use ::vulkan::prelude::version1_2::*;
use ::vulkan::VulkanDescriptorSetLayoutBinding;
use ::vulkan::VulkanDescriptorType;
use ::vulkan::VulkanShaderStageFlagS;

use crate::termination::TerminationProcessMain;


pub struct ApplicationVulkanTransformD3Descriptor {}

impl ApplicationVulkanTransformD3Descriptor {
    pub unsafe fn create_set_layout_binding()
     -> Result<VulkanDescriptorSetLayoutBinding, TerminationProcessMain>
    {
        let vulkan_descriptor_set_layout_binding =
            VulkanDescriptorSetLayoutBinding::builder()
            .binding(0)
            .descriptor_type(VulkanDescriptorType::UNIFORM_BUFFER)
            .descriptor_count(1)
            .stage_flags(VulkanShaderStageFlagS::VERTEX)
            .build();
        Ok(vulkan_descriptor_set_layout_binding)
    }
}