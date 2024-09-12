use ::library_foundation_reintroduction::vulkan::VulkanVertexInputBindingDescription;
use ::library_foundation_reintroduction::vulkan::VulkanVertexInputAttributeDescription;


#[derive(Debug)]
pub struct VulkanVertexInputDescriptionS {
    binding_s: Vec<VulkanVertexInputBindingDescription>,
    attribute_s: Vec<VulkanVertexInputAttributeDescription>,
}

impl VulkanVertexInputDescriptionS {
    pub fn new_empty() -> Self {
        Self {
            binding_s: Vec::new(),
            attribute_s: Vec::new(),
        }
    }

    pub fn get_binding_s(&self) -> &[VulkanVertexInputBindingDescription] {
        &self.binding_s
    }

    pub fn get_attribute_s(&self) -> &[VulkanVertexInputAttributeDescription] {
        &self.attribute_s
    }

    pub fn add_binding_and_attribute_s(
        &mut self,
        new_binding_description: VulkanVertexInputBindingDescription,
        new_attribute_description_s: Vec<VulkanVertexInputAttributeDescription>)
    {
        assert!(new_attribute_description_s.iter().all(|d| new_binding_description.binding == d.binding));
        //
        self.binding_s.push(new_binding_description);
        new_attribute_description_s.into_iter().for_each(|d| self.attribute_s.push(d));
    }
}