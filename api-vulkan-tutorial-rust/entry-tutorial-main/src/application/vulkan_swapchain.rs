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
use ::vulkan::extend::VulkanQueueFamilyIndexGraphic;
use ::vulkan::extend::VulkanQueueFamilyIndexSurface;
use ::vulkan::extend::VulkanErrorCode;
use ::vulkan::VulkanSharingMode;
use ::vulkan::VulkanSwapchainCreateInformationKhr;
use ::vulkan::VulkanImageUsageFlagS;
use ::vulkan::VulkanCompositeAlphaFlagSKhr;
use ::vulkan::extend::VulkanSwapchainImageCount;

use crate::termination::TerminationProcessMain;


pub struct ApplicationVulkanSwapchain {}

impl ApplicationVulkanSwapchain {
    pub unsafe fn create(
        window: &WindowUniformWindow,
        vulkan_instance: &VulkanInstance,
        vulkan_surface: VulkanSurfaceKhr,
        vulkan_logical_device: &VulkanDeviceLogical,
        vulkan_physical_device: VulkanDevicePhysical,
        vulkan_graphic_queue_family_index: VulkanQueueFamilyIndexGraphic,
        vulkan_surface_queue_family_index: VulkanQueueFamilyIndexSurface)
     -> Result<(VulkanFormat, VulkanExtentD2, VulkanSwapchainKhr, Vec<VulkanImage>),
               TerminationProcessMain>
    {
        let (vulkan_surface_capability_s, vulkan_surface_format_s, vulkan_present_mode_s) =
            Self::get_surface_support(vulkan_instance, vulkan_surface, vulkan_physical_device)?;
        let vulkan_surface_format = Self::pick_surface_format(&vulkan_surface_format_s);
        let vulkan_present_mode = Self::pick_present_mode(&vulkan_present_mode_s);
        let vulkan_extent = Self::pick_extent(window, vulkan_surface_capability_s);
        let vulkan_swapchain_image_count = Self::get_image_count(&vulkan_surface_capability_s);
        let (vulkan_swapchain_image_sharing_mode, vulkan_queue_family_index_s) =
            Self::get_sharing_mode(vulkan_graphic_queue_family_index, vulkan_surface_queue_family_index);
        //
        let vulkan_swapchain_create_information =
            VulkanSwapchainCreateInformationKhr::builder()
            .surface(vulkan_surface)
            .min_image_count(vulkan_swapchain_image_count.as_raw())
            .image_format(vulkan_surface_format.format)
            .image_color_space(vulkan_surface_format.color_space)
            .image_extent(vulkan_extent)
            .image_array_layers(1)
            .image_usage(VulkanImageUsageFlagS::COLOR_ATTACHMENT)
            .image_sharing_mode(vulkan_swapchain_image_sharing_mode)
            .queue_family_indices(&vulkan_queue_family_index_s)
            .pre_transform(vulkan_surface_capability_s.current_transform)
            .composite_alpha(VulkanCompositeAlphaFlagSKhr::OPAQUE)
            .present_mode(vulkan_present_mode)
            .clipped(true)
            .old_swapchain(VulkanSwapchainKhr::null());
        let create_vulkan_swapchain_result =
            vulkan_logical_device.create_swapchain_khr(&vulkan_swapchain_create_information, None);
        let vulkan_swapchain =
            match create_vulkan_swapchain_result {
                Err(error) => {
                    let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                    return Err(TerminationProcessMain::InitializationVulkanSwapchainCreateFail(vulkan_error_code));
                },
                Ok(swapchain) => swapchain,
            };
        let get_vulkan_swapchain_image_s_result =
            vulkan_logical_device.get_swapchain_images_khr(vulkan_swapchain);
        let vulkan_swapchain_image_s =
            match get_vulkan_swapchain_image_s_result {
                Err(error) => {
                    let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                    return Err(TerminationProcessMain::InitializationVulkanSwapchainImageSGetFail(vulkan_error_code));
                },
                Ok(image_s) => image_s,
            };
        Ok((vulkan_surface_format.format, vulkan_extent, vulkan_swapchain, vulkan_swapchain_image_s))
    }

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

    fn pick_surface_format(vulkan_surface_format_s: &[VulkanSurfaceFormatKhr]) -> VulkanSurfaceFormatKhr {
        vulkan_surface_format_s
        .iter()
        .cloned()
        .find(|f|
            f.format == VulkanFormat::B8G8R8A8_SRGB &&
            f.color_space == VulkanColorSpaceKhr::SRGB_NONLINEAR)
        .unwrap_or_else(|| vulkan_surface_format_s[0])
    }

    fn pick_present_mode(vulkan_present_mode_s: &[VulkanPresentModeKhr]) -> VulkanPresentModeKhr {
        vulkan_present_mode_s
        .iter()
        .cloned()
        .find(|m| *m == VulkanPresentModeKhr::MAILBOX)
        .unwrap_or(VulkanPresentModeKhr::FIFO)
    }

    fn pick_extent(
        window: &WindowUniformWindow, vulkan_surface_capability_s: VulkanSurfaceCapabilitySKhr)
     -> VulkanExtentD2
    {
        if vulkan_surface_capability_s.current_extent.width == u32::max_value() {
            let window_inner_size = window.inner_size();
            let clamp = |min: u32, max: u32, n: u32| min.max(max.min(n));
            VulkanExtentD2::builder()
            .width(clamp(
                vulkan_surface_capability_s.min_image_extent.width,
                vulkan_surface_capability_s.max_image_extent.width,
                window_inner_size.width
            ))
            .height(clamp(
                vulkan_surface_capability_s.min_image_extent.height,
                vulkan_surface_capability_s.max_image_extent.height,
                window_inner_size.height
            ))
            .build()
        } else {
            vulkan_surface_capability_s.current_extent
        }
    }

    fn get_image_count(vulkan_surface_capability_s: &VulkanSurfaceCapabilitySKhr) -> VulkanSwapchainImageCount {
        let prefer_vulkan_swapchain_image_count = vulkan_surface_capability_s.min_image_count + 1;
        let picked_vulkan_swapchain_image_count =
            if vulkan_surface_capability_s.max_image_count != 0 {
                prefer_vulkan_swapchain_image_count.min(vulkan_surface_capability_s.max_image_count)
            } else {
                prefer_vulkan_swapchain_image_count
            };
        VulkanSwapchainImageCount::new(picked_vulkan_swapchain_image_count)
    }

    fn get_sharing_mode(
        vulkan_graphic_queue_family_index: VulkanQueueFamilyIndexGraphic,
        vulkan_surface_queue_family_index: VulkanQueueFamilyIndexSurface)
     -> (VulkanSharingMode, Vec<u32>)
    {
        let mut vulkan_queue_family_index_s = vec![];
        if vulkan_graphic_queue_family_index.as_raw() == vulkan_surface_queue_family_index.as_raw() {
            (VulkanSharingMode::EXCLUSIVE, vulkan_queue_family_index_s)
        } else {
            vulkan_queue_family_index_s.push(vulkan_graphic_queue_family_index.as_raw());
            vulkan_queue_family_index_s.push(vulkan_surface_queue_family_index.as_raw());
            (VulkanSharingMode::CONCURRENT, vulkan_queue_family_index_s)
        }
    }
}
