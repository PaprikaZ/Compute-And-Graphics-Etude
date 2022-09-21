use ::vulkan::prelude::version1_2::*;
use ::vulkan::extend::VulkanErrorCode;
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
use ::vulkan::VulkanDevicePhysical;

use crate::termination::TerminationProcessMain;
use crate::application::vulkan_depth::ApplicationVulkanDepth;


pub struct ApplicationVulkanRenderPass {}

impl ApplicationVulkanRenderPass {
    pub unsafe fn create(
        vulkan_instance: &VulkanInstance,
        vulkan_physical_device: VulkanDevicePhysical,
        vulkan_logical_device: &VulkanDeviceLogical,
        vulkan_swapchain_format: VulkanFormat,
        vulkan_anti_aliasing_multisampling_number: VulkanSampleCountFlagS)
     -> Result<VulkanRenderPass, TerminationProcessMain>
    {
        let (vulkan_color_attachment, vulkan_color_attachment_reference) =
            Self::create_attachment_color(vulkan_swapchain_format, vulkan_anti_aliasing_multisampling_number);
        let (vulkan_depth_stencil_attachment, vulkan_depth_stencil_attachment_reference) =
            Self::create_attachment_depth_stencil(
                vulkan_instance, vulkan_physical_device, vulkan_anti_aliasing_multisampling_number)?;
        let (vulkan_anti_aliasing_multisampling_attachment, vulkan_anti_aliasing_multisampling_attachment_reference) =
            Self::create_attachment_anti_aliasing_multisampling(vulkan_swapchain_format);
        let vulkan_color_attachment_s = &[vulkan_color_attachment_reference];
        let vulkan_resolve_attachment_s = &[vulkan_anti_aliasing_multisampling_attachment_reference];
        let vulkan_subpass =
            VulkanSubpassDescription::builder()
            .pipeline_bind_point(VulkanPipelineBindPoint::GRAPHICS)
            .color_attachments(vulkan_color_attachment_s)
            .depth_stencil_attachment(&vulkan_depth_stencil_attachment_reference)
            .resolve_attachments(vulkan_resolve_attachment_s);
        let vulkan_subpass_dependency=
            VulkanSubpassDependency::builder()
            .src_subpass(VULKAN_SUBPASS_EXTERNAL)
            .dst_subpass(0)
            .src_stage_mask(VulkanPipelineStageFlagS::COLOR_ATTACHMENT_OUTPUT | VulkanPipelineStageFlagS::EARLY_FRAGMENT_TESTS)
            .src_access_mask(VulkanAccessFlagS::empty())
            .dst_stage_mask(VulkanPipelineStageFlagS::COLOR_ATTACHMENT_OUTPUT | VulkanPipelineStageFlagS::EARLY_FRAGMENT_TESTS)
            .dst_access_mask(VulkanAccessFlagS::COLOR_ATTACHMENT_WRITE | VulkanAccessFlagS::DEPTH_STENCIL_ATTACHMENT_WRITE);
        let vulkan_attachment_s =
            &[vulkan_color_attachment, vulkan_depth_stencil_attachment, vulkan_anti_aliasing_multisampling_attachment];
        let vulkan_subpass_s = &[vulkan_subpass];
        let vulkan_subpass_dependency_s = &[vulkan_subpass_dependency];
        let vulkan_render_pass_create_infomation =
            VulkanRenderPassCreateInformation::builder()
            .attachments(vulkan_attachment_s)
            .subpasses(vulkan_subpass_s)
            .dependencies(vulkan_subpass_dependency_s);
        let create_vulkan_render_pass_result =
            vulkan_logical_device.create_render_pass(&vulkan_render_pass_create_infomation, None);
        termination_vulkan_error!(normal1,
            create_vulkan_render_pass_result, TerminationProcessMain::InitializationVulkanRenderPassCreateFail)
    }

    unsafe fn create_attachment_color(
        vulkan_swapchain_format: VulkanFormat, vulkan_anti_aliasing_multisampling_number: VulkanSampleCountFlagS)
     -> (VulkanAttachmentDescription, VulkanAttachmentReference)
    {
        let vulkan_color_attachment =
            VulkanAttachmentDescription::builder()
            .format(vulkan_swapchain_format)
            .samples(vulkan_anti_aliasing_multisampling_number)
            .load_op(VulkanAttachmentLoadOperation::CLEAR)
            .store_op(VulkanAttachmentStoreOperation::STORE)
            .stencil_load_op(VulkanAttachmentLoadOperation::DONT_CARE)
            .stencil_store_op(VulkanAttachmentStoreOperation::DONT_CARE)
            .initial_layout(VulkanImageLayout::UNDEFINED)
            .final_layout(VulkanImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .build();
        let vulkan_color_attachment_reference =
            VulkanAttachmentReference::builder()
            .attachment(0)
            .layout(VulkanImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .build();
        (vulkan_color_attachment, vulkan_color_attachment_reference)
    }

    unsafe fn create_attachment_depth_stencil(
        vulkan_instance: &VulkanInstance,
        vulkan_physical_device: VulkanDevicePhysical,
        vulkan_anti_aliasing_multisampling_number: VulkanSampleCountFlagS)
     -> Result<(VulkanAttachmentDescription, VulkanAttachmentReference), TerminationProcessMain>
    {
        let vulkan_depth_format = ApplicationVulkanDepth::get_format(vulkan_instance, vulkan_physical_device)?;
        let vulkan_depth_stencil_attachment =
            VulkanAttachmentDescription::builder()
            .format(vulkan_depth_format)
            .samples(vulkan_anti_aliasing_multisampling_number)
            .load_op(VulkanAttachmentLoadOperation::CLEAR)
            .store_op(VulkanAttachmentStoreOperation::DONT_CARE)
            .stencil_load_op(VulkanAttachmentLoadOperation::DONT_CARE)
            .stencil_store_op(VulkanAttachmentStoreOperation::DONT_CARE)
            .initial_layout(VulkanImageLayout::UNDEFINED)
            .final_layout(VulkanImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL)
            .build();
        let vulkan_depth_stencil_attachment_reference =
            VulkanAttachmentReference::builder()
            .attachment(1)
            .layout(VulkanImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL)
            .build();
        Ok((vulkan_depth_stencil_attachment, vulkan_depth_stencil_attachment_reference))
    }

    unsafe fn create_attachment_anti_aliasing_multisampling(vulkan_swapchain_format: VulkanFormat)
     -> (VulkanAttachmentDescription, VulkanAttachmentReference)
    {
        let vulkan_anti_aliasing_multisampling_attachment =
            VulkanAttachmentDescription::builder()
            .format(vulkan_swapchain_format)
            .samples(VulkanSampleCountFlagS::_1)
            .load_op(VulkanAttachmentLoadOperation::DONT_CARE)
            .store_op(VulkanAttachmentStoreOperation::STORE)
            .stencil_load_op(VulkanAttachmentLoadOperation::DONT_CARE)
            .stencil_store_op(VulkanAttachmentStoreOperation::DONT_CARE)
            .initial_layout(VulkanImageLayout::UNDEFINED)
            .final_layout(VulkanImageLayout::PRESENT_SRC_KHR)
            .build();
        let vulkan_anti_aliasing_multisampling_attachment_reference =
            VulkanAttachmentReference::builder()
            .attachment(2)
            .layout(VulkanImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .build();
        (vulkan_anti_aliasing_multisampling_attachment, vulkan_anti_aliasing_multisampling_attachment_reference)
    }
}