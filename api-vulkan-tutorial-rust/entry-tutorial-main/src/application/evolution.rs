use std::mem::size_of;
use std::ptr::copy_nonoverlapping;

use ::nalgebra_glm as glm;
use ::vulkan::prelude::version1_2::*;
use ::vulkan::VulkanErrorCode;
use ::vulkan::VulkanMemoryMapFlagS;

use crate::termination::TerminationProcessMain;
use crate::application::main::Application;
use crate::lib::transform_d3_view_projection::TransformD3ViewProjection;
use crate::lib::transform_d3_model_view_projection::TransformD3ModelViewProjection;


pub struct ApplicationEvolution {}

impl ApplicationEvolution {
    pub unsafe fn update_state_transform_d3_view_projection(application: &Application)
     -> Result<(), TerminationProcessMain>
    {
        let main_view_3d_transform =
            glm::look_at(
                &glm::vec3(6.0, 0.0, 2.0),
                &glm::vec3(0.0, 0.0, 0.0),
                &glm::vec3(0.0, 0.0, 1.0));
        let mut main_projection_3d_transform =
            glm::perspective_rh_zo(
                application.vulkan_swapchain_extent.width as f32
                / application.vulkan_swapchain_extent.height as f32,
                glm::radians(&glm::vec1(45.0))[0],
                0.1,
                10.0);
        main_projection_3d_transform[(1, 1)] *= -1.0;
        let new_main_3d_transform =
            TransformD3ViewProjection::new(
                main_view_3d_transform,
                main_projection_3d_transform);
        //
        let current_main_3d_transform_vulkan_buffer_memory =
            application.vulkan_transform_d3_main_buffer_memory_s[application.vulkan_frame_index_current];
        let map_current_frame_main_3d_transform_vulkan_buffer_memory_address_result =
            application.vulkan_device_logical.map_memory(
                current_main_3d_transform_vulkan_buffer_memory,
                0,
                size_of::<TransformD3ModelViewProjection>() as u64,
                VulkanMemoryMapFlagS::empty());
        let current_frame_main_3d_transform_vulkan_buffer_memory_address =
            match map_current_frame_main_3d_transform_vulkan_buffer_memory_address_result {
                Err(error) => {
                    let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                    return Err(TerminationProcessMain::InitializationVulkanMemoryMapFail(vulkan_error_code));
                },
                Ok(address) => address,
            };
        copy_nonoverlapping(&new_main_3d_transform, current_frame_main_3d_transform_vulkan_buffer_memory_address.cast(), 1);
        application.vulkan_device_logical.unmap_memory(current_main_3d_transform_vulkan_buffer_memory);
        Ok(())
    }

    pub unsafe fn update_state_transform_d3_model_view_projection(
        application: &Application
    ) -> Result<(), TerminationProcessMain>
    {
        let elapsed_time = application.instant_start.elapsed().as_secs_f32();

        let main_model_3d_transform =
            glm::rotate(
                &glm::identity(),
                elapsed_time * glm::radians(&glm::vec1(90.0))[0],
                &glm::vec3(0.0, 0.0, 1.0));
        let main_view_3d_transform =
            glm::look_at(
                &glm::vec3(2.0, 2.0, 2.0),
                &glm::vec3(0.0, 0.0, 0.0),
                &glm::vec3(0.0, 0.0, 1.0));
        let mut main_projection_3d_transform =
            glm::perspective_rh_zo(
                application.vulkan_swapchain_extent.width as f32
                / application.vulkan_swapchain_extent.height as f32,
                glm::radians(&glm::vec1(45.0))[0],
                0.1,
                10.0);
        main_projection_3d_transform[(1, 1)] *= -1.0;
        let new_main_3d_transform =
            TransformD3ModelViewProjection::new(
                main_model_3d_transform,
                main_view_3d_transform,
                main_projection_3d_transform);
        //
        let current_main_3d_transform_vulkan_buffer_memory =
            application.vulkan_transform_d3_main_buffer_memory_s[application.vulkan_frame_index_current];
        let map_current_frame_main_3d_transform_vulkan_buffer_memory_address_result =
            application.vulkan_device_logical.map_memory(
                current_main_3d_transform_vulkan_buffer_memory,
                0,
                size_of::<TransformD3ModelViewProjection>() as u64,
                VulkanMemoryMapFlagS::empty());
        let current_frame_main_3d_transform_vulkan_buffer_memory_address =
            match map_current_frame_main_3d_transform_vulkan_buffer_memory_address_result {
                Err(error) => {
                    let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                    return Err(TerminationProcessMain::InitializationVulkanMemoryMapFail(vulkan_error_code));
                },
                Ok(address) => address,
            };
        copy_nonoverlapping(&new_main_3d_transform, current_frame_main_3d_transform_vulkan_buffer_memory_address.cast(), 1);
        application.vulkan_device_logical.unmap_memory(current_main_3d_transform_vulkan_buffer_memory);
        Ok(())
    }
}