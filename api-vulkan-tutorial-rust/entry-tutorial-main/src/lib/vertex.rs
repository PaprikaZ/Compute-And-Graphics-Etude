use std::hash::{Hash, Hasher};
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

impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position &&
        self.color == other.color &&
        self.texture_coordinate == other.texture_coordinate
    }
}

impl Eq for Vertex {}

impl Hash for Vertex {
    fn hash<TH: Hasher>(&self, hasher: &mut TH) -> () {
        self.position[0].to_bits().hash(hasher);
        self.position[1].to_bits().hash(hasher);
        self.position[2].to_bits().hash(hasher);
        self.color[0].to_bits().hash(hasher);
        self.color[1].to_bits().hash(hasher);
        self.color[2].to_bits().hash(hasher);
        self.texture_coordinate[0].to_bits().hash(hasher);
        self.texture_coordinate[1].to_bits().hash(hasher);
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