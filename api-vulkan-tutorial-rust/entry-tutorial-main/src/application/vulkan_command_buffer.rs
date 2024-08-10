use std::time::Instant;

use ::nalgebra_glm as glm;
use ::vulkan::prelude::version1_2::*;
use ::vulkan::VulkanRenderPass;
use ::vulkan::VulkanExtentD2;
use ::vulkan::VulkanPipeline;
use ::vulkan::extend::VulkanErrorCode;
use ::vulkan::VulkanFrameBuffer;
use ::vulkan::VulkanCommandPool;
use ::vulkan::VulkanCommandBufferAllocateInformation;
use ::vulkan::VulkanCommandBufferLevel;
use ::vulkan::VulkanCommandBufferBeginInformation;
use ::vulkan::VulkanClearValue;
use ::vulkan::VulkanClearColorValue;
use ::vulkan::VulkanRectangleD2;
use ::vulkan::VulkanOffsetD2;
use ::vulkan::VulkanRenderPassBeginInformation;
use ::vulkan::VulkanSubpassContents;
use ::vulkan::VulkanPipelineBindPoint;
use ::vulkan::VulkanCommandBuffer;
use ::vulkan::VulkanBuffer;
use ::vulkan::VulkanIndexType;
use ::vulkan::VulkanPipelineLayout;
use ::vulkan::VulkanDescriptorSet;
use ::vulkan::VulkanCommandBufferUsageFlagS;
use ::vulkan::VulkanSubmitInformation;
use ::vulkan::VulkanQueue;
use ::vulkan::VulkanFence;
use ::vulkan::VulkanClearDepthStencilValue;
use ::vulkan::VulkanShaderStageFlagS;
use ::vulkan::VulkanImage;
use ::vulkan::VulkanCommandPoolResetFlagS;
use ::vulkan::VulkanCommandBufferInheritanceInformation;

use crate::termination::TerminationProcessMain;
use crate::lib::d3_model_mesh::D3ModelMesh;
use crate::lib::window_viewport::WindowViewportLogicalNumber;
use crate::lib::window_viewport::WindowViewportLogicalIndex;


pub struct ApplicationVulkanCommandBufferOld {}

impl ApplicationVulkanCommandBufferOld {
    pub unsafe fn create_all(
        vulkan_logical_device: &VulkanDeviceLogical,
        vulkan_pipeline_layout: VulkanPipelineLayout,
        vulkan_command_pool: VulkanCommandPool,
        vulkan_frame_buffer_s: &Vec<VulkanFrameBuffer>,
        vulkan_extent: VulkanExtentD2,
        vulkan_render_pass: VulkanRenderPass,
        vulkan_pipeline: VulkanPipeline,
        vulkan_vertex_buffer: VulkanBuffer,
        vulkan_vertex_index_buffer: VulkanBuffer,
        d3_model_mesh: &D3ModelMesh,
        vulkan_descriptor_set_s: &Vec<VulkanDescriptorSet>)
     -> Result<Vec<VulkanCommandBuffer>, TerminationProcessMain>
    {
        let input_vertex_index_number =
            match d3_model_mesh {
                D3ModelMesh::TutorialSimple(mesh) => mesh.vertex_index_s.len(),
                D3ModelMesh::TutorialFormatObj(mesh) => mesh.vertex_index_s.len(),
            };
        let vulkan_command_buffer_allocate_information =
            VulkanCommandBufferAllocateInformation::builder()
            .command_pool(vulkan_command_pool)
            .level(VulkanCommandBufferLevel::PRIMARY)
            .command_buffer_count(vulkan_frame_buffer_s.len() as u32);
        let allocate_vulkan_command_buffer_s_result =
            vulkan_logical_device.allocate_command_buffers(&vulkan_command_buffer_allocate_information);
        let vulkan_command_buffer_s = termination_vulkan_error!(return1,
            allocate_vulkan_command_buffer_s_result, TerminationProcessMain::InitializationVulkanCommandBufferSAllocateFail);
        let model_3d_transform =
            glm::rotate(&glm::identity(), 0.0f32, &glm::vec3(0.0, 0.0, 1.0));
        let (_, model_3d_transform_byte_s, _) = model_3d_transform.as_slice().align_to::<u8>();
        let opacity = 0.25f32;
        let opacity_byte_s = &opacity.to_ne_bytes()[..];

        for (index, vulkan_command_buffer) in vulkan_command_buffer_s.iter().enumerate() {
            let vulkan_command_buffer_begin_information = VulkanCommandBufferBeginInformation::builder();
            let begin_vulkan_command_buffer_result =
                vulkan_logical_device.begin_command_buffer(
                    *vulkan_command_buffer, &vulkan_command_buffer_begin_information);
            termination_vulkan_error!(return1,
                begin_vulkan_command_buffer_result, TerminationProcessMain::InitializationVulkanCommandBufferBeginFail);
            let vulkan_render_area =
                VulkanRectangleD2::builder()
                .offset(VulkanOffsetD2::default())
                .extent(vulkan_extent);
            let vulkan_color_clear_value =
                VulkanClearValue { color: VulkanClearColorValue { float32: [0.0, 0.0, 0.0, 1.0] } };
            let vulkan_depth_stencil_clear_value =
                VulkanClearValue { depth_stencil: VulkanClearDepthStencilValue { depth: 1.0, stencil: 0 } };
            let vulkan_clear_value_s = &[vulkan_color_clear_value, vulkan_depth_stencil_clear_value];
            let vulkan_render_pass_begin_information =
                VulkanRenderPassBeginInformation::builder()
                .render_pass(vulkan_render_pass)
                .framebuffer(vulkan_frame_buffer_s[index])
                .render_area(vulkan_render_area)
                .clear_values(vulkan_clear_value_s);
            vulkan_logical_device.cmd_begin_render_pass(
                *vulkan_command_buffer, &vulkan_render_pass_begin_information, VulkanSubpassContents::INLINE);
            vulkan_logical_device.cmd_bind_pipeline(
                *vulkan_command_buffer, VulkanPipelineBindPoint::GRAPHICS, vulkan_pipeline);
            vulkan_logical_device.cmd_bind_vertex_buffers(
                *vulkan_command_buffer, 0, &[vulkan_vertex_buffer], &[0]);
            match d3_model_mesh {
                D3ModelMesh::TutorialSimple(_) => vulkan_logical_device.cmd_bind_index_buffer(
                    *vulkan_command_buffer, vulkan_vertex_index_buffer, 0, VulkanIndexType::UINT16),
                D3ModelMesh::TutorialFormatObj(_) => vulkan_logical_device.cmd_bind_index_buffer(
                    *vulkan_command_buffer, vulkan_vertex_index_buffer, 0, VulkanIndexType::UINT32),
            };
            vulkan_logical_device.cmd_bind_descriptor_sets(
                *vulkan_command_buffer, VulkanPipelineBindPoint::GRAPHICS,
                vulkan_pipeline_layout, 0, &[vulkan_descriptor_set_s[index]], &[]);
            vulkan_logical_device.cmd_push_constants(
                *vulkan_command_buffer, vulkan_pipeline_layout,
                VulkanShaderStageFlagS::VERTEX, 0, model_3d_transform_byte_s);
            vulkan_logical_device.cmd_push_constants(
                *vulkan_command_buffer, vulkan_pipeline_layout,
                VulkanShaderStageFlagS::FRAGMENT, 64, opacity_byte_s);
            vulkan_logical_device.cmd_draw_indexed(
                *vulkan_command_buffer, input_vertex_index_number as u32, 1, 0, 0, 0);
            vulkan_logical_device.cmd_end_render_pass(*vulkan_command_buffer);
            let end_vulkan_command_buffer_result =
                vulkan_logical_device.end_command_buffer(*vulkan_command_buffer);
            termination_vulkan_error!(return1,
                end_vulkan_command_buffer_result, TerminationProcessMain::InitializationVulkanCommandBufferEndFail);
        };
        Ok(vulkan_command_buffer_s)
    }
}

pub struct ApplicationVulkanCommandBufferSwapchainImage {}

impl ApplicationVulkanCommandBufferSwapchainImage {
    pub unsafe fn create_blank_all(
        vulkan_logical_device: &VulkanDeviceLogical,
        vulkan_swapchain_image_s: &Vec<VulkanImage>,
        vulkan_command_pool_s: &Vec<VulkanCommandPool>)
     -> Result<Vec<VulkanCommandBuffer>, TerminationProcessMain> {
        let mut vulkan_command_buffer_s: Vec<VulkanCommandBuffer> = Vec::new();
        vulkan_command_buffer_s.reserve_exact(vulkan_swapchain_image_s.len());
        for image_index in 0..vulkan_swapchain_image_s.len() {
            let vulkan_command_buffer_allocate_information =
                VulkanCommandBufferAllocateInformation::builder()
                .command_pool(vulkan_command_pool_s[image_index])
                .level(VulkanCommandBufferLevel::PRIMARY)
                .command_buffer_count(1);
            let allocate_vulkan_command_buffer_s_result =
                vulkan_logical_device.allocate_command_buffers(&vulkan_command_buffer_allocate_information);
            let vulkan_command_buffer = termination_vulkan_error!(return1,
                allocate_vulkan_command_buffer_s_result,
                TerminationProcessMain::InitializationVulkanCommandBufferSAllocateFail)[0];
            vulkan_command_buffer_s.push(vulkan_command_buffer);
        }
        Ok(vulkan_command_buffer_s)
    }

    pub unsafe fn update_by_swapchain_image_index(
        vulkan_logical_device: &VulkanDeviceLogical,
        vulkan_pipeline: VulkanPipeline,
        vulkan_command_pool_s: &mut Vec<VulkanCommandPool>,
        vulkan_swapchain_image_command_buffer_s: &Vec<VulkanCommandBuffer>,
        vulkan_swapchain_image_logical_viewport_command_buffer_tt: &mut Vec<Vec<VulkanCommandBuffer>>,
        vulkan_swapchain_extent: VulkanExtentD2,
        vulkan_swapchain_image_index: usize,
        vulkan_render_pass: VulkanRenderPass,
        vulkan_frame_buffer_s: &Vec<VulkanFrameBuffer>,
        vulkan_vertex_buffer: VulkanBuffer,
        vulkan_vertex_index_buffer: VulkanBuffer,
        vulkan_pipeline_layout: VulkanPipelineLayout,
        start_instant: Instant,
        d3_model_mesh: &D3ModelMesh,
        vulkan_descriptor_set_s: &Vec<VulkanDescriptorSet>,
        window_logical_viewport_number: &WindowViewportLogicalNumber)
     -> Result<(), TerminationProcessMain>
    {
        let vulkan_swapchain_image_command_pool = vulkan_command_pool_s[vulkan_swapchain_image_index];
        let reset_vulkan_command_pool_result =
            vulkan_logical_device.reset_command_pool(vulkan_swapchain_image_command_pool, VulkanCommandPoolResetFlagS::empty());
        termination_vulkan_error!(return1,
            reset_vulkan_command_pool_result, TerminationProcessMain::InitializationVulkanResetCommandPoolFail);
        let vulkan_logical_viewport_command_buffer_t =
            &mut vulkan_swapchain_image_logical_viewport_command_buffer_tt[vulkan_swapchain_image_index];
        let vulkan_swapchain_image_command_buffer = vulkan_swapchain_image_command_buffer_s[vulkan_swapchain_image_index];
        let vulkan_swapchain_image_frame_buffer = vulkan_frame_buffer_s[vulkan_swapchain_image_index];
        let vulkan_swapchain_image_descriptor_set = vulkan_descriptor_set_s[vulkan_swapchain_image_index];
        //
        let vulkan_command_buffer_begin_information =
            VulkanCommandBufferBeginInformation::builder()
            .flags(VulkanCommandBufferUsageFlagS::ONE_TIME_SUBMIT);
        let begin_vulkan_command_buffer_result =
            vulkan_logical_device.begin_command_buffer(
                vulkan_swapchain_image_command_buffer, &vulkan_command_buffer_begin_information);
        termination_vulkan_error!(return1,
            begin_vulkan_command_buffer_result, TerminationProcessMain::InitializationVulkanCommandBufferBeginFail);
        let vulkan_render_area =
            VulkanRectangleD2::builder()
            .offset(VulkanOffsetD2::default())
            .extent(vulkan_swapchain_extent);
        let vulkan_color_clear_value =
            VulkanClearValue { color: VulkanClearColorValue { float32: [0.0, 0.0, 0.0, 1.0] } };
        let vulkan_depth_stencil_clear_value =
            VulkanClearValue { depth_stencil: VulkanClearDepthStencilValue { depth: 1.0, stencil: 0 } };
        let vulkan_clear_value_s = &[vulkan_color_clear_value, vulkan_depth_stencil_clear_value];
        let vulkan_render_pass_begin_information =
            VulkanRenderPassBeginInformation::builder()
            .render_pass(vulkan_render_pass)
            .framebuffer(vulkan_swapchain_image_frame_buffer)
            .render_area(vulkan_render_area)
            .clear_values(vulkan_clear_value_s);
        vulkan_logical_device.cmd_begin_render_pass(
            vulkan_swapchain_image_command_buffer, &vulkan_render_pass_begin_information, VulkanSubpassContents::SECONDARY_COMMAND_BUFFERS);
        let vulkan_logical_viewport_command_buffer_s =
            (0..window_logical_viewport_number.as_raw())
            .map(|viewport_index| {
                Self::record_window_logical_viewport(
                    vulkan_logical_device,
                    vulkan_pipeline,
                    vulkan_logical_viewport_command_buffer_t,
                    WindowViewportLogicalIndex::new(viewport_index),
                    vulkan_render_pass,
                    vulkan_swapchain_image_frame_buffer,
                    vulkan_vertex_buffer,
                    vulkan_vertex_index_buffer,
                    vulkan_pipeline_layout,
                    start_instant,
                    d3_model_mesh,
                    vulkan_swapchain_image_descriptor_set)
            })
            .collect::<Result<Vec<_>, _>>()?;
        vulkan_logical_device.cmd_execute_commands(
            vulkan_swapchain_image_command_buffer, &vulkan_logical_viewport_command_buffer_s[..]);
        vulkan_logical_device.cmd_end_render_pass(vulkan_swapchain_image_command_buffer);
        let end_vulkan_command_buffer_result =
            vulkan_logical_device.end_command_buffer(vulkan_swapchain_image_command_buffer);
        termination_vulkan_error!(return1,
            end_vulkan_command_buffer_result, TerminationProcessMain::InitializationVulkanCommandBufferEndFail);
        Ok(())
    }

    unsafe fn record_window_logical_viewport(
        vulkan_logical_device: &VulkanDeviceLogical,
        vulkan_pipeline: VulkanPipeline,
        vulkan_logical_viewport_command_buffer_t: &mut Vec<VulkanCommandBuffer>,
        window_logical_viewport_index: WindowViewportLogicalIndex,
        vulkan_render_pass: VulkanRenderPass,
        vulkan_frame_buffer: VulkanFrameBuffer,
        vulkan_vertex_buffer: VulkanBuffer,
        vulkan_vertex_index_buffer: VulkanBuffer,
        vulkan_pipeline_layout: VulkanPipelineLayout,
        start_instant: Instant,
        d3_model_mesh: &D3ModelMesh,
        vulkan_descriptor_set: VulkanDescriptorSet)
     -> Result<VulkanCommandBuffer, TerminationProcessMain>
    {
        let input_vertex_index_number =
            match d3_model_mesh {
                D3ModelMesh::TutorialSimple(mesh) => mesh.vertex_index_s.len(),
                D3ModelMesh::TutorialFormatObj(mesh) => mesh.vertex_index_s.len(),
            };
        //
        let vulkan_logical_viewport_command_buffer =
            vulkan_logical_viewport_command_buffer_t[window_logical_viewport_index.as_raw()];
        //
        let y_axis_modification = (((window_logical_viewport_index.as_raw() % 2) as f32) * 2.5) - 1.25;
        let z_axis_modification = (((window_logical_viewport_index.as_raw() / 2) as f32) * -2.0) + 1.0;
        let model_3d_transform =
            glm::translate( &glm::identity(), &glm::vec3(0.0, y_axis_modification, z_axis_modification));
        let elapsed_time = start_instant.elapsed().as_secs_f32();
        let model_3d_transform =
            glm::rotate(
                &model_3d_transform,
                elapsed_time * glm::radians(&glm::vec1(90.0))[0],
                &glm::vec3(0.0, 0.0, 1.0));
        let (_, model_3d_transform_byte_s, _) = model_3d_transform.as_slice().align_to::<u8>();
        let opacity = (window_logical_viewport_index.as_raw() + 1) as f32 * 0.25;
        let opacity_byte_s = &opacity.to_ne_bytes()[..];
        //
        let vulkan_command_buffer_inheritance_information =
            VulkanCommandBufferInheritanceInformation::builder()
            .render_pass(vulkan_render_pass)
            .subpass(0)
            .framebuffer(vulkan_frame_buffer);
        let vulkan_command_buffer_begin_information =
            VulkanCommandBufferBeginInformation::builder()
            .flags(VulkanCommandBufferUsageFlagS::RENDER_PASS_CONTINUE)
            .inheritance_info(&vulkan_command_buffer_inheritance_information);
        let begin_vulkan_command_buffer_result =
            vulkan_logical_device.begin_command_buffer(
                vulkan_logical_viewport_command_buffer, &vulkan_command_buffer_begin_information);
        termination_vulkan_error!(return1,
            begin_vulkan_command_buffer_result, TerminationProcessMain::InitializationVulkanCommandBufferBeginFail);
        vulkan_logical_device.cmd_bind_pipeline(
            vulkan_logical_viewport_command_buffer, VulkanPipelineBindPoint::GRAPHICS, vulkan_pipeline);
        vulkan_logical_device.cmd_bind_vertex_buffers(
            vulkan_logical_viewport_command_buffer, 0, &[vulkan_vertex_buffer], &[0]);
        vulkan_logical_device.cmd_bind_index_buffer(
            vulkan_logical_viewport_command_buffer, vulkan_vertex_index_buffer, 0, VulkanIndexType::UINT32);
        vulkan_logical_device.cmd_bind_descriptor_sets(
            vulkan_logical_viewport_command_buffer, VulkanPipelineBindPoint::GRAPHICS,
            vulkan_pipeline_layout, 0, &[vulkan_descriptor_set], &[]);
        vulkan_logical_device.cmd_push_constants(
            vulkan_logical_viewport_command_buffer, vulkan_pipeline_layout,
            VulkanShaderStageFlagS::VERTEX, 0, model_3d_transform_byte_s);
        vulkan_logical_device.cmd_push_constants(
            vulkan_logical_viewport_command_buffer, vulkan_pipeline_layout,
            VulkanShaderStageFlagS::FRAGMENT, 64, opacity_byte_s);
        vulkan_logical_device.cmd_draw_indexed(
            vulkan_logical_viewport_command_buffer, input_vertex_index_number as u32, 1, 0, 0, 0);
        let end_vulkan_command_buffer_result =
            vulkan_logical_device.end_command_buffer(vulkan_logical_viewport_command_buffer);
        termination_vulkan_error!(return1,
            end_vulkan_command_buffer_result, TerminationProcessMain::InitializationVulkanCommandBufferEndFail);
        Ok(vulkan_logical_viewport_command_buffer)
    }
}

pub struct ApplicationVulkanCommandBufferOneTime {}

impl ApplicationVulkanCommandBufferOneTime {
    pub unsafe fn create_and_begin(
        vulkan_logical_device: &VulkanDeviceLogical,
        vulkan_command_pool: VulkanCommandPool)
     -> Result<VulkanCommandBuffer, TerminationProcessMain>
    {
        let vulkan_command_buffer_allocate_information =
            VulkanCommandBufferAllocateInformation::builder()
            .level(VulkanCommandBufferLevel::PRIMARY)
            .command_pool(vulkan_command_pool)
            .command_buffer_count(1);
        let allocate_vulkan_command_buffer_result =
            vulkan_logical_device.allocate_command_buffers(&vulkan_command_buffer_allocate_information);
        let vulkan_command_buffer = termination_vulkan_error!(return1,
            allocate_vulkan_command_buffer_result,
            TerminationProcessMain::InitializationVulkanCommandBufferSAllocateFail)[0];
        //
        let vulkan_command_buffer_begin_information =
            VulkanCommandBufferBeginInformation::builder()
            .flags(VulkanCommandBufferUsageFlagS::ONE_TIME_SUBMIT);
        let begin_vulkan_command_buffer_result =
            vulkan_logical_device.begin_command_buffer(vulkan_command_buffer, &vulkan_command_buffer_begin_information);
        termination_vulkan_error!(return1,
            begin_vulkan_command_buffer_result, TerminationProcessMain::InitializationVulkanCommandBufferBeginFail);
        Ok(vulkan_command_buffer)
    }

    pub unsafe fn end_submit_wait(
        vulkan_logical_device: &VulkanDeviceLogical,
        vulkan_command_pool: VulkanCommandPool,
        vulkan_command_buffer: VulkanCommandBuffer,
        vulkan_graphic_queue: VulkanQueue)
     -> Result<(), TerminationProcessMain>
    {
        let end_vulkan_command_buffer_result = vulkan_logical_device.end_command_buffer(vulkan_command_buffer);
        termination_vulkan_error!(return1,
            end_vulkan_command_buffer_result, TerminationProcessMain::InitializationVulkanCommandBufferEndFail);
        let vulkan_command_buffer_s = &[vulkan_command_buffer];
        let vulkan_submit_information =
            VulkanSubmitInformation::builder()
            .command_buffers(vulkan_command_buffer_s);
        let vulkan_submit_result =
            vulkan_logical_device.queue_submit(
                vulkan_graphic_queue, &[vulkan_submit_information], VulkanFence::null());
        termination_vulkan_error!(return1,
            vulkan_submit_result, TerminationProcessMain::InitializationVulkanQueueSubmitFail);
        let wait_vulkan_queue_idle_result = vulkan_logical_device.queue_wait_idle(vulkan_graphic_queue);
        termination_vulkan_error!(return1,
            wait_vulkan_queue_idle_result, TerminationProcessMain::InitializationVulkanDeviceWaitIdleFail);
        vulkan_logical_device.free_command_buffers(vulkan_command_pool, vulkan_command_buffer_s);
        Ok(())
    }
}