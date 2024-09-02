use crate::error::foundation_application_guide::ErrorFoundationApplicationGuide;


#[derive(Debug)]
enum ApplicationNegotiationVulkanDevicePhysicalRequirementNotMatch {
    VersionApiNotFulfilled,
    QueueFamilyNotFulfilled,
    ExtensionNotFulfilled,
    SwapchainFormatNoneFulfilled,
    SwapchainPresentModeNoneFulfilled,
    FeatureNotFulfilled,
}

#[derive(Debug)]
enum ApplicationNegotiationVulkanDevicePhysicalRequirementPickError {
    RequirementNotMatch(ApplicationNegotiationVulkanDevicePhysicalRequirementNotMatch),
    Error(ErrorFoundationApplicationGuide),
}
