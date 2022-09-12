use ::vulkan::prelude::version1_2::*;
use ::vulkan::VulkanErrorCode;
use ::vulkan::VulkanDescriptorSetLayout;
use ::vulkan::VulkanDescriptorSetLayoutBinding;
use ::vulkan::VulkanDescriptorType;
use ::vulkan::VulkanShaderStageFlagS;
use ::vulkan::VulkanDescriptorSetLayoutCreateInformation;

use crate::termination::TerminationProcessMain;


pub struct ApplicationVulkanTransformD3Descriptor {}

impl ApplicationVulkanTransformD3Descriptor {
    pub unsafe fn create_set_layout_main(vulkan_logical_device: &VulkanDeviceLogical)
     -> Result<VulkanDescriptorSetLayout, TerminationProcessMain>
    {
        let vulkan_descriptor_set_layout_binding =
            VulkanDescriptorSetLayoutBinding::builder()
            .binding(0)
            .descriptor_type(VulkanDescriptorType::UNIFORM_BUFFER)
            .descriptor_count(1)
            .stage_flags(VulkanShaderStageFlagS::VERTEX);
        let vulkan_descriptor_set_layout_binding_s = &[vulkan_descriptor_set_layout_binding];
        let vulkan_descriptor_set_layout_create_infomation =
            VulkanDescriptorSetLayoutCreateInformation::builder()
            .bindings(vulkan_descriptor_set_layout_binding_s);
        let create_descriptor_set_layout_result =
            vulkan_logical_device.create_descriptor_set_layout(&vulkan_descriptor_set_layout_create_infomation, None);
        match create_descriptor_set_layout_result {
            Err(error) => {
                let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                Err(TerminationProcessMain::InitializationVulkanDescriptorSetLayoutCreateFail(vulkan_error_code))
            },
            Ok(layout) => Ok(layout),
        }
    }
}