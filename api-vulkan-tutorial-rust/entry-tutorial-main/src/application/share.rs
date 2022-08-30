use ::window_uniform::prelude::*;
use ::vulkan::VulkanBuilderHas;
use ::vulkan::VulkanApplicationInfomation;
use ::vulkan::VulkanApplicationInformationBuilder;

use crate::application::main::Application;


impl Application {
    pub fn create_vulkan_instance_application_information() -> VulkanApplicationInformationBuilder<'static> {
        VulkanApplicationInfomation::builder()
        .application_name(b"Vulkan Tutorial Rust\0")
        .application_version(::vulkan::vk::make_version(1, 0, 0))
        .engine_name(b"No Engine\0")
        .engine_version(::vulkan::vk::make_version(1, 0, 0))
        .api_version(::vulkan::vk::make_version(1, 0, 0))
    }

    pub fn create_vulkan_instance_application_extension_s(window: &WindowUniformWindow) -> Vec<*const i8> {
        ::vulkan::window::get_required_instance_extensions(window)
        .iter()
        .map(|e| e.as_ptr())
        .collect::<Vec<_>>()
    }
}