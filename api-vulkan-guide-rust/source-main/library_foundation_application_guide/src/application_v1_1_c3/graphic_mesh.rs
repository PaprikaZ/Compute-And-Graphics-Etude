use std::path::Path;

use ::library_foundation_reintroduction::nalgebra_glm as glm;
use ::library_foundation_reintroduction::vulkan::VulkanBuffer;
use ::library_foundation_reintroduction::vulkan::VulkanDeviceMemory;
use ::library_foundation_reintroduction::vulkan::VulkanBuilderHas;
use ::library_foundation_vulkan_cooked::graphic_mesh_vertex_index::GraphicMeshVertexIndexU16;
use ::library_foundation_vulkan_cooked::graphic_mesh_vertex::normal::GraphicMeshVertexNormal;
use ::library_foundation_vulkan_cooked::graphic_mesh_vertex::normal_color::GraphicMeshVertexNormalColor;
use ::library_foundation_vulkan_cooked::graphic_mesh_loader::GraphicMeshLoader;
use ::library_foundation_vulkan_cooked::vulkan_memory_raw_prefab::allocator::VulkanMemoryRawPrefabAllocator;

use crate::error::foundation_application_guide::ErrorFoundationApplicationGuide;


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


#[derive(Debug, Clone)]
pub struct ApplicationGraphicMeshDeviceLoadedN {
    pub vertex_s: Vec<GraphicMeshVertexNormalColor>,
}

impl ApplicationGraphicMeshDeviceLoadedN {
    pub fn new(vertex_s: Vec<GraphicMeshVertexNormalColor>) -> Self {
        Self {
            vertex_s: vertex_s,
        }
    }

    pub fn load_from_file_obj(file_path: &Path) -> Result<Self, ErrorFoundationApplicationGuide>
    {
        let (graphic_mesh_vertex_s, _graphic_mesh_vertex_index_s) =
            GraphicMeshLoader::<GraphicMeshVertexNormal>::load_from_path_file_obj::<GraphicMeshVertexIndexU16>(
                file_path, Box::new(|index| GraphicMeshVertexIndexU16::new(index as u16)))?;
        let graphic_mesh_vertex_s =
            graphic_mesh_vertex_s
            .into_iter()
            .map(|vertex| GraphicMeshVertexNormalColor::new(vertex.position, vertex.normal, glm::vec3(1.0, 1.0, 1.0)))
            .collect::<Vec<_>>();
        Ok(Self::new(graphic_mesh_vertex_s))
    }

    pub fn load_to_device(self, allocator: &VulkanMemoryRawPrefabAllocator)
    -> Result<ApplicationGraphicMeshDeviceLoadedY, ErrorFoundationApplicationGuide>
    {
        let (dedicated_device_buffer, dedidcated_device_buffer_memory) =
            allocator.allocate_buffer_graphic_mesh(&self.vertex_s, None)?;
        Ok(ApplicationGraphicMeshDeviceLoadedY::new(
            self.vertex_s, dedicated_device_buffer, dedidcated_device_buffer_memory))
    }
}