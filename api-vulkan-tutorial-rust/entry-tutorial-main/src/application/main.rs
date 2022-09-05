use vulkan::VulkanExtensionDebugUtility;
use vulkan::VulkanSurfaceExtensionKhr;
use vulkan::VulkanSwapchainExtensionKhr;
use ::window_uniform::prelude::*;
use ::vulkan::VulkanExtensionName;
use ::vulkan::VulkanExtensionDebugUtilityMessenger;
use ::vulkan::prelude::version1_2::*;
use ::vulkan::VulkanQueue;
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

use crate::termination::TerminationProcessMain;
use crate::application::vulkan_instance_validation_wi::ApplicationVulkanInstanceValidationWi;
use crate::application::vulkan_instance_validation_wo::ApplicationVulkanInstanceValidationWo;


pub struct Application {
    pub vulkan_entry: VulkanEntry,
    pub vulkan_instance: VulkanInstance,
    pub vulkan_debug_messenger: Option<VulkanExtensionDebugUtilityMessenger>,
    pub vulkan_device_physical: VulkanDevicePhysical,
    pub vulkan_device_logical: VulkanDeviceLogical,
    pub vulkan_queue_graphic: VulkanQueue,
    pub vulkan_surface: VulkanSurfaceKhr,
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
}

impl Application {
    pub unsafe fn create(
        window: &WindowUniformWindow,
        optional_validation_layer: Option<&VulkanExtensionName>,
        vulkan_physical_device_extension_s: &[VulkanExtensionName])
     -> Result<Self, TerminationProcessMain>
    {
        match optional_validation_layer {
            None =>
                ApplicationVulkanInstanceValidationWo::create(window, vulkan_physical_device_extension_s),
            Some(validation_layer) =>
                ApplicationVulkanInstanceValidationWi::create(window, validation_layer, vulkan_physical_device_extension_s),
        }
    }

    pub unsafe fn render(&mut self, _window: &WindowUniformWindow) -> Result<(), ()> {
        Ok(())
    }

    pub unsafe fn destroy(&mut self) -> () {
        self.vulkan_device_logical.destroy_command_pool(self.vulkan_command_pool, None);
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
        self.vulkan_device_logical.destroy_device(None);
        self.vulkan_instance.destroy_surface_khr(self.vulkan_surface, None);
        if Option::is_some(&self.vulkan_debug_messenger) {
            self.vulkan_instance.destroy_debug_utils_messenger_ext(self.vulkan_debug_messenger.unwrap(), None);
        };
        self.vulkan_instance.destroy_instance(None);
    }
}