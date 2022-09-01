use vulkan::VulkanSurfaceExtensionKhr;
use vulkan::VulkanSwapchainExtensionKhr;
use ::window_uniform::WindowUniformWindow;
use ::vulkan::prelude::version1_2::*;
use ::vulkan::VulkanInstance;
use ::vulkan::VulkanSurfaceKhr;
use ::vulkan::VulkanDevicePhysical;
use ::vulkan::VulkanFormat;
use ::vulkan::VulkanColorSpaceKhr;
use ::vulkan::VulkanExtentD2;
use ::vulkan::VulkanSwapchainKhr;
use ::vulkan::VulkanImage;
use ::vulkan::VulkanSurfaceCapabilitySKhr;
use ::vulkan::VulkanSurfaceFormatKhr;
use ::vulkan::VulkanPresentModeKhr;
use ::vulkan::VulkanQueueFamilyIndexGraphic;
use ::vulkan::VulkanQueueFamilyIndexSurface;
use ::vulkan::VulkanErrorCode;
use ::vulkan::VulkanSharingMode;
use ::vulkan::VulkanSwapchainCreateInformationKhr;
use ::vulkan::VulkanImageUsageFlagS;
use ::vulkan::VulkanCompositeAlphaFlagSKhr;
use ::vulkan::VulkanSwapchainImageCount;

use crate::termination::TerminationProcessMain;


pub struct ApplicationVulkanInstanceSwapchain {}

impl ApplicationVulkanInstanceSwapchain {
    pub unsafe fn get_surface_support(
        vulkan_instance: &VulkanInstance,
        vulkan_surface: VulkanSurfaceKhr,
        vulkan_physical_device: VulkanDevicePhysical)
     -> Result<(VulkanSurfaceCapabilitySKhr, Vec<VulkanSurfaceFormatKhr>, Vec<VulkanPresentModeKhr>),
               TerminationProcessMain>
    {
        let get_vulkan_physical_device_surface_capability_s_result =
            vulkan_instance.get_physical_device_surface_capabilities_khr(vulkan_physical_device, vulkan_surface);
        let vulkan_physical_device_capability_s =
            match get_vulkan_physical_device_surface_capability_s_result {
                Err(error) => {
                    let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                    return Err(TerminationProcessMain::InitializationVulkanDevicePhysicalSurfaceCapabilitySGetFail(vulkan_error_code));
                },
                Ok(capability_s) => capability_s,
            };
        let get_vulkan_physical_device_surface_format_s_result =
            vulkan_instance.get_physical_device_surface_formats_khr(vulkan_physical_device, vulkan_surface);
        let vulkan_physical_device_surface_format_s =
            match get_vulkan_physical_device_surface_format_s_result {
                Err(error) => {
                    let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                    return Err(TerminationProcessMain::InitializationVulkanDevicePhysicalSurfaceFormatSGetFail(vulkan_error_code));
                },
                Ok(format_s) => format_s,
            };
        let get_vulkan_physical_device_surface_present_mode_s_result =
            vulkan_instance.get_physical_device_surface_present_modes_khr(vulkan_physical_device, vulkan_surface);
        let vulkan_physical_device_surface_present_mode_s =
            match get_vulkan_physical_device_surface_present_mode_s_result {
                Err(error) => {
                    let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                    return Err(TerminationProcessMain::InitializationVulkanDevicePhysicalSurfacePresentModeSGetFail(vulkan_error_code));
                },
                Ok(present_mode_s) => present_mode_s,
            };
        Ok((vulkan_physical_device_capability_s,
            vulkan_physical_device_surface_format_s,
            vulkan_physical_device_surface_present_mode_s))
    }
}
