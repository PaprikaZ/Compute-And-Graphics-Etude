#[derive(Clone, Copy, Debug)]
pub struct VulkanMipmapLevel(u32);

impl VulkanMipmapLevel {
    pub fn new(level: u32) -> Self {
        Self(level)
    }

    pub fn as_raw(&self) -> u32 {
        self.0
    }
}