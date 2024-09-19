use ::library_foundation_reintroduction::vulkan::VulkanPipeline;
use ::library_foundation_reintroduction::vulkan::VulkanPipelineLayout;


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