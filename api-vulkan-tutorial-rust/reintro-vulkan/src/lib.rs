pub use ::vulkanalia::loader::LIBRARY as VULKAN_LIBRARY_FILE_NAME;
pub use ::vulkanalia::loader::LibloadingLoader as VulkanLibraryLoader;
pub use ::vulkanalia::window;
pub use ::vulkanalia::vk;
pub use ::vulkanalia::vk::Bool32 as VulkanBool32;
pub use ::vulkanalia::vk::Handle as VulkanHandler;
pub use ::vulkanalia::vk::HasBuilder as VulkanBuilderHas;
pub use ::vulkanalia::vk::ApplicationInfo as VulkanApplicationInfomation;
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
pub use ::vulkanalia::vk::QueueFlags as VulkanQueueFlagS;

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
        pub use ::vulkanalia::Device as VulkanDevice;
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


pub struct VulkanErrorCode(i32);

impl VulkanErrorCode {
    pub fn new(code: i32) -> Self {
        Self(code)
    }
}