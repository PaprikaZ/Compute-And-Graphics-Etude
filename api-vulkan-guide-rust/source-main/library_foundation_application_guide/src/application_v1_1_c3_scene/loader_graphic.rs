use ::library_foundation_reintroduction::nalgebra_glm as glm;
use ::library_foundation_reintroduction::vulkan::VulkanBuilderHas;
use ::library_foundation_reintroduction::vulkan::VulkanVertexInputBindingDescription;
use ::library_foundation_reintroduction::vulkan::VulkanVertexInputAttributeDescription;
use ::library_foundation_reintroduction::vulkan::VulkanVertexInputRate;
use ::library_foundation_reintroduction::vulkan::VulkanFormat;
use ::library_foundation_vulkan_cooked::graphic_mesh_vertex::normal_color::GraphicMeshVertexNormalColor;
use ::library_foundation_vulkan_cooked::vulkan_memory_raw_prefab::allocator::VulkanMemoryRawPrefabAllocator;
use ::library_foundation_vulkan_cooked::vulkan_vertex_input::description::VulkanVertexInputDescriptionS;

use crate::error::foundation_application_guide::ErrorFoundationApplicationGuide;
use crate::application_v1_1_c3_scene::config::ApplicationConfig;
use crate::application_v1_1_c3_scene::graphic_resource::ApplicationGraphicResourceDestroyDirective;
use crate::application_v1_1_c3_scene::graphic_resource::ApplicationGraphicResourceDestroyStack;
use crate::application_v1_1_c3_scene::graphic_mesh::ApplicationGraphicMeshDeviceLoadedN;
use crate::application_v1_1_c3_scene::graphic_mesh::ApplicationGraphicMeshDeviceLoadedY;


impl ApplicationGraphicMeshDeviceLoadedN {
    pub(super) fn load_to_device(
        self,
        vulkan_memory_allocator: &VulkanMemoryRawPrefabAllocator,
        graphic_resource_destroy_stack: &mut ApplicationGraphicResourceDestroyStack)
    -> Result<ApplicationGraphicMeshDeviceLoadedY, ErrorFoundationApplicationGuide>
    {
        type DD = ApplicationGraphicResourceDestroyDirective;
        let (dedicated_device_vulkan_buffer, dedicated_device_vulkan_buffer_memory) =
            vulkan_memory_allocator.allocate_buffer_graphic_mesh(&self.vertex_s, None)?;
        graphic_resource_destroy_stack.push(DD::DestroyVulkanBuffer(dedicated_device_vulkan_buffer));
        graphic_resource_destroy_stack.push(DD::FreeVulkanDeviceMemory(dedicated_device_vulkan_buffer_memory));
        Ok(ApplicationGraphicMeshDeviceLoadedY::new(
            self.vertex_s, dedicated_device_vulkan_buffer, dedicated_device_vulkan_buffer_memory))
    }
}


pub struct ApplicationLoaderGraphic {}

impl ApplicationLoaderGraphic {
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
            .offset(std::mem::offset_of!(GraphicMeshVertexNormalColor, position) as u32)
            .build();
        let normal_vulkan_vertex_input_attribute_description =
            VulkanVertexInputAttributeDescription::builder()
            .binding(0)
            .location(1)
            .format(VulkanFormat::R32G32B32_SFLOAT)
            .offset(std::mem::offset_of!(GraphicMeshVertexNormalColor, normal) as u32)
            .build();
        let color_vulkan_vertex_input_attribute_description =
            VulkanVertexInputAttributeDescription::builder()
            .binding(0)
            .location(2)
            .format(VulkanFormat::R32G32B32_SFLOAT)
            .offset(std::mem::offset_of!(GraphicMeshVertexNormalColor, color) as u32)
            .build();
        let vulkan_vertex_input_attribute_description_s =
            vec!(position_vulkan_vertex_input_attribute_description,
                 normal_vulkan_vertex_input_attribute_description,
                 color_vulkan_vertex_input_attribute_description);
        vulkan_vertex_input_description_s.add_binding_and_attribute_s(
            vulkan_vertex_input_binding_description, vulkan_vertex_input_attribute_description_s);
    }
}