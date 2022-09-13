pub use ::vulkanalia::loader::LIBRARY as VULKAN_LIBRARY_FILE_NAME;
pub use ::vulkanalia::loader::LibloadingLoader as VulkanLibraryLoader;
pub use ::vulkanalia::window as VulkanWindow;
pub use ::vulkanalia::vk;
pub use ::vulkanalia::vk::Bool32 as VulkanBool32;
pub use ::vulkanalia::vk::Handle as VulkanHandler;
pub use ::vulkanalia::vk::HasBuilder as VulkanBuilderHas;
pub use ::vulkanalia::vk::ErrorCode as VulkanErrorCode_;
pub use ::vulkanalia::vk::SuccessCode as VulkanSuccessCode_;
pub use ::vulkanalia::vk::ApplicationInfo as VulkanApplicationInformation;
pub use ::vulkanalia::vk::ApplicationInfoBuilder as VulkanApplicationInformationBuilder;
pub use ::vulkanalia::vk::InstanceCreateInfo as VulkanInstanceCreateInformation;
pub use ::vulkanalia::vk::ExtensionName as VulkanExtensionName;
pub use ::vulkanalia::vk::ExtDebugUtilsExtension as VulkanExtensionDebugUtility;
pub use ::vulkanalia::vk::EXT_DEBUG_UTILS_EXTENSION as VULKAN_EXTENSION_DEBUG_UTILITY;
pub use ::vulkanalia::vk::DebugUtilsMessengerEXT as VulkanExtensionDebugUtilityMessenger;
pub use ::vulkanalia::vk::DebugUtilsMessengerCreateInfoEXT as VulkanExtensionDebugUtilityMessengerCreateInformation;
pub use ::vulkanalia::vk::DebugUtilsMessengerCreateInfoEXTBuilder as VulkanExtensionDebugUtilityMessengerCreateInformationBuilder;
pub use ::vulkanalia::vk::DebugUtilsMessageSeverityFlagsEXT as VulkanExtensionDebugUtilityMessageSeverityFlagS;
pub use ::vulkanalia::vk::DebugUtilsMessageTypeFlagsEXT as VulkanExtensionDebugUtilityMessageTypeFlagS;
pub use ::vulkanalia::vk::DebugUtilsMessengerCallbackDataEXT as VulkanExtensionDebugUtilityMessengerCallbackData;
pub use ::vulkanalia::vk::PhysicalDevice as VulkanDevicePhysical;
pub use ::vulkanalia::vk::Queue as VulkanQueue;
pub use ::vulkanalia::vk::QueueFlags as VulkanQueueFlagS;
pub use ::vulkanalia::vk::DeviceCreateInfo as VulkanDeviceLogicalCreateInformation;
pub use ::vulkanalia::vk::DeviceCreateInfoBuilder as VulkanDeviceLogicalCreateInformationBuilder;
pub use ::vulkanalia::vk::DeviceQueueCreateInfo as VulkanDeviceLogicalQueueCreateInformation;
pub use ::vulkanalia::vk::DeviceQueueCreateInfoBuilder as VulkanDeviceLogicalQueueCreateInformationBuilder;
pub use ::vulkanalia::vk::PhysicalDeviceFeatures as VulkanDevicePhysicalFeatureS;
pub use ::vulkanalia::vk::PhysicalDeviceFeaturesBuilder as VulkanDevicePhysicalFeatureSBuilder;

pub use ::vulkanalia::vk::KhrSurfaceExtension as VulkanSurfaceExtensionKhr;
pub use ::vulkanalia::vk::SurfaceKHR as VulkanSurfaceKhr;
pub use ::vulkanalia::vk::Win32SurfaceCreateInfoKHR as VulkanSurfaceCreateInformationKhr;
pub use ::vulkanalia::vk::Win32SurfaceCreateInfoKHRBuilder as VulkanSurfaceCreateInformationBuilderKhr;
pub use ::vulkanalia::vk::KhrWin32SurfaceExtension as VulkanSurfaceExtensionWin32;
pub use ::vulkanalia::vk::SurfaceCapabilitiesKHR as VulkanSurfaceCapabilitySKhr;
pub use ::vulkanalia::vk::SurfaceFormatKHR as VulkanSurfaceFormatKhr;
pub use ::vulkanalia::vk::PresentModeKHR as VulkanPresentModeKhr;

pub use ::vulkanalia::vk::KhrSwapchainExtension as VulkanSwapchainExtensionKhr;
pub use ::vulkanalia::vk::SwapchainKHR as VulkanSwapchainKhr;
pub use ::vulkanalia::vk::Format as VulkanFormat;
pub use ::vulkanalia::vk::ColorSpaceKHR as VulkanColorSpaceKhr;
pub use ::vulkanalia::vk::Extent2D as VulkanExtentD2;
pub use ::vulkanalia::vk::Extent3D as VulkanExtentD3;
pub use ::vulkanalia::vk::Image as VulkanImage;
pub use ::vulkanalia::vk::SharingMode as VulkanSharingMode;
pub use ::vulkanalia::vk::SwapchainCreateInfoKHR as VulkanSwapchainCreateInformationKhr;
pub use ::vulkanalia::vk::SwapchainCreateInfoKHRBuilder as VulkanSwapchainCreateInformationBuilderKhr;
pub use ::vulkanalia::vk::ImageUsageFlags as VulkanImageUsageFlagS;
pub use ::vulkanalia::vk::CompositeAlphaFlagsKHR as VulkanCompositeAlphaFlagSKhr;

pub use ::vulkanalia::vk::ImageView as VulkanImageView;
pub use ::vulkanalia::vk::ComponentMapping as VulkanComponentMapping;
pub use ::vulkanalia::vk::ComponentMappingBuilder as VulkanComponentMappingBuilder;
pub use ::vulkanalia::vk::ComponentSwizzle as VulkanComponentSwizzle;
pub use ::vulkanalia::vk::ImageSubresourceRange as VulkanImageSubResourceRange;
pub use ::vulkanalia::vk::ImageSubresourceRangeBuilder as VulkanImageSubResourceRangeBuilder;
pub use ::vulkanalia::vk::ImageAspectFlags as VulkanImageAspectFlagS;
pub use ::vulkanalia::vk::ImageViewCreateInfo as VulkanImageViewCreateInformation;
pub use ::vulkanalia::vk::ImageViewType as VulkanImageViewType;

pub use ::vulkanalia::vk::ShaderModule as VulkanShaderModule;
pub use ::vulkanalia::vk::ShaderModuleCreateInfo as VulkanShaderModuleCreateInformation;
pub use ::vulkanalia::vk::ShaderModuleCreateInfoBuilder as VulkanShaderModuleCreateInformationBuilder;
pub use ::vulkanalia::vk::PipelineShaderStageCreateInfo as VulkanPipelineShaderStageCreateInformation;
pub use ::vulkanalia::vk::PipelineShaderStageCreateInfoBuilder as VulkanPipelineShaderStageCreateInformationBuilder;
pub use ::vulkanalia::vk::ShaderStageFlags as VulkanShaderStageFlagS;

pub use ::vulkanalia::vk::PipelineVertexInputStateCreateInfo as VulkanPipelineVertexInputStateCreateInformation;
pub use ::vulkanalia::vk::PipelineVertexInputStateCreateInfoBuilder as VulkanPipelineVertexInputStateCreateInformationBuilder;
pub use ::vulkanalia::vk::PipelineInputAssemblyStateCreateInfo as VulkanPipelineInputAssemblyStateCreateInformation;
pub use ::vulkanalia::vk::PipelineInputAssemblyStateCreateInfoBuilder as VulkanPipelineInputAssemblyStateCreateInformationBuilder;
pub use ::vulkanalia::vk::PrimitiveTopology as VulkanPrimitiveTopology;
pub use ::vulkanalia::vk::Viewport as VulkanViewport;
pub use ::vulkanalia::vk::ViewportBuilder as VulkanViewportBuilder;
pub use ::vulkanalia::vk::Rect2D as VulkanRectangleD2;
pub use ::vulkanalia::vk::Rect2DBuilder as VulkanRectangleD2Builder;
pub use ::vulkanalia::vk::Offset2D as VulkanOffsetD2;
pub use ::vulkanalia::vk::Offset3D as VulkanOffsetD3;
pub use ::vulkanalia::vk::PipelineViewportStateCreateInfo as VulkanPipelineViewportStateCreateInformation;
pub use ::vulkanalia::vk::PipelineViewportStateCreateInfoBuilder as VulkanPipelineViewportStateCreateInformationBuilder;
pub use ::vulkanalia::vk::PipelineRasterizationStateCreateInfo as VulkanPipelineRasterizationStateCreateInformation;
pub use ::vulkanalia::vk::PipelineRasterizationStateCreateInfoBuilder as VulkanPipelineRasterizationStateCreateInformationBuilder;
pub use ::vulkanalia::vk::PolygonMode as VulkanPolygonMode;
pub use ::vulkanalia::vk::CullModeFlags as VulkanCullModeFlagS;
pub use ::vulkanalia::vk::FrontFace as VulkanFrontFace;
pub use ::vulkanalia::vk::PipelineMultisampleStateCreateInfo as VulkanPipelineMultisampleStateCreateInformation;
pub use ::vulkanalia::vk::PipelineMultisampleStateCreateInfoBuilder as VulkanPipelineMultisampleStateCreateInformationBuilder;
pub use ::vulkanalia::vk::SampleCountFlags as VulkanSampleCountFlagS;
pub use ::vulkanalia::vk::PipelineDepthStencilStateCreateInfo as VulkanPipelineDepthStencilStateCreateInformation;
pub use ::vulkanalia::vk::PipelineColorBlendAttachmentState as VulkanPipelineColorBlendAttachmentState;
pub use ::vulkanalia::vk::PipelineColorBlendStateCreateInfo as VulkanPipelineColorBlendStateCreateInformation;
pub use ::vulkanalia::vk::PipelineColorBlendStateCreateInfoBuilder as VulkanPipelineColorBlendStateCreateInformationBuilder;
pub use ::vulkanalia::vk::ColorComponentFlags as VulkanColorComponentFlagS;
pub use ::vulkanalia::vk::BlendFactor as VulkanBlendFactor;
pub use ::vulkanalia::vk::BlendOp as VulkanBlendOperation;
pub use ::vulkanalia::vk::LogicOp as VulkanLogicOperation;
pub use ::vulkanalia::vk::PipelineDynamicStateCreateInfo as VulkanPipelineDynamicStateCreateInformation;
pub use ::vulkanalia::vk::DynamicState as VulkanDynamicState;
pub use ::vulkanalia::vk::PipelineLayout as VulkanPipelineLayout;
pub use ::vulkanalia::vk::PipelineLayoutCreateInfo as VulkanPipelineLayoutCreateInformation;
pub use ::vulkanalia::vk::PipelineLayoutCreateInfoBuilder as VulkanPipelineLayoutCreateInformationBuilder;

pub use ::vulkanalia::vk::AttachmentDescription as VulkanAttachmentDescription;
pub use ::vulkanalia::vk::AttachmentLoadOp as VulkanAttachmentLoadOperation;
pub use ::vulkanalia::vk::AttachmentStoreOp as VulkanAttachmentStoreOperation;
pub use ::vulkanalia::vk::ImageLayout as VulkanImageLayout;
pub use ::vulkanalia::vk::AttachmentReference as VulkanAttachmentReference;
pub use ::vulkanalia::vk::SubpassDescription as VulkanSubpassDescription;
pub use ::vulkanalia::vk::PipelineBindPoint as VulkanPipelineBindPoint;
pub use ::vulkanalia::vk::RenderPass as VulkanRenderPass;
pub use ::vulkanalia::vk::RenderPassCreateInfo as VulkanRenderPassCreateInformation;
pub use ::vulkanalia::vk::RenderPassCreateInfoBuilder as VulkanRenderPassCreateInformationBuilder;

pub use ::vulkanalia::vk::GraphicsPipelineCreateInfo as VulkanGraphicsPipelineCreateInformation;
pub use ::vulkanalia::vk::GraphicsPipelineCreateInfoBuilder as VulkanGraphicsPipelineCreateInformationBuilder;
pub use ::vulkanalia::vk::PipelineCreateFlags as VulkanPipelineCreateFlagS;
pub use ::vulkanalia::vk::PipelineCache as VulkanPipelineCache;
pub use ::vulkanalia::vk::Pipeline as VulkanPipeline;

pub use ::vulkanalia::vk::Framebuffer as VulkanFrameBuffer;
pub use ::vulkanalia::vk::FramebufferCreateInfo as VulkanFrameBufferCreateInformation;
pub use ::vulkanalia::vk::FramebufferCreateInfoBuilder as VulkanFrameBufferCreateInformationBuilder;

pub use ::vulkanalia::vk::CommandPool as VulkanCommandPool;
pub use ::vulkanalia::vk::CommandPoolCreateFlags as VulkanCommandPoolCreateFlagS;
pub use ::vulkanalia::vk::CommandBuffer as VulkanCommandBuffer;
pub use ::vulkanalia::vk::CommandBufferAllocateInfo as VulkanCommandBufferAllocateInformation;
pub use ::vulkanalia::vk::CommandBufferAllocateInfoBuilder as VulkanCommandBufferAllocateInformationBuilder;
pub use ::vulkanalia::vk::CommandBufferLevel as VulkanCommandBufferLevel;
pub use ::vulkanalia::vk::CommandBufferBeginInfo as VulkanCommandBufferBeginInformation;
pub use ::vulkanalia::vk::CommandBufferInheritanceInfo as VulkanCommandBufferInheritanceInformation;
pub use ::vulkanalia::vk::CommandBufferInheritanceInfoBuilder as VulkanCommandBufferInheritanceInformationBuilder;
pub use ::vulkanalia::vk::CommandBufferUsageFlags as VulkanCommandBufferUsageFlagS;
pub use ::vulkanalia::vk::ClearValue as VulkanClearValue;
pub use ::vulkanalia::vk::ClearColorValue as VulkanClearColorValue;
pub use ::vulkanalia::vk::RenderPassBeginInfo as VulkanRenderPassBeginInformation;
pub use ::vulkanalia::vk::SubpassContents as VulkanSubpassContents;
pub use ::vulkanalia::vk::CommandPoolCreateInfo as VulkanCommandPoolCreateInformation;
pub use ::vulkanalia::vk::CommandPoolCreateInfoBuilder as VulkanCommandPoolCreateInformationBuilder;

pub use ::vulkanalia::vk::Semaphore as VulkanSemaphore;
pub use ::vulkanalia::vk::SemaphoreCreateInfo as VulkanSemaphoreCreateInformation;
pub use ::vulkanalia::vk::SemaphoreCreateInfoBuilder as VulkanSemaphoreCreateInformationBuilder;
pub use ::vulkanalia::vk::PipelineStageFlags as VulkanPipelineStageFlagS;
pub use ::vulkanalia::vk::SubmitInfo as VulkanSubmitInformation;
pub use ::vulkanalia::vk::Fence as VulkanFence;
pub use ::vulkanalia::vk::FenceCreateInfo as VulkanFenceCreateInformation;
pub use ::vulkanalia::vk::FenceCreateInfoBuilder as VulkanFenceCreateInformationBuilder;
pub use ::vulkanalia::vk::SubpassDependency as VulkanSubpassDependency;
pub use ::vulkanalia::vk::AccessFlags as VulkanAccessFlagS;
pub use ::vulkanalia::vk::PresentInfoKHR as VulkanPresentInformationKhr;
pub use ::vulkanalia::vk::FenceCreateFlags as VulkanFenceCreateFlagS;

pub use ::vulkanalia::vk::VertexInputBindingDescription as VulkanVertexInputBindingDescription;
pub use ::vulkanalia::vk::VertexInputRate as VulkanVertexInputRate;
pub use ::vulkanalia::vk::VertexInputAttributeDescription as VulkanVertexInputAttributeDescription;
pub use ::vulkanalia::vk::PipelineVertexInputStateCreateInfo as VulkanVertexInputStateCreateInformation;
pub use ::vulkanalia::vk::PipelineVertexInputStateCreateInfoBuilder as VulkanVertexInputStateCreateInformationBuilder;

pub use ::vulkanalia::vk::BufferCreateInfo as VulkanBufferCreateInformation;
pub use ::vulkanalia::vk::BufferCreateInfoBuilder as VulkanBufferCreateInformationBuilder;
pub use ::vulkanalia::vk::BufferUsageFlags as VulkanBufferUsageFlagS;
pub use ::vulkanalia::vk::MemoryType as VulkanMemoryType;
pub use ::vulkanalia::vk::MemoryPropertyFlags as VulkanMemoryPropertyFlagS;
pub use ::vulkanalia::vk::MemoryAllocateInfo as VulkanMemoryAllocateInformation;
pub use ::vulkanalia::vk::MemoryMapFlags as VulkanMemoryMapFlagS;
pub use ::vulkanalia::vk::MemoryRequirements as VulkanMemoryRequirementS;

pub use ::vulkanalia::vk::Buffer as VulkanBuffer;
pub use ::vulkanalia::vk::DeviceMemory as VulkanDeviceMemory;

pub use ::vulkanalia::vk::BufferCopy as VulkanBufferCopy;
pub use ::vulkanalia::vk::DeviceSize as VulkanDeviceSize;

pub use ::vulkanalia::vk::IndexType as VulkanIndexType;

pub use ::vulkanalia::vk::DescriptorSetLayout as VulkanDescriptorSetLayout;
pub use ::vulkanalia::vk::DescriptorSetLayoutBinding as VulkanDescriptorSetLayoutBinding;
pub use ::vulkanalia::vk::DescriptorType as VulkanDescriptorType;
pub use ::vulkanalia::vk::DescriptorSetLayoutCreateInfo as VulkanDescriptorSetLayoutCreateInformation;
pub use ::vulkanalia::vk::DescriptorSetLayoutCreateInfoBuilder as VulkanDescriptorSetLayoutCreateInformationBuilder;

pub use ::vulkanalia::vk::DescriptorPoolSize as VulkanDescriptorPoolSize;
pub use ::vulkanalia::vk::DescriptorPoolSizeBuilder as VulkanDescriptorPoolSizeBuilder;
pub use ::vulkanalia::vk::DescriptorPool as VulkanDescriptorPool;
pub use ::vulkanalia::vk::DescriptorPoolCreateInfo as VulkanDescriptorPoolCreateInformation;
pub use ::vulkanalia::vk::DescriptorSetAllocateInfo as VulkanDescriptorSetAllocateInformation;
pub use ::vulkanalia::vk::DescriptorSetAllocateInfoBuilder as VulkanDescriptorSetAllocateInformationBuilder;
pub use ::vulkanalia::vk::DescriptorSet as VulkanDescriptorSet;
pub use ::vulkanalia::vk::DescriptorBufferInfo as VulkanDescriptorBufferInformation;
pub use ::vulkanalia::vk::WriteDescriptorSet as VulkanWriteDescriptorSet;
pub use ::vulkanalia::vk::CopyDescriptorSet as VulkanCopyDescriptorSet;

pub use ::vulkanalia::vk::ImageCreateInfo as VulkanImageCreateInformation;
pub use ::vulkanalia::vk::ImageCreateInfoBuilder as VulkanImageCreateInformationBuilder;
pub use ::vulkanalia::vk::ImageType as VulkanImageType;
pub use ::vulkanalia::vk::ImageTiling as VulkanImageTiling;
pub use ::vulkanalia::vk::DependencyFlags as VulkanDependencyFlagS;
pub use ::vulkanalia::vk::BufferImageCopy as VulkanBufferImageCopy;
pub use ::vulkanalia::vk::ImageSubresourceLayers as VulkanImageSubresourceLayerS;
pub use ::vulkanalia::vk::ImageMemoryBarrier as VulkanImageMemoryBarrier;
pub use ::vulkanalia::vk::MemoryBarrier as VulkanMemoryBarrier;
pub use ::vulkanalia::vk::BufferMemoryBarrier as VulkanBufferMemoryBarrier;

pub use ::vulkanalia::vk::ATTACHMENT_UNUSED as VULKAN_ATTACHMENT_UNUSED;
pub use ::vulkanalia::vk::FALSE as VULKAN_FALSE;
pub use ::vulkanalia::vk::LOD_CLAMP_NONE as VULKAN_LOD_CLAMP_NONE;
pub use ::vulkanalia::vk::LUID_SIZE as VULKAN_LUID_SIZE;
pub use ::vulkanalia::vk::MAX_DESCRIPTION_SIZE as VULKAN_MAX_DESCRIPTION_SIZE;
pub use ::vulkanalia::vk::MAX_DEVICE_GROUP_SIZE as VULKAN_MAX_DEVICE_GROUP_SIZE;
pub use ::vulkanalia::vk::MAX_DRIVER_INFO_SIZE as VULKAN_MAX_DRIVER_INFO_SIZE;
pub use ::vulkanalia::vk::MAX_DRIVER_NAME_SIZE as VULKAN_MAX_DRIVER_NAME_SIZE;
pub use ::vulkanalia::vk::MAX_EXTENSION_NAME_SIZE as VULKAN_MAX_EXTENSION_NAME_SIZE;
pub use ::vulkanalia::vk::MAX_GLOBAL_PRIORITY_SIZE_EXT as VULKAN_MAX_GLOBAL_PRIORITY_SIZE_EXT;
pub use ::vulkanalia::vk::MAX_MEMORY_HEAPS as VULKAN_MAX_MEMORY_HEAPS;
pub use ::vulkanalia::vk::MAX_MEMORY_TYPES as VULKAN_MAX_MEMORY_TYPES;
pub use ::vulkanalia::vk::MAX_PHYSICAL_DEVICE_NAME_SIZE as VULKAN_MAX_PHYSICAL_DEVICE_NAME_SIZE;
pub use ::vulkanalia::vk::QUEUE_FAMILY_EXTERNAL as VULKAN_QUEUE_FAMILY_EXTERNAL;
pub use ::vulkanalia::vk::QUEUE_FAMILY_FOREIGN_EXT as VULKAN_QUEUE_FAMILY_FOREIGN_EXT;
pub use ::vulkanalia::vk::QUEUE_FAMILY_IGNORED as VULKAN_QUEUE_FAMILY_IGNORED;
pub use ::vulkanalia::vk::REMAINING_ARRAY_LAYERS as VULKAN_REMAINING_ARRAY_LAYERS;
pub use ::vulkanalia::vk::REMAINING_MIP_LEVELS as VULKAN_REMAINING_MIP_LEVELS;
pub use ::vulkanalia::vk::SHADER_UNUSED_KHR as VULKAN_SHADER_UNUSED_KHR;
pub use ::vulkanalia::vk::SUBPASS_EXTERNAL as VULKAN_SUBPASS_EXTERNAL;
pub use ::vulkanalia::vk::TRUE as VULKAN_TRUE;
pub use ::vulkanalia::vk::UUID_SIZE as VULKAN_UUID_SIZE;
pub use ::vulkanalia::vk::WHOLE_SIZE as VULKAN_WHOLE_SIZE;
pub use ::vulkanalia::vk::KHR_SWAPCHAIN_EXTENSION as VULKAN_SWAPCHAIN_EXTENSION_KHR;

pub use ::vulkanalia::Device as VulkanDevice;
pub use ::vulkanalia::Entry as VulkanEntry;
pub use ::vulkanalia::Instance as VulkanInstance;
pub use ::vulkanalia::VkResult as VulkanResult;
pub use ::vulkanalia::VkSuccessResult as VulkanResultSuccess;
pub use ::vulkanalia::vk::DeviceV1_0 as VulkanDeviceVersion1_0;
pub use ::vulkanalia::vk::EntryV1_0 as VulkanEntryVersion1_0;
pub use ::vulkanalia::vk::InstanceV1_0 as VulkanInstanceVersion1_0;
pub use ::vulkanalia::vk::DeviceV1_1 as VulkanDeviceVersion1_1;
pub use ::vulkanalia::vk::EntryV1_1 as VulkanEntryVersion1_1;
pub use ::vulkanalia::vk::InstanceV1_1 as VulkanInstanceVersion1_1;
pub use ::vulkanalia::vk::DeviceV1_2 as VulkanDeviceVersion1_2;
pub use ::vulkanalia::vk::EntryV1_2 as VulkanEntryVersion1_2;
pub use ::vulkanalia::vk::InstanceV1_2 as VulkanInstanceVersion1_2;

pub mod prelude {
    pub mod version1_0 {
        pub use ::vulkanalia::vk::Handle as VulkanHandler;
        pub use ::vulkanalia::vk::HasBuilder as VulkanBuilderHas;
        pub use ::vulkanalia::Device as VulkanDeviceLogical;
        pub use ::vulkanalia::Entry as VulkanEntry;
        pub use ::vulkanalia::Instance as VulkanInstance;
        pub use ::vulkanalia::VkResult as VulkanResult;
        pub use ::vulkanalia::VkSuccessResult as VulkanResultSuccess;
        pub use ::vulkanalia::vk::DeviceV1_0 as VulkanDeviceVersion1_0;
        pub use ::vulkanalia::vk::EntryV1_0 as VulkanEntryVersion1_0;
        pub use ::vulkanalia::vk::InstanceV1_0 as VulkanInstanceVersion1_0;
    }

    pub mod version1_1 {
        pub use crate::prelude::version1_0::*;
        pub use ::vulkanalia::vk::DeviceV1_1 as VulkanDeviceVersion1_1;
        pub use ::vulkanalia::vk::EntryV1_1 as VulkanEntryVersion1_1;
        pub use ::vulkanalia::vk::InstanceV1_1 as VulkanInstanceVersion1_1;
    }

    pub mod version1_2 {
        pub use crate::prelude::version1_0::*;
        pub use ::vulkanalia::vk::DeviceV1_2 as VulkanDeviceVersion1_2;
        pub use ::vulkanalia::vk::EntryV1_2 as VulkanEntryVersion1_2;
        pub use ::vulkanalia::vk::InstanceV1_2 as VulkanInstanceVersion1_2;
    }
}


#[derive(Clone, Copy)]
pub struct VulkanErrorCode(i32);

impl VulkanErrorCode {
    pub fn new(code: i32) -> Self {
        Self(code)
    }
}

#[derive(Clone, Copy)]
pub struct VulkanQueueFamilyIndexGraphic(u32);

impl VulkanQueueFamilyIndexGraphic {
    pub fn new(queue_index: u32) -> Self {
        Self(queue_index)
    }

    pub fn as_raw(&self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy)]
pub struct VulkanQueueFamilyIndexSurface(u32);

impl VulkanQueueFamilyIndexSurface {
    pub fn new(queue_index: u32) -> Self {
        Self(queue_index)
    }

    pub fn as_raw(&self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy)]
pub struct VulkanSwapchainImageCount(u32);

impl VulkanSwapchainImageCount {
    pub fn new(image_count: u32) -> Self {
        Self(image_count)
    }

    pub fn as_raw(&self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy)]
pub struct VulkanMemoryTypeIndex(u32);

impl VulkanMemoryTypeIndex {
    pub fn new(index: u32) -> Self {
        Self(index)
    }

    pub fn as_raw(&self) -> u32 {
        self.0
    }
}