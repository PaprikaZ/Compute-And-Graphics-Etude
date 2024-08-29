#[derive(Clone, Copy, Debug)]
pub struct VulkanMemoryTypeIndex(u32);

impl VulkanMemoryTypeIndex {
    pub fn new(index: u32) -> Self {
        Self(index)
    }

    pub fn as_raw(&self) -> u32 {
        self.0
    }
}