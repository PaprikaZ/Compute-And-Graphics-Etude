use ::library_foundation_reintroduction::vulkan::VulkanBool32;
use ::library_foundation_reintroduction::vulkan::VulkanDevicePhysicalFeatureS;
use ::library_foundation_reintroduction::vulkan::VulkanDevicePhysicalFeatureSBuilder;


pub trait VulkanDevicePhysicalFeatureTableLookupEnable {
    fn lookup(&self, physical_device_feature_s: &VulkanDevicePhysicalFeatureS) -> VulkanBool32;
    fn enable(&self, builder: VulkanDevicePhysicalFeatureSBuilder) -> VulkanDevicePhysicalFeatureSBuilder;
}