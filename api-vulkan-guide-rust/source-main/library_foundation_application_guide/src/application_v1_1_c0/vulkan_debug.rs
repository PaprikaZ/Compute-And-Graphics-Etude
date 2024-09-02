use std::os::raw::c_void;
use std::ffi::CStr;

//use ::library_foundation_reintroduction::vulkan::VULKAN_FALSE;
use ::library_foundation_reintroduction::vulkan::VulkanBool32;
use ::library_foundation_reintroduction::vulkan::VulkanExtensionDebugUtilityMessageSeverityFlagS;
use ::library_foundation_reintroduction::vulkan::VulkanExtensionDebugUtilityMessageTypeFlagS;
use ::library_foundation_reintroduction::vulkan::VulkanExtensionDebugUtilityMessengerCallbackData;


pub struct ApplicationVulkanDebug {}

impl ApplicationVulkanDebug {
    pub extern "system" fn callback(
        message_severity_flag_s: VulkanExtensionDebugUtilityMessageSeverityFlagS,
        _message_type_flag_s: VulkanExtensionDebugUtilityMessageTypeFlagS,
        data: *const VulkanExtensionDebugUtilityMessengerCallbackData,
        _: *mut c_void)
    -> VulkanBool32
    {
        type SeverityFlagS = VulkanExtensionDebugUtilityMessageSeverityFlagS;
        let data = unsafe { *data };
        let _message = unsafe { CStr::from_ptr(data.message) }.to_string_lossy();

        if SeverityFlagS::ERROR <= message_severity_flag_s {
            todo!()
        } else if SeverityFlagS::WARNING <= message_severity_flag_s {
            todo!()
        } else if SeverityFlagS::INFO <= message_severity_flag_s {
            todo!()
        } else {
            todo!()
        }
        //VULKAN_FALSE
    }
}