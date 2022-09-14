use std::mem::size_of;

use ::nalgebra_glm as glm;
use ::vulkan::VulkanBuilderHas;
use ::vulkan::VulkanVertexInputBindingDescription;
use ::vulkan::VulkanVertexInputAttributeDescription;
use ::vulkan::VulkanVertexInputRate;
use ::vulkan::VulkanFormat;

use crate::lib::vertex::Vertex;


pub struct DataVertex {}

impl DataVertex {
    pub fn get_default() -> Vec<Vertex> {
        vec![
            Vertex::new(glm::vec2(-0.5, -0.5), glm::vec3(1.0, 0.0, 0.0), glm::vec2(1.0, 0.0)),
            Vertex::new(glm::vec2(0.5, -0.5), glm::vec3(0.0, 1.0, 0.0), glm::vec2(0.0, 0.0)),
            Vertex::new(glm::vec2(0.5, 0.5), glm::vec3(0.0, 0.0, 1.0), glm::vec2(0.0, 1.0)),
            Vertex::new(glm::vec2(-0.5, 0.5), glm::vec3(1.0, 1.0, 1.0), glm::vec2(1.0, 1.0)),
        ]
    }

    pub fn get_input_binding_descrption() -> VulkanVertexInputBindingDescription {
        VulkanVertexInputBindingDescription::builder()
        .binding(0)
        .stride(size_of::<Vertex>() as u32)
        .input_rate(VulkanVertexInputRate::VERTEX)
        .build()
    }

    fn get_input_attribute_description_position() -> VulkanVertexInputAttributeDescription {
        VulkanVertexInputAttributeDescription::builder()
        .binding(0)
        .location(0)
        .format(VulkanFormat::R32G32_SFLOAT)
        .offset(0)
        .build()
    }

    fn get_input_attribute_description_color() -> VulkanVertexInputAttributeDescription {
        VulkanVertexInputAttributeDescription::builder()
        .binding(0)
        .location(1)
        .format(VulkanFormat::R32G32B32_SFLOAT)
        .offset(size_of::<glm::Vec2>() as u32)
        .build()
    }

    fn get_input_attribute_description_texture_coordinate() -> VulkanVertexInputAttributeDescription {
        VulkanVertexInputAttributeDescription::builder()
        .binding(0)
        .location(2)
        .format(VulkanFormat::R32G32_SFLOAT)
        .offset((size_of::<glm::Vec2>() + size_of::<glm::Vec3>()) as u32)
        .build()
    }

    pub fn get_input_attribute_description()
     -> (VulkanVertexInputAttributeDescription, VulkanVertexInputAttributeDescription,
         VulkanVertexInputAttributeDescription)
    {
        (Self::get_input_attribute_description_position(),
         Self::get_input_attribute_description_color(),
         Self::get_input_attribute_description_texture_coordinate())
    }
}

pub struct DataVertexIndex {}

impl DataVertexIndex {
    pub fn get_default() -> Vec<u16> {
        vec![0, 1, 2, 2, 3, 0]
    }
}