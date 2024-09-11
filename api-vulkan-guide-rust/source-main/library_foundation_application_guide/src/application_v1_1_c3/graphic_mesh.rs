use ::library_foundation_reintroduction::vulkan::VulkanBuffer;
use ::library_foundation_reintroduction::vulkan::VulkanDeviceMemory;
use ::library_foundation_reintroduction::vulkan::VulkanBuilderHas;
use ::library_foundation_vulkan_cooked::graphic_mesh_vertex::normal_color::GraphicMeshVertexNormalColor;


#[derive(Debug, Clone)]
pub struct ApplicationGraphicMeshDeviceLoadedY {
    pub vertex_s: Vec<GraphicMeshVertexNormalColor>,
    pub vulkan_buffer: VulkanBuffer,
    pub vulkan_buffer_memory: VulkanDeviceMemory,
}

impl ApplicationGraphicMeshDeviceLoadedY {
    pub fn new(
        vertex_s: Vec<GraphicMeshVertexNormalColor>,
        vulkan_buffer: VulkanBuffer,
        vulkan_buffer_memory: VulkanDeviceMemory)
    -> Self
    {
        Self {
            vertex_s: vertex_s,
            vulkan_buffer: vulkan_buffer,
            vulkan_buffer_memory: vulkan_buffer_memory,
        }
    }
}