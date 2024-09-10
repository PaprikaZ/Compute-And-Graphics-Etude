use std::hash::Hash;
use std::hash::Hasher;

use ::library_foundation_reintroduction::nalgebra_glm as glm;


#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GraphicMeshVertexNormalColor {
    pub position: glm::Vec3,
    pub normal: glm::Vec3,
    pub color: glm::Vec3,
}

impl GraphicMeshVertexNormalColor {
    pub fn new(position: glm::Vec3, normal: glm::Vec3, color: glm::Vec3) -> Self {
        Self {
            position: position,
            normal: normal,
            color: color,
        }
    }
}

impl PartialEq for GraphicMeshVertexNormalColor {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position &&
        self.normal == other.normal &&
        self.color == other.color
    }
}

impl Eq for GraphicMeshVertexNormalColor {}

impl Hash for GraphicMeshVertexNormalColor {
    fn hash<TH: Hasher>(&self, hasher: &mut TH) {
        self.position[0].to_bits().hash(hasher);
        self.position[1].to_bits().hash(hasher);
        self.position[2].to_bits().hash(hasher);
        self.normal[0].to_bits().hash(hasher);
        self.normal[1].to_bits().hash(hasher);
        self.normal[2].to_bits().hash(hasher);
        self.color[0].to_bits().hash(hasher);
        self.color[1].to_bits().hash(hasher);
        self.color[2].to_bits().hash(hasher);
    }
}