use std::hash::Hash;
use std::hash::Hasher;

use ::library_foundation_reintroduction::nalgebra_glm as glm;


#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GraphicMeshVertexNormal {
    pub position: glm::Vec3,
    pub normal: glm::Vec3,
}

impl GraphicMeshVertexNormal {
    pub fn new(position: glm::Vec3, normal: glm::Vec3) -> Self {
        Self {
            position: position,
            normal: normal,
        }
    }
}

impl PartialEq for GraphicMeshVertexNormal {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position &&
        self.normal == other.normal
    }
}

impl Eq for GraphicMeshVertexNormal {}

impl Hash for GraphicMeshVertexNormal {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.position[0].to_bits().hash(hasher);
        self.position[1].to_bits().hash(hasher);
        self.position[2].to_bits().hash(hasher);
        self.normal[0].to_bits().hash(hasher);
        self.normal[1].to_bits().hash(hasher);
        self.normal[2].to_bits().hash(hasher);
    }
}