#[derive(Debug, Clone)]
pub struct VulkanApplicationName<'t>(&'t str);

impl<'t> VulkanApplicationName<'t> {
    pub fn new(name: &'t str) -> Self {
        Self(name)
    }

    pub fn unwrap(self) -> &'t str {
        self.0
    }
}