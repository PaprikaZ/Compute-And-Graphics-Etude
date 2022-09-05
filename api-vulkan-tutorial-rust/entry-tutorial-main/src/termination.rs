use ::libloading::Error as LibraryLoadingError;
use ::window_uniform::WindowUniformErrorOperationSystem;
use ::vulkan::VulkanErrorCode;


pub enum TerminationProcessMain {
    Ok,
    InitializationLoggerConsoleFail,
    InitializationWindowUniformFail(WindowUniformErrorOperationSystem),
    InitializationVulkanLibraryLoadingFail(LibraryLoadingError),
    InitializationVulkanEntryCreateFail(Box<dyn std::error::Error + Send + Sync + 'static>),
    InitializationVulkanInstanceCreateFail(VulkanErrorCode),
    InitializationVulkanValidationLayerNotSupport,
    InitializationVulkanEnumeratePhysicalDeviceFail(VulkanErrorCode),
    InitializationVulkanDevicePhysicalAllNotQualified,
    InitializationVulkanDeviceLogicalCreateFail(VulkanErrorCode),
    InitializationVulkanSurfaceCreateFail(VulkanErrorCode),
    InitializationVulkanDevicePhysicalSurfaceCapabilitySGetFail(VulkanErrorCode),
    InitializationVulkanDevicePhysicalSurfaceFormatSGetFail(VulkanErrorCode),
    InitializationVulkanDevicePhysicalSurfacePresentModeSGetFail(VulkanErrorCode),
    InitializationVulkanSwapchainCreateFail(VulkanErrorCode),
    InitializationVulkanSwapchainImageSGetFail(VulkanErrorCode),
    InitializationVulkanDevicePhysicalExtensionPropertySEnumerateFail(VulkanErrorCode),
    InitializationVulkanImageViewCreateFail(VulkanErrorCode),
    InitializationVulkanShaderByteCodeAlignmentIncorrect,
    InitializationVulkanShaderModuleCreateFail(VulkanErrorCode),
    InitializationVulkanPipelineLayoutCreateFail(VulkanErrorCode),
    InitializationVulkanRenderPassCreateFail(VulkanErrorCode),
    InitializationVulkanGraphicsPipelineSCreateFail(VulkanErrorCode),
    InitializationVulkanFrameBufferCreateFail(VulkanErrorCode),
    InitializationVulkanCommandPoolCreateFail(VulkanErrorCode),
    InitializationVulkanCommandBufferSAllocateFail(VulkanErrorCode),
    InitializationVulkanCommandBufferBeginFail(VulkanErrorCode),
    InitializationVulkanCommandBufferEndFail(VulkanErrorCode),
    InitializationVulkanSemaphoreCreateFail(VulkanErrorCode),
    InitializationVulkanFenceCreateFail(VulkanErrorCode),
}

impl TerminationProcessMain {
    fn to_exit_code(self) -> u8 {
        match self {
            Self::Ok => 0u8,
            Self::InitializationLoggerConsoleFail => 1u8,
            Self::InitializationWindowUniformFail(_) => 2u8,
            Self::InitializationVulkanLibraryLoadingFail(_) => 3u8,
            Self::InitializationVulkanEntryCreateFail(_) => 4u8,
            Self::InitializationVulkanInstanceCreateFail(_) => 5u8,
            Self::InitializationVulkanValidationLayerNotSupport => 6u8,
            Self::InitializationVulkanEnumeratePhysicalDeviceFail(_) => 7u8,
            Self::InitializationVulkanDevicePhysicalAllNotQualified => 8u8,
            Self::InitializationVulkanDeviceLogicalCreateFail(_) => 9u8,
            Self::InitializationVulkanSurfaceCreateFail(_) => 10u8,
            Self::InitializationVulkanDevicePhysicalSurfaceCapabilitySGetFail(_) => 11u8,
            Self::InitializationVulkanDevicePhysicalSurfaceFormatSGetFail(_) => 12u8,
            Self::InitializationVulkanDevicePhysicalSurfacePresentModeSGetFail(_) => 13u8,
            Self::InitializationVulkanSwapchainCreateFail(_) => 14u8,
            Self::InitializationVulkanSwapchainImageSGetFail(_) => 15u8,
            Self::InitializationVulkanDevicePhysicalExtensionPropertySEnumerateFail(_) => 16u8,
            Self::InitializationVulkanImageViewCreateFail(_) => 17u8,
            Self::InitializationVulkanShaderByteCodeAlignmentIncorrect => 18u8,
            Self::InitializationVulkanShaderModuleCreateFail(_) => 19u8,
            Self::InitializationVulkanPipelineLayoutCreateFail(_) => 20u8,
            Self::InitializationVulkanRenderPassCreateFail(_) => 21u8,
            Self::InitializationVulkanGraphicsPipelineSCreateFail(_) => 22u8,
            Self::InitializationVulkanFrameBufferCreateFail(_) => 23u8,
            Self::InitializationVulkanCommandPoolCreateFail(_) => 24u8,
            Self::InitializationVulkanCommandBufferSAllocateFail(_) => 25u8,
            Self::InitializationVulkanCommandBufferBeginFail(_) => 26u8,
            Self::InitializationVulkanCommandBufferEndFail(_) => 27u8,
            Self::InitializationVulkanSemaphoreCreateFail(_) => 28u8,
            Self::InitializationVulkanFenceCreateFail(_) => 29u8,
        }
    }
}

impl std::process::Termination for TerminationProcessMain {
    fn report(self) -> std::process::ExitCode {
        std::process::ExitCode::from(self.to_exit_code())
    }
}