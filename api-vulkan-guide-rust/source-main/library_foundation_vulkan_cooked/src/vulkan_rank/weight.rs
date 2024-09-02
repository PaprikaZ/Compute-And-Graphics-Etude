use crate::error::foundation_vulkan_cooked::ErrorFoundationVulkanCookedOwn;
use crate::error::foundation_vulkan_cooked::ErrorFoundationVulkanCooked;
use crate::vulkan_rank::score::VulkanRankScore;


#[derive(Debug, Clone, Copy)]
pub struct VulkanRankWeightFactorExponential(u32);

impl VulkanRankWeightFactorExponential {
    const MAX: u32 = 16;

    pub fn try_new(value: u32) -> Result<Self, ErrorFoundationVulkanCooked> {
        if Self::MAX < value {
            return Err(ErrorFoundationVulkanCookedOwn::VulkanNegotiationRankWeightFactorExponentialInvalid)?
        }
        Ok(Self(value))
    }

    pub fn as_raw(self) -> u32 {
        self.0
    }
}

impl From<VulkanRankWeightFactorExponential> for VulkanRankScore {
    fn from(factor: VulkanRankWeightFactorExponential) -> Self {
        Self::new(1 << factor.0)
    }
}