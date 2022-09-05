use ::vulkan::prelude::version1_2::*;
use ::vulkan::VulkanErrorCode;
use ::vulkan::VulkanFormat;
use ::vulkan::VulkanAttachmentDescription;
use ::vulkan::VulkanAttachmentLoadOperation;
use ::vulkan::VulkanAttachmentStoreOperation;
use ::vulkan::VulkanImageLayout;
use ::vulkan::VulkanAttachmentReference;
use ::vulkan::VulkanPipelineBindPoint;
use ::vulkan::VulkanRenderPass;
use ::vulkan::VulkanRenderPassCreateInformation;
use ::vulkan::VulkanSampleCountFlagS;
use ::vulkan::VulkanSubpassDescription;
use ::vulkan::VULKAN_SUBPASS_EXTERNAL;
use ::vulkan::VulkanSubpassDependency;
use ::vulkan::VulkanPipelineStageFlagS;
use ::vulkan::VulkanAccessFlagS;

use crate::termination::TerminationProcessMain;


pub struct ApplicationVulkanRenderPass {}

impl ApplicationVulkanRenderPass {
    pub unsafe fn create(
        vulkan_logical_device: &VulkanDeviceLogical,
        vulkan_swapchain_format: VulkanFormat)
     -> Result<VulkanRenderPass, TerminationProcessMain>
    {
        let vulkan_color_attachment =
            VulkanAttachmentDescription::builder()
            .format(vulkan_swapchain_format)
            .samples(VulkanSampleCountFlagS::_1)
            .load_op(VulkanAttachmentLoadOperation::CLEAR)
            .store_op(VulkanAttachmentStoreOperation::STORE)
            .stencil_load_op(VulkanAttachmentLoadOperation::DONT_CARE)
            .stencil_store_op(VulkanAttachmentStoreOperation::DONT_CARE)
            .initial_layout(VulkanImageLayout::UNDEFINED)
            .final_layout(VulkanImageLayout::PRESENT_SRC_KHR);
        let vulkan_color_attachment_reference =
            VulkanAttachmentReference::builder()
            .attachment(0)
            .layout(VulkanImageLayout::COLOR_ATTACHMENT_OPTIMAL);
        let vulkan_color_attachment_s = &[vulkan_color_attachment_reference];
        let vulkan_subpass =
            VulkanSubpassDescription::builder()
            .pipeline_bind_point(VulkanPipelineBindPoint::GRAPHICS)
            .color_attachments(vulkan_color_attachment_s);
        let vulkan_subpass_dependency=
            VulkanSubpassDependency::builder()
            .src_subpass(VULKAN_SUBPASS_EXTERNAL)
            .dst_subpass(0)
            .src_stage_mask(VulkanPipelineStageFlagS::COLOR_ATTACHMENT_OUTPUT)
            .src_access_mask(VulkanAccessFlagS::empty())
            .dst_stage_mask(VulkanPipelineStageFlagS::COLOR_ATTACHMENT_OUTPUT)
            .dst_access_mask(VulkanAccessFlagS::COLOR_ATTACHMENT_WRITE);
        let vulkan_attachment_s = &[vulkan_color_attachment];
        let vulkan_subpass_s = &[vulkan_subpass];
        let vulkan_subpass_dependency_s = &[vulkan_subpass_dependency];
        let vulkan_render_pass_create_infomation =
            VulkanRenderPassCreateInformation::builder()
            .attachments(vulkan_attachment_s)
            .subpasses(vulkan_subpass_s)
            .dependencies(vulkan_subpass_dependency_s);
        let create_vulkan_render_pass_result =
            vulkan_logical_device.create_render_pass(&vulkan_render_pass_create_infomation, None);
        match create_vulkan_render_pass_result {
            Err(error) => {
                let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                return Err(TerminationProcessMain::InitializationVulkanRenderPassCreateFail(vulkan_error_code));
            },
            Ok(render_pass) => Ok(render_pass),
        }
    }
}