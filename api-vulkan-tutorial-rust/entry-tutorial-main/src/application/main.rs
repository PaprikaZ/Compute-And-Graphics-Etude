use std::time::Instant;

use ::window_uniform::prelude::*;
use ::vulkan::VulkanExtensionName;
use ::vulkan::VulkanExtensionDebugUtilityMessenger;
use ::vulkan::prelude::version1_2::*;
use ::vulkan::VulkanErrorCode;
use ::vulkan::VulkanErrorCode_;
use ::vulkan::VulkanSuccessCode_;
use ::vulkan::VulkanQueue;
use ::vulkan::VulkanBuffer;
use ::vulkan::VulkanDevicePhysical;
use ::vulkan::VulkanSurfaceKhr;
use ::vulkan::VulkanFormat;
use ::vulkan::VulkanExtentD2;
use ::vulkan::VulkanSwapchainKhr;
use ::vulkan::VulkanImage;
use ::vulkan::VulkanImageView;
use ::vulkan::VulkanRenderPass;
use ::vulkan::VulkanPipelineLayout;
use ::vulkan::VulkanPipeline;
use ::vulkan::VulkanFrameBuffer;
use ::vulkan::VulkanCommandPool;
use ::vulkan::VulkanCommandBuffer;
use ::vulkan::VulkanSemaphore;
use ::vulkan::VulkanFence;
use ::vulkan::VulkanDeviceMemory;
use ::vulkan::VulkanQueueFamilyIndexGraphic;
use ::vulkan::VulkanQueueFamilyIndexSurface;
use ::vulkan::VulkanExtensionDebugUtility;
use ::vulkan::VulkanSurfaceExtensionKhr;
use ::vulkan::VulkanSwapchainExtensionKhr;
use ::vulkan::VulkanPipelineStageFlagS;
use ::vulkan::VulkanSubmitInformation;
use ::vulkan::VulkanPresentInformationKhr;
use ::vulkan::VulkanDescriptorSetLayout;
use ::vulkan::VulkanDescriptorPool;
use ::vulkan::VulkanDescriptorSet;
use ::vulkan::VulkanSampler;

use crate::config::vulkan::VULKAN_FRAME_IN_FLIGHT_MAX;
use crate::lib::d3_model_mesh::D3ModelMesh;
use crate::data::d3_model_resource::DataD3ModelResource;
use crate::termination::TerminationProcessMain;
use crate::application::vulkan_instance_swapchain::ApplicationVulkanInstanceSwapchain;
use crate::application::vulkan_instance_swapchain_image_view::ApplicationInstanceSwapchainImageView;
use crate::application::vulkan_render_pass::ApplicationVulkanRenderPass;
use crate::application::vulkan_pipeline::ApplicationVulkanPipeline;
use crate::application::vulkan_frame_buffer::ApplicationVulkanFrameBuffer;
use crate::application::vulkan_command_buffer::ApplicationVulkanCommandBuffer;
use crate::application::vulkan_instance_validation_wi::ApplicationVulkanInstanceValidationWi;
use crate::application::vulkan_instance_validation_wo::ApplicationVulkanInstanceValidationWo;
use crate::application::evolution::ApplicationEvolution;
use crate::application::vulkan_transform_d3_buffer::ApplicationVulkanTransformD3Buffer;
use crate::application::vulkan_descriptor::ApplicationVulkanDescriptorPool;
use crate::application::vulkan_descriptor::ApplicationVulkanDescriptorSet;
use crate::application::vulkan_depth::ApplicationVulkanDepth;


pub struct Application {
    pub instant_start: Instant,
    pub signal_window_resized: bool,
    pub vulkan_entry: VulkanEntry,
    pub vulkan_instance: VulkanInstance,
    pub vulkan_debug_messenger: Option<VulkanExtensionDebugUtilityMessenger>,
    pub vulkan_device_physical: VulkanDevicePhysical,
    pub vulkan_device_logical: VulkanDeviceLogical,
    pub vulkan_queue_family_index_graphic: VulkanQueueFamilyIndexGraphic,
    pub vulkan_queue_graphic: VulkanQueue,
    pub vulkan_surface: VulkanSurfaceKhr,
    pub vulkan_queue_family_index_present: VulkanQueueFamilyIndexSurface,
    pub vulkan_queue_present: VulkanQueue,
    pub vulkan_swapchain_format: VulkanFormat,
    pub vulkan_swapchain_extent: VulkanExtentD2,
    pub vulkan_swapchain: VulkanSwapchainKhr,
    pub vulkan_swapchain_image_s: Vec<VulkanImage>,
    pub vulkan_swapchain_image_view_s: Vec<VulkanImageView>,
    pub vulkan_render_pass: VulkanRenderPass,
    pub vulkan_pipeline_layout: VulkanPipelineLayout,
    pub vulkan_pipeline: VulkanPipeline,
    pub vulkan_frame_buffer_s: Vec<VulkanFrameBuffer>,
    pub vulkan_command_pool: VulkanCommandPool,
    pub vulkan_command_buffer_s: Vec<VulkanCommandBuffer>,
    pub vulkan_semaphore_s_image_available: Vec<VulkanSemaphore>,
    pub vulkan_semaphore_s_render_finished: Vec<VulkanSemaphore>,
    pub vulkan_fence_s_in_flight_slide: Vec<VulkanFence>,
    pub vulkan_fence_s_in_flight_image: Vec<VulkanFence>,
    pub vulkan_frame_index_current: usize,
    pub vulkan_vertex_buffer: VulkanBuffer,
    pub vulkan_vertex_buffer_memory: VulkanDeviceMemory,
    pub vulkan_vertex_index_buffer: VulkanBuffer,
    pub vulkan_vertex_index_buffer_memory: VulkanDeviceMemory,
    pub vulkan_transform_d3_main_buffer_s: Vec<VulkanBuffer>,
    pub vulkan_transform_d3_main_buffer_memory_s: Vec<VulkanDeviceMemory>,
    pub vulkan_descriptor_set_layout: VulkanDescriptorSetLayout,
    pub vulkan_descriptor_pool: VulkanDescriptorPool,
    pub vulkan_descriptor_set_s: Vec<VulkanDescriptorSet>,
    pub vulkan_texture_image: VulkanImage,
    pub vulkan_texture_image_memory: VulkanDeviceMemory,
    pub vulkan_texture_image_view: VulkanImageView,
    pub vulkan_texture_sampler: VulkanSampler,
    pub vulkan_depth_image: VulkanImage,
    pub vulkan_depth_image_memory: VulkanDeviceMemory,
    pub vulkan_depth_image_view: VulkanImageView,
    pub d3_model_mesh: D3ModelMesh,
}

impl Application {
    pub unsafe fn create(
        window: &WindowUniformWindow,
        optional_validation_layer: Option<&VulkanExtensionName>,
        vulkan_physical_device_extension_s: &[VulkanExtensionName],
        d3_model_resource_name: DataD3ModelResource)
     -> Result<Self, TerminationProcessMain>
    {
        match optional_validation_layer {
            None => ApplicationVulkanInstanceValidationWo::create(
                window, vulkan_physical_device_extension_s, d3_model_resource_name),
            Some(validation_layer) => ApplicationVulkanInstanceValidationWi::create(
                window, validation_layer, vulkan_physical_device_extension_s, d3_model_resource_name),
        }
    }

    pub unsafe fn render(&mut self, window: &WindowUniformWindow) -> Result<(), TerminationProcessMain> {
        let vulkan_slide_in_flight_fence = self.vulkan_fence_s_in_flight_slide[self.vulkan_frame_index_current];
        let vulkan_image_in_flight_fence = self.vulkan_fence_s_in_flight_image[self.vulkan_frame_index_current];
        let vulkan_available_image_semaphore = self.vulkan_semaphore_s_image_available[self.vulkan_frame_index_current];
        let wait_vulkan_in_flight_fence_result =
            self.vulkan_device_logical.wait_for_fences(&[vulkan_slide_in_flight_fence], true, u64::max_value());
        let _ =
            match wait_vulkan_in_flight_fence_result {
                Err(error) => {
                    let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                    return Err(TerminationProcessMain::InitializationVulkanFenceWaitFail(vulkan_error_code));
                },
                Ok(success_code) => success_code,
            };
        let acquire_vulkan_next_image_index_result =
            self.vulkan_device_logical.acquire_next_image_khr(
                self.vulkan_swapchain, u64::max_value(), vulkan_available_image_semaphore, VulkanFence::null());
        let vulkan_next_image_index =
            match acquire_vulkan_next_image_index_result {
                Err(VulkanErrorCode_::OUT_OF_DATE_KHR) => return self.recreate_swapchain(window),
                Err(error) => {
                    let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                    return Err(TerminationProcessMain::InitializationVulkanAcquireNextImageFail(vulkan_error_code));
                },
                Ok((image_index, _success_code)) => image_index as usize,
            };
        if !vulkan_image_in_flight_fence.is_null() {
            let wait_vulkan_unknown_fence_result =
                self.vulkan_device_logical.wait_for_fences(&[vulkan_image_in_flight_fence], true, u64::max_value());
            match wait_vulkan_unknown_fence_result {
                Err(error) => {
                    let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                    return Err(TerminationProcessMain::InitializationVulkanFenceWaitFail(vulkan_error_code));
                },
                Ok(_success_code) => (),
            }
        }
        self.vulkan_fence_s_in_flight_image[vulkan_next_image_index] = vulkan_slide_in_flight_fence;
        //
        ApplicationEvolution::update_state_transform_d3_main(self)?;
        let wait_vulkan_semaphore_s = &[self.vulkan_semaphore_s_image_available[self.vulkan_frame_index_current]];
        let wait_vulkan_pipeline_stage_flag_s = &[VulkanPipelineStageFlagS::COLOR_ATTACHMENT_OUTPUT];
        let vulkan_command_buffer_s = &[self.vulkan_command_buffer_s[vulkan_next_image_index]];
        let vulkan_signal_semaphore_s = &[self.vulkan_semaphore_s_render_finished[self.vulkan_frame_index_current]];
        let vulkan_submit_information =
            VulkanSubmitInformation::builder()
            .wait_semaphores(wait_vulkan_semaphore_s)
            .wait_dst_stage_mask(wait_vulkan_pipeline_stage_flag_s)
            .command_buffers(vulkan_command_buffer_s)
            .signal_semaphores(vulkan_signal_semaphore_s);
        let reset_vulkan_fence_s_result =
            self.vulkan_device_logical.reset_fences(&[vulkan_slide_in_flight_fence]);
        match reset_vulkan_fence_s_result {
            Err(error) => {
                let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                return Err(TerminationProcessMain::InitializationVulkanFenceResetFail(vulkan_error_code));
            },
            Ok(()) => (),
        };
        let submit_vulkan_queue_result =
            self.vulkan_device_logical.queue_submit(self.vulkan_queue_graphic, &[vulkan_submit_information], vulkan_slide_in_flight_fence);
        match submit_vulkan_queue_result {
            Err(error) => {
                let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                return Err(TerminationProcessMain::InitializationVulkanQueueSubmitFail(vulkan_error_code));
            },
            Ok(()) => (),
        };
        let vulkan_swapchain_s = &[self.vulkan_swapchain];
        let vulkan_image_index_s = &[vulkan_next_image_index as u32];
        let vulkan_present_information =
            VulkanPresentInformationKhr::builder()
            .wait_semaphores(vulkan_signal_semaphore_s)
            .swapchains(vulkan_swapchain_s)
            .image_indices(vulkan_image_index_s);
        let present_vulkan_queue_result =
            self.vulkan_device_logical.queue_present_khr(self.vulkan_queue_present, &vulkan_present_information);
        let present_result_need_recreate_swapchain =
            match present_vulkan_queue_result {
                Err(VulkanErrorCode_::OUT_OF_DATE_KHR) => true,
                Ok(VulkanSuccessCode_::SUBOPTIMAL_KHR) => true,
                Err(error) => {
                    let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                    return Err(TerminationProcessMain::InitializationVulkanQueuePresentFail(vulkan_error_code));
                },
                Ok(_) => false,
            };
        match (self.signal_window_resized, present_result_need_recreate_swapchain) {
            (true, true) => {
                self.signal_window_resized = false;
                let _result = self.recreate_swapchain(window);
            },
            _ => (),
        }
        self.vulkan_frame_index_current = (self.vulkan_frame_index_current + 1) % (VULKAN_FRAME_IN_FLIGHT_MAX as usize);
        Ok(())
    }

    unsafe fn recreate_swapchain(&mut self, window: &WindowUniformWindow) -> Result<(), TerminationProcessMain> {
        match self.vulkan_device_logical.device_wait_idle() {
            Err(error) => {
                let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                return Err(TerminationProcessMain::InitializationVulkanDeviceWaitIdleFail(vulkan_error_code));
            },
            Ok(()) => (),
        };
        self.destroy_swapchain();
        let (vulkan_format, vulkan_extent, vulkan_swapchain, vulkan_image_s) =
            ApplicationVulkanInstanceSwapchain::create(
                window,
                &self.vulkan_instance,
                self.vulkan_surface,
                &self.vulkan_device_logical,
                self.vulkan_device_physical,
                self.vulkan_queue_family_index_graphic,
                self.vulkan_queue_family_index_present)?;
        let vulkan_image_view_s =
            ApplicationInstanceSwapchainImageView::create_all(
                &self.vulkan_device_logical,
                vulkan_format,
                &vulkan_image_s)?;
        let vulkan_render_pass =
            ApplicationVulkanRenderPass::create(
                &self.vulkan_instance, self.vulkan_device_physical, &self.vulkan_device_logical, vulkan_format)?;
        let (vulkan_pipeline, vulkan_pipeline_layout) =
            ApplicationVulkanPipeline::create_layout(
                &self.vulkan_device_logical, vulkan_extent, vulkan_render_pass, self.vulkan_descriptor_set_layout)?;
        let (vulkan_depth_image, vulkan_depth_image_memory, vulkan_depth_image_view) =
            ApplicationVulkanDepth::create_image_memory_view(
                &self.vulkan_instance, self.vulkan_device_physical, &self.vulkan_device_logical, self.vulkan_swapchain_extent)?;
        let vulkan_frame_buffer_s =
            ApplicationVulkanFrameBuffer::create_all(
                &self.vulkan_device_logical, &vulkan_image_view_s,
                self.vulkan_depth_image_view, vulkan_render_pass, vulkan_extent)?;
        let (vulkan_main_3d_transform_buffer_s, vulkan_main_3d_transform_buffer_memory_s) =
            ApplicationVulkanTransformD3Buffer::create_main_all(
                &self.vulkan_instance, self.vulkan_device_physical, &self.vulkan_device_logical, &vulkan_image_s)?;
        let vulkan_descriptor_pool =
            ApplicationVulkanDescriptorPool::create(&self.vulkan_device_logical, &vulkan_image_s)?;
        let vulkan_descriptor_set_s =
            ApplicationVulkanDescriptorSet::create_all(
                &self.vulkan_device_logical, &vulkan_image_s,
                self.vulkan_descriptor_set_layout, &vulkan_main_3d_transform_buffer_s, vulkan_descriptor_pool,
                self.vulkan_texture_image_view, self.vulkan_texture_sampler)?;
        let vulkan_command_buffer_s =
            ApplicationVulkanCommandBuffer::create_all(
                &self.vulkan_device_logical, self.vulkan_pipeline_layout, self.vulkan_command_pool,
                &vulkan_frame_buffer_s, vulkan_extent, vulkan_render_pass, vulkan_pipeline,
                self.vulkan_vertex_buffer, self.vulkan_vertex_index_buffer,
                &self.d3_model_mesh, &self.vulkan_descriptor_set_s)?;
        self.vulkan_swapchain_format = vulkan_format;
        self.vulkan_swapchain_extent = vulkan_extent;
        self.vulkan_swapchain = vulkan_swapchain;
        self.vulkan_swapchain_image_s = vulkan_image_s;
        self.vulkan_swapchain_image_view_s = vulkan_image_view_s;
        self.vulkan_render_pass = vulkan_render_pass;
        self.vulkan_pipeline = vulkan_pipeline;
        self.vulkan_pipeline_layout = vulkan_pipeline_layout;
        self.vulkan_frame_buffer_s = vulkan_frame_buffer_s;
        self.vulkan_transform_d3_main_buffer_s = vulkan_main_3d_transform_buffer_s;
        self.vulkan_transform_d3_main_buffer_memory_s = vulkan_main_3d_transform_buffer_memory_s;
        self.vulkan_descriptor_pool = vulkan_descriptor_pool;
        self.vulkan_descriptor_set_s = vulkan_descriptor_set_s;
        self.vulkan_command_buffer_s = vulkan_command_buffer_s;
        self.vulkan_depth_image = vulkan_depth_image;
        self.vulkan_depth_image_memory = vulkan_depth_image_memory;
        self.vulkan_depth_image_view = vulkan_depth_image_view;
        self.vulkan_fence_s_in_flight_image.resize(self.vulkan_swapchain_image_s.len(), VulkanFence::null());
        Ok(())
    }

    pub unsafe fn destroy(&mut self) -> Result<(), TerminationProcessMain> {
        match self.vulkan_device_logical.device_wait_idle() {
            Err(error) => {
                let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                return Err(TerminationProcessMain::InitializationVulkanDeviceWaitIdleFail(vulkan_error_code));
            },
            Ok(()) => (),
        };
        self.destroy_swapchain();
        self.vulkan_fence_s_in_flight_slide
        .iter()
        .for_each(|f| self.vulkan_device_logical.destroy_fence(*f, None));
        self.vulkan_semaphore_s_render_finished
        .iter()
        .for_each(|s| self.vulkan_device_logical.destroy_semaphore(*s, None));
        self.vulkan_semaphore_s_image_available
        .iter()
        .for_each(|s| self.vulkan_device_logical.destroy_semaphore(*s, None));
        //
        self.vulkan_device_logical.free_memory(self.vulkan_vertex_index_buffer_memory, None);
        self.vulkan_device_logical.destroy_buffer(self.vulkan_vertex_index_buffer, None);
        self.vulkan_device_logical.free_memory(self.vulkan_vertex_buffer_memory, None);
        self.vulkan_device_logical.destroy_buffer(self.vulkan_vertex_buffer, None);
        //
        self.vulkan_device_logical.destroy_sampler(self.vulkan_texture_sampler, None);
        self.vulkan_device_logical.destroy_image_view(self.vulkan_texture_image_view, None);
        self.vulkan_device_logical.free_memory(self.vulkan_texture_image_memory, None);
        self.vulkan_device_logical.destroy_image(self.vulkan_texture_image, None);
        self.vulkan_device_logical.destroy_command_pool(self.vulkan_command_pool, None);
        self.vulkan_device_logical.destroy_descriptor_set_layout(self.vulkan_descriptor_set_layout, None);
        self.vulkan_device_logical.destroy_device(None);
        self.vulkan_instance.destroy_surface_khr(self.vulkan_surface, None);
        if Option::is_some(&self.vulkan_debug_messenger) {
            self.vulkan_instance.destroy_debug_utils_messenger_ext(self.vulkan_debug_messenger.unwrap(), None);
        };
        self.vulkan_instance.destroy_instance(None);
        Ok(())
    }

    unsafe fn destroy_swapchain(&mut self) -> () {
        self.vulkan_device_logical.free_command_buffers(self.vulkan_command_pool, &self.vulkan_command_buffer_s);
        self.vulkan_device_logical.destroy_descriptor_pool(self.vulkan_descriptor_pool, None);
        self.vulkan_transform_d3_main_buffer_memory_s
        .iter()
        .for_each(|m| self.vulkan_device_logical.free_memory(*m, None));
        self.vulkan_transform_d3_main_buffer_s
        .iter()
        .for_each(|b| self.vulkan_device_logical.destroy_buffer(*b, None));
        self.vulkan_device_logical.destroy_image_view(self.vulkan_depth_image_view, None);
        self.vulkan_device_logical.free_memory(self.vulkan_depth_image_memory, None);
        self.vulkan_device_logical.destroy_image(self.vulkan_depth_image, None);
        self.vulkan_frame_buffer_s
        .iter()
        .for_each(|f| self.vulkan_device_logical.destroy_framebuffer(*f, None));
        self.vulkan_device_logical.destroy_pipeline(self.vulkan_pipeline, None);
        self.vulkan_device_logical.destroy_pipeline_layout(self.vulkan_pipeline_layout, None);
        self.vulkan_device_logical.destroy_render_pass(self.vulkan_render_pass, None);
        self.vulkan_swapchain_image_view_s
        .iter()
        .for_each(|v| self.vulkan_device_logical.destroy_image_view(*v, None));
        self.vulkan_device_logical.destroy_swapchain_khr(self.vulkan_swapchain, None);
    }
}