use std::collections::HashSet;

use ::library_foundation_reintroduction::vulkan::VulkanEntryVersion1_0;
use ::library_foundation_reintroduction::vulkan::VulkanEntry;
use ::library_foundation_reintroduction::vulkan::VulkanLayerName;
use ::library_foundation_reintroduction::vulkan::VulkanExtensionName;

use crate::error::foundation_vulkan_cooked::ErrorFoundationVulkanCookedOwn;
use crate::error::foundation_vulkan_cooked::ErrorFoundationVulkanCooked;


pub struct VulkanRequirementInstance {}

impl VulkanRequirementInstance {
    pub fn fulfill_layer_name_s<'t>(
        vulkan_entry: &VulkanEntry,
        required_vulkan_layer_name_s: &'t HashSet<VulkanLayerName>,
        optional_vulkan_layer_name_s: &'t HashSet<VulkanLayerName>)
    -> Result<Vec<&'t VulkanLayerName>, ErrorFoundationVulkanCooked>
    {
        let available_layer_property_s_s =
            match unsafe { vulkan_entry.enumerate_instance_layer_properties() } {
                Err(_e) => Err(ErrorFoundationVulkanCookedOwn::VulkanInstanceLayerPropertySEnumerateFail)?,
                Ok(layer_property_s_s) => layer_property_s_s,
            };
        let available_layer_name_s =
            available_layer_property_s_s
            .iter()
            .map(|p| p.layer_name)
            .collect::<HashSet<_>>();
        //
        let be_required_layer_s_fulfilled =
            required_vulkan_layer_name_s.is_subset(&available_layer_name_s);
        if !be_required_layer_s_fulfilled {
            return Err(ErrorFoundationVulkanCookedOwn::VulkanRequirementInstanceLayerSNotFulfilled)?
        }
        //
        let matched_optional_layer_name_s =
            optional_vulkan_layer_name_s
            .iter()
            .filter(|n| available_layer_name_s.contains(n));
        let matched_layer_name_s =
            required_vulkan_layer_name_s
            .iter()
            .chain(matched_optional_layer_name_s)
            .collect::<Vec<_>>();
        //
        Ok(matched_layer_name_s)
    }

    pub fn fulfill_extension_name_s<'t>(
        vulkan_entry: &VulkanEntry,
        window_vulkan_extension_name_s: &'t HashSet<VulkanExtensionName>,
        required_vulkan_extension_name_s: &'t HashSet<VulkanExtensionName>,
        optional_vulkan_extension_name_s: &'t HashSet<VulkanExtensionName>)
    -> Result<Vec<&'t VulkanExtensionName>, ErrorFoundationVulkanCooked>
    {
        //TODO available instance extensions should be queried by layer list
        let available_extension_property_s_s =
            match unsafe { vulkan_entry.enumerate_instance_extension_properties(None) } {
                Err(_e) => Err(ErrorFoundationVulkanCookedOwn::VulkanInstanceExtensionPropertySEnumerateFail)?,
                Ok(extension_property_s_s) => extension_property_s_s,
            };
        let available_extension_name_s =
            available_extension_property_s_s
            .iter()
            .map(|p| p.extension_name)
            .collect::<HashSet<_>>();
        //
        let be_required_extension_s_fulfilled =
            window_vulkan_extension_name_s.is_subset(&available_extension_name_s) &&
            required_vulkan_extension_name_s.is_subset(&available_extension_name_s);
        if !be_required_extension_s_fulfilled {
            return Err(ErrorFoundationVulkanCookedOwn::VulkanRequirementInstanceExtensionSNotFulfilled)?
        }
        //
        let matched_optional_extension_name_s =
            optional_vulkan_extension_name_s
            .iter()
            .filter(|n| available_extension_name_s.contains(n));
        let matched_extension_name_s =
            window_vulkan_extension_name_s
            .iter()
            .chain(required_vulkan_extension_name_s.iter())
            .chain(matched_optional_extension_name_s)
            .collect::<Vec<_>>();
        //
        Ok(matched_extension_name_s)
    }
}