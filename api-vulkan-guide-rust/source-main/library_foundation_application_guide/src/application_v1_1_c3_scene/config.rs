use std::path::PathBuf;

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
    //
    pub path_directory_shader: PathBuf,
    pub file_name_shader_main_vertex: PathBuf,
    pub file_name_shader_main_fragment: PathBuf,
    //
    pub path_directory_graphic_mesh: PathBuf,
    pub file_name_graphic_mesh_monkey: PathBuf,
}

impl<'t> ApplicationConfig<'t> {
    pub fn new(
        window_title: &'t str,
        window_inner_size: WindowUniformDpiLogicalSize<i32>,
        base_vulkan_config: ApplicationConfigVulkan<'t>,
        rank_vulkan_config: ApplicationConfigVulkanRank,
        swapchain_vulkan_config: ApplicationConfigVulkanSwapchain,
        //
        shader_source_directory_path: PathBuf,
        main_vertex_shader_file_name: PathBuf,
        main_fragment_shader_file_name: PathBuf,
        //
        graphic_mesh_directory_path: PathBuf,
        monkey_graphic_mesh_file_name: PathBuf)
    -> Self
    {
        Self {
            window_title: window_title,
            window_inner_size: window_inner_size,
            vulkan: base_vulkan_config,
            vulkan_rank: rank_vulkan_config,
            vulkan_swapchain: swapchain_vulkan_config,
            //
            path_directory_shader: shader_source_directory_path,
            file_name_shader_main_vertex: main_vertex_shader_file_name,
            file_name_shader_main_fragment: main_fragment_shader_file_name,
            //
            path_directory_graphic_mesh: graphic_mesh_directory_path,
            file_name_graphic_mesh_monkey: monkey_graphic_mesh_file_name,
        }
    }
}