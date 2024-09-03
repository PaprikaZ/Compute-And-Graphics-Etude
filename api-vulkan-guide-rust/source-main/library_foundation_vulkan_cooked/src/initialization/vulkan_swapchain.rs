use ::library_foundation_reintroduction::vulkan::VulkanDeviceVersion1_0;
use ::library_foundation_reintroduction::vulkan::VulkanHandler;
use ::library_foundation_reintroduction::vulkan::VulkanSwapchainExtensionKhr;
use ::library_foundation_reintroduction::vulkan::VulkanBuilderHas;
use ::library_foundation_reintroduction::vulkan::VulkanSurfaceKhr;
use ::library_foundation_reintroduction::vulkan::VulkanDeviceLogical;
use ::library_foundation_reintroduction::vulkan::VulkanExtentD2;
use ::library_foundation_reintroduction::vulkan::VulkanSwapchainKhr;
use ::library_foundation_reintroduction::vulkan::VulkanSurfaceCapabilitySKhr;
use ::library_foundation_reintroduction::vulkan::VulkanSurfaceFormatKhr;
use ::library_foundation_reintroduction::vulkan::VulkanPresentModeKhr;
use ::library_foundation_reintroduction::vulkan::VulkanSharingMode;
use ::library_foundation_reintroduction::vulkan::VulkanSwapchainCreateInformationKhr;
use ::library_foundation_reintroduction::vulkan::VulkanImageUsageFlagS;
use ::library_foundation_reintroduction::vulkan::VulkanImage;
use ::library_foundation_reintroduction::vulkan::VulkanImageView;
use ::library_foundation_reintroduction::vulkan::VulkanFormat;
use ::library_foundation_reintroduction::vulkan::VulkanCompositeAlphaFlagSKhr;
use ::library_foundation_reintroduction::vulkan::VulkanComponentSwizzle;
use ::library_foundation_reintroduction::vulkan::VulkanComponentMapping;
use ::library_foundation_reintroduction::vulkan::VulkanImageSubResourceRange;
use ::library_foundation_reintroduction::vulkan::VulkanImageAspectFlagS;
use ::library_foundation_reintroduction::vulkan::VulkanImageViewType;
use ::library_foundation_reintroduction::vulkan::VulkanImageViewCreateInformation;
use ::library_foundation_reintroduction::vulkan::swapchain::VulkanSwapchainImageNumber;
use ::library_foundation_reintroduction::vulkan::queue::VulkanQueueFamilyIndex;

use crate::error::foundation_vulkan_cooked::ErrorFoundationVulkanCookedOwn;
use crate::error::foundation_vulkan_cooked::ErrorFoundationVulkanCooked;


pub struct InitializationVulkanSwapchain {}

impl InitializationVulkanSwapchain {
    pub fn initialize(
        vulkan_surface: VulkanSurfaceKhr,
        vulkan_logical_device: &VulkanDeviceLogical,
        vulkan_surface_capability_s: VulkanSurfaceCapabilitySKhr,
        vulkan_sharing_mode: VulkanSharingMode,
        vulkan_queue_family_index_s: &Vec<VulkanQueueFamilyIndex>,
        vulkan_swapchain_image_number: VulkanSwapchainImageNumber,
        vulkan_2d_extent: VulkanExtentD2,
        vulkan_surface_format: VulkanSurfaceFormatKhr,
        vulkan_present_mode: VulkanPresentModeKhr,
        old_vulkan_swapchain_o: Option<VulkanSwapchainKhr>)
    -> Result<VulkanSwapchainKhr, ErrorFoundationVulkanCooked>
    {
        let raw_queue_family_index_s =
            vulkan_queue_family_index_s
            .iter()
            .map(|i| i.as_raw())
            .collect::<Vec<_>>();
        let create_information =
            VulkanSwapchainCreateInformationKhr::builder()
            .surface(vulkan_surface)
            .min_image_count(vulkan_swapchain_image_number.as_raw())
            .image_format(vulkan_surface_format.format)
            .image_color_space(vulkan_surface_format.color_space)
            .image_extent(vulkan_2d_extent)
            .image_array_layers(1)
            .image_usage(VulkanImageUsageFlagS::COLOR_ATTACHMENT)
            .image_sharing_mode(vulkan_sharing_mode)
            .queue_family_indices(&raw_queue_family_index_s)
            .pre_transform(vulkan_surface_capability_s.current_transform)
            .composite_alpha(VulkanCompositeAlphaFlagSKhr::OPAQUE)
            .present_mode(vulkan_present_mode)
            .clipped(true)
            .old_swapchain(old_vulkan_swapchain_o.unwrap_or(VulkanSwapchainKhr::null()));
        match unsafe { vulkan_logical_device.create_swapchain_khr(&create_information, None) } {
            Err(_e) => Err(ErrorFoundationVulkanCookedOwn::VulkanSwapchainCreateFail)?,
            Ok(c) => Ok(c),
        }
    }

    pub fn initialize_with_image_s(
        vulkan_surface: VulkanSurfaceKhr,
        vulkan_logical_device: &VulkanDeviceLogical,
        vulkan_surface_capability_s: VulkanSurfaceCapabilitySKhr,
        vulkan_sharing_mode: VulkanSharingMode,
        vulkan_queue_family_index_s: &Vec<VulkanQueueFamilyIndex>,
        vulkan_swapchain_image_number: VulkanSwapchainImageNumber,
        vulkan_2d_extent: VulkanExtentD2,
        vulkan_surface_format: VulkanSurfaceFormatKhr,
        vulkan_present_mode: VulkanPresentModeKhr,
        old_vulkan_swapchain_o: Option<VulkanSwapchainKhr>)
    -> Result<(VulkanSwapchainKhr, Vec<VulkanImage>), ErrorFoundationVulkanCooked>
    {
        let vulkan_swapchain =
            Self::initialize(
                vulkan_surface, vulkan_logical_device, vulkan_surface_capability_s,
                vulkan_sharing_mode, vulkan_queue_family_index_s, vulkan_swapchain_image_number,
                vulkan_2d_extent, vulkan_surface_format, vulkan_present_mode, old_vulkan_swapchain_o)?;
        let vulkan_swapchain_image_s =
            unsafe { vulkan_logical_device.get_swapchain_images_khr(vulkan_swapchain) }
            .or(Err(ErrorFoundationVulkanCookedOwn::VulkanSwapchainImageSGetFail))?;
        Ok((vulkan_swapchain, vulkan_swapchain_image_s))
    }
}
