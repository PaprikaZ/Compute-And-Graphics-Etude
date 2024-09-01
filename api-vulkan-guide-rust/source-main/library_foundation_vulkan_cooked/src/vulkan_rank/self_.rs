use ::library_foundation_reintroduction::vulkan::VulkanDevicePhysicalType;
use ::library_foundation_reintroduction::vulkan::version::VulkanVersionApi;

use crate::vulkan_requirement::version::VulkanRequirementVersionApiLeast;
use crate::vulkan_rank::score::VulkanRankScore;
use crate::vulkan_rank::weight::VulkanRankWeightFactorExponential;


#[derive(Debug, Clone)]
pub struct VulkanRank {
    pub weight_factor_device_physical_type_gpu_discrete: VulkanRankWeightFactorExponential,
    pub weight_factor_device_physical_type_gpu_integrated: VulkanRankWeightFactorExponential,
    pub weight_factor_version_api_least_minor_exceeding_point: VulkanRankWeightFactorExponential,
    pub weight_factor_device_physical_extension_one: VulkanRankWeightFactorExponential,
    pub weight_factor_device_physical_feature_one: VulkanRankWeightFactorExponential,
}

impl VulkanRank {
    pub fn new(
        discrete_gpu_device_type_weight_factor: VulkanRankWeightFactorExponential,
        integrated_gpu_device_type_weight_factor: VulkanRankWeightFactorExponential,
        minor_exceeding_point_least_api_version_weight_factor: VulkanRankWeightFactorExponential,
        one_physical_device_extension_weight_factor: VulkanRankWeightFactorExponential,
        one_physical_device_feature_weight_factor: VulkanRankWeightFactorExponential)
    -> Self
    {
        Self {
            weight_factor_device_physical_type_gpu_discrete: discrete_gpu_device_type_weight_factor,
            weight_factor_device_physical_type_gpu_integrated: integrated_gpu_device_type_weight_factor,
            weight_factor_version_api_least_minor_exceeding_point: minor_exceeding_point_least_api_version_weight_factor,
            weight_factor_device_physical_extension_one: one_physical_device_extension_weight_factor,
            weight_factor_device_physical_feature_one: one_physical_device_feature_weight_factor,
        }
    }

    pub fn calculate_score_device_physical(
        &self,
        physical_device_type: VulkanDevicePhysicalType,
        least_api_version_requirement: VulkanRequirementVersionApiLeast,
        physical_device_api_version: VulkanVersionApi,
        optional_physical_device_extension_hit_number: u32,
        optional_physical_device_feature_hit_number: u32)
    -> VulkanRankScore
    {
        assert!(least_api_version_requirement.get_minor() <= physical_device_api_version.get_minor());
        (match physical_device_type {
            VulkanDevicePhysicalType::DISCRETE_GPU =>
                VulkanRankScore::from(self.weight_factor_device_physical_type_gpu_discrete),
            VulkanDevicePhysicalType::INTEGRATED_GPU =>
                VulkanRankScore::from(self.weight_factor_device_physical_type_gpu_integrated),
            _ =>
                VulkanRankScore::ZERO,
        })
        +
        (VulkanRankScore::from(self.weight_factor_version_api_least_minor_exceeding_point) *
         (physical_device_api_version.get_minor() - least_api_version_requirement.get_minor()))
        +
        (VulkanRankScore::from(self.weight_factor_device_physical_extension_one) *
         optional_physical_device_extension_hit_number)
        +
        (VulkanRankScore::from(self.weight_factor_device_physical_feature_one) *
         optional_physical_device_feature_hit_number)
    }
}