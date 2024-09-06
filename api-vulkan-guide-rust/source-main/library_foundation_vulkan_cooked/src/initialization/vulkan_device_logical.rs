use std::collections::HashSet;

use ::library_foundation_reintroduction::vulkan::VulkanBuilderHas;
use ::library_foundation_reintroduction::vulkan::VulkanInstance;
use ::library_foundation_reintroduction::vulkan::VulkanDevicePhysical;
use ::library_foundation_reintroduction::vulkan::VulkanExtensionName;
use ::library_foundation_reintroduction::vulkan::VulkanDeviceLogical;
use ::library_foundation_reintroduction::vulkan::VulkanDevicePhysicalFeatureS;
use ::library_foundation_reintroduction::vulkan::VulkanDeviceLogicalQueueCreateInformation;
use ::library_foundation_reintroduction::vulkan::VulkanDeviceLogicalCreateInformationBuilder;
use ::library_foundation_reintroduction::vulkan::queue::VulkanQueueFamilyIndexGraphic;
use ::library_foundation_reintroduction::vulkan::queue::VulkanQueueFamilyIndexPresent;

use crate::error::foundation_vulkan_cooked::ErrorFoundationVulkanCookedOwn;
use crate::error::foundation_vulkan_cooked::ErrorFoundationVulkanCooked;
use crate::vulkan_device_physical::feature::VulkanDevicePhysicalFeatureStandardName;
use crate::vulkan_device_physical::feature::VulkanDevicePhysicalFeatureTableLookupEnable;


pub struct InitializationVulkanDeviceLogical {}

impl InitializationVulkanDeviceLogical {
    pub fn initialize(
        vulkan_instance: &VulkanInstance,
        vulkan_physical_device: VulkanDevicePhysical,
        vulkan_extension_name_s: &Vec<VulkanExtensionName>,
        vulkan_physical_device_feature_name_s: &Vec<VulkanDevicePhysicalFeatureStandardName>,
        vulkan_graphic_queue_family_index: VulkanQueueFamilyIndexGraphic,
        vulkan_present_queue_family_index: VulkanQueueFamilyIndexPresent,
        vulkan_logical_device_create_inforamtion_builder: VulkanDeviceLogicalCreateInformationBuilder)
    -> Result<VulkanDeviceLogical, ErrorFoundationVulkanCooked>
    {
        let vulkan_queue_family_index_set = {
            let mut s = HashSet::new();
            s.insert(vulkan_graphic_queue_family_index.as_raw());
            s.insert(vulkan_present_queue_family_index.as_raw());
            s
        };
        //
        let vulkan_graphic_queue_priority_s = &[1.0];
        let vulkan_graphic_queue_create_information_s =
            vulkan_queue_family_index_set
            .iter()
            .map(|i| {
                VulkanDeviceLogicalQueueCreateInformation::builder()
                .queue_family_index(*i)
                .queue_priorities(vulkan_graphic_queue_priority_s)
            })
            .collect::<Vec<_>>();
        let vulkan_logical_device_layer_s: Vec<*const i8> = Vec::new();
        let vulkan_extension_name_ptr_s =
            vulkan_extension_name_s
            .iter()
            .map(|n| n.as_ptr())
            .collect::<Vec<_>>();
        let vulkan_physical_device_feature_s =
            vulkan_physical_device_feature_name_s
            .iter()
            .fold(VulkanDevicePhysicalFeatureS::builder(), |builder, feature_name| feature_name.enable(builder))
            .build();
        //
        let vulkan_logical_device_create_information =
            vulkan_logical_device_create_inforamtion_builder
            .queue_create_infos(&vulkan_graphic_queue_create_information_s)
            .enabled_layer_names(&vulkan_logical_device_layer_s)
            .enabled_extension_names(&vulkan_extension_name_ptr_s)
            .enabled_features(&vulkan_physical_device_feature_s)
            .build();
        match unsafe { vulkan_instance.create_device(vulkan_physical_device, &vulkan_logical_device_create_information, None) } {
            Err(_e) => Err(ErrorFoundationVulkanCookedOwn::VulkanDeviceLogicalCreateFail)?,
            Ok(ld) => Ok(ld)
        }
    }
}