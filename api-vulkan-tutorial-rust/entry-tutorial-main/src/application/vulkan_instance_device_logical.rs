use ::vulkan::prelude::version1_2::*;
use ::vulkan::VulkanInstance;
use ::vulkan::VulkanDevicePhysical;
use ::vulkan::VulkanQueueFamilyIndexGraphic;
use ::vulkan::VulkanDeviceLogicalCreateInformation;
use ::vulkan::VulkanDeviceLogicalQueueCreateInformation;
use ::vulkan::VulkanDevicePhysicalFeatureS;
use ::vulkan::VulkanErrorCode;
use ::vulkan::VulkanQueue;

use crate::termination::TerminationProcessMain;

pub struct ApplicationVulkanInstanceDeviceLogical {}

impl ApplicationVulkanInstanceDeviceLogical {
    pub unsafe fn create(
        vulkan_instance: &VulkanInstance,
        vulkan_physical_device: VulkanDevicePhysical,
        vulkan_graphic_queue_family_index: VulkanQueueFamilyIndexGraphic)
     -> Result<(VulkanDeviceLogical, VulkanQueue), TerminationProcessMain>
    {
        let vulkan_graphic_queue_priority_s = &[1.0];
        let vulkan_graphic_queue_create_information =
            VulkanDeviceLogicalQueueCreateInformation::builder()
            .queue_family_index(vulkan_graphic_queue_family_index.as_raw())
            .queue_priorities(vulkan_graphic_queue_priority_s);
        let vulkan_device_layer_s: Vec<*const i8> = vec![];
        let vulkan_physical_device_feature_s = VulkanDevicePhysicalFeatureS::builder();
        let vulkan_graphic_queue_create_information_s = &[vulkan_graphic_queue_create_information];
        let vulkan_logical_device_create_information =
            VulkanDeviceLogicalCreateInformation::builder()
            .queue_create_infos(vulkan_graphic_queue_create_information_s)
            .enabled_layer_names(&vulkan_device_layer_s)
            .enabled_features(&vulkan_physical_device_feature_s);
        let create_vulkan_logical_device_result =
            vulkan_instance.create_device(vulkan_physical_device, &vulkan_logical_device_create_information, None);
        let vulkan_logical_device =
            match create_vulkan_logical_device_result {
                Err(error) => {
                    let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                    return Err(TerminationProcessMain::InitializationVulkanDeviceLogicalCreateFail(vulkan_error_code));
                },
                Ok(device) => device,
            };
        let vulkan_graphic_queue =
            vulkan_logical_device.get_device_queue(vulkan_graphic_queue_family_index.as_raw(), 0);
        Ok((vulkan_logical_device, vulkan_graphic_queue))
    }
}