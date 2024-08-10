use ::nalgebra_glm as glm;

use crate::termination::TerminationProcessMain;
use crate::lib::vertex::Vertex;
use crate::lib::vertex::VertexIndexU16;
use crate::lib::d3_model_mesh::D3ModelMeshTutorialSimple;
use crate::data::d3_model_resource::DataD3ModelResourceTutorialSimple;


pub struct DataD3ModelMeshTutorialSimple {}

impl DataD3ModelMeshTutorialSimple {
    pub fn load(resource_name: DataD3ModelResourceTutorialSimple) -> Result<D3ModelMeshTutorialSimple, TerminationProcessMain> {
        Ok(D3ModelMeshTutorialSimple::new(
            Self::get_vertex_s(resource_name),
            Self::get_vertex_index_s(resource_name)))
    }

    fn get_vertex_s(resource_name: DataD3ModelResourceTutorialSimple) -> Vec<Vertex> {
        match resource_name {
            DataD3ModelResourceTutorialSimple::Default =>
                vec![
                    Vertex::new(glm::vec3(-0.5, -0.5, 0.0), glm::vec3(1.0, 0.0, 0.0), glm::vec2(1.0, 0.0)),
                    Vertex::new(glm::vec3(0.5, -0.5, 0.0), glm::vec3(0.0, 1.0, 0.0), glm::vec2(0.0, 0.0)),
                    Vertex::new(glm::vec3(0.5, 0.5, 0.0), glm::vec3(0.0, 0.0, 1.0), glm::vec2(0.0, 1.0)),
                    Vertex::new(glm::vec3(-0.5, 0.5, 0.0), glm::vec3(1.0, 1.0, 1.0), glm::vec2(1.0, 1.0)),

                    Vertex::new(glm::vec3(-0.5, -0.5, -0.5), glm::vec3(1.0, 0.0, 0.0), glm::vec2(1.0, 0.0)),
                    Vertex::new(glm::vec3(0.5, -0.5, -0.5), glm::vec3(0.0, 1.0, 0.0), glm::vec2(0.0, 0.0)),
                    Vertex::new(glm::vec3(0.5, 0.5, -0.5), glm::vec3(0.0, 0.0, 1.0), glm::vec2(0.0, 1.0)),
                    Vertex::new(glm::vec3(-0.5, 0.5, -0.5), glm::vec3(1.0, 1.0, 1.0), glm::vec2(1.0, 1.0)),
                ],
        }
    }

    fn get_vertex_index_s(resource_name: DataD3ModelResourceTutorialSimple) -> Vec<VertexIndexU16> {
        match resource_name {
            DataD3ModelResourceTutorialSimple::Default =>
                vec![0, 1, 2, 2, 3, 0, 4, 5, 6, 6, 7, 4]
                .iter()
                .map(|i| VertexIndexU16::new(*i as u16))
                .collect::<_>(),
        }
    }
}