use ::library_foundation_reintroduction::nalgebra_glm as glm;
use ::library_foundation_reintroduction::vulkan::VulkanPipeline;
use ::library_foundation_reintroduction::vulkan::VulkanPipelineLayout;

use crate::application_v1_1_c3_scene::graphic_mesh::ApplicationGraphicMeshName;


#[derive(Debug)]
pub struct ApplicationScenePipeline {
    pub vulkan_pipeline: VulkanPipeline,
    pub vulkan_pipeline_layout: VulkanPipelineLayout,
}

impl ApplicationScenePipeline {
    pub fn new(vulkan_pipeline: VulkanPipeline, vulkan_pipeline_layout: VulkanPipelineLayout) -> Self
    {
        Self {
            vulkan_pipeline: vulkan_pipeline,
            vulkan_pipeline_layout: vulkan_pipeline_layout,
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ApplicationScenePipelineName {
    Main,
}

impl ApplicationScenePipelineName {
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ApplicationSceneEntityRenderableName {
    Triangle(u32),
    Monkey,
}

impl ApplicationSceneEntityRenderableName {
}


#[derive(Debug, Clone)]
pub struct ApplicationSceneEntityRenderable {
    pub name: ApplicationSceneEntityRenderableName,
    pub pipeline_name: ApplicationScenePipelineName,
    pub graphic_mesh_name: ApplicationGraphicMeshName,
    pub graphic_transform: glm::Mat4,
}

impl ApplicationSceneEntityRenderable {
    pub fn new(
        name: ApplicationSceneEntityRenderableName,
        pipeline_name: ApplicationScenePipelineName,
        graphic_mesh_name: ApplicationGraphicMeshName,
        graphic_transform: glm::Mat4)
    -> Self
    {
        Self {
            name: name,
            pipeline_name: pipeline_name,
            graphic_mesh_name,
            graphic_transform: graphic_transform,
        }
    }
}