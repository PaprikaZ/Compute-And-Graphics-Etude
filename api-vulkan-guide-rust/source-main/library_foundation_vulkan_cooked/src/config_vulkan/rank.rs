use crate::vulkan_rank::weight::VulkanRankWeightFactorExponential;
use crate::vulkan_rank::VulkanRank;


#[derive(Debug, Clone)]
pub struct ConfigVulkanRank {
    pub weight_factor_device_physical_type_gpu_discrete: VulkanRankWeightFactorExponential,
    pub weight_factor_device_physical_type_gpu_integrated: VulkanRankWeightFactorExponential,
    pub weight_factor_device_physical_version_api_least_minor_exceeding_point: VulkanRankWeightFactorExponential,
    pub weight_factor_device_physical_extension_one: VulkanRankWeightFactorExponential,
    pub weight_factor_device_physical_feature_one: VulkanRankWeightFactorExponential,
}

impl ConfigVulkanRank {
    pub fn create_vulkan_rank(&self) -> VulkanRank {
        VulkanRank::new(
            self.weight_factor_device_physical_type_gpu_discrete,
            self.weight_factor_device_physical_type_gpu_integrated,
            self.weight_factor_device_physical_version_api_least_minor_exceeding_point,
            self.weight_factor_device_physical_extension_one,
            self.weight_factor_device_physical_feature_one)
    }
}