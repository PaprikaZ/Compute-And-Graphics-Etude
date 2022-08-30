use ::window_uniform::prelude::*;
use ::vulkan::VULKAN_LIBRARY_FILE_NAME;
use ::vulkan::VulkanErrorCode;
use ::vulkan::VulkanLibraryLoader;
use ::vulkan::VulkanInstanceCreateInformation;
use ::vulkan::prelude::version1_2::*;

use crate::termination::TerminationProcessMain;
use crate::application::main::Application;


impl Application {
    pub unsafe fn create_validation_wo(window: &WindowUniformWindow) -> Result<Self, TerminationProcessMain> {
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
            match Application::create_vulkan_instance_validation_wo(window, &vulkan_entry) {
                Err(error) => return Err(error),
                Ok(instance) => instance,
            };
        Ok(Self {
            vulkan_entry: vulkan_entry,
            vulkan_instance: vulkan_instance,
            vulkan_debug_messenger: None,
        })
    }

    unsafe fn create_vulkan_instance_validation_wo(
        window: &WindowUniformWindow, vulkan_entry: &VulkanEntry) -> Result<VulkanInstance, TerminationProcessMain> {
        let vulkan_application_information = Self::create_vulkan_instance_application_information();
        let vulkan_application_extension_s = Self::create_vulkan_instance_application_extension_s(window);
        let vulkan_instance_create_information =
            VulkanInstanceCreateInformation::builder()
            .application_info(&vulkan_application_information)
            .enabled_extension_names(&vulkan_application_extension_s);
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