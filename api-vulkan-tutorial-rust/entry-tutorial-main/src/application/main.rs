use ::window_uniform::prelude::*;
use ::vulkan::VULKAN_LIBRARY_FILE_NAME;
use ::vulkan::VulkanErrorCode;
use ::vulkan::VulkanLibraryLoader;
use ::vulkan::VulkanApplicationInfomation;
use ::vulkan::VulkanInstanceCreateInformation;
use ::vulkan::prelude::version1_2::*;

use crate::termination::TerminationProcessMain;


pub struct Application {
    vulkan_entry: VulkanEntry,
    vulkan_instance: VulkanInstance,
}

impl Application {
    pub unsafe fn create(window: &WindowUniformWindow) -> Result<Self, TerminationProcessMain> {
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
        let vulkan_instance =
            match Application::create_vulkan_instance(window, &vulkan_entry) {
                Err(error) => return Err(TerminationProcessMain::InitializationVulkanInstanceCreateFail(error)),
                Ok(instance) => instance,
            };
        Ok(Self {
            vulkan_entry: vulkan_entry,
            vulkan_instance: vulkan_instance,
        })
    }

    pub unsafe fn render(&mut self, _window: &WindowUniformWindow) -> Result<(), ()> {
        Ok(())
    }

    pub unsafe fn destroy(&mut self) -> () {
        self.vulkan_instance.destroy_instance(None);
    }

    unsafe fn create_vulkan_instance(
        window: &WindowUniformWindow, vulkan_entry: &VulkanEntry) -> Result<VulkanInstance, VulkanErrorCode> {
        let vulkan_application_information =
            VulkanApplicationInfomation::builder()
            .application_name(b"Vulkan Tutorial Rust\0")
            .application_version(::vulkan::vk::make_version(1, 0, 0))
            .engine_name(b"No Engine\0")
            .engine_version(::vulkan::vk::make_version(1, 0, 0))
            .api_version(::vulkan::vk::make_version(1, 0, 0));
        let vulkan_application_extension_s =
            ::vulkan::window::get_required_instance_extensions(window)
            .iter()
            .map(|e| e.as_ptr())
            .collect::<Vec<_>>();
        let vulkan_instance_create_information =
            VulkanInstanceCreateInformation::builder()
            .application_info(&vulkan_application_information)
            .enabled_extension_names(&vulkan_application_extension_s);
        match vulkan_entry.create_instance(&vulkan_instance_create_information, None) {
            Err(error) => Err(VulkanErrorCode::new(error.as_raw())),
            Ok(instance) => Ok(instance),
        }
    }
}