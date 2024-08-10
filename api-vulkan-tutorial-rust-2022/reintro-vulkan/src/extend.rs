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

#[derive(Clone, Copy)]
pub struct VulkanMipLevel(u32);

impl VulkanMipLevel {
    pub fn new(level: u32) -> Self {
        Self(level)
    }

    pub fn as_raw(&self) -> u32 {
        self.0
    }
}