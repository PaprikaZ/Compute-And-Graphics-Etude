use std::slice::Iter;
use std::collections::HashMap;
use std::collections::HashSet;

use ::library_foundation_reintroduction::nalgebra_glm as glm;
use ::library_foundation_reintroduction::vulkan::VulkanPipeline;
use ::library_foundation_reintroduction::vulkan::VulkanPipelineLayout;

use crate::error::foundation_application_guide::ErrorFoundationApplicationGuideOwn;
use crate::error::foundation_application_guide::ErrorFoundationApplicationGuide;
use crate::application_v1_1_c3_scene::graphic_mesh::ApplicationGraphicMeshName;
use crate::application_v1_1_c3_scene::graphic_mesh::ApplicationGraphicMeshDeviceLoadedY;


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


#[derive(Debug)]
pub struct ApplicationScene {
    pipeline_table: HashMap<ApplicationScenePipelineName, ApplicationScenePipeline>,
    graphic_mesh_table: HashMap<ApplicationGraphicMeshName, ApplicationGraphicMeshDeviceLoadedY>,
    entity_renderable_s: Vec<ApplicationSceneEntityRenderable>,
    entity_renderable_name_s: HashSet<ApplicationSceneEntityRenderableName>,
}

impl ApplicationScene {
    pub fn new() -> Self
    {
        Self {
            pipeline_table: HashMap::new(),
            graphic_mesh_table: HashMap::new(),
            entity_renderable_s: Vec::new(),
            entity_renderable_name_s: HashSet::new(),
        }
    }

    pub fn add_pipeline(
        &mut self,
        new_vulkan_pipeline_name: ApplicationScenePipelineName,
        new_vulkan_pipeline: ApplicationScenePipeline)
    -> Result<(), ErrorFoundationApplicationGuide>
    {
        if self.pipeline_table.get(&new_vulkan_pipeline_name).is_some() {
            return Err(ErrorFoundationApplicationGuideOwn::ApplicationSceneVulkanPipelineAlreadyAdded)?
        }
        let old_vulkan_pipeline_o = self.pipeline_table.insert(new_vulkan_pipeline_name, new_vulkan_pipeline);
        assert!(old_vulkan_pipeline_o.is_none());
        Ok(())
    }

    pub fn lookup_pipeline(&self, pipeline_name: &ApplicationScenePipelineName)
    -> Option<&ApplicationScenePipeline>
    {
        self.pipeline_table.get(pipeline_name)
    }

    pub fn add_graphic_mesh(
        &mut self,
        new_graphic_mesh_name: ApplicationGraphicMeshName,
        new_graphic_mesh: ApplicationGraphicMeshDeviceLoadedY)
    -> Result<(), ErrorFoundationApplicationGuide>
    {
        if self.graphic_mesh_table.get(&new_graphic_mesh_name).is_some() {
            return Err(ErrorFoundationApplicationGuideOwn::ApplicationSceneGraphicMeshAlreadyAdded)?
        }
        let old_graphic_mesh_o = self.graphic_mesh_table.insert(new_graphic_mesh_name, new_graphic_mesh);
        assert!(old_graphic_mesh_o.is_none());
        Ok(())
    }

    pub fn lookup_graphic_mesh(&self, graphic_mesh_name: &ApplicationGraphicMeshName)
    -> Option<&ApplicationGraphicMeshDeviceLoadedY>
    {
        self.graphic_mesh_table.get(graphic_mesh_name)
    }

    pub fn add_entity_renderable(
        &mut self, new_renderable_entity: ApplicationSceneEntityRenderable)
    -> Result<(), ErrorFoundationApplicationGuide>
    {
        if self.pipeline_table.get(&new_renderable_entity.pipeline_name).is_none() {
            return Err(ErrorFoundationApplicationGuideOwn::ApplicationSceneEntityRenderablePipelineNotExist)?
        }
        if self.graphic_mesh_table.get(&new_renderable_entity.graphic_mesh_name).is_none() {
            return Err(ErrorFoundationApplicationGuideOwn::ApplicationSceneEntityRenderableGraphicMeshNotExist)?
        }
        if self.entity_renderable_name_s.get(&new_renderable_entity.name).is_some() {
            return Err(ErrorFoundationApplicationGuideOwn::ApplicationSceneEntityRenderableAlreadyAdded)?
        }
        let be_inserted = self.entity_renderable_name_s.insert(new_renderable_entity.name);
        assert!(be_inserted);
        self.entity_renderable_s.push(new_renderable_entity);
        Ok(())
    }

    pub fn iter_entity_renderable_s(&self) -> Iter<ApplicationSceneEntityRenderable>
    {
        self.entity_renderable_s.iter()
    }
}