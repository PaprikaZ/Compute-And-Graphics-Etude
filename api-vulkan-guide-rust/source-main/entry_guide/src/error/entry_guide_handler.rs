use ::library_foundation_reintroduction::error::foundation_reintroduction::ErrorFoundationReintroductionOwn;
use ::library_foundation_vulkan_cooked::error::foundation_vulkan_cooked::ErrorFoundationVulkanCookedOwn;
use ::library_foundation_application_guide::error::foundation_application_guide::ErrorFoundationApplicationGuideOwn;

use crate::error::entry_guide::ErrorEntryGuideOwn;
use crate::error::entry_guide::ErrorEntryGuide;


pub struct ErrorEntryGuideHandler {}

impl ErrorEntryGuideHandler {
    fn handle_reintroduction_own(error: ErrorFoundationReintroductionOwn)
    -> Option<ErrorFoundationReintroductionOwn>
    {
        let (error_message, be_bypass) =
            match &error {
                ErrorFoundationReintroductionOwn::VulkanApplicationNameInvalid =>
                    ("vulkan application name invalid", true),
                ErrorFoundationReintroductionOwn::VulkanEngineNameInvalid =>
                    ("vulkan engine name invalid", true),
            };
        println!("{error_message}");
        if be_bypass { Some(error) } else { None }
    }

    fn handle_vulkan_cooked_own(error: ErrorFoundationVulkanCookedOwn)
    -> Option<ErrorFoundationVulkanCookedOwn>
    {
        let (error_message, be_bypass) =
            match &error {
                ErrorFoundationVulkanCookedOwn::WindowUniformCreateFail =>
                    ("window uniform create fail", true),
                ErrorFoundationVulkanCookedOwn::WindowUniformEventLoopCreateFail =>
                    ("window uniform event loop create fail", true),
                ErrorFoundationVulkanCookedOwn::VulkanNegotiationRankWeightFactorExponentialInvalid =>
                    ("vulkan negotiation rank weight factor exponential invalid", true),
                ErrorFoundationVulkanCookedOwn::VulkanRequirementVersionApiLeastInstanceNotFulfilled =>
                    ("vulkan requirement version api least instance not fulfilled", true),
                ErrorFoundationVulkanCookedOwn::VulkanRequirementVersionApiLeastDevicePhysicalNotFulfilled =>
                    ("vulkan requirement version api least device physical not fulfilled", true),
                ErrorFoundationVulkanCookedOwn::VulkanLibraryLoaderInitializeFail =>
                    ("vulkan library loader initialize fail", true),
                ErrorFoundationVulkanCookedOwn::VulkanEntryInitializeFail =>
                    ("vulkan entry initialize fail", true),
                ErrorFoundationVulkanCookedOwn::VulkanInstanceLayerPropertySEnumerateFail =>
                    ("vulkan instance layer property s enumerate fail", true),
                ErrorFoundationVulkanCookedOwn::VulkanInstanceExtensionPropertySEnumerateFail =>
                    ("vulkan instance extension property s enumerate fail", true),
                ErrorFoundationVulkanCookedOwn::VulkanRequirementInstanceLayerSNotFulfilled =>
                    ("vulkan requirement instance layer s not fulfilled", true),
                ErrorFoundationVulkanCookedOwn::VulkanRequirementInstanceExtensionSNotFulfilled =>
                    ("vulkan requirement instance extension s not fulfilled", true),
                ErrorFoundationVulkanCookedOwn::VulkanDevicePhysicalExtensionPropertySEnumerateFail =>
                    ("vulkan device physical extension property s enumerate fail", true),
                ErrorFoundationVulkanCookedOwn::VulkanRequirementDevicePhysicalExtensionSNotFulfilled =>
                    ("vulkan requirement device physical extension s not fulfilled", true),
                ErrorFoundationVulkanCookedOwn::VulkanDevicePhysicalSurfaceCapabilitySGetFail =>
                    ("vulkan device physical surface capability s get fail", true),
                ErrorFoundationVulkanCookedOwn::VulkanDevicePhysicalSurfaceFormatSGetFail =>
                    ("vulkan device physical surface format s get fail", true),
                ErrorFoundationVulkanCookedOwn::VulkanDevicePhysicalSurfacePresentModeSGetFail =>
                    ("vulkan device physical surface present mode s get fail", true),
                ErrorFoundationVulkanCookedOwn::VulkanRequirementDevicePhysicalSurfaceFormatNoneFulfilled =>
                    ("vulkan requirement device physical surface format none fulfilled", true),
                ErrorFoundationVulkanCookedOwn::VulkanRequirementDevicePhysicalSurfacePresentModeNoneFulfilled =>
                    ("vulkan requirement device physical surface present mode none fulfilled", true),
                ErrorFoundationVulkanCookedOwn::VulkanRequirementDevicePhysicalFeatureNotFulfilled =>
                    ("vulkan requirement device physical feature not fulfilled", true),
                ErrorFoundationVulkanCookedOwn::VulkanRequirementDevicePhysicalFeatureSNotFulfilled =>
                    ("vulkan requirement device physical feature s not fulfilled", true),
                ErrorFoundationVulkanCookedOwn::VulkanRequirementDevicePhysicalQueueFamilySNotFulfilled =>
                    ("vulkan requirement device physical queue family s not fulfilled", true),
                ErrorFoundationVulkanCookedOwn::VulkanDeviceLogicalCreateFail =>
                    ("vulkan device logical create fail", true),
                ErrorFoundationVulkanCookedOwn::VulkanSwapchainCreateFail =>
                    ("vulkan swapchain create fail", true),
                ErrorFoundationVulkanCookedOwn::VulkanSwapchainImageSGetFail =>
                    ("vulkan swapchain image s get fail", true),
                ErrorFoundationVulkanCookedOwn::VulkanSwapchainImageViewSCreateFail =>
                    ("vulkan swapchain image view s create fail", true),
                ErrorFoundationVulkanCookedOwn::PathFileGraphicMeshOpenFail =>
                    ("path file graphic mesh open fail", true),
                ErrorFoundationVulkanCookedOwn::PathFileGraphicMeshDataCorrupted =>
                    ("path file graphic mesh data corrupted", true),
                ErrorFoundationVulkanCookedOwn::VulkanMemoryTypeNotSupport =>
                    ("vulkan memory type not support", true),
                ErrorFoundationVulkanCookedOwn::VulkanMemoryAllocateFail =>
                    ("vulkan memory allocate fail", true),
                ErrorFoundationVulkanCookedOwn::VulkanBufferCreateFail =>
                    ("vulkan buffer create fail", true),
                ErrorFoundationVulkanCookedOwn::VulkanImageCreateFail =>
                    ("vulkan image create fail", true),
                ErrorFoundationVulkanCookedOwn::VulkanBufferMemoryBindFail =>
                    ("vulkan buffer memory bind fail", true),
                ErrorFoundationVulkanCookedOwn::VulkanImageMemoryBindFail =>
                    ("vulkan image memory bind fail", true),
                ErrorFoundationVulkanCookedOwn::VulkanCommandBufferBeginFail =>
                    ("vulkan command buffer begin fail", true),
                ErrorFoundationVulkanCookedOwn::VulkanCommandBufferEndFail =>
                    ("vulkan command buffer end fail", true),
                ErrorFoundationVulkanCookedOwn::VulkanCommandBufferSubmitFail =>
                    ("vulkan command buffer submit fail", true),
                ErrorFoundationVulkanCookedOwn::VulkanCommandBufferResetFail =>
                    ("vulkan command buffer reset fail", true),
                ErrorFoundationVulkanCookedOwn::VulkanQueueWaitIdleFail =>
                    ("vulkan queue wait idle fail", true),
                ErrorFoundationVulkanCookedOwn::VulkanFenceWaitFail =>
                    ("vulkan fence wait fail", true),
                ErrorFoundationVulkanCookedOwn::VulkanFenceResetFail =>
                    ("vulkan fence reset fail", true),
                ErrorFoundationVulkanCookedOwn::VulkanMemoryMapFail =>
                    ("vulkan memory map fail", true),
                ErrorFoundationVulkanCookedOwn::VulkanImageDepthFormatFeatureNotSupport =>
                    ("vulkan image depth format feature not support", true),
            };
        println!("{error_message}");
        if be_bypass { Some(error) } else { None }
    }

    fn handle_application_guide_own(error: ErrorFoundationApplicationGuideOwn)
    -> Option<ErrorFoundationApplicationGuideOwn>
    {
        let (error_message, be_bypass) =
            match &error {
                ErrorFoundationApplicationGuideOwn::WindowEventLoopRunAbort =>
                    ("window event loop run abort", true),
                ErrorFoundationApplicationGuideOwn::VulkanInstanceVersionApiQueryFail =>
                    ("vulkan instance version api query fail", true),
                ErrorFoundationApplicationGuideOwn::VulkanEntryVersionGetFail =>
                    ("vulkan entry version get fail", true),
                ErrorFoundationApplicationGuideOwn::VulkanInstanceCreateFail =>
                    ("vulkan instance create fail", true),
                ErrorFoundationApplicationGuideOwn::VulkanDebugUtilityMessengerCreateFail =>
                    ("vulkan debug utility messenger create fail", true),
                ErrorFoundationApplicationGuideOwn::VulkanSurfaceCreateFail =>
                    ("vulkan surface create fail", true),
                ErrorFoundationApplicationGuideOwn::VulkanDevicePhysicalEnumerateFail =>
                    ("vulkan device physical enumerate fail", true),
                ErrorFoundationApplicationGuideOwn::VulkanDevicePhysicalRequirementNoneFulfilled =>
                    ("vulkan device physical requirement none fulfilled", true),
                ErrorFoundationApplicationGuideOwn::VulkanRenderPassCreateFail =>
                    ("vulkan render pass create fail", true),
                ErrorFoundationApplicationGuideOwn::VulkanFrameBufferCreateFail =>
                    ("vulkan frame buffer create fail", true),
                ErrorFoundationApplicationGuideOwn::VulkanCommandPoolCreateFail =>
                    ("vulkan command pool create fail", true),
                ErrorFoundationApplicationGuideOwn::VulkanCommandBufferAllocateFail =>
                    ("vulkan command buffer allocate fail", true),
                ErrorFoundationApplicationGuideOwn::VulkanFenceCreateFail =>
                    ("vulkan fence create fail", true),
                ErrorFoundationApplicationGuideOwn::VulkanSemaphoreCreateFail =>
                    ("vulkan semaphore create fail", true),
                ErrorFoundationApplicationGuideOwn::VulkanDeviceLogicalWaitIdleFail =>
                    ("vulkan device logical wait idle fail", true),
                ErrorFoundationApplicationGuideOwn::VulkanDeviceLogicalFenceWaitFail =>
                    ("vulkan device logical fence wait fail", true),
                ErrorFoundationApplicationGuideOwn::VulkanDeviceLogicalFenceResetFail =>
                    ("vulkan device logical fence reset fail", true),
                ErrorFoundationApplicationGuideOwn::VulkanDeviceLogicalCommandBufferResetFail =>
                    ("vulkan device logical command buffer reset fail", true),
                ErrorFoundationApplicationGuideOwn::VulkanDeviceLogicalSwapchainImageIndexNextAcquireFail =>
                    ("vulkan device logical swapchain image index next acquire fail", true),
                ErrorFoundationApplicationGuideOwn::VulkanDeviceLogicalCommandBufferBeginFail =>
                    ("vulkan device logical command buffer begin fail", true),
                ErrorFoundationApplicationGuideOwn::VulkanDeviceLogicalCommandBufferEndFail =>
                    ("vulkan device logical command buffer end fail", true),
                ErrorFoundationApplicationGuideOwn::VulkanDeviceLogicalQueueSubmitFail =>
                    ("vulkan device logical queue submit fail", true),
                ErrorFoundationApplicationGuideOwn::VulkanDeviceLogicalQueuePresentFail =>
                    ("vulkan device logical queue present fail", true),
                ErrorFoundationApplicationGuideOwn::VulkanShaderBytecodeFileReadFail =>
                    ("vulkan shader bytecode file read fail", true),
                ErrorFoundationApplicationGuideOwn::VulkanShaderBytecodeDataAlignmentCorrupted =>
                    ("vulkan shader bytecode data alignment corrupted", true),
                ErrorFoundationApplicationGuideOwn::VulkanShaderModuleCreateFail =>
                    ("vulkan shader module create fail", true),
                ErrorFoundationApplicationGuideOwn::VulkanPipelineLayoutCreateFail =>
                    ("vulkan pipeline layout create fail", true),
                ErrorFoundationApplicationGuideOwn::VulkanPipelineCreateFail =>
                    ("vulkan pipeline create fail", true),
                ErrorFoundationApplicationGuideOwn::VulkanImageViewDepthCreateFail =>
                    ("vulkan image view depth create fail", true),
                ErrorFoundationApplicationGuideOwn::ApplicationSceneVulkanPipelineAlreadyAdded =>
                    ("application scene vulkan pipeline already added", true),
            };
        println!("{error_message}");
        if be_bypass { Some(error) } else { None }
    }

    fn handle_entry_guide_own(error: ErrorEntryGuideOwn)
    -> Option<ErrorEntryGuideOwn>
    {
        let (error_message, be_bypass) =
            match &error {
                ErrorEntryGuideOwn::EntryArgumentParseFail(message) =>
                    (message.as_str(), false),
            };
        println!("{error_message}");
        if be_bypass { Some(error) } else { None }
    }

    pub fn handle(error: ErrorEntryGuide) -> Option<ErrorEntryGuide> {
        match error {
            ErrorEntryGuide::Own(err) =>
                Self::handle_entry_guide_own(err).map(|e| e.into()),
            ErrorEntryGuide::ScopeApplicationGuide(err) =>
                Self::handle_application_guide_own(err).map(|e| e.into()),
            ErrorEntryGuide::ScopeVulkanCooked(err) =>
                Self::handle_vulkan_cooked_own(err).map(|e| e.into()),
            ErrorEntryGuide::ScopeReintroduction(err) =>
                Self::handle_reintroduction_own(err).map(|e| e.into()),
        }
    }
}
