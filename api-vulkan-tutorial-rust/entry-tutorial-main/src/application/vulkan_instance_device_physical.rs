use std::collections::HashSet;
use std::process::Termination;

use ::vulkan::VULKAN_TRUE;
use ::vulkan::VulkanExtensionName;
use ::vulkan::VulkanInstance;
use ::vulkan::VulkanInstanceVersion1_0;
use ::vulkan::VulkanDevicePhysical;
use ::vulkan::VulkanErrorCode;
use ::vulkan::VulkanQueueFamilyIndexGraphic;
use ::vulkan::VulkanQueueFamilyIndexSurface;
use ::vulkan::VulkanQueueFlagS;
use ::vulkan::VulkanSurfaceExtensionKhr;
use ::vulkan::VulkanSurfaceKhr;
use ::console_log::{console_log_info, console_log_warn};

use crate::termination::TerminationProcessMain;
use crate::application::vulkan_instance_swapchain::ApplicationVulkanInstanceSwapchain;


enum ApplicationVulkanInstanceDevicePhysicalPickRequirementNotMatch {
    QueueFamilyGraphicMissing,
    QueueFamilySurfaceMissing,
    ExtensionMissing,
    SwapchainFormatMissing,
    SwapchainPresentModeMissing,
}

enum ApplicationVulkanInstanceDevicePhysicalPickError {
    RequirementNotMatch(ApplicationVulkanInstanceDevicePhysicalPickRequirementNotMatch),
    Termination(TerminationProcessMain),
}

pub struct ApplicationVulkanInstanceDevicePhysical {}

impl ApplicationVulkanInstanceDevicePhysical {
    pub unsafe fn pick(
        vulkan_instance: &VulkanInstance,
        vulkan_surface: VulkanSurfaceKhr,
        vulkan_extension_s: &[VulkanExtensionName])
     -> Result<(VulkanDevicePhysical, VulkanQueueFamilyIndexGraphic, VulkanQueueFamilyIndexSurface), TerminationProcessMain>
    {
        let vulkan_physical_device_s =
            match vulkan_instance.enumerate_physical_devices() {
                Err(error) => {
                    let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                    return Err(TerminationProcessMain::InitializationVulkanEnumeratePhysicalDeviceFail(vulkan_error_code));
                },
                Ok(device_s) => device_s,
            };
        for vulkan_physical_device in vulkan_physical_device_s {
            let physical_device_property_s = vulkan_instance.get_physical_device_properties(vulkan_physical_device);
            let check_result =
                Self::check(vulkan_instance, vulkan_physical_device, vulkan_surface, vulkan_extension_s);
            match check_result {
                Err(ApplicationVulkanInstanceDevicePhysicalPickError::RequirementNotMatch(
                    ApplicationVulkanInstanceDevicePhysicalPickRequirementNotMatch::QueueFamilyGraphicMissing)) => {
                    console_log_warn!(
                        "Physical device (`{}`) skipping: missing graphic queue family",
                        physical_device_property_s.device_name);
                },
                Err(ApplicationVulkanInstanceDevicePhysicalPickError::RequirementNotMatch(
                    ApplicationVulkanInstanceDevicePhysicalPickRequirementNotMatch::QueueFamilySurfaceMissing)) => {
                    console_log_warn!(
                        "Physical device (`{}`) skipping: missing surface queue family",
                        physical_device_property_s.device_name);
                },
                Err(ApplicationVulkanInstanceDevicePhysicalPickError::RequirementNotMatch(
                    ApplicationVulkanInstanceDevicePhysicalPickRequirementNotMatch::ExtensionMissing)) => {
                    console_log_warn!(
                        "Physical device (`{}`) skipping: missing extension",
                        physical_device_property_s.device_name);
                },
                Err(ApplicationVulkanInstanceDevicePhysicalPickError::RequirementNotMatch(
                    ApplicationVulkanInstanceDevicePhysicalPickRequirementNotMatch::SwapchainFormatMissing)) => {
                    console_log_warn!(
                        "Physical device (`{}`) skipping: missing swapchain format",
                        physical_device_property_s.device_name);
                },
                Err(ApplicationVulkanInstanceDevicePhysicalPickError::RequirementNotMatch(
                    ApplicationVulkanInstanceDevicePhysicalPickRequirementNotMatch::SwapchainPresentModeMissing)) => {
                    console_log_warn!(
                        "Physical device (`{}`) skipping: missing swapchain present mode",
                        physical_device_property_s.device_name);
                },
                Err(ApplicationVulkanInstanceDevicePhysicalPickError::Termination(termination)) =>
                    return Err(termination),
                Ok((graphic_queue_index, surface_queue_index)) => {
                    console_log_info!("Physical device (`{}`) selected", physical_device_property_s.device_name);
                    return Ok((vulkan_physical_device, graphic_queue_index, surface_queue_index));
                }
            };
        }
        Err(TerminationProcessMain::InitializationVulkanDevicePhysicalAllNotQualified)
    }

    unsafe fn check(
        vulkan_instance: &VulkanInstance,
        vulkan_physical_device: VulkanDevicePhysical,
        vulkan_surface: VulkanSurfaceKhr,
        vulkan_extension_s: &[VulkanExtensionName])
     -> Result<(VulkanQueueFamilyIndexGraphic, VulkanQueueFamilyIndexSurface),
               ApplicationVulkanInstanceDevicePhysicalPickError>
    {
        let check_queue_family_result =
            Self::check_queue_family(vulkan_instance, vulkan_physical_device, vulkan_surface);
        let (vulkan_graphic_queue_family_index, vulkan_surface_queue_family_index) =
            match check_queue_family_result {
                Err(error) =>
                    return Err(ApplicationVulkanInstanceDevicePhysicalPickError::RequirementNotMatch(error)),
                Ok(graphic_and_surface_index) => graphic_and_surface_index,
            };
        Self::check_extension_s(vulkan_instance, vulkan_physical_device, vulkan_extension_s)?;
        Self::check_swapchain(vulkan_instance, vulkan_physical_device, vulkan_surface)?;
        match Self::check_feature_s(vulkan_instance, vulkan_physical_device) {
            Err(error) => return Err(ApplicationVulkanInstanceDevicePhysicalPickError::Termination(error)),
            Ok(()) => (),
        };
        Ok((vulkan_graphic_queue_family_index, vulkan_surface_queue_family_index))
    }

    unsafe fn check_queue_family(
        vulkan_instance: &VulkanInstance,
        vulkan_physical_device: VulkanDevicePhysical,
        vulkan_surface: VulkanSurfaceKhr)
     -> Result<(VulkanQueueFamilyIndexGraphic, VulkanQueueFamilyIndexSurface),
               ApplicationVulkanInstanceDevicePhysicalPickRequirementNotMatch>
    {
        let vulkan_physical_device_queue_family_property_s =
            vulkan_instance.get_physical_device_queue_family_properties(vulkan_physical_device);
        let optional_graphic_queue_family_index =
            vulkan_physical_device_queue_family_property_s
            .iter()
            .position(|p| p.queue_flags.contains(VulkanQueueFlagS::GRAPHICS))
            .map(|i| VulkanQueueFamilyIndexGraphic::new(i as u32));
        let optional_surface_queue_family_index =
            vulkan_physical_device_queue_family_property_s
            .iter()
            .enumerate()
            .find(|(index, _property_s)| {
                let support_result =
                    vulkan_instance.get_physical_device_surface_support_khr(
                        vulkan_physical_device,
                        *index as u32,
                        vulkan_surface);
                match support_result {
                    Err(_error) => false,
                    Ok(be_support) => be_support,
                }
            })
            .map(|(index, _property_s)| VulkanQueueFamilyIndexSurface::new(index as u32));
        match (optional_graphic_queue_family_index, optional_surface_queue_family_index) {
            (None, _) => Err(ApplicationVulkanInstanceDevicePhysicalPickRequirementNotMatch::QueueFamilyGraphicMissing),
            (_, None) => Err(ApplicationVulkanInstanceDevicePhysicalPickRequirementNotMatch::QueueFamilySurfaceMissing),
            (Some(graphic_index), Some(surface_index)) =>
                Ok((graphic_index, surface_index)),
        }
    }

    unsafe fn check_swapchain(
        vulkan_instance: &VulkanInstance,
        vulkan_physical_device: VulkanDevicePhysical,
        vulkan_surface: VulkanSurfaceKhr)
     -> Result<(), ApplicationVulkanInstanceDevicePhysicalPickError>
    {
        let get_vulkan_surface_support_result =
            ApplicationVulkanInstanceSwapchain::get_surface_support(vulkan_instance, vulkan_surface, vulkan_physical_device);
        let (_vulkan_surface_capability_s, vulkan_surface_format_s, vulkan_present_mode_s) =
            match get_vulkan_surface_support_result {
                Err(error) => return Err(ApplicationVulkanInstanceDevicePhysicalPickError::Termination(error)),
                Ok(support_info) => support_info,
            };
        match (vulkan_surface_format_s.is_empty(), vulkan_present_mode_s.is_empty()) {
            (true, _) => Err(ApplicationVulkanInstanceDevicePhysicalPickError::RequirementNotMatch(
                ApplicationVulkanInstanceDevicePhysicalPickRequirementNotMatch::SwapchainFormatMissing)),
            (_, true) => Err(ApplicationVulkanInstanceDevicePhysicalPickError::RequirementNotMatch(
                ApplicationVulkanInstanceDevicePhysicalPickRequirementNotMatch::SwapchainPresentModeMissing)),
            (false, false) => Ok(())
        }
    }

    unsafe fn check_extension_s(
        vulkan_instance: &VulkanInstance,
        vulkan_physical_device: VulkanDevicePhysical,
        vulkan_extension_s: &[VulkanExtensionName])
     -> Result<(), ApplicationVulkanInstanceDevicePhysicalPickError>
    {
        let enumerate_vulkan_physical_device_extension_property_s_result =
            vulkan_instance.enumerate_device_extension_properties(vulkan_physical_device, None);
        let vulkan_physical_device_extension_property_s =
            match enumerate_vulkan_physical_device_extension_property_s_result {
                Err(error) => {
                    let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                    let termination = TerminationProcessMain::InitializationVulkanDevicePhysicalExtensionPropertySEnumerateFail(vulkan_error_code);
                    return Err(ApplicationVulkanInstanceDevicePhysicalPickError::Termination(termination));
                },
                Ok(property_s) => property_s,
            };
        let all_vulkan_physical_device_extension_s =
            vulkan_physical_device_extension_property_s
            .iter()
            .map(|e| e.extension_name)
            .collect::<HashSet<_>>();
        let be_required_vulkan_physical_device_extension_s_supported =
            vulkan_extension_s.iter().all(|e| all_vulkan_physical_device_extension_s.contains(e));
        match be_required_vulkan_physical_device_extension_s_supported {
            true => Ok(()),
            false => return Err(ApplicationVulkanInstanceDevicePhysicalPickError::RequirementNotMatch(
                ApplicationVulkanInstanceDevicePhysicalPickRequirementNotMatch::ExtensionMissing)),
        }
    }

    unsafe fn check_feature_s(vulkan_instance: &VulkanInstance, vulkan_physical_device: VulkanDevicePhysical)
     -> Result<(), TerminationProcessMain>
    {
        let vulkan_physical_device_feature_s =
            vulkan_instance.get_physical_device_features(vulkan_physical_device);
        match vulkan_physical_device_feature_s.sampler_anisotropy == VULKAN_TRUE {
            true => Ok(()),
            false => Err(TerminationProcessMain::InitializationVulkanPhysicalDeviceFeatureSamplerAnisotropyNotSupport),
        }
    }
}