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
use ::vulkan::VulkanBool32;
use ::vulkan::VULKAN_EXTENSION_DEBUG_UTILITY;
use ::vulkan::prelude::version1_2::*;

use crate::termination::TerminationProcessMain;
use crate::application::main::Application;
use crate::application::vulkan_instance_share::ApplicationVulkanInstanceShare;
use crate::application::vulkan_instance_device_physical::ApplicationVulkanInstanceDevicePhysical;


pub struct ApplicationVulkanInstanceValidationWi {}

impl ApplicationVulkanInstanceValidationWi {
    pub unsafe fn create(
        window: &WindowUniformWindow, vulkan_validation_layer: &VulkanExtensionName)
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
        let vulkan_physical_device =
            match ApplicationVulkanInstancePhysicalDevice::pick(&vulkan_instance) {
                Err(error) => return Err(error),
                Ok(physical_device) => physical_device,
            };
        Ok(Application {
            vulkan_entry: vulkan_entry,
            vulkan_instance: vulkan_instance,
            vulkan_debug_messenger: None,
            vulkan_device_physical: vulkan_physical_device,
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