use std::vec::IntoIter;

use ::library_foundation_reintroduction::vulkan::VulkanBuffer;
use ::library_foundation_reintroduction::vulkan::VulkanDeviceMemory;


#[derive(Debug, Clone, Copy)]
pub(super) enum ApplicationGraphicResourceDestroyDirective {
    DestroyVulkanCommandPoolMain,
    DestroyVulkanFenceRenderFinished,
    DestroyVulkanSemaphoreImageAvailable,
    DestroyVulkanSemaphoreRenderFinished,
    DestroyVulkanSwapchain,
    DestroyVulkanRenderPassMain,
    DestroyVulkanSwapchainFrameBufferS,
    DestroyVulkanSwapchainImageViewS,
    DestroyVulkanImageDepthView,
    DestroyVulkanPipelineTriangleRed,
    DestroyVulkanPipelineTriangleColor,
    DestroyVulkanPipelineMesh,
    DestroyVulkanPipelineLayoutStatic,
    DestroyVulkanPipelineLayoutDynamic,
    DestroyVulkanBuffer(VulkanBuffer),
    FreeVulkanDeviceMemory(VulkanDeviceMemory),
}

impl ApplicationGraphicResourceDestroyDirective {
}


#[derive(Debug, Clone)]
pub(super) struct ApplicationGraphicResourceDestroyStack {
    directive_s: Vec<ApplicationGraphicResourceDestroyDirective>,
}

impl ApplicationGraphicResourceDestroyStack {
    pub(super) fn new_empty() -> Self
    {
        Self {
            directive_s: Vec::new(),
        }
    }

    pub(super) fn push(&mut self, new_directive: ApplicationGraphicResourceDestroyDirective) {
        self.directive_s.push(new_directive)
    }

    pub(super) fn into_iter_flush(mut self) -> IntoIter<ApplicationGraphicResourceDestroyDirective>
    {
        let reversed_directive_s = { self.directive_s.reverse(); self.directive_s };
        reversed_directive_s.into_iter()
    }
}