use ::vulkan::prelude::version1_2::*;
use ::vulkan::extend::VulkanErrorCode;
use ::vulkan::extend::VulkanQueueFamilyIndexGraphic;
use ::vulkan::VulkanCommandPool;
use ::vulkan::VulkanCommandPoolCreateInformation;
use ::vulkan::VulkanCommandPoolCreateFlagS;
use ::vulkan::VulkanImage;

use crate::termination::TerminationProcessMain;


pub struct ApplicationVulkanCommandPool {}

impl ApplicationVulkanCommandPool {
    pub unsafe fn create_main(
        vulkan_logical_device: &VulkanDeviceLogical,
        vulkan_graphic_queue_family_index: VulkanQueueFamilyIndexGraphic)
     -> Result<VulkanCommandPool, TerminationProcessMain>
    {
        Self::create(vulkan_logical_device, vulkan_graphic_queue_family_index)
    }

    pub unsafe fn create_swapchain_image_all(
        vulkan_logical_device: &VulkanDeviceLogical,
        vulkan_swapchain_image_s: &Vec<VulkanImage>,
        vulkan_graphic_queue_family_index: VulkanQueueFamilyIndexGraphic)
     -> Result<Vec<VulkanCommandPool>, TerminationProcessMain>
    {
        let mut vulkan_command_buffer_s = Vec::new();
        vulkan_command_buffer_s.reserve_exact(vulkan_swapchain_image_s.len());
        for _ in 0..vulkan_swapchain_image_s.len() {
            let vulkan_command_buffer =
                Self::create_swapchain_image(vulkan_logical_device, vulkan_graphic_queue_family_index)?;
            vulkan_command_buffer_s.push(vulkan_command_buffer);
        }
        Ok(vulkan_command_buffer_s)
    }

    unsafe fn create_swapchain_image(
        vulkan_logical_device: &VulkanDeviceLogical,
        vulkan_graphic_queue_family_index: VulkanQueueFamilyIndexGraphic)
     -> Result<VulkanCommandPool, TerminationProcessMain>
    {
        Self::create(vulkan_logical_device, vulkan_graphic_queue_family_index)
    }

    unsafe fn create(
        vulkan_logical_device: &VulkanDeviceLogical,
        vulkan_graphic_queue_family_index: VulkanQueueFamilyIndexGraphic)
     -> Result<VulkanCommandPool, TerminationProcessMain>
    {
        let vulkan_command_pool_create_information =
            VulkanCommandPoolCreateInformation::builder()
            .flags(VulkanCommandPoolCreateFlagS::TRANSIENT)
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
