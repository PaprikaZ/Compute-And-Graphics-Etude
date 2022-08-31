use ::vulkan::VulkanInstance;
use ::vulkan::VulkanInstanceVersion1_0;
use ::vulkan::VulkanDevicePhysical;
use ::vulkan::VulkanErrorCode;
use ::vulkan::VulkanQueueFamilyIndexGraphic;
use ::vulkan::VulkanQueueFlagS;
use ::console_log::{console_log_info, console_log_warn};

use crate::termination::TerminationProcessMain;


pub struct ApplicationVulkanInstanceDevicePhysical {}

impl ApplicationVulkanInstanceDevicePhysical {
    pub unsafe fn pick(vulkan_instance: &VulkanInstance)
     -> Result<(VulkanDevicePhysical, VulkanQueueFamilyIndexGraphic), TerminationProcessMain>
    {
        let vulkan_physical_device_s =
            match vulkan_instance.enumerate_physical_devices() {
                Err(error) => {
                    let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                    return Err(TerminationProcessMain::InitializationVulkanEnumeratePhysicalDeviceFail(vulkan_error_code));
                },
                Ok(device_s) => device_s,
            };
        for vulkan_physical_device in vulkan_physical_device_s {
            let physical_device_property_s = vulkan_instance.get_physical_device_properties(vulkan_physical_device);
            match Self::check(vulkan_instance, vulkan_physical_device) {
                Err(()) => {
                    console_log_warn!(
                        "Physical device (`{}`) skipping: missing graphic queue family",
                        physical_device_property_s.device_name);
                },
                Ok(graphic_queue_index) => {
                    console_log_info!("Physical device (`{}`) selected", physical_device_property_s.device_name);
                    return Ok((vulkan_physical_device, graphic_queue_index));
                }
            };
        }
        Err(TerminationProcessMain::InitializationVulkanDevicePhysicalAllQueueFamilyGraphicNotSupport)
    }

    unsafe fn check(
        vulkan_instance: &VulkanInstance,
        vulkan_physical_device: VulkanDevicePhysical)
     -> Result<VulkanQueueFamilyIndexGraphic, ()>
    {
        let vulkan_physical_device_queue_family_property_s =
            vulkan_instance.get_physical_device_queue_family_properties(vulkan_physical_device);
        let optional_graphic_queue_family_index =
            vulkan_physical_device_queue_family_property_s
            .iter()
            .position(|p| p.queue_flags.contains(VulkanQueueFlagS::GRAPHICS))
            .map(|i| VulkanQueueFamilyIndexGraphic::new(i as u32));
        match optional_graphic_queue_family_index {
            None => Err(()),
            Some(graphic_queue_index) => Ok(graphic_queue_index),
        }
    }
}