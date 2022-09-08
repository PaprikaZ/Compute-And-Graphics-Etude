use std::collections::HashSet;
use std::ffi::CStr;
use std::os::raw::c_void;

use ::console_log::prelude::*;
use ::window_uniform::prelude::*;
use ::vulkan::VULKAN_LIBRARY_FILE_NAME;
use ::vulkan::VulkanErrorCode;
use ::vulkan::VulkanLibraryLoader;
use ::vulkan::VulkanInstanceCreateInformation;
use ::vulkan::VulkanExtensionName;
use ::vulkan::VulkanExtensionDebugUtilityMessengerCreateInformation;
use ::vulkan::VulkanExtensionDebugUtilityMessageSeverityFlagS;
use ::vulkan::VulkanExtensionDebugUtilityMessageTypeFlagS;
use ::vulkan::VulkanExtensionDebugUtilityMessengerCallbackData;
use ::vulkan::VulkanWindow;
use ::vulkan::VulkanBool32;
use ::vulkan::VULKAN_EXTENSION_DEBUG_UTILITY;
use ::vulkan::prelude::version1_2::*;

use crate::termination::TerminationProcessMain;
use crate::data::vertex::DataVertex;
use crate::application::main::Application;
use crate::application::vulkan_instance_share::ApplicationVulkanInstanceShare;
use crate::application::vulkan_instance_device_physical::ApplicationVulkanInstanceDevicePhysical;
use crate::application::vulkan_instance_device_logical::ApplicationVulkanInstanceDeviceLogical;
use crate::application::vulkan_instance_swapchain::ApplicationVulkanInstanceSwapchain;
use crate::application::vulkan_instance_swapchain_image_view::ApplicationInstanceSwapchainImageView;
use crate::application::vulkan_pipeline::ApplicationVulkanPipeline;
use crate::application::vulkan_render_pass::ApplicationVulkanRenderPass;
use crate::application::vulkan_frame_buffer::ApplicationVulkanFrameBuffer;
use crate::application::vulkan_command_pool::ApplicationVulkanCommandPool;
use crate::application::vulkan_command_buffer::ApplicationVulkanCommandBuffer;
use crate::application::vulkan_synchronization::ApplicationVulkanSynchronization;
use crate::application::vulkan_vertex::ApplicationVulkanVertexBuffer;


pub struct ApplicationVulkanInstanceValidationWi {}

impl ApplicationVulkanInstanceValidationWi {
    pub unsafe fn create(
        window: &WindowUniformWindow,
        vulkan_validation_layer: &VulkanExtensionName,
        vulkan_extension_s: &[VulkanExtensionName])
     -> Result<Application, TerminationProcessMain>
    {
        let vulkan_library_loader =
            match VulkanLibraryLoader::new(VULKAN_LIBRARY_FILE_NAME) {
                Err(error) => return Err(TerminationProcessMain::InitializationVulkanLibraryLoadingFail(error)),
                Ok(loader) => loader,
            };
        let vulkan_entry =
            match VulkanEntry::new(vulkan_library_loader) {
                Err(error) => return Err(TerminationProcessMain::InitializationVulkanEntryCreateFail(error)),
                Ok(entry) => entry,
            };
        let create_vulkan_instance_result =
            Self::create_vulkan_instance(window, &vulkan_entry, vulkan_validation_layer);
        let vulkan_instance =
            match create_vulkan_instance_result {
                Err(error) => return Err(error),
                Ok(instance) => instance,
            };
        let create_vulkan_surface_result = VulkanWindow::create_surface(&vulkan_instance, window);
        let vulkan_surface =
            match create_vulkan_surface_result {
                Err(error) => {
                    let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                    return Err(TerminationProcessMain::InitializationVulkanSurfaceCreateFail(vulkan_error_code));
                },
                Ok(surface) => surface,
            };
        let (vulkan_physical_device,
             vulkan_graphic_queue_family_index,
             vulkan_surface_queue_family_index) =
            match ApplicationVulkanInstanceDevicePhysical::pick(&vulkan_instance, vulkan_surface, vulkan_extension_s) {
                Err(error) => return Err(error),
                Ok(device_and_queue_index) => device_and_queue_index,
            };
        let create_vulkan_logical_device_result =
            ApplicationVulkanInstanceDeviceLogical::create(
                &vulkan_instance,
                vulkan_physical_device,
                vulkan_extension_s,
                vulkan_graphic_queue_family_index,
                vulkan_surface_queue_family_index);
        let (vulkan_logical_device, vulkan_graphic_queue, vulkan_present_queue) =
            match create_vulkan_logical_device_result {
                Err(error) => return Err(error),
                Ok(device_and_queue) => device_and_queue,
            };
        let (vulkan_surface_format, vulkan_extent, vulkan_swapchain, vulkan_image_s) =
            ApplicationVulkanInstanceSwapchain::create(
                window, &vulkan_instance, vulkan_surface, &vulkan_logical_device,
                vulkan_physical_device, vulkan_graphic_queue_family_index, vulkan_surface_queue_family_index
            )?;
        let vulkan_image_view_s =
            ApplicationInstanceSwapchainImageView::create_all(
                &vulkan_logical_device, vulkan_surface_format, &vulkan_image_s)?;
        let vulkan_render_pass =
            ApplicationVulkanRenderPass::create(&vulkan_logical_device, vulkan_surface_format)?;
        let (vulkan_pipeline, vulkan_pipeline_layout) =
            ApplicationVulkanPipeline::create_layout(&vulkan_logical_device, vulkan_extent, vulkan_render_pass)?;
        let vulkan_frame_buffer_s =
            ApplicationVulkanFrameBuffer::create_all(&vulkan_logical_device, &vulkan_image_view_s, vulkan_render_pass, vulkan_extent)?;
        let vulkan_command_pool =
            ApplicationVulkanCommandPool::create(&vulkan_logical_device, vulkan_graphic_queue_family_index)?;
        let input_vertex_s = DataVertex::get_default();
        let (vulkan_vertex_buffer, vulkan_vertex_buffer_memory) =
            ApplicationVulkanVertexBuffer::create(
                &vulkan_instance, vulkan_physical_device, &vulkan_logical_device, &input_vertex_s)?;
        let vulkan_command_buffer_s =
            ApplicationVulkanCommandBuffer::create_all(
                &vulkan_logical_device, vulkan_command_pool,
                &vulkan_frame_buffer_s, vulkan_extent, vulkan_render_pass, vulkan_pipeline)?;
        let (vulkan_image_available_semaphore_s, vulkan_render_finished_semaphore_s,
             vulkan_slide_in_flight_fence_s, vulkan_image_in_flight_fence_s) =
            ApplicationVulkanSynchronization::create_all(&vulkan_logical_device, &vulkan_image_s)?;
        Ok(Application {
            signal_window_resized: false,
            vulkan_entry: vulkan_entry,
            vulkan_instance: vulkan_instance,
            vulkan_debug_messenger: None,
            vulkan_device_physical: vulkan_physical_device,
            vulkan_device_logical: vulkan_logical_device,
            vulkan_queue_family_index_graphic: vulkan_graphic_queue_family_index,
            vulkan_queue_graphic: vulkan_graphic_queue,
            vulkan_surface: vulkan_surface,
            vulkan_queue_family_index_present: vulkan_surface_queue_family_index,
            vulkan_queue_present: vulkan_present_queue,
            vulkan_swapchain_format: vulkan_surface_format,
            vulkan_swapchain_extent: vulkan_extent,
            vulkan_swapchain: vulkan_swapchain,
            vulkan_swapchain_image_s: vulkan_image_s,
            vulkan_swapchain_image_view_s: vulkan_image_view_s,
            vulkan_render_pass: vulkan_render_pass,
            vulkan_pipeline_layout: vulkan_pipeline_layout,
            vulkan_pipeline: vulkan_pipeline,
            vulkan_frame_buffer_s: vulkan_frame_buffer_s,
            vulkan_command_pool: vulkan_command_pool,
            vulkan_command_buffer_s: vulkan_command_buffer_s,
            vulkan_semaphore_s_image_available: vulkan_image_available_semaphore_s,
            vulkan_semaphore_s_render_finished: vulkan_render_finished_semaphore_s,
            vulkan_fence_s_in_flight_slide: vulkan_slide_in_flight_fence_s,
            vulkan_fence_s_in_flight_image: vulkan_image_in_flight_fence_s,
            vulkan_frame_index_current: 0,
            vulkan_vertex_buffer: vulkan_vertex_buffer,
            vulkan_vertex_buffer_memory: vulkan_vertex_buffer_memory,
            input_vertex_s: input_vertex_s,
        })
    }

    unsafe fn create_vulkan_instance(
        window: &WindowUniformWindow, vulkan_entry: &VulkanEntry, vulkan_validation_layer: &VulkanExtensionName)
     -> Result<VulkanInstance, TerminationProcessMain>
    {
        let vulkan_application_information =
            ApplicationVulkanInstanceShare::create_vulkan_instance_application_information();
        let available_vulkan_layer_s =
            match vulkan_entry.enumerate_instance_layer_properties() {
                Err(error) => {
                    let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                    return Err(TerminationProcessMain::InitializationVulkanInstanceCreateFail(vulkan_error_code));
                },
                Ok(layer_property_s) => layer_property_s,
            }
            .iter()
            .map(|l| l.layer_name)
            .collect::<HashSet<_>>();
        let vulkan_validation_layer_s =
            match available_vulkan_layer_s.contains(&vulkan_validation_layer) {
                false => return Err(TerminationProcessMain::InitializationVulkanValidationLayerNotSupport),
                true => vec![vulkan_validation_layer.as_ptr()],
            };
        let vulkan_application_extension_s = {
            let mut extension_s = ApplicationVulkanInstanceShare::create_vulkan_instance_application_extension_s(window);
            extension_s.push(VULKAN_EXTENSION_DEBUG_UTILITY.name.as_ptr());
            extension_s
        };
        let vulkan_instance_create_information =
            VulkanInstanceCreateInformation::builder()
            .application_info(&vulkan_application_information)
            .enabled_layer_names(&vulkan_validation_layer_s)
            .enabled_extension_names(&vulkan_application_extension_s);
        let mut vulkan_debug_messenger_create_information =
            VulkanExtensionDebugUtilityMessengerCreateInformation::builder()
            .message_severity(VulkanExtensionDebugUtilityMessageSeverityFlagS::all())
            .message_type(VulkanExtensionDebugUtilityMessageTypeFlagS::all())
            .user_callback(Some(vulkan_debug_callback));
        let vulkan_instance_create_information =
            vulkan_instance_create_information.push_next(&mut vulkan_debug_messenger_create_information);
        let vulkan_instance =
            match vulkan_entry.create_instance(&vulkan_instance_create_information, None) {
                Err(error) => {
                    let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                    return Err(TerminationProcessMain::InitializationVulkanInstanceCreateFail(vulkan_error_code));
                } ,
                Ok(instance) => instance,
            };
        Ok(vulkan_instance)
    }
}


extern "system" fn vulkan_debug_callback(
    message_severity_flag_s: VulkanExtensionDebugUtilityMessageSeverityFlagS,
    message_type_flag_s: VulkanExtensionDebugUtilityMessageTypeFlagS,
    data: *const VulkanExtensionDebugUtilityMessengerCallbackData,
    _: *mut c_void)
 -> VulkanBool32 {
    let data = unsafe { *data };
    let message = unsafe { CStr::from_ptr(data.message) }.to_string_lossy();

    if VulkanExtensionDebugUtilityMessageSeverityFlagS::ERROR <= message_severity_flag_s {
        console_log_error!("({:?}) {}", message_type_flag_s, message);
    } else if VulkanExtensionDebugUtilityMessageSeverityFlagS::WARNING <= message_severity_flag_s {
        console_log_warn!("({:?}) {}", message_type_flag_s, message);
    } else if VulkanExtensionDebugUtilityMessageSeverityFlagS::INFO <= message_severity_flag_s {
        console_log_debug!("({:?}) {}", message_type_flag_s, message);
    } else {
        console_log_trace!("({:?}) {}", message_type_flag_s, message);
    }

    vulkan::VULKAN_FALSE
}