use ::vulkan::VulkanErrorCode;
use ::vulkan::prelude::version1_2::*;
use ::vulkan::VulkanExtentD2;
use ::vulkan::VulkanShaderModule;
use ::vulkan::VulkanShaderModuleCreateInformation;
use ::vulkan::VulkanPipelineShaderStageCreateInformation;
use ::vulkan::VulkanShaderStageFlagS;
use ::vulkan::VulkanPipelineVertexInputStateCreateInformation;
use ::vulkan::VulkanPipelineInputAssemblyStateCreateInformation;
use ::vulkan::VulkanPrimitiveTopology;
use ::vulkan::VulkanViewport;
use ::vulkan::VulkanRectangleD2;
use ::vulkan::VulkanOffsetD2;
use ::vulkan::VulkanPipelineViewportStateCreateInformation;
use ::vulkan::VulkanPipelineRasterizationStateCreateInformation;
use ::vulkan::VulkanPolygonMode;
use ::vulkan::VulkanCullModeFlagS;
use ::vulkan::VulkanFrontFace;
use ::vulkan::VulkanPipelineMultisampleStateCreateInformation;
use ::vulkan::VulkanPipelineColorBlendStateCreateInformation;
use ::vulkan::VulkanPipelineLayoutCreateInformation;
use ::vulkan::VulkanColorComponentFlagS;
use ::vulkan::VulkanLogicOperation;
use ::vulkan::VulkanBlendOperation;
use ::vulkan::VulkanSampleCountFlagS;
use ::vulkan::VulkanPipelineColorBlendAttachmentState;
use ::vulkan::VulkanPipelineLayout;

use crate::termination::TerminationProcessMain;


pub struct ApplicationVulkanPipeline {}

impl ApplicationVulkanPipeline {
    pub unsafe fn create(
        vulkan_logical_device: &VulkanDeviceLogical, vulkan_2d_extent: VulkanExtentD2)
     -> Result<VulkanPipelineLayout, TerminationProcessMain>
    {
        let vulkan_vertex_shader_bytecode_data = include_bytes!("../../shader/vert.spv");
        let vulkan_fragment_shader_bytecode_data = include_bytes!("../../shader/frag.spv");
        let vulkan_vertex_shader_module =
            Self::create_shader_module(vulkan_logical_device, vulkan_vertex_shader_bytecode_data)?;
        let vulkan_fragment_shader_module =
            Self::create_shader_module(vulkan_logical_device, vulkan_fragment_shader_bytecode_data)?;
        let _vulkan_vertex_shader_stage_create_information =
            VulkanPipelineShaderStageCreateInformation::builder()
            .stage(VulkanShaderStageFlagS::VERTEX)
            .module(vulkan_vertex_shader_module)
            .name(b"main\0");
        let _vulkan_fragment_shader_stage_create_infomation =
            VulkanPipelineShaderStageCreateInformation::builder()
            .stage(VulkanShaderStageFlagS::FRAGMENT)
            .module(vulkan_fragment_shader_module)
            .name(b"main\0");

        let _vulkan_vertex_input_state_create_infomation = VulkanPipelineVertexInputStateCreateInformation::builder();

        let _vulkan_input_assembly_state_create_information =
            VulkanPipelineInputAssemblyStateCreateInformation::builder()
            .topology(VulkanPrimitiveTopology::TRIANGLE_LIST)
            .primitive_restart_enable(false);

        let vulkan_viewport =
            VulkanViewport::builder()
            .x(0.0)
            .y(0.0)
            .width(vulkan_2d_extent.width as f32)
            .height(vulkan_2d_extent.height as f32)
            .min_depth(0.0)
            .max_depth(1.0);

        let vulkan_scissor =
            VulkanRectangleD2::builder()
            .offset(VulkanOffsetD2 { x: 0, y: 0 })
            .extent(vulkan_2d_extent);

        let vulkan_viewport_s = &[vulkan_viewport];
        let vulkan_scissor_s = &[vulkan_scissor];

        let _vulkan_viewport_state_create_information =
            VulkanPipelineViewportStateCreateInformation::builder()
            .viewports(vulkan_viewport_s)
            .scissors(vulkan_scissor_s);

        let _vulkan_rasterization_state_create_information =
            VulkanPipelineRasterizationStateCreateInformation::builder()
            .depth_clamp_enable(false)
            .rasterizer_discard_enable(false)
            .polygon_mode(VulkanPolygonMode::FILL)
            .line_width(1.0)
            .cull_mode(VulkanCullModeFlagS::BACK)
            .front_face(VulkanFrontFace::CLOCKWISE)
            .depth_bias_enable(false);

        let _vulkan_pipeline_multisample_state_create_information =
            VulkanPipelineMultisampleStateCreateInformation::builder()
            .sample_shading_enable(false)
            .rasterization_samples(VulkanSampleCountFlagS::_1);

        let vulkan_color_blend_attachment_state =
            VulkanPipelineColorBlendAttachmentState::builder()
            .color_write_mask(VulkanColorComponentFlagS::all())
            .blend_enable(false);

        let vulkan_color_blend_attachment_state_s = &[vulkan_color_blend_attachment_state];

        let _vulkan_color_blend_state =
            VulkanPipelineColorBlendStateCreateInformation::builder()
            .logic_op_enable(false)
            .logic_op(VulkanLogicOperation::COPY)
            .attachments(vulkan_color_blend_attachment_state_s)
            .blend_constants([0.0, 0.0, 0.0, 0.0]);

        let vulkan_pipeline_layout_create_infomation = VulkanPipelineLayoutCreateInformation::builder();

        let create_vulkan_pipeline_layout_result =
            vulkan_logical_device.create_pipeline_layout(&vulkan_pipeline_layout_create_infomation, None);

        let vulkan_pipeline_layout =
            match create_vulkan_pipeline_layout_result {
                Err(error) => {
                    let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                    return Err(TerminationProcessMain::InitializationVulkanPipelineLayoutCreateFail(vulkan_error_code));
                },
                Ok(layout) => layout,
            };

        vulkan_logical_device.destroy_shader_module(vulkan_vertex_shader_module, None);
        vulkan_logical_device.destroy_shader_module(vulkan_fragment_shader_module, None);

        Ok(vulkan_pipeline_layout)
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