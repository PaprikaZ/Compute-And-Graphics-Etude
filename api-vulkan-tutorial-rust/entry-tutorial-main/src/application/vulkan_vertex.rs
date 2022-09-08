use std::mem::size_of;
use std::ptr::copy_nonoverlapping;

use ::vulkan::prelude::version1_2::*;
use ::vulkan::VulkanErrorCode;
use ::vulkan::VulkanDevicePhysical;
use ::vulkan::VulkanInstance;
use ::vulkan::VulkanSharingMode;
use ::vulkan::VulkanBufferUsageFlagS;
use ::vulkan::VulkanBufferCreateInformation;
use ::vulkan::VulkanMemoryAllocateInfomration;
use ::vulkan::VulkanMemoryPropertyFlagS;
use ::vulkan::VulkanMemoryMapFlagS;
use ::vulkan::VulkanBuffer;
use ::vulkan::VulkanDeviceMemory;

use crate::termination::TerminationProcessMain;
use crate::lib::vertex::Vertex;
use crate::application::vulkan_memory::ApplicationVulkanMemory;


//struct ApplicationVulkanVertex {}

pub struct ApplicationVulkanVertexBuffer {}

impl ApplicationVulkanVertexBuffer {
    pub unsafe fn create(
        vulkan_instance: &VulkanInstance,
        vulkan_physical_device: VulkanDevicePhysical,
        vulkan_logical_device: &VulkanDeviceLogical,
        input_vertex_s: &Vec<Vertex>,
     )
     -> Result<(VulkanBuffer, VulkanDeviceMemory), TerminationProcessMain>
    {
        let vulkan_vertex_buffer_create_information =
            VulkanBufferCreateInformation::builder()
            .size((size_of::<Vertex>() * input_vertex_s.len()) as u64)
            .usage(VulkanBufferUsageFlagS::VERTEX_BUFFER)
            .sharing_mode(VulkanSharingMode::EXCLUSIVE);
        let create_vulkan_vertex_buffer_result =
            vulkan_logical_device.create_buffer(&vulkan_vertex_buffer_create_information, None);
        let vulkan_vertex_buffer =
            match create_vulkan_vertex_buffer_result {
                Err(error) => {
                    let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                    return Err(TerminationProcessMain::InitializationVulkanVertexBufferCreateFail(vulkan_error_code));
                },
                Ok(buffer) => buffer,
            };
        let vulkan_memory_requirement_s =
            vulkan_logical_device.get_buffer_memory_requirements(vulkan_vertex_buffer);
        let vulkan_memory_type_index =
            ApplicationVulkanMemory::get_type_index(
                vulkan_instance,
                vulkan_physical_device,
                VulkanMemoryPropertyFlagS::HOST_COHERENT | VulkanMemoryPropertyFlagS::HOST_VISIBLE,
                vulkan_memory_requirement_s
            )?;
        let vulkan_memory_allocate_infomation =
            VulkanMemoryAllocateInfomration::builder()
            .allocation_size(vulkan_memory_requirement_s.size)
            .memory_type_index(vulkan_memory_type_index.as_raw());
        let allocate_vulkan_vertex_buffer_memory_result =
            vulkan_logical_device.allocate_memory(&vulkan_memory_allocate_infomation, None);
        let vulkan_vertex_buffer_memory =
            match allocate_vulkan_vertex_buffer_memory_result {
                Err(error) => {
                    let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                    return Err(TerminationProcessMain::InitializationVulkanMemoryAllocateFail(vulkan_error_code));
                },
                Ok(memory) => memory,
            };
        let bind_buffer_memory_result =
            vulkan_logical_device.bind_buffer_memory(vulkan_vertex_buffer, vulkan_vertex_buffer_memory, 0);
        match bind_buffer_memory_result {
            Err(error) => {
                let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                return Err(TerminationProcessMain::InitializationVulkanMemoryBufferBindFail(vulkan_error_code));
            },
            Ok(()) => (),
        };
        let map_vulkan_memory_result =
            vulkan_logical_device.map_memory(
                vulkan_vertex_buffer_memory,
                0,
                vulkan_vertex_buffer_create_information.size,
                VulkanMemoryMapFlagS::empty(),
            );
        let vulkan_memory_address =
            match map_vulkan_memory_result {
                Err(error) => {
                    let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                    return Err(TerminationProcessMain::InitializationVulkanMemoryMapFail(vulkan_error_code));
                },
                Ok(address) => address,
            };
        copy_nonoverlapping(input_vertex_s.as_ptr(), vulkan_memory_address.cast(), input_vertex_s.len());
        vulkan_logical_device.unmap_memory(vulkan_vertex_buffer_memory);
        Ok((vulkan_vertex_buffer, vulkan_vertex_buffer_memory))
    }
}