use crate::lib::vertex::Vertex;
use crate::lib::vertex::VertexIndexU16;
use crate::lib::vertex::VertexIndexU32;


pub enum D3ModelMesh {
    TutorialSimple(D3ModelMeshTutorialSimple),
    TutorialFormatObj(D3ModelMeshTutorialFormatObj),
}

pub struct D3ModelMeshTutorialSimple {
    pub vertex_s: Vec<Vertex>,
    pub vertex_index_s: Vec<VertexIndexU16>,
}

impl D3ModelMeshTutorialSimple {
    pub fn new(vertex_s: Vec<Vertex>, vertex_index_s: Vec<VertexIndexU16>) -> Self {
        Self {
            vertex_s: vertex_s,
            vertex_index_s: vertex_index_s,
        }
    }
}

pub struct D3ModelMeshTutorialFormatObj {
    pub vertex_s: Vec<Vertex>,
    pub vertex_index_s: Vec<VertexIndexU32>,
}

impl D3ModelMeshTutorialFormatObj {
    pub fn new(vertex_s: Vec<Vertex>, vertex_index_s: Vec<VertexIndexU32>) -> Self {
        Self {
            vertex_s: vertex_s,
            vertex_index_s: vertex_index_s,
        }
    }
}