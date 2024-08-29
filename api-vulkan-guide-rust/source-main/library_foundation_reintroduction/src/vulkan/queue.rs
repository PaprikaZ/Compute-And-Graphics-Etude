#[derive(Clone, Copy, Debug)]
pub struct VulkanQueueFamilyIndexGraphic(u32);

impl VulkanQueueFamilyIndexGraphic {
    pub fn new(queue_index: u32) -> Self {
        Self(queue_index)
    }

    pub fn as_raw(&self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy, Debug)]
pub struct VulkanQueueFamilyIndexSurface(u32);

impl VulkanQueueFamilyIndexSurface {
    pub fn new(queue_index: u32) -> Self {
        Self(queue_index)
    }

    pub fn as_raw(&self) -> u32 {
        self.0
    }
}