use ::vulkan::prelude::version1_2::*;
use ::vulkan::VulkanErrorCode;
use ::vulkan::VulkanDevicePhysical;
use ::vulkan::VulkanInstance;
use ::vulkan::VulkanSharingMode;
use ::vulkan::VulkanBufferUsageFlagS;
use ::vulkan::VulkanBufferCreateInformation;
use ::vulkan::VulkanMemoryAllocateInfomration;
use ::vulkan::VulkanMemoryPropertyFlagS;
use ::vulkan::VulkanBuffer;
use ::vulkan::VulkanDeviceMemory;
use ::vulkan::VulkanDeviceSize;
use ::vulkan::VulkanCommandBufferAllocateInformation;
use ::vulkan::VulkanCommandBufferLevel;
use ::vulkan::VulkanCommandPool;
use ::vulkan::VulkanCommandBufferUsageFlagS;
use ::vulkan::VulkanCommandBufferBeginInformation;
use ::vulkan::VulkanBufferCopy;
use ::vulkan::VulkanSubmitInformation;
use ::vulkan::VulkanQueue;
use ::vulkan::VulkanFence;

use crate::termination::TerminationProcessMain;
use crate::application::vulkan_memory::ApplicationVulkanMemory;


pub struct ApplicationVulkanBuffer {}

impl ApplicationVulkanBuffer {
    pub unsafe fn create_with_memory(
        vulkan_instance: &VulkanInstance,
        vulkan_physical_device: VulkanDevicePhysical,
        vulkan_logical_device: &VulkanDeviceLogical,
        vulkan_buffer_size: VulkanDeviceSize,
        vulkan_buffer_usage: VulkanBufferUsageFlagS,
        vulkan_memory_property_flag_s: VulkanMemoryPropertyFlagS,
    ) -> Result<(VulkanBuffer, VulkanDeviceMemory), TerminationProcessMain>
    {
        let vulkan_buffer_create_information =
            VulkanBufferCreateInformation::builder()
            .size(vulkan_buffer_size)
            .usage(vulkan_buffer_usage)
            .sharing_mode(VulkanSharingMode::EXCLUSIVE);
        let create_vulkan_buffer_result =
            vulkan_logical_device.create_buffer(&vulkan_buffer_create_information, None);
        let vulkan_buffer =
            match create_vulkan_buffer_result {
                Err(error) => {
                    let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                    return Err(TerminationProcessMain::InitializationVulkanBufferCreateFail(vulkan_error_code));
                },
                Ok(buffer) => buffer,
            };
        let vulkan_memory_requirement_s =
            vulkan_logical_device.get_buffer_memory_requirements(vulkan_buffer);
        let vulkan_memory_type_index =
            ApplicationVulkanMemory::get_type_index(
                vulkan_instance, vulkan_physical_device, vulkan_memory_property_flag_s, vulkan_memory_requirement_s)?;
        let vulkan_memory_allocate_infomation =
            VulkanMemoryAllocateInfomration::builder()
            .allocation_size(vulkan_memory_requirement_s.size)
            .memory_type_index(vulkan_memory_type_index.as_raw());
        let allocate_vulkan_buffer_memory_result =
            vulkan_logical_device.allocate_memory(&vulkan_memory_allocate_infomation, None);
        let vulkan_buffer_memory =
            match allocate_vulkan_buffer_memory_result {
                Err(error) => {
                    let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                    return Err(TerminationProcessMain::InitializationVulkanMemoryAllocateFail(vulkan_error_code));
                },
                Ok(memory) => memory,
            };
        let bind_buffer_memory_result =
            vulkan_logical_device.bind_buffer_memory(vulkan_buffer, vulkan_buffer_memory, 0);
        match bind_buffer_memory_result {
            Err(error) => {
                let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                return Err(TerminationProcessMain::InitializationVulkanMemoryBufferBindFail(vulkan_error_code));
            },
            Ok(()) => (),
        };
        Ok((vulkan_buffer, vulkan_buffer_memory))
    }
}