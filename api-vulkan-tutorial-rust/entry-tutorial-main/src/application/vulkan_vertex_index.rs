use std::mem::size_of;
use std::ptr::copy_nonoverlapping;

use ::vulkan::prelude::version1_2::*;
use ::vulkan::VulkanErrorCode;
use ::vulkan::VulkanDevicePhysical;
use ::vulkan::VulkanInstance;
use ::vulkan::VulkanBufferUsageFlagS;
use ::vulkan::VulkanMemoryPropertyFlagS;
use ::vulkan::VulkanMemoryMapFlagS;
use ::vulkan::VulkanBuffer;
use ::vulkan::VulkanDeviceMemory;
use ::vulkan::VulkanCommandPool;
use ::vulkan::VulkanQueue;

use crate::termination::TerminationProcessMain;
use crate::lib::d3_model_mesh::D3ModelMesh;
use crate::application::vulkan_buffer::ApplicationVulkanBuffer;


//struct ApplicationVulkanVertexIndex {}

pub struct ApplicationVulkanVertexIndexBuffer {}

impl ApplicationVulkanVertexIndexBuffer {
    pub unsafe fn create(
        vulkan_instance: &VulkanInstance,
        vulkan_physical_device: VulkanDevicePhysical,
        vulkan_logical_device: &VulkanDeviceLogical,
        vulkan_command_pool: VulkanCommandPool,
        vulkan_graphic_queue: VulkanQueue,
        d3_model_mesh: &D3ModelMesh)
     -> Result<(VulkanBuffer, VulkanDeviceMemory), TerminationProcessMain>
    {
        let (input_vertex_index_number, vulkan_vertex_index_buffer_size) =
            match d3_model_mesh {
                D3ModelMesh::TutorialSimple(mesh) => {
                    let index_number = mesh.vertex_index_s.len();
                    (index_number, (size_of::<u16>() * index_number) as u64)
                },
                D3ModelMesh::TutorialFormatObj(mesh) => {
                    let index_number = mesh.vertex_index_s.len();
                    (index_number, (size_of::<u32>() * index_number) as u64)
                },
            };
        //
        let (vulkan_vertex_index_staging_buffer, vulkan_vertex_index_staging_buffer_memory) =
            ApplicationVulkanBuffer::create_with_memory(
                vulkan_instance,
                vulkan_physical_device,
                vulkan_logical_device,
                vulkan_vertex_index_buffer_size,
                VulkanBufferUsageFlagS::TRANSFER_SRC,
                VulkanMemoryPropertyFlagS::HOST_COHERENT | VulkanMemoryPropertyFlagS::HOST_VISIBLE)?;
        let map_vulkan_vertex_index_staging_buffer_memory_result =
            vulkan_logical_device.map_memory(
                vulkan_vertex_index_staging_buffer_memory,
                0,
                vulkan_vertex_index_buffer_size,
                VulkanMemoryMapFlagS::empty());
        let vulkan_vertex_index_staging_buffer_memory_address =
            match map_vulkan_vertex_index_staging_buffer_memory_result {
                Err(error) => {
                    let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                    return Err(TerminationProcessMain::InitializationVulkanMemoryMapFail(vulkan_error_code));
                },
                Ok(address) => address,
            };
        match d3_model_mesh {
            D3ModelMesh::TutorialSimple(mesh) => {
                copy_nonoverlapping(
                    mesh.vertex_index_s.as_ptr(),
                    vulkan_vertex_index_staging_buffer_memory_address.cast(),
                    input_vertex_index_number);
            },
            D3ModelMesh::TutorialFormatObj(mesh) => {
                copy_nonoverlapping(
                    mesh.vertex_index_s.as_ptr(),
                    vulkan_vertex_index_staging_buffer_memory_address.cast(),
                    input_vertex_index_number);
            },
        };
        vulkan_logical_device.unmap_memory(vulkan_vertex_index_staging_buffer_memory);
        //
        let (vulkan_vertex_index_buffer, vulkan_vertex_index_buffer_memory) =
            ApplicationVulkanBuffer::create_with_memory(
                vulkan_instance,
                vulkan_physical_device,
                vulkan_logical_device,
                vulkan_vertex_index_buffer_size,
                VulkanBufferUsageFlagS::TRANSFER_DST | VulkanBufferUsageFlagS::INDEX_BUFFER,
                VulkanMemoryPropertyFlagS::DEVICE_LOCAL)?;
        ApplicationVulkanBuffer::copy(
            &vulkan_logical_device,
            vulkan_command_pool,
            vulkan_graphic_queue,
            vulkan_vertex_index_staging_buffer,
            vulkan_vertex_index_buffer,
            vulkan_vertex_index_buffer_size)?;
        vulkan_logical_device.destroy_buffer(vulkan_vertex_index_staging_buffer, None);
        vulkan_logical_device.free_memory(vulkan_vertex_index_staging_buffer_memory, None);
        Ok((vulkan_vertex_index_buffer, vulkan_vertex_index_buffer_memory))
    }
}