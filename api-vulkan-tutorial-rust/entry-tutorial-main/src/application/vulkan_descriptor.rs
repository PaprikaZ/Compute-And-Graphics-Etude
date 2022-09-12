use std::mem::size_of;

use ::vulkan::prelude::version1_2::*;
use ::vulkan::VulkanErrorCode;
use ::vulkan::VulkanBuffer;
use ::vulkan::VulkanImage;
use ::vulkan::VulkanDescriptorPool;
use ::vulkan::VulkanDescriptorPoolSize;
use ::vulkan::VulkanDescriptorPoolCreateInformation;
use ::vulkan::VulkanDescriptorType;
use ::vulkan::VulkanDescriptorSetLayout;
use ::vulkan::VulkanDescriptorBufferInformation;
use ::vulkan::VulkanDescriptorSetAllocateInformation;
use ::vulkan::VulkanDescriptorSet;
use ::vulkan::VulkanWriteDescriptorSet;
use ::vulkan::VulkanCopyDescriptorSet;

use crate::termination::TerminationProcessMain;
use crate::lib::transform_d3_model_view_projection::TransformD3ModelViewProjection;


pub struct ApplicationVulkanDescriptorPool {}

impl ApplicationVulkanDescriptorPool {
    pub unsafe fn create(vulkan_logical_device: &VulkanDeviceLogical, vulkan_swapchain_image_s: &Vec<VulkanImage>)
     -> Result<VulkanDescriptorPool, TerminationProcessMain>
    {
        let vulkan_descriptor_pool_size =
            VulkanDescriptorPoolSize::builder()
            .type_(VulkanDescriptorType::UNIFORM_BUFFER)
            .descriptor_count(vulkan_swapchain_image_s.len() as u32);
        let vulkan_descriptor_pool_size_s = &[vulkan_descriptor_pool_size];
        let vulkan_descriptor_pool_create_information =
            VulkanDescriptorPoolCreateInformation::builder()
            .pool_sizes(vulkan_descriptor_pool_size_s)
            .max_sets(vulkan_swapchain_image_s.len() as u32);
        let create_vulkan_descriptor_pool_result =
            vulkan_logical_device.create_descriptor_pool(&vulkan_descriptor_pool_create_information, None);
        match create_vulkan_descriptor_pool_result {
            Err(error) => {
                let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                return Err(TerminationProcessMain::InitializationVulkanDescriptorPoolCreateFail(vulkan_error_code));
            },
            Ok(pool) => Ok(pool),
        }
    }
}