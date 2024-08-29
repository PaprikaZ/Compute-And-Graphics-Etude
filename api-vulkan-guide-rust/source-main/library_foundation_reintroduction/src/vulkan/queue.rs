#[derive(Debug, Clone, Copy)]
pub struct VulkanQueueFamilyIndex(u32);

impl VulkanQueueFamilyIndex {
    pub fn new(queue_index: u32) -> Self {
        Self(queue_index)
    }

    pub fn as_raw(&self) -> u32 {
        self.0
    }
}


#[derive(Debug, Clone, Copy)]
pub struct VulkanQueueFamilyIndexGraphic(u32);

impl VulkanQueueFamilyIndexGraphic {
    pub fn new(queue_index: u32) -> Self {
        Self(queue_index)
    }

    pub fn as_raw(&self) -> u32 {
        self.0
    }
}


#[derive(Debug, Clone, Copy)]
pub struct VulkanQueueFamilyIndexPresent(u32);

impl VulkanQueueFamilyIndexPresent {
    pub fn new(queue_index: u32) -> Self {
        Self(queue_index)
    }

    pub fn as_raw(&self) -> u32 {
        self.0
    }
}


#[derive(Debug, Clone, Copy)]
pub struct VulkanQueueFamilyIndexCompute(u32);

impl VulkanQueueFamilyIndexCompute {
    pub fn new(queue_index: u32) -> Self {
        Self(queue_index)
    }

    pub fn as_raw(&self) -> u32 {
        self.0
    }
}


#[derive(Debug, Clone, Copy)]
pub struct VulkanQueueFamilyIndexTransfer(u32);

impl VulkanQueueFamilyIndexTransfer {
    pub fn new(queue_index: u32) -> Self {
        Self(queue_index)
    }

    pub fn as_raw(&self) -> u32 {
        self.0
    }
}

impl From<VulkanQueueFamilyIndexGraphic> for VulkanQueueFamilyIndex {
    fn from(index: VulkanQueueFamilyIndexGraphic) -> Self {
        Self::new(index.as_raw())
    }
}

impl From<VulkanQueueFamilyIndexCompute> for VulkanQueueFamilyIndex {
    fn from(index: VulkanQueueFamilyIndexCompute) -> Self {
        Self::new(index.as_raw())
    }
}

impl From<VulkanQueueFamilyIndexPresent> for VulkanQueueFamilyIndex {
    fn from(index: VulkanQueueFamilyIndexPresent) -> Self {
        Self::new(index.as_raw())
    }
}

impl From<VulkanQueueFamilyIndexTransfer> for VulkanQueueFamilyIndex {
    fn from(index: VulkanQueueFamilyIndexTransfer) -> Self {
        Self::new(index.as_raw())
    }
}

impl From<VulkanQueueFamilyIndex> for VulkanQueueFamilyIndexGraphic {
    fn from(index: VulkanQueueFamilyIndex) -> Self {
        Self::new(index.as_raw())
    }
}

impl From<VulkanQueueFamilyIndex> for VulkanQueueFamilyIndexCompute {
    fn from(index: VulkanQueueFamilyIndex) -> Self {
        Self::new(index.as_raw())
    }
}

impl From<VulkanQueueFamilyIndex> for VulkanQueueFamilyIndexPresent {
    fn from(index: VulkanQueueFamilyIndex) -> Self {
        Self::new(index.as_raw())
    }
}

impl From<VulkanQueueFamilyIndex> for VulkanQueueFamilyIndexTransfer {
    fn from(index: VulkanQueueFamilyIndex) -> Self {
        Self::new(index.as_raw())
    }
}
