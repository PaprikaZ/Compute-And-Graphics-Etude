use ::library_foundation_reintroduction::window_uniform::WindowUniformDpiLogicalSize;
use ::library_foundation_vulkan_cooked::config_vulkan::base::ConfigVulkanBase;
use ::library_foundation_vulkan_cooked::config_vulkan::rank::ConfigVulkanRank;
use ::library_foundation_vulkan_cooked::config_vulkan::swapchain::ConfigVulkanSwapchain;

type ApplicationConfigVulkan<'t> = ConfigVulkanBase<'t>;
type ApplicationConfigVulkanRank = ConfigVulkanRank;
type ApplicationConfigVulkanSwapchain = ConfigVulkanSwapchain;


#[derive(Debug, Clone)]
pub struct ApplicationConfig<'t> {
    pub window_title: &'t str,
    pub window_inner_size: WindowUniformDpiLogicalSize<i32>,
    pub vulkan: ApplicationConfigVulkan<'t>,
    pub vulkan_rank: ApplicationConfigVulkanRank,
    pub vulkan_swapchain: ApplicationConfigVulkanSwapchain,
}

impl<'t> ApplicationConfig<'t> {
}