use std::collections::HashSet;

use ::library_foundation_reintroduction::vulkan::VulkanEntryVersion1_0;
use ::library_foundation_reintroduction::vulkan::VulkanEntry;
use ::library_foundation_reintroduction::vulkan::VulkanLayerName;

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
        if be_required_layer_s_fulfilled {
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
}