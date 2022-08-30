use ::window_uniform::prelude::*;
use ::vulkan::VulkanExtensionName;
use ::vulkan::VulkanExtensionDebugUtilityMessenger;
use ::vulkan::prelude::version1_2::*;

use crate::termination::TerminationProcessMain;


pub struct Application {
    pub vulkan_entry: VulkanEntry,
    pub vulkan_instance: VulkanInstance,
    pub vulkan_debug_messenger: Option<VulkanExtensionDebugUtilityMessenger>,
}

impl Application {
    pub unsafe fn create(
        window: &WindowUniformWindow, optional_validation_layer: Option<&VulkanExtensionName>)
     -> Result<Self, TerminationProcessMain>
    {
        match optional_validation_layer {
            None => Self::create_validation_wo(window),
            Some(validation_layer) => Self::create_validation_wi(window, validation_layer),
        }
    }

    pub unsafe fn render(&mut self, _window: &WindowUniformWindow) -> Result<(), ()> {
        Ok(())
    }

    pub unsafe fn destroy(&mut self) -> () {
        self.vulkan_instance.destroy_instance(None);
    }
}