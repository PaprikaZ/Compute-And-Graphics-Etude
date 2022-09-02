use ::vulkan::prelude::version1_2::*;
use ::vulkan::VulkanShaderModule;
use ::vulkan::VulkanShaderModuleCreateInformation;
use ::vulkan::VulkanErrorCode;
use ::vulkan::VulkanPipelineShaderStageCreateInformation;
use ::vulkan::VulkanShaderStageFlagS;

use crate::termination::TerminationProcessMain;


pub struct ApplicationVulkanPipeline {}

impl ApplicationVulkanPipeline {
    pub unsafe fn create(vulkan_logical_device: &VulkanDeviceLogical) -> Result<(), TerminationProcessMain> {
        let vulkan_vertex_shader_bytecode_data = include_bytes!("../../shader/vert.spv");
        let vulkan_fragment_shader_bytecode_data = include_bytes!("../../shader/frag.spv");
        let vulkan_vertex_shader_module =
            Self::create_shader_module(vulkan_logical_device, vulkan_vertex_shader_bytecode_data)?;
        let vulkan_fragment_shader_module =
            Self::create_shader_module(vulkan_logical_device, vulkan_fragment_shader_bytecode_data)?;
        let _vulkan_vertex_stage =
            VulkanPipelineShaderStageCreateInformation::builder()
            .stage(VulkanShaderStageFlagS::VERTEX)
            .module(vulkan_vertex_shader_module)
            .name(b"main\0");
        let _vulkan_fragment_stage =
            VulkanPipelineShaderStageCreateInformation::builder()
            .stage(VulkanShaderStageFlagS::FRAGMENT)
            .module(vulkan_fragment_shader_module)
            .name(b"main\0");
        vulkan_logical_device.destroy_shader_module(vulkan_vertex_shader_module, None);
        vulkan_logical_device.destroy_shader_module(vulkan_fragment_shader_module, None);
        Ok(())
    }

    unsafe fn create_shader_module(
        vulkan_logical_device: &VulkanDeviceLogical, byte_code_data: &[u8])
     -> Result<VulkanShaderModule, TerminationProcessMain>
    {
        let bytecode_byte_s = Vec::<u8>::from(byte_code_data);
        let (align_prefix, byte_s, align_suffix) = bytecode_byte_s.align_to::<u32>();
        if !align_prefix.is_empty() || !align_suffix.is_empty() {
            return Err(TerminationProcessMain::InitializationVulkanShaderByteCodeAlignmentIncorrect);
        }
        let vulkan_shader_module_create_information =
            VulkanShaderModuleCreateInformation::builder()
            .code_size(byte_s.len())
            .code(byte_s);
        let create_vulkan_shader_module_result =
            vulkan_logical_device.create_shader_module(&vulkan_shader_module_create_information, None);
        match create_vulkan_shader_module_result {
            Err(error) => {
                let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                return Err(TerminationProcessMain::InitializationVulkanShaderModuleCreateFail(vulkan_error_code));
            },
            Ok(module) => Ok(module),
        }
    }
}