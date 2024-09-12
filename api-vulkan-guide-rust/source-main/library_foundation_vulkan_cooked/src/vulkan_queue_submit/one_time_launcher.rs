use ::library_foundation_reintroduction::vulkan::VulkanHandler;
use ::library_foundation_reintroduction::vulkan::VulkanDeviceVersion1_0;
use ::library_foundation_reintroduction::vulkan::VulkanBuilderHas;
use ::library_foundation_reintroduction::vulkan::VulkanDeviceLogical;
use ::library_foundation_reintroduction::vulkan::VulkanQueue;
use ::library_foundation_reintroduction::vulkan::VulkanFence;
use ::library_foundation_reintroduction::vulkan::VulkanCommandBuffer;
use ::library_foundation_reintroduction::vulkan::VulkanCommandBufferBeginInformation;
use ::library_foundation_reintroduction::vulkan::VulkanCommandBufferUsageFlagS;
use ::library_foundation_reintroduction::vulkan::VulkanSubmitInformation;
use ::library_foundation_reintroduction::vulkan::VulkanCommandBufferResetFlagS;

use crate::error::foundation_vulkan_cooked::ErrorFoundationVulkanCookedOwn;
use crate::error::foundation_vulkan_cooked::ErrorFoundationVulkanCooked;


#[derive(Debug, Clone)]
pub struct VulkanQueueSubmitOneTimeLauncher<'t> {
    pub device_logical: &'t VulkanDeviceLogical,
    pub queue: VulkanQueue,
    pub command_buffer: VulkanCommandBuffer,
}

impl<'t> VulkanQueueSubmitOneTimeLauncher<'t> {
    pub fn new(
        vulkan_logical_device: &'t VulkanDeviceLogical,
        vulkan_queue: VulkanQueue,
        vulkan_command_buffer: VulkanCommandBuffer)
    -> Self
    {
        Self {
            device_logical: vulkan_logical_device,
            queue: vulkan_queue,
            command_buffer: vulkan_command_buffer,
        }
    }

    pub fn launch<TE>(
        self,
        action: impl FnOnce(&VulkanDeviceLogical, VulkanCommandBuffer) -> Result<(), TE>,
        vulkan_fence_o: Option<VulkanFence>)
    -> Result<(), TE>
    where TE: From<ErrorFoundationVulkanCooked> + From<ErrorFoundationVulkanCookedOwn>
    {
        let vulkan_command_buffer_begin_information =
            VulkanCommandBufferBeginInformation::builder()
            .flags(VulkanCommandBufferUsageFlagS::ONE_TIME_SUBMIT)
            .build();
        unsafe { self.device_logical.begin_command_buffer(self.command_buffer, &vulkan_command_buffer_begin_information) }
        .or(Err(ErrorFoundationVulkanCookedOwn::VulkanCommandBufferBeginFail))?;
        //
        action(self.device_logical, self.command_buffer)?;
        //
        unsafe { self.device_logical.end_command_buffer(self.command_buffer) }
        .or(Err(ErrorFoundationVulkanCookedOwn::VulkanCommandBufferEndFail))?;
        let vulkan_submit_information =
            VulkanSubmitInformation::builder().command_buffers(&[self.command_buffer]).build();
        match vulkan_fence_o {
            Some(vulkan_fence) => {
                unsafe { self.device_logical.queue_submit(self.queue, &[vulkan_submit_information], vulkan_fence) }
                .or(Err(ErrorFoundationVulkanCookedOwn::VulkanCommandBufferSubmitFail))?;
                unsafe { self.device_logical.wait_for_fences(&[vulkan_fence], true, 8000_000_000) }
                .or(Err(ErrorFoundationVulkanCookedOwn::VulkanFenceWaitFail))?;
                unsafe { self.device_logical.reset_fences(&[vulkan_fence]) }
                .or(Err(ErrorFoundationVulkanCookedOwn::VulkanFenceResetFail))?;
            },
            None => {
                unsafe { self.device_logical.queue_submit(self.queue, &[vulkan_submit_information], VulkanFence::null()) }
                .or(Err(ErrorFoundationVulkanCookedOwn::VulkanCommandBufferSubmitFail))?;
                unsafe { self.device_logical.queue_wait_idle(self.queue) }
                .or(Err(ErrorFoundationVulkanCookedOwn::VulkanQueueWaitIdleFail))?;
            }
        }
        unsafe { self.device_logical.reset_command_buffer(self.command_buffer, VulkanCommandBufferResetFlagS::empty()) }
        .or(Err(ErrorFoundationVulkanCookedOwn::VulkanCommandBufferResetFail))?;
        Ok(())
    }
}