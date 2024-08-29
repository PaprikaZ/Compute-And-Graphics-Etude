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