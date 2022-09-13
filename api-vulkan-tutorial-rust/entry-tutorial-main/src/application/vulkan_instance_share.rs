use ::window_uniform::prelude::*;
use ::vulkan::VulkanWindow;
use ::vulkan::VulkanBuilderHas;
use ::vulkan::VulkanApplicationInformation;
use ::vulkan::VulkanApplicationInformationBuilder;


pub struct ApplicationVulkanInstanceShare {}

impl ApplicationVulkanInstanceShare {
    pub fn create_vulkan_instance_application_information() -> VulkanApplicationInformationBuilder<'static> {
        VulkanApplicationInformation::builder()
        .application_name(b"Vulkan Tutorial Rust\0")
        .application_version(::vulkan::vk::make_version(1, 0, 0))
        .engine_name(b"No Engine\0")
        .engine_version(::vulkan::vk::make_version(1, 0, 0))
        .api_version(::vulkan::vk::make_version(1, 0, 0))
    }

    pub fn create_vulkan_instance_application_extension_s(window: &WindowUniformWindow) -> Vec<*const i8> {
        VulkanWindow::get_required_instance_extensions(window)
        .iter()
        .map(|e| e.as_ptr())
        .collect::<Vec<_>>()
    }
}