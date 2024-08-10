use ::vulkan::prelude::version1_2::*;
use ::vulkan::extend::VulkanErrorCode;
use ::vulkan::VulkanExtentD2;
use ::vulkan::VulkanImageView;
use ::vulkan::VulkanRenderPass;
use ::vulkan::VulkanFrameBufferCreateInformation;
use ::vulkan::VulkanFrameBuffer;

use crate::termination::TerminationProcessMain;


pub struct ApplicationVulkanFrameBuffer {}

impl ApplicationVulkanFrameBuffer {
    pub unsafe fn create_all(
        vulkan_logical_device: &VulkanDeviceLogical,
        vulkan_image_view_s: &Vec<VulkanImageView>,
        vulkan_depth_stencil_image_view: VulkanImageView,
        vulkan_anti_aliasing_multisampling_image_view: VulkanImageView,
        vulkan_render_pass: VulkanRenderPass,
        vulkan_extent: VulkanExtentD2)
     -> Result<Vec<VulkanFrameBuffer>, TerminationProcessMain>
    {
        let create_vulkan_frame_buffer_s_result =
            vulkan_image_view_s
            .iter()
            .map(|i| {
                let vulkan_attachment_s =
                    &[vulkan_anti_aliasing_multisampling_image_view, vulkan_depth_stencil_image_view, *i];
                let vulkan_frame_buffer_create_information =
                    VulkanFrameBufferCreateInformation::builder()
                    .render_pass(vulkan_render_pass)
                    .attachments(vulkan_attachment_s)
                    .width(vulkan_extent.width)
                    .height(vulkan_extent.height)
                    .layers(1);
                vulkan_logical_device.create_framebuffer(&vulkan_frame_buffer_create_information, None)
            })
            .collect::<Result<Vec<_>, _>>();
        termination_vulkan_error!(normal1,
            create_vulkan_frame_buffer_s_result, TerminationProcessMain::InitializationVulkanFrameBufferCreateFail)
    }
}