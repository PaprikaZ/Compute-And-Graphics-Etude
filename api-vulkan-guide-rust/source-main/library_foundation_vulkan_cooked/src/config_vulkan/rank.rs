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
    pub fn new(
        discrete_gpu_type_physical_device_factor_weight: VulkanRankWeightFactorExponential,
        integrated_gpu_type_physical_device_factor_weight: VulkanRankWeightFactorExponential,
        least_api_version_minor_exceeding_point_physical_device_factor_weight: VulkanRankWeightFactorExponential,
        one_extension_physical_device_factor_weight: VulkanRankWeightFactorExponential,
        one_feature_physical_device_factor_weight: VulkanRankWeightFactorExponential)
    -> Self
    {
        Self {
            weight_factor_device_physical_type_gpu_discrete:
                discrete_gpu_type_physical_device_factor_weight,
            weight_factor_device_physical_type_gpu_integrated:
                integrated_gpu_type_physical_device_factor_weight,
            weight_factor_device_physical_version_api_least_minor_exceeding_point:
                least_api_version_minor_exceeding_point_physical_device_factor_weight,
            weight_factor_device_physical_extension_one:
                one_extension_physical_device_factor_weight,
            weight_factor_device_physical_feature_one:
                one_feature_physical_device_factor_weight,
        }
    }

    pub fn create_vulkan_rank(&self) -> VulkanRank {
        VulkanRank::new(
            self.weight_factor_device_physical_type_gpu_discrete,
            self.weight_factor_device_physical_type_gpu_integrated,
            self.weight_factor_device_physical_version_api_least_minor_exceeding_point,
            self.weight_factor_device_physical_extension_one,
            self.weight_factor_device_physical_feature_one)
    }
}