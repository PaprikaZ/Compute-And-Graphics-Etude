#[derive(Debug)]
pub enum ErrorFoundationVulkanCookedOwn {
    WindowUniformCreateFail,
    WindowUniformEventLoopCreateFail,
    VulkanNegotiationRankWeightFactorExponentialInvalid,
    VulkanRequirementVersionApiLeastInstanceNotFulfilled,
    VulkanRequirementVersionApiLeastDevicePhysicalNotFulfilled,
    VulkanLibraryLoaderInitializeFail,
    VulkanEntryInitializeFail,
    VulkanInstanceLayerPropertySEnumerateFail,
    VulkanInstanceExtensionPropertySEnumerateFail,
    VulkanRequirementInstanceLayerSNotFulfilled,
    VulkanRequirementInstanceExtensionSNotFulfilled,
    VulkanDevicePhysicalExtensionPropertySEnumerateFail,
    VulkanRequirementDevicePhysicalExtensionSNotFulfilled,
    VulkanDevicePhysicalSurfaceCapabilitySGetFail,
    VulkanDevicePhysicalSurfaceFormatSGetFail,
    VulkanDevicePhysicalSurfacePresentModeSGetFail,
    VulkanRequirementDevicePhysicalSurfaceFormatNoneFulfilled,
    VulkanRequirementDevicePhysicalSurfacePresentModeNoneFulfilled,
    VulkanRequirementDevicePhysicalFeatureNotFulfilled,
    VulkanRequirementDevicePhysicalFeatureSNotFulfilled,
    VulkanRequirementDevicePhysicalQueueFamilySNotFulfilled,
    VulkanDeviceLogicalCreateFail,
    VulkanSwapchainCreateFail,
    VulkanSwapchainImageSGetFail,
    VulkanSwapchainImageViewSCreateFail,
    PathFileGraphicMeshOpenFail,
    PathFileGraphicMeshDataCorrupted,
    VulkanMemoryTypeNotSupport,
    VulkanBufferCreateFail,
    VulkanMemoryAllocateFail,
    VulkanBufferMemoryBindFail,
    VulkanCommandBufferBeginFail,
    VulkanCommandBufferEndFail,
    VulkanCommandBufferSubmitFail,
    VulkanCommandBufferResetFail,
    VulkanQueueWaitIdleFail,
    VulkanFenceWaitFail,
    VulkanFenceResetFail,
}


#[derive(Debug)]
pub enum ErrorFoundationVulkanCooked {
    Own(ErrorFoundationVulkanCookedOwn),
}

impl From<ErrorFoundationVulkanCookedOwn> for ErrorFoundationVulkanCooked {
    fn from(error: ErrorFoundationVulkanCookedOwn) -> Self {
        Self::Own(error)
    }
}
