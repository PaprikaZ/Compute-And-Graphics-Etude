pub mod version1_0 {
    pub use crate::vulkan::self_::VulkanHandler;
    pub use crate::vulkan::self_::VulkanBuilderHas;
    pub use crate::vulkan::self_::VulkanDeviceLogical;
    pub use crate::vulkan::self_::VulkanEntry;
    pub use crate::vulkan::self_::VulkanInstance;
    pub use crate::vulkan::self_::VulkanResult;
    pub use crate::vulkan::self_::VulkanResultSuccess;
    pub use crate::vulkan::self_::VulkanDeviceVersion1_0;
    pub use crate::vulkan::self_::VulkanEntryVersion1_0;
    pub use crate::vulkan::self_::VulkanInstanceVersion1_0;
}

pub mod version1_1 {
    pub use super::version1_0::*;
    pub use crate::vulkan::self_::VulkanDeviceVersion1_1;
    pub use crate::vulkan::self_::VulkanEntryVersion1_1;
    pub use crate::vulkan::self_::VulkanInstanceVersion1_1;
}

pub mod version1_2 {
    pub use super::version1_0::*;
    pub use crate::vulkan::self_::VulkanDeviceVersion1_2;
    pub use crate::vulkan::self_::VulkanEntryVersion1_2;
    pub use crate::vulkan::self_::VulkanInstanceVersion1_2;
}

pub mod version1_3 {
    pub use super::version1_0::*;
    pub use crate::vulkan::self_::VulkanDeviceVersion1_3;
    pub use crate::vulkan::self_::VulkanEntryVersion1_3;
    pub use crate::vulkan::self_::VulkanInstanceVersion1_3;
}