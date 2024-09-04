use ::library_foundation_reintroduction::error::foundation_reintroduction::ErrorFoundationReintroductionOwn;
use ::library_foundation_reintroduction::error::foundation_reintroduction::ErrorFoundationReintroduction;
use ::library_foundation_vulkan_cooked::error::foundation_vulkan_cooked::ErrorFoundationVulkanCookedOwn;
use ::library_foundation_vulkan_cooked::error::foundation_vulkan_cooked::ErrorFoundationVulkanCooked;


#[derive(Debug)]
pub enum ErrorFoundationApplicationGuideOwn {
    WindowEventLoopRunAbort,
    VulkanInstanceVersionApiQueryFail,

    VulkanInstanceCreateFail,
    VulkanSurfaceCreateFail,
    VulkanDevicePhysicalEnumerateFail,
    VulkanDevicePhysicalRequirementNoneFulfilled,
    VulkanRenderPassCreateFail,
    VulkanFrameBufferCreateFail,
    VulkanCommandPoolCreateFail,
    VulkanCommandBufferAllocateFail,
    VulkanFenceCreateFail,
    VulkanSemaphoreCreateFail,

    VulkanDeviceLogicalWaitIdleFail,
    VulkanDeviceLogicalFenceWaitFail,
    VulkanDeviceLogicalFenceResetFail,
    VulkanDeviceLogicalCommandBufferResetFail,
    VulkanDeviceLogicalSwapchainImageIndexNextAcquireFail,
    VulkanDeviceLogicalCommandBufferBeginFail,
    VulkanDeviceLogicalCommandBufferEndFail,
    VulkanDeviceLogicalQueueSubmitFail,
    VulkanDeviceLogicalQueuePresentFail,
}


#[derive(Debug)]
pub enum ErrorFoundationApplicationGuide {
    Own(ErrorFoundationApplicationGuideOwn),
    ScopeReintroduction(ErrorFoundationReintroductionOwn),
    ScopeVulkanCooked(ErrorFoundationVulkanCookedOwn),
}

impl From<ErrorFoundationApplicationGuideOwn> for ErrorFoundationApplicationGuide {
    fn from(error: ErrorFoundationApplicationGuideOwn) -> Self {
        Self::Own(error)
    }
}

impl From<ErrorFoundationReintroduction> for ErrorFoundationApplicationGuide {
    fn from(error: ErrorFoundationReintroduction) -> Self {
        match error {
            ErrorFoundationReintroduction::Own(e) =>
                Self::ScopeReintroduction(e),
        }
    }
}

impl From<ErrorFoundationReintroductionOwn> for ErrorFoundationApplicationGuide {
    fn from(error: ErrorFoundationReintroductionOwn) -> Self {
        Self::ScopeReintroduction(error)
    }
}

impl From<ErrorFoundationVulkanCooked> for ErrorFoundationApplicationGuide {
    fn from(error: ErrorFoundationVulkanCooked) -> Self {
        match error {
            ErrorFoundationVulkanCooked::Own(e) =>
                Self::ScopeVulkanCooked(e),
        }
    }
}

impl From<ErrorFoundationVulkanCookedOwn> for ErrorFoundationApplicationGuide {
    fn from(error: ErrorFoundationVulkanCookedOwn) -> Self {
        Self::ScopeVulkanCooked(error)
    }
}