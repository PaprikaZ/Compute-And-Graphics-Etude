use ::nalgebra_glm as glm;


#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: glm::Vec3,
    pub color: glm::Vec3,
    pub texture_coordinate: glm::Vec2,
}

impl Vertex {
    pub fn new(position: glm::Vec3, color: glm::Vec3, texture_coordinate: glm::Vec2) -> Self {
        Self {
            position: position,
            color: color,
            texture_coordinate: texture_coordinate,
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