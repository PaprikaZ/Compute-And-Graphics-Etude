use ::library_foundation_reintroduction::window_uniform::WindowUniformDpiLogicalSize;
use ::library_foundation_vulkan_cooked::config_vulkan::base::ConfigVulkanBase;
use ::library_foundation_vulkan_cooked::config_vulkan::rank::ConfigVulkanRank;

type ApplicationConfigVulkan<'t> = ConfigVulkanBase<'t>;
type ApplicationConfigVulkanRank = ConfigVulkanRank;


#[derive(Debug, Clone)]
pub struct ApplicationConfig<'t> {
    pub window_title: &'t str,
    pub window_inner_size: WindowUniformDpiLogicalSize<i32>,
    pub vulkan: ApplicationConfigVulkan<'t>,
    pub vulkan_rank: ApplicationConfigVulkanRank,
}

impl<'t> ApplicationConfig<'t> {
}

impl<'t> Default for ApplicationConfig<'t> {
    fn default() -> Self {
        todo!()
    }
}