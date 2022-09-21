use ::vulkan::prelude::version1_2::*;
use ::vulkan::extend::VulkanErrorCode;
use ::vulkan::VulkanSemaphore;
use ::vulkan::VulkanSemaphoreCreateInformation;
use ::vulkan::VulkanFence;
use ::vulkan::VulkanFenceCreateFlagS;
use ::vulkan::VulkanFenceCreateInformation;
use ::vulkan::VulkanImage;

use crate::config::vulkan::VULKAN_FRAME_IN_FLIGHT_MAX;
use crate::termination::TerminationProcessMain;


pub struct ApplicationVulkanSynchronization {}

impl ApplicationVulkanSynchronization {
    pub unsafe fn create_all(
        vulkan_logical_device: &VulkanDeviceLogical,
        vulkan_image_s: &Vec<VulkanImage>)
     -> Result<(Vec<VulkanSemaphore>, Vec<VulkanSemaphore>, Vec<VulkanFence>, Vec<VulkanFence>),
               TerminationProcessMain>
    {
        let vulkan_semaphore_create_information = VulkanSemaphoreCreateInformation::builder();
        let vulkan_fence_create_information =
            VulkanFenceCreateInformation::builder().flags(VulkanFenceCreateFlagS::SIGNALED);
        let mut vulkan_available_image_semaphore_s: Vec<VulkanSemaphore> = Vec::new();
        vulkan_available_image_semaphore_s.reserve_exact(VULKAN_FRAME_IN_FLIGHT_MAX as usize);
        let mut vulkan_render_finished_semaphire_s: Vec<VulkanSemaphore> = Vec::new();
        vulkan_render_finished_semaphire_s.reserve_exact(VULKAN_FRAME_IN_FLIGHT_MAX as usize);
        let mut vulkan_slide_in_flight_fence_s: Vec<VulkanFence> = Vec::new();
        vulkan_slide_in_flight_fence_s.reserve_exact(VULKAN_FRAME_IN_FLIGHT_MAX as usize);
        for _ in 0..VULKAN_FRAME_IN_FLIGHT_MAX {
            let (vulkan_semaphore_a, vulkan_semaphore_b)  =
                (Self::create_semaphore(vulkan_logical_device, &vulkan_semaphore_create_information)?,
                 Self::create_semaphore(vulkan_logical_device, &vulkan_semaphore_create_information)?);
            vulkan_available_image_semaphore_s.push(vulkan_semaphore_a);
            vulkan_render_finished_semaphire_s.push(vulkan_semaphore_b);
            let vulkan_fence = Self::create_fence(vulkan_logical_device, &vulkan_fence_create_information)?;
            vulkan_slide_in_flight_fence_s.push(vulkan_fence);
        }
        let vulklan_image_in_flight_fence_s: Vec<VulkanFence> = vulkan_image_s.iter().map(|_| VulkanFence::null()).collect();
        Ok((vulkan_available_image_semaphore_s, vulkan_render_finished_semaphire_s,
            vulkan_slide_in_flight_fence_s, vulklan_image_in_flight_fence_s))
    }

    unsafe fn create_semaphore(
        vulkan_logical_device: &VulkanDeviceLogical,
        vulkan_semaphore_create_information: &VulkanSemaphoreCreateInformation)
     -> Result<VulkanSemaphore, TerminationProcessMain>
    {
        let create_vulkan_semaphore_result =
            vulkan_logical_device.create_semaphore(vulkan_semaphore_create_information, None);
        match create_vulkan_semaphore_result {
            Err(error) => {
                let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                Err(TerminationProcessMain::InitializationVulkanSemaphoreCreateFail(vulkan_error_code))
            },
            Ok(semaphore) => Ok(semaphore),
        }
    }

    unsafe fn create_fence(
        vulkan_logical_device: &VulkanDeviceLogical,
        vulkan_fence_create_information: &VulkanFenceCreateInformation)
     -> Result<VulkanFence, TerminationProcessMain>
    {
        let create_vulkan_fence_result =
            vulkan_logical_device.create_fence(vulkan_fence_create_information, None);
        match create_vulkan_fence_result {
            Err(error) => {
                let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                Err(TerminationProcessMain::InitializationVulkanFenceCreateFail(vulkan_error_code))
            },
            Ok(fence) => Ok(fence),
        }
    }
}
