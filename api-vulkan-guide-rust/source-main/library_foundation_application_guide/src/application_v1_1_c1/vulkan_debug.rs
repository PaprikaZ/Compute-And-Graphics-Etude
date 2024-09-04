use std::os::raw::c_void;
use std::ffi::CStr;

use ::library_foundation_reintroduction::vulkan::VULKAN_FALSE;
use ::library_foundation_reintroduction::vulkan::VulkanBool32;
use ::library_foundation_reintroduction::vulkan::VulkanExtensionDebugUtilityMessageSeverityFlagS;
use ::library_foundation_reintroduction::vulkan::VulkanExtensionDebugUtilityMessageTypeFlagS;
use ::library_foundation_reintroduction::vulkan::VulkanExtensionDebugUtilityMessengerCallbackData;


pub struct ApplicationVulkanDebug {}

impl ApplicationVulkanDebug {
    pub extern "system" fn callback(
        message_severity_flag_s: VulkanExtensionDebugUtilityMessageSeverityFlagS,
        message_type_flag_s: VulkanExtensionDebugUtilityMessageTypeFlagS,
        data: *const VulkanExtensionDebugUtilityMessengerCallbackData,
        _: *mut c_void)
    -> VulkanBool32
    {
        type SFS = VulkanExtensionDebugUtilityMessageSeverityFlagS;
        let data = unsafe { *data };
        let message = unsafe { CStr::from_ptr(data.message) }.to_string_lossy();
        //
        if SFS::ERROR <= message_severity_flag_s {
            println!("({:?}) {}", message_type_flag_s, message);
        } else if SFS::WARNING <= message_severity_flag_s {
            println!("({:?}) {}", message_type_flag_s, message);
        } else if SFS::INFO <= message_severity_flag_s {
            println!("({:?}) {}", message_type_flag_s, message);
        } else {
            println!("({:?}) {}", message_type_flag_s, message);
        }
        VULKAN_FALSE
    }
}