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
    pub file_name_shader_triangle_red_vertex: PathBuf,
    pub file_name_shader_triangle_red_fragment: PathBuf,
    pub file_name_shader_triangle_colored_vertex: PathBuf,
    pub file_name_shader_triangle_colored_dynamic_vertex: PathBuf,
    pub file_name_shader_triangle_colored_fragment: PathBuf,
    //
    pub path_directory_graphic_mesh: PathBuf,
    pub file_name_graphic_mesh_monkey: PathBuf,
}

impl<'t> ApplicationConfig<'t> {
    pub fn new(
        window_title: &'t str,
        window_inner_size: WindowUniformDpiLogicalSize<i32>,
        base_vulkan_config: ApplicationConfigVulkan<'t>,
        base_rank_vulkan_config: ApplicationConfigVulkanRank,
        swapchain_vulkan_config: ApplicationConfigVulkanSwapchain,
        //
        shader_directory_path: PathBuf,
        red_triangle_vertex_shader_file_name: PathBuf,
        red_triangle_fragment_shader_file_name: PathBuf,
        colored_triangle_vertex_shader_file_name: PathBuf,
        dynamic_colored_triangle_vertex_shader_file_name: PathBuf,
        colored_triangle_fragment_shader_file_name: PathBuf,
        //
        graphic_mesh_directory_path: PathBuf,
        monkey_graphic_mesh_file_name: PathBuf)
    -> Self
    {
        Self {
            window_title: window_title,
            window_inner_size: window_inner_size,
            vulkan: base_vulkan_config,
            vulkan_rank: base_rank_vulkan_config,
            vulkan_swapchain: swapchain_vulkan_config,
            //
            path_directory_shader: shader_directory_path,
            file_name_shader_triangle_red_vertex: red_triangle_vertex_shader_file_name,
            file_name_shader_triangle_red_fragment: red_triangle_fragment_shader_file_name,
            file_name_shader_triangle_colored_vertex: colored_triangle_vertex_shader_file_name,
            file_name_shader_triangle_colored_dynamic_vertex: dynamic_colored_triangle_vertex_shader_file_name,
            file_name_shader_triangle_colored_fragment: colored_triangle_fragment_shader_file_name,
            //
            path_directory_graphic_mesh: graphic_mesh_directory_path,
            file_name_graphic_mesh_monkey: monkey_graphic_mesh_file_name,
        }
    }
}