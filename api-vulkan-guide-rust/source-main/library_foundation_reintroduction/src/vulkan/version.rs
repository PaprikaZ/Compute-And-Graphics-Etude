use std::cmp::PartialOrd;

use crate::vulkan::make_version;
use crate::vulkan::version_major;
use crate::vulkan::version_minor;
use crate::vulkan::version_patch;
use crate::vulkan::VulkanVersionVanilla;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VulkanVersionEntry(u32);

impl VulkanVersionEntry {
    pub const fn new(major_version: u32, minor_version: u32, patch_version: u32) -> Self {
        Self(make_version(major_version, minor_version, patch_version))
    }

    pub fn as_raw(self) -> u32 {
        self.0
    }

    pub fn get_major(&self) -> u32 {
        version_major(self.0)
    }

    pub fn get_minor(&self) -> u32 {
        version_minor(self.0)
    }

    pub fn get_patch(&self) -> u32 {
        version_patch(self.0)
    }
}

impl PartialOrd<Self> for VulkanVersionEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl From<VulkanVersionVanilla> for VulkanVersionEntry {
    fn from(version: VulkanVersionVanilla) -> Self {
        Self::new(version.major, version.minor, version.patch)
    }
}

impl From<VulkanVersionEntry> for VulkanVersionVanilla {
    fn from(version: VulkanVersionEntry) -> Self {
        Self::new(version.get_major(), version.get_minor(), version.get_patch())
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VulkanVersionApplication(u32);

impl VulkanVersionApplication {
    pub fn new(major_version: u32, minor_version: u32, patch_version: u32) -> Self {
        Self(make_version(major_version, minor_version, patch_version))
    }

    pub fn unwrap(self) -> u32 {
        self.0
    }

    pub fn get_major(&self) -> u32 {
        version_major(self.0)
    }

    pub fn get_minor(&self) -> u32 {
        version_minor(self.0)
    }

    pub fn get_patch(&self) -> u32 {
        version_patch(self.0)
    }
}

impl PartialOrd<Self> for VulkanVersionApplication {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl From<VulkanVersionVanilla> for VulkanVersionApplication {
    fn from(version: VulkanVersionVanilla) -> Self {
        Self::new(version.major, version.minor, version.patch)
    }
}

impl From<VulkanVersionApplication> for VulkanVersionVanilla {
    fn from(version: VulkanVersionApplication) -> Self {
        Self::new(version.get_major(), version.get_minor(), version.get_patch())
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VulkanVersionEngine(u32);

impl VulkanVersionEngine {
    pub fn new(major_version: u32, minor_version: u32, patch_version: u32) -> Self {
        Self(make_version(major_version, minor_version, patch_version))
    }

    pub fn unwrap(self) -> u32 {
        self.0
    }

    pub fn get_major(&self) -> u32 {
        version_major(self.0)
    }

    pub fn get_minor(&self) -> u32 {
        version_minor(self.0)
    }

    pub fn get_patch(&self) -> u32 {
        version_patch(self.0)
    }
}

impl PartialOrd<Self> for VulkanVersionEngine {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl From<VulkanVersionVanilla> for VulkanVersionEngine {
    fn from(version: VulkanVersionVanilla) -> Self {
        Self::new(version.major, version.minor, version.patch)
    }
}

impl From<VulkanVersionEngine> for VulkanVersionVanilla {
    fn from(version: VulkanVersionEngine) -> Self {
        Self::new(version.get_major(), version.get_minor(), version.get_patch())
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VulkanVersionApi(u32);

impl VulkanVersionApi {
    pub fn new(major_version: u32, minor_version: u32, patch_version: u32) -> Self {
        Self(make_version(major_version, minor_version, patch_version))
    }

    pub fn unwrap(self) -> u32 {
        self.0
    }

    pub fn get_major(&self) -> u32 {
        version_major(self.0)
    }

    pub fn get_minor(&self) -> u32 {
        version_minor(self.0)
    }

    pub fn get_patch(&self) -> u32 {
        version_patch(self.0)
    }
}

impl PartialOrd<Self> for VulkanVersionApi {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl From<VulkanVersionVanilla> for VulkanVersionApi {
    fn from(version: VulkanVersionVanilla) -> Self {
        Self::new(version.major, version.minor, version.patch)
    }
}

impl From<VulkanVersionApi> for VulkanVersionVanilla {
    fn from(version: VulkanVersionApi) -> Self {
        Self::new(version.get_major(), version.get_minor(), version.get_patch())
    }
}
