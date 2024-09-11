use std::path::Path;

use ::library_foundation_reintroduction::nalgebra_glm as glm;
use ::library_foundation_reintroduction::vulkan::VulkanBuffer;
use ::library_foundation_reintroduction::vulkan::VulkanDeviceMemory;
use ::library_foundation_reintroduction::vulkan::VulkanBuilderHas;
use ::library_foundation_reintroduction::vulkan::VulkanVertexInputBindingDescription;
use ::library_foundation_reintroduction::vulkan::VulkanVertexInputAttributeDescription;
use ::library_foundation_reintroduction::vulkan::VulkanVertexInputRate;
use ::library_foundation_reintroduction::vulkan::VulkanFormat;
use ::library_foundation_vulkan_cooked::graphic_mesh_vertex_index::GraphicMeshVertexIndexU16;
use ::library_foundation_vulkan_cooked::vulkan_vertex_input::description::VulkanVertexInputDescriptionS;
use ::library_foundation_vulkan_cooked::graphic_mesh_vertex::normal::GraphicMeshVertexNormal;
use ::library_foundation_vulkan_cooked::graphic_mesh_vertex::normal_color::GraphicMeshVertexNormalColor;
use ::library_foundation_vulkan_cooked::graphic_mesh_loader::GraphicMeshLoader;
use ::library_foundation_vulkan_cooked::vulkan_memory_raw_prefab::allocator::VulkanMemoryRawPrefabAllocator;

use crate::error::foundation_application_guide::ErrorFoundationApplicationGuide;
use crate::application_v1_1_c3::config::ApplicationConfig;


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


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ApplicationGraphicMeshName {
    Triangle,
    Monkey,
}

impl ApplicationGraphicMeshName {
}


pub struct ApplicationGraphicMeshLoader {}

impl ApplicationGraphicMeshLoader {
    pub fn create_mesh_triangle() -> ApplicationGraphicMeshDeviceLoadedN {
        let mut vertex_s: Vec<GraphicMeshVertexNormalColor> = Vec::new();
        vertex_s.reserve_exact(3);
        vertex_s.push(GraphicMeshVertexNormalColor::new(
            glm::vec3(1.0, 1.0, 0.0),
            glm::vec3(0.0, 0.0, 0.0),
            glm::vec3(0.0, 1.0, 0.0)));
        vertex_s.push(GraphicMeshVertexNormalColor::new(
            glm::vec3(-1.0, 1.0, 0.0),
            glm::vec3(0.0, 0.0, 0.0),
            glm::vec3(0.0, 1.0, 0.0)));
        vertex_s.push(GraphicMeshVertexNormalColor::new(
            glm::vec3(0.0, -1.0, 0.0),
            glm::vec3(0.0, 0.0, 0.0),
            glm::vec3(0.0, 1.0, 0.0)));
        ApplicationGraphicMeshDeviceLoadedN::new(vertex_s)
    }

    pub fn load_mesh_monkey(config: &ApplicationConfig)
    -> Result<ApplicationGraphicMeshDeviceLoadedN, ErrorFoundationApplicationGuide>
    {
        let file_path =
            config.path_directory_graphic_mesh.join(config.file_name_graphic_mesh_monkey.clone());
        ApplicationGraphicMeshDeviceLoadedN::load_from_file_obj(file_path.as_path())
    }

    pub fn create_vulkan_vertex_input_description_s() -> VulkanVertexInputDescriptionS
    {
        let mut vulkan_vertex_input_description_s = VulkanVertexInputDescriptionS::new_empty();
        Self::add_vulkan_vertex_input_description_main(&mut vulkan_vertex_input_description_s);
        vulkan_vertex_input_description_s
    }

    fn add_vulkan_vertex_input_description_main(
        vulkan_vertex_input_description_s: &mut VulkanVertexInputDescriptionS)
    {
        let vulkan_vertex_input_binding_description =
            VulkanVertexInputBindingDescription::builder()
            .binding(0)
            .stride(size_of::<GraphicMeshVertexNormalColor>() as u32)
            .input_rate(VulkanVertexInputRate::VERTEX)
            .build();
        let position_vulkan_vertex_input_attribute_description =
            VulkanVertexInputAttributeDescription::builder()
            .binding(0)
            .location(0)
            .format(VulkanFormat::R32G32B32_SFLOAT)
            .offset(0)
            .build();
        let normal_vulkan_vertex_input_attribute_description =
            VulkanVertexInputAttributeDescription::builder()
            .binding(0)
            .location(1)
            .format(VulkanFormat::R32G32B32_SFLOAT)
            .offset(size_of::<glm::Vec3>() as u32)
            .build();
        let color_vulkan_vertex_input_attribute_description =
            VulkanVertexInputAttributeDescription::builder()
            .binding(0)
            .location(2)
            .format(VulkanFormat::R32G32B32_SFLOAT)
            .offset((size_of::<glm::Vec3>() + size_of::<glm::Vec3>()) as u32)
            .build();
        let vulkan_vertex_input_attribute_description_s =
            vec!(position_vulkan_vertex_input_attribute_description,
                 normal_vulkan_vertex_input_attribute_description,
                 color_vulkan_vertex_input_attribute_description);
        vulkan_vertex_input_description_s.add_binding_and_attribute_s(
            vulkan_vertex_input_binding_description, vulkan_vertex_input_attribute_description_s);
    }
}