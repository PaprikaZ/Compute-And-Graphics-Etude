use std::collections::HashSet;

use ::library_foundation_reintroduction::vulkan::VulkanBuilderHas;
use ::library_foundation_reintroduction::vulkan::VulkanApplicationInformation;
use ::library_foundation_reintroduction::vulkan::VulkanLayerName;
use ::library_foundation_reintroduction::vulkan::VulkanExtensionName;
use ::library_foundation_reintroduction::vulkan::VulkanExtensionDebugUtilityMessageTypeFlagS;
use ::library_foundation_reintroduction::vulkan::VulkanExtensionDebugUtilityMessageSeverityFlagS;
use ::library_foundation_reintroduction::vulkan::VulkanInstanceCreateFlagS;
use ::library_foundation_reintroduction::vulkan::VULKAN_EXTENSION_DEBUG_UTILITY;
use ::library_foundation_reintroduction::vulkan::VULKAN_LAYER_VALIDATION_NAME;
use ::library_foundation_reintroduction::vulkan::version::VulkanVersionApi;
use ::library_foundation_reintroduction::vulkan::version::VulkanVersionEngine;
use ::library_foundation_reintroduction::vulkan::version::VulkanVersionApplication;
use ::library_foundation_reintroduction::vulkan::engine::VulkanEngineName;
use ::library_foundation_reintroduction::vulkan::application::VulkanApplicationName;
use ::library_foundation_reintroduction::vulkan::validation::VulkanValidationBeToEnable;

use crate::vulkan_device_physical::feature::VulkanDevicePhysicalFeatureStandardName;
use crate::vulkan_requirement::version::VulkanRequirementVersionApiLeast;


#[derive(Debug, Clone)]
pub struct ConfigVulkanBaseMixinInstanceLayerExtension {
    pub instance_layer_name_s: HashSet<VulkanLayerName>,
    pub instance_extension_name_s: HashSet<VulkanExtensionName>,
}

impl ConfigVulkanBaseMixinInstanceLayerExtension {
    pub fn create(be_to_enable_validation: VulkanValidationBeToEnable) -> Self {
        let mut instance_layer_name_s = HashSet::<VulkanLayerName>::new();
        let mut instance_extension_name_s = HashSet::<VulkanExtensionName>::new();
        if let VulkanValidationBeToEnable::Y = be_to_enable_validation {
            instance_layer_name_s.insert(VULKAN_LAYER_VALIDATION_NAME);
            instance_extension_name_s.insert(VULKAN_EXTENSION_DEBUG_UTILITY.name);
        }
        Self {
            instance_layer_name_s: instance_layer_name_s,
            instance_extension_name_s: instance_extension_name_s,
        }
    }
}


#[derive(Debug, Clone)]
pub struct ConfigVulkanBase<'t> {
    pub version_api_least_requirement: VulkanRequirementVersionApiLeast,
    pub engine_name: VulkanEngineName<'t>,
    pub engine_version: VulkanVersionEngine,
    pub application_name: VulkanApplicationName<'t>,
    pub application_version: VulkanVersionApplication,
    pub instance_create_flag_s: VulkanInstanceCreateFlagS,
    pub instance_layer_name_s_required: HashSet<VulkanLayerName>,
    pub instance_layer_name_s_optional: HashSet<VulkanLayerName>,
    pub instance_extension_window_name_s: HashSet<VulkanLayerName>,
    pub instance_extension_name_s_required: HashSet<VulkanExtensionName>,
    pub instance_extension_name_s_optional: HashSet<VulkanExtensionName>,
    //pub device_physical_layer_name_s_required: HashSet<VulkanLayerName>,
    //pub device_physical_layer_name_s_optional: HashSet<VulkanLayerName>,
    pub device_physical_extension_name_s_required: HashSet<VulkanExtensionName>,
    pub device_physical_extension_name_s_optional: HashSet<VulkanExtensionName>,
    pub device_physical_feature_name_s_required: HashSet<VulkanDevicePhysicalFeatureStandardName>,
    pub device_physical_feature_name_s_optional: HashSet<VulkanDevicePhysicalFeatureStandardName>,
    pub extension_debug_utility_message_type_flag_s: VulkanExtensionDebugUtilityMessageTypeFlagS,
    pub extension_debug_utility_message_severity_flag_s: VulkanExtensionDebugUtilityMessageSeverityFlagS,
    //NOTE be to enable effect required by mixin, record this field for debug/diagnostic purpose
    pub be_to_enable_validation: VulkanValidationBeToEnable,
}

impl<'t> ConfigVulkanBase<'t> {
    fn merge_mixin_instance_layer_extension(
        instance_layer_extension_mixin: ConfigVulkanBaseMixinInstanceLayerExtension,
        required_instance_layer_name_s: &mut HashSet<VulkanLayerName>,
        required_instance_extension_name_s: &mut HashSet<VulkanExtensionName>)
    {
        instance_layer_extension_mixin.instance_layer_name_s
        .iter()
        .for_each(|n| { let _ = required_instance_layer_name_s.insert(*n); });
        instance_layer_extension_mixin.instance_extension_name_s
        .iter()
        .for_each(|n| { let _ = required_instance_extension_name_s.insert(*n); });
    }

    pub fn create(
        least_api_version_requirement: VulkanRequirementVersionApiLeast,
        engine_name: VulkanEngineName<'t>,
        engine_version: VulkanVersionEngine,
        application_name: VulkanApplicationName<'t>,
        application_version: VulkanVersionApplication,
        mut required_instance_layer_name_s: HashSet<VulkanLayerName>,
        optional_instance_layer_name_s: HashSet<VulkanLayerName>,
        instance_window_extension_name_s: HashSet<VulkanLayerName>,
        mut required_instance_extension_name_s: HashSet<VulkanExtensionName>,
        optional_instance_extension_name_s: HashSet<VulkanExtensionName>,
        required_physical_device_extension_name_s: HashSet<VulkanExtensionName>,
        optional_physical_device_extension_name_s: HashSet<VulkanExtensionName>,
        required_physical_device_feature_name_s: HashSet<VulkanDevicePhysicalFeatureStandardName>,
        optional_physical_device_feature_name_s: HashSet<VulkanDevicePhysicalFeatureStandardName>,
        debug_utility_extension_message_type_flag_s: VulkanExtensionDebugUtilityMessageTypeFlagS,
        debug_utility_extension_message_severity_flag_s: VulkanExtensionDebugUtilityMessageSeverityFlagS,
        be_to_enable_validation: VulkanValidationBeToEnable)
    -> Self
    {
        let instance_layer_extension_mixin =
            ConfigVulkanBaseMixinInstanceLayerExtension::create(be_to_enable_validation);
        Self::merge_mixin_instance_layer_extension(
            instance_layer_extension_mixin,
            &mut required_instance_layer_name_s, &mut required_instance_extension_name_s);
        Self {
            version_api_least_requirement: least_api_version_requirement,
            engine_name: engine_name,
            engine_version: engine_version,
            application_name: application_name,
            application_version: application_version,
            instance_create_flag_s: VulkanInstanceCreateFlagS::empty(),
            instance_layer_name_s_required: required_instance_layer_name_s,
            instance_layer_name_s_optional: optional_instance_layer_name_s,
            instance_extension_window_name_s: instance_window_extension_name_s,
            instance_extension_name_s_required: required_instance_extension_name_s,
            instance_extension_name_s_optional: optional_instance_extension_name_s,
            device_physical_extension_name_s_required: required_physical_device_extension_name_s,
            device_physical_extension_name_s_optional: optional_physical_device_extension_name_s,
            device_physical_feature_name_s_required: required_physical_device_feature_name_s,
            device_physical_feature_name_s_optional: optional_physical_device_feature_name_s,
            extension_debug_utility_message_type_flag_s: debug_utility_extension_message_type_flag_s,
            extension_debug_utility_message_severity_flag_s: debug_utility_extension_message_severity_flag_s,
            be_to_enable_validation: be_to_enable_validation,
        }
    }

    pub fn create_vulkan_application_information(
        &self, negotiated_api_version: &VulkanVersionApi)
    -> VulkanApplicationInformation
    {
        VulkanApplicationInformation::builder()
        .application_name(self.application_name.as_ref_byte_s_with_nul())
        .application_version(self.application_version.as_raw())
        .engine_name(self.engine_name.as_ref_byte_s_with_nul())
        .engine_version(self.engine_version.as_raw())
        .api_version(negotiated_api_version.as_raw())
        .build()
    }
}