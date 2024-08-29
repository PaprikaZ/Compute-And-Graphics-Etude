#[derive(Clone, Copy, Debug)]
pub struct VulkanErrorCode(i32);

impl VulkanErrorCode {
    pub fn new(code: i32) -> Self {
        Self(code)
    }

    pub fn unwrap(self) -> i32 {
        self.0
    }
}