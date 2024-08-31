use std::collections::HashSet;

use ::library_foundation_reintroduction::vulkan::VulkanBuilderHas;
use ::library_foundation_reintroduction::vulkan::VulkanInstanceVersion1_0;
use ::library_foundation_reintroduction::vulkan::VulkanInstanceVersion1_1;
use ::library_foundation_reintroduction::vulkan::VulkanInstance;
use ::library_foundation_reintroduction::vulkan::VulkanDevicePhysical;
use ::library_foundation_reintroduction::vulkan::VulkanQueueFamilyPropertyS;
use ::library_foundation_reintroduction::vulkan::VulkanSurfaceKhr;
use ::library_foundation_reintroduction::vulkan::VulkanQueueFlagS;
use ::library_foundation_reintroduction::vulkan::VulkanExtensionName;
use ::library_foundation_reintroduction::vulkan::VulkanVersionVanilla;
use ::library_foundation_reintroduction::vulkan::version::VulkanVersionApi;
use ::library_foundation_reintroduction::vulkan::queue::VulkanQueueFamilyIndexGraphic;
use ::library_foundation_reintroduction::vulkan::queue::VulkanQueueFamilyIndexPresent;
use ::library_foundation_reintroduction::vulkan::queue::VulkanQueueFamilyIndexCompute;
use ::library_foundation_reintroduction::vulkan::queue::VulkanQueueFamilyIndexTransfer;

use crate::error::foundation_vulkan_cooked::ErrorFoundationVulkanCooked;
use crate::error::foundation_vulkan_cooked::ErrorFoundationVulkanCookedOwn;
use crate::vulkan_requirement::version::VulkanRequirementVersionApiLeast;


pub struct VulkanRequirementDevicePhysical {}

impl VulkanRequirementDevicePhysical {
    pub fn fulfill_version(
        vulkan_instance: &VulkanInstance,
        vulkan_physical_device: VulkanDevicePhysical,
        least_vulkan_api_version_requirement: &VulkanRequirementVersionApiLeast)
    -> Result<(), ErrorFoundationVulkanCooked>
    {
        let property_s = unsafe { vulkan_instance.get_physical_device_properties(vulkan_physical_device) };
        let api_version = VulkanVersionApi::from(VulkanVersionVanilla::from(property_s.api_version));
        least_vulkan_api_version_requirement.fulfill_device_physical(&api_version)?;
        Ok(())
    }

    //
    fn find_queue_family_index_graphic(vulkan_queue_family_property_s: &[VulkanQueueFamilyPropertyS])
    -> Option<VulkanQueueFamilyIndexGraphic>
    {
        vulkan_queue_family_property_s
        .iter()
        .position(|p| p.queue_flags.contains(VulkanQueueFlagS::GRAPHICS))
        .map(|i| VulkanQueueFamilyIndexGraphic::new(i as u32))
    }

    fn find_queue_family_index_present(
        vulkan_instance: &VulkanInstance,
        vulkan_physical_device: VulkanDevicePhysical,
        vulkan_surface: VulkanSurfaceKhr,
        vulkan_queue_family_property_s: &[VulkanQueueFamilyPropertyS])
    -> Option<VulkanQueueFamilyIndexPresent>
    {
        vulkan_queue_family_property_s
        .iter()
        .enumerate()
        .find(|(index, _property_s)| { unsafe {
            vulkan_instance.get_physical_device_surface_support_khr(
                vulkan_physical_device, *index as u32, vulkan_surface)
            .unwrap_or(false)
        }})
        .map(|(index, _property_s)| VulkanQueueFamilyIndexPresent::new(index as u32))
    }

    fn _find_queue_family_index_compute(vulkan_queue_family_property_s: &[VulkanQueueFamilyPropertyS])
    -> Option<VulkanQueueFamilyIndexCompute>
    {
        vulkan_queue_family_property_s
        .iter()
        .position(|p| p.queue_flags.contains(VulkanQueueFlagS::COMPUTE))
        .map(|i| VulkanQueueFamilyIndexCompute::new(i as u32))
    }

    fn _find_queue_family_index_transfer(vulkan_queue_family_property_s: &[VulkanQueueFamilyPropertyS])
    -> Option<VulkanQueueFamilyIndexTransfer>
    {
        vulkan_queue_family_property_s
        .iter()
        .position(|p| p.queue_flags.contains(VulkanQueueFlagS::TRANSFER))
        .map(|i| VulkanQueueFamilyIndexTransfer::new(i as u32))
    }

    pub fn fulfill_queue_family_graphic_present(
        vulkan_instance: &VulkanInstance,
        vulkan_physical_device: VulkanDevicePhysical,
        vulkan_surface: VulkanSurfaceKhr)
    -> Result<(VulkanQueueFamilyIndexGraphic, VulkanQueueFamilyIndexPresent),
              ErrorFoundationVulkanCooked>
    {
        let vulkan_queue_family_property_s =
            unsafe { vulkan_instance.get_physical_device_queue_family_properties(vulkan_physical_device) };
        None
        .and_then(|_: ()|
            Self::find_queue_family_index_graphic(vulkan_queue_family_property_s.as_slice()))
        .and_then(|gi|
            Self::find_queue_family_index_present(
                vulkan_instance, vulkan_physical_device, vulkan_surface, vulkan_queue_family_property_s.as_slice())
            .map(|pi| (gi, pi)))
        .ok_or(ErrorFoundationVulkanCookedOwn::VulkanRequirementDevicePhysicalQueueFamilySNotFulfilled.into())
    }

    pub fn fulfill_queue_family_graphic_present_transfer(
        _vulkan_instance: &VulkanInstance,
        _vulkan_physical_device: VulkanDevicePhysical,
        _vulkan_surface: VulkanSurfaceKhr)
    -> Result<(VulkanQueueFamilyIndexGraphic, VulkanQueueFamilyIndexPresent, VulkanQueueFamilyIndexTransfer),
              ErrorFoundationVulkanCooked>
    {
        todo!()
    }

    pub fn fulfill_queue_family_compute(
        _vulkan_instance: &VulkanInstance,
        _vulkan_physical_device: VulkanDevicePhysical)
    -> Result<VulkanQueueFamilyIndexCompute, ErrorFoundationVulkanCooked>
    {
        todo!()
    }

    pub fn fulfill_queue_family_compute_transfer(
        _vulkan_instance: &VulkanInstance,
        _vulkan_physical_device: VulkanDevicePhysical)
    -> Result<(VulkanQueueFamilyIndexCompute, VulkanQueueFamilyIndexTransfer),
              ErrorFoundationVulkanCooked>
    {
        todo!()
    }

    pub fn fulfill_queue_family_compute_present(
        _vulkan_instance: &VulkanInstance,
        _vulkan_physical_device: VulkanDevicePhysical,
        _vulkan_surface: VulkanSurfaceKhr)
    -> Result<(VulkanQueueFamilyIndexCompute, VulkanQueueFamilyIndexPresent),
              ErrorFoundationVulkanCooked>
    {
        todo!()
    }

    pub fn fulfill_queue_family_compute_present_transfer(
        _vulkan_instance: &VulkanInstance,
        _vulkan_physical_device: VulkanDevicePhysical,
        _vulkan_surface: VulkanSurfaceKhr)
    -> Result<(VulkanQueueFamilyIndexCompute, VulkanQueueFamilyIndexPresent, VulkanQueueFamilyIndexTransfer),
              ErrorFoundationVulkanCooked>
    {
        todo!()
    }

    pub fn fulfill_queue_family_graphic_compute_present(
        _vulkan_instance: &VulkanInstance,
        _vulkan_physical_device: VulkanDevicePhysical,
        _vulkan_surface: VulkanSurfaceKhr)
    -> Result<(VulkanQueueFamilyIndexGraphic, VulkanQueueFamilyIndexCompute, VulkanQueueFamilyIndexPresent),
              ErrorFoundationVulkanCooked>
    {
        todo!()
    }

    pub fn fulfill_queue_family_graphic_compute_present_transfer(
        _vulkan_instance: &VulkanInstance,
        _vulkan_physical_device: VulkanDevicePhysical,
        _vulkan_surface: VulkanSurfaceKhr)
    -> Result<(VulkanQueueFamilyIndexGraphic, VulkanQueueFamilyIndexCompute,
               VulkanQueueFamilyIndexPresent, VulkanQueueFamilyIndexTransfer),
              ErrorFoundationVulkanCooked>
    {
        todo!()
    }

    //
    pub fn fulfill_extension_name_s<'t>(
        vulkan_instance: &VulkanInstance,
        vulkan_physical_device: VulkanDevicePhysical,
        required_vulkan_extension_name_s: &'t HashSet<VulkanExtensionName>,
        optional_vulkan_extension_name_s: &'t HashSet<VulkanExtensionName>)
    -> Result<(Vec<&'t VulkanExtensionName>, u32), ErrorFoundationVulkanCooked>
    {
        //TODO available physical device extensions should be queried by layer list
        let available_vulkan_extension_property_s_s =
            match unsafe { vulkan_instance.enumerate_device_extension_properties(vulkan_physical_device, None) } {
                Err(_e) => Err(ErrorFoundationVulkanCookedOwn::VulkanDevicePhysicalExtensionPropertySEnumerateFail)?,
                Ok(ps) => ps,
            };
        let available_vulkan_extension_name_s =
            available_vulkan_extension_property_s_s
            .iter()
            .map(|p| p.extension_name)
            .collect::<HashSet<_>>();
        //
        let be_required_vulkan_extension_s_fulfilled =
            required_vulkan_extension_name_s.is_subset(&available_vulkan_extension_name_s);
        if be_required_vulkan_extension_s_fulfilled {
            return Err(ErrorFoundationVulkanCookedOwn::VulkanRequirementDevicePhysicalExtensionSNotFulfilled)?
        }
        //
        let matched_optional_vulkan_extension_name_s =
            optional_vulkan_extension_name_s
            .iter()
            .filter(|n| available_vulkan_extension_name_s.contains(n));
        let matched_optional_vulkan_extension_number = matched_optional_vulkan_extension_name_s.clone().count() as u32;
        let matched_vulkan_extension_name_s =
            required_vulkan_extension_name_s
            .iter()
            .chain(matched_optional_vulkan_extension_name_s)
            .collect::<Vec<_>>();
        //
        Ok((matched_vulkan_extension_name_s, matched_optional_vulkan_extension_number))
    }
}