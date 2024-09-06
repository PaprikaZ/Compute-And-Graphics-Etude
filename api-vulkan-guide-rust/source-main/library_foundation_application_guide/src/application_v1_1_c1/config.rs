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
    pub fn new(
        window_title: &'t str,
        window_inner_size: WindowUniformDpiLogicalSize<i32>,
        base_vulkan_config: ApplicationConfigVulkan<'t>,
        base_rank_vulkan_config: ApplicationConfigVulkanRank,
        swapchain_vulkan_config: ApplicationConfigVulkanSwapchain)
    -> Self
    {
        Self {
            window_title: window_title,
            window_inner_size: window_inner_size,
            vulkan: base_vulkan_config,
            vulkan_rank: base_rank_vulkan_config,
            vulkan_swapchain: swapchain_vulkan_config,
        }
    }
}