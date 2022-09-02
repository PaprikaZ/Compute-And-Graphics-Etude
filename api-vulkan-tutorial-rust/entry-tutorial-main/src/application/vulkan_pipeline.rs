use ::vulkan::prelude::version1_2::*;

use crate::termination::TerminationProcessMain;


pub struct ApplicationVulkanPipeline {};

impl ApplicationVulkanPipeline {
    pub unsafe fn create(vulkan_logical_device: VulkanDeviceLogical) -> Result<(), TerminationProcessMain> {
        Ok(())
    }
}