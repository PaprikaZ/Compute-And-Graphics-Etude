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


enum ApplicationVulkanInstanceDevicePhysicalQueueNotSupport {
    Graphic,
    Surface,
}

pub struct ApplicationVulkanInstanceDevicePhysical {}

impl ApplicationVulkanInstanceDevicePhysical {
    pub unsafe fn pick(vulkan_instance: &VulkanInstance, vulkan_surface: VulkanSurfaceKhr)
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
            match Self::check(vulkan_instance, vulkan_physical_device, vulkan_surface) {
                Err(ApplicationVulkanInstanceDevicePhysicalQueueNotSupport::Graphic) => {
                    console_log_warn!(
                        "Physical device (`{}`) skipping: missing graphic queue family",
                        physical_device_property_s.device_name);
                },
                Err(ApplicationVulkanInstanceDevicePhysicalQueueNotSupport::Surface) => {
                    console_log_warn!(
                        "Physical device (`{}`) skipping: missing surface queue family",
                        physical_device_property_s.device_name);
                },
                Ok((graphic_queue_index, surface_queue_index)) => {
                    console_log_info!("Physical device (`{}`) selected", physical_device_property_s.device_name);
                    return Ok((vulkan_physical_device, graphic_queue_index, surface_queue_index));
                }
            };
        }
        Err(TerminationProcessMain::InitializationVulkanDevicePhysicalAllQueueFamilyQualifiedNotSupport)
    }

    unsafe fn check(
        vulkan_instance: &VulkanInstance,
        vulkan_physical_device: VulkanDevicePhysical,
        vulkan_surface: VulkanSurfaceKhr)
     -> Result<(VulkanQueueFamilyIndexGraphic, VulkanQueueFamilyIndexSurface),
               ApplicationVulkanInstanceDevicePhysicalQueueNotSupport>
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
            (None, _) => Err(ApplicationVulkanInstanceDevicePhysicalQueueNotSupport::Graphic),
            (_, None) => Err(ApplicationVulkanInstanceDevicePhysicalQueueNotSupport::Surface),
            (Some(graphic_index), Some(surface_index)) =>
                Ok((graphic_index, surface_index)),
        }
    }
}