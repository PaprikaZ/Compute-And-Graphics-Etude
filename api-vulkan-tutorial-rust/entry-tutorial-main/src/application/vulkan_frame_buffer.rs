use ::vulkan::prelude::version1_2::*;
use ::vulkan::VulkanErrorCode;
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
        vulkan_render_pass: VulkanRenderPass,
        vulkan_extent: VulkanExtentD2)
     -> Result<Vec<VulkanFrameBuffer>, TerminationProcessMain>
    {
        let create_vulkan_frame_buffer_s_result =
            vulkan_image_view_s
            .iter()
            .map(|i| {
                let vulkan_attachment_s = &[*i, vulkan_depth_stencil_image_view];
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
        match create_vulkan_frame_buffer_s_result {
            Err(error) => {
                let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                Err(TerminationProcessMain::InitializationVulkanFrameBufferCreateFail(vulkan_error_code))
            },
            Ok(frame_buffer_s) => Ok(frame_buffer_s)
        }
    }
}