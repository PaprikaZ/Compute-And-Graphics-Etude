pub use ::vulkanalia::loader::LIBRARY as VULKAN_LIBRARY_FILE_NAME;
pub use ::vulkanalia::loader::LibloadingLoader as VulkanLibraryLoader;
pub use ::vulkanalia::window;
pub use ::vulkanalia::vk;
pub use ::vulkanalia::vk::Handle as VulkanHandler;
pub use ::vulkanalia::vk::HasBuilder as VulkanBuilderHas;
pub use ::vulkanalia::vk::ApplicationInfo as VulkanApplicationInfomation;
pub use ::vulkanalia::vk::InstanceCreateInfo as VulkanInstanceCreateInformation;

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