use vulkan::VulkanExtensionDebugUtility;
use vulkan::VulkanSurfaceExtensionKhr;
use ::window_uniform::prelude::*;
use ::vulkan::VulkanExtensionName;
use ::vulkan::VulkanExtensionDebugUtilityMessenger;
use ::vulkan::prelude::version1_2::*;
use ::vulkan::VulkanQueue;
use ::vulkan::VulkanDevicePhysical;
use ::vulkan::VulkanSurfaceKhr;

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
}

impl Application {
    pub unsafe fn create(
        window: &WindowUniformWindow, optional_validation_layer: Option<&VulkanExtensionName>)
     -> Result<Self, TerminationProcessMain>
    {
        match optional_validation_layer {
            None => ApplicationVulkanInstanceValidationWo::create(window),
            Some(validation_layer) => ApplicationVulkanInstanceValidationWi::create(window, validation_layer),
        }
    }

    pub unsafe fn render(&mut self, _window: &WindowUniformWindow) -> Result<(), ()> {
        Ok(())
    }

    pub unsafe fn destroy(&mut self) -> () {
        self.vulkan_device_logical.destroy_device(None);
        self.vulkan_instance.destroy_surface_khr(self.vulkan_surface, None);
        if Option::is_some(&self.vulkan_debug_messenger) {
            self.vulkan_instance.destroy_debug_utils_messenger_ext(self.vulkan_debug_messenger.unwrap(), None);
        };
        self.vulkan_instance.destroy_instance(None);
    }
}