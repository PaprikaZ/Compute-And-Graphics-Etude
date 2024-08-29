#[derive(Debug, Clone, Copy)]
pub struct VulkanSwapchainImageNumber(u32);

impl VulkanSwapchainImageNumber {
    pub fn new(image_count: u32) -> Self {
        Self(image_count)
    }

    pub fn as_raw(self) -> u32 {
        self.0
    }
}


#[derive(Debug, Clone, Copy)]
pub struct VulkanSwapchainImageIndex(u32);

impl VulkanSwapchainImageIndex {
    pub fn new(image_index: u32) -> Self {
        Self(image_index)
    }

    pub fn as_raw(self) -> u32 {
        self.0
    }
}