use ::nalgebra_glm as glm;


#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: glm::Vec2,
    pub color: glm::Vec3,
}

impl Vertex {
    pub fn new(position: glm::Vec2, color: glm::Vec3) -> Self {
        Self {
            position: position,
            color: color,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct VertexIndex(u16);

impl VertexIndex {
    pub fn new(index: u16) -> Self {
        Self(index)
    }

    pub fn as_raw(&self) -> u16 {
        self.0
    }
}