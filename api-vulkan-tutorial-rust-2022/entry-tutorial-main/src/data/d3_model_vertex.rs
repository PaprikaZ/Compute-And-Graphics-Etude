use std::mem::size_of;

use ::nalgebra_glm as glm;
use ::vulkan::VulkanBuilderHas;
use ::vulkan::VulkanVertexInputBindingDescription;
use ::vulkan::VulkanVertexInputAttributeDescription;
use ::vulkan::VulkanVertexInputRate;
use ::vulkan::VulkanFormat;

use crate::lib::vertex::Vertex;


pub struct DataD3ModelVulkanVertexInput {}

impl DataD3ModelVulkanVertexInput {
    pub fn get_binding_descrption()
     -> VulkanVertexInputBindingDescription
    {
        VulkanVertexInputBindingDescription::builder()
        .binding(0)
        .stride(size_of::<Vertex>() as u32)
        .input_rate(VulkanVertexInputRate::VERTEX)
        .build()
}

    fn get_attribute_description_position()
     -> VulkanVertexInputAttributeDescription
    {
        VulkanVertexInputAttributeDescription::builder()
        .binding(0)
        .location(0)
        .format(VulkanFormat::R32G32B32_SFLOAT)
        .offset(0)
        .build()
    }

    fn get_attribute_description_color()
     -> VulkanVertexInputAttributeDescription
    {
        VulkanVertexInputAttributeDescription::builder()
        .binding(0)
        .location(1)
        .format(VulkanFormat::R32G32B32_SFLOAT)
        .offset(size_of::<glm::Vec3>() as u32)
        .build()
    }

    fn get_attribute_description_texture_coordinate()
     -> VulkanVertexInputAttributeDescription
    {
        VulkanVertexInputAttributeDescription::builder()
        .binding(0)
        .location(2)
        .format(VulkanFormat::R32G32_SFLOAT)
        .offset((size_of::<glm::Vec3>() + size_of::<glm::Vec3>()) as u32)
        .build()
    }

    pub fn get_attribute_description_s()
     -> (VulkanVertexInputAttributeDescription, VulkanVertexInputAttributeDescription,
         VulkanVertexInputAttributeDescription)
    {
        (Self::get_attribute_description_position(),
         Self::get_attribute_description_color(),
         Self::get_attribute_description_texture_coordinate())
    }
}