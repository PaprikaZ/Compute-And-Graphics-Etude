#[derive(Debug)]
pub enum ErrorFoundationVulkanCookedOwn {
    WindowUniformCreateFail,
    WindowUniformEventLoopCreateFail,
    VulkanNegotiationRankWeightFactorExponentialInvalid,
    VulkanRequirementVersionApiLeastInstanceNotFulfilled,
    VulkanRequirementVersionApiLeastDevicePhysicalNotFulfilled,
    VulkanInstanceLayerPropertySEnumerateFail,
    VulkanInstanceExtensionPropertySEnumerateFail,
    VulkanRequirementInstanceLayerSNotFulfilled,
    VulkanRequirementInstanceExtensionSNotFulfilled,
    VulkanRequirementDevicePhysicalQueueFamilySNotFulfilled,
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