use ::vulkan::prelude::version1_2::*;
use ::vulkan::VulkanErrorCode;
use ::vulkan::VulkanQueueFamilyIndexGraphic;
use ::vulkan::VulkanCommandPool;
use ::vulkan::VulkanCommandPoolCreateInformation;

use crate::termination::TerminationProcessMain;


pub struct ApplicationVulkanCommandPool {}

impl ApplicationVulkanCommandPool {
    pub unsafe fn create(
        vulkan_logical_device: &VulkanDeviceLogical,
        vulkan_graphic_queue_family_index: VulkanQueueFamilyIndexGraphic)
     -> Result<VulkanCommandPool, TerminationProcessMain>
    {
        let vulkan_command_pool_create_information =
            VulkanCommandPoolCreateInformation::builder()
            .queue_family_index(vulkan_graphic_queue_family_index.as_raw());
        let create_vulkan_command_pool_result =
            vulkan_logical_device.create_command_pool(&vulkan_command_pool_create_information, None);
        match create_vulkan_command_pool_result {
            Err(error) => {
                let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                Err(TerminationProcessMain::InitializationVulkanCommandPoolCreateFail(vulkan_error_code))
            },
            Ok(command_pool) => Ok(command_pool),
        }
    }
}
