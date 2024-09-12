use ::library_foundation_reintroduction::vulkan::VulkanInstance;
use ::library_foundation_reintroduction::vulkan::VulkanDevicePhysical;
use ::library_foundation_reintroduction::vulkan::VulkanDeviceLogical;
use ::library_foundation_reintroduction::vulkan::VulkanCommandBuffer;
use ::library_foundation_reintroduction::vulkan::VulkanBuffer;
use ::library_foundation_reintroduction::vulkan::VulkanDeviceMemory;
use ::library_foundation_reintroduction::vulkan::VulkanQueue;
use ::library_foundation_reintroduction::vulkan::VulkanImage;
use ::library_foundation_reintroduction::vulkan::VulkanFormat;
use ::library_foundation_reintroduction::vulkan::VulkanFence;

use crate::error::foundation_vulkan_cooked::ErrorFoundationVulkanCooked;
use crate::vulkan_memory_raw_prefab::buffer::VulkanMemoryRawPrefabBufferGraphicMesh;
use crate::vulkan_memory_raw_prefab::image::VulkanMemoryRawPrefabImageDepth;
use crate::vulkan_queue_submit::one_time_launcher::VulkanQueueSubmitOneTimeLauncher;


pub struct VulkanMemoryRawPrefabAllocator<'t> {
    pub instance: &'t VulkanInstance,
    pub device_physical: VulkanDevicePhysical,
    pub device_logical: &'t VulkanDeviceLogical,
    pub queue_transfer: VulkanQueue,
    pub command_buffer: VulkanCommandBuffer,
}

impl<'t> VulkanMemoryRawPrefabAllocator<'t> {
    pub fn new(
        vulkan_instance: &'t VulkanInstance,
        vulkan_physical_device: VulkanDevicePhysical,
        vulkan_logical_device: &'t VulkanDeviceLogical,
        vulkan_transfer_queue: VulkanQueue,
        vulkan_command_buffer: VulkanCommandBuffer)
    -> Self
    {
        Self {
            instance: vulkan_instance,
            device_physical: vulkan_physical_device,
            device_logical: vulkan_logical_device,
            queue_transfer: vulkan_transfer_queue,
            command_buffer: vulkan_command_buffer,
        }
    }

    fn create_queue_submit_one_time_launcher(&self) -> VulkanQueueSubmitOneTimeLauncher {
        VulkanQueueSubmitOneTimeLauncher::new(self.device_logical, self.queue_transfer, self.command_buffer)
    }

    pub fn allocate_buffer_graphic_mesh<T>(&self, vertex_s: &Vec<T>, vulkan_fence_o: Option<VulkanFence>)
    -> Result<(VulkanBuffer, VulkanDeviceMemory), ErrorFoundationVulkanCooked>
    {
        VulkanMemoryRawPrefabBufferGraphicMesh::allocate(
            &self.instance, self.device_physical, &self.device_logical,
            self.create_queue_submit_one_time_launcher(), vulkan_fence_o,
            vertex_s)
    }

    pub fn allocate_image_depth(&self, vulkan_depth_image_extent: (u32, u32))
    -> Result<(VulkanFormat, VulkanImage, VulkanDeviceMemory), ErrorFoundationVulkanCooked>
    {
        VulkanMemoryRawPrefabImageDepth::allocate(
            &self.instance, self.device_physical, &self.device_logical, vulkan_depth_image_extent)
    }
}