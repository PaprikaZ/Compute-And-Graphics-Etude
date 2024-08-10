use std::io::BufReader;
use std::path::PathBuf;
use std::fs::File;
use std::collections::HashMap;

use ::tobj;
use ::nalgebra_glm as glm;

use crate::termination::TerminationProcessMain;
use crate::lib::vertex::Vertex;
use crate::lib::vertex::VertexIndexU32;
use crate::lib::d3_model_mesh::D3ModelMeshTutorialFormatObj;
use crate::data::d3_model_resource::DataD3ModelResourceTutorialFormatObj;
use crate::config::path::ConfigPathFile;


pub struct DataD3ModelMeshTutorialFormatObj {}

impl DataD3ModelMeshTutorialFormatObj {
    pub fn load(resource_name: DataD3ModelResourceTutorialFormatObj)
     -> Result<D3ModelMeshTutorialFormatObj, TerminationProcessMain>
    {
        let model_file_path = Self::file_path_from_resource_name(resource_name);
        let open_model_file_result = File::open(model_file_path);
        let model_file =
            match open_model_file_result {
                Err(error) => return Err(TerminationProcessMain::InitializationFileOpenFail(error.to_string())),
                Ok(file) => file,
            };
        let mut model_file_reader = BufReader::new(model_file);
        let be_triangulate_face = true;
        let load_model_result =
            tobj::load_obj_buf(&mut model_file_reader, be_triangulate_face, |_| {
                let mut map = HashMap::new();
                map.insert("Texture1".to_string(), 0);
                Ok((vec![tobj::Material::empty()], map))
            });
        let (model_s, _material_s) =
            match load_model_result {
                Err(error) => return Err(TerminationProcessMain::InitializationModelFormatObjLoadingError(error)),
                Ok(model_s_and_material_s) => model_s_and_material_s,
            };
        //
        Self::load_core(model_s, _material_s)
    }

    fn file_path_from_resource_name(resource_name: DataD3ModelResourceTutorialFormatObj) -> PathBuf {
        match resource_name {
            DataD3ModelResourceTutorialFormatObj::VikingRoom =>
                ConfigPathFile::get_model_viking_room(),
        }
    }

    fn load_core(model_s: Vec<tobj::Model>, _material_s: Vec<tobj::Material>)
     -> Result<D3ModelMeshTutorialFormatObj, TerminationProcessMain>
    {
        let mut vertex_table: HashMap<Vertex, usize> = HashMap::new();
        let mut vertex_s: Vec<Vertex> = Vec::new();
        let mut vertex_index_s: Vec<VertexIndexU32> = Vec::new();

        for model in &model_s {
            for index in &model.mesh.indices {
                let position_offset = (index * 3) as usize;
                let texture_coordinate_offset = (index * 2) as usize;
                let vertex = Vertex::new(
                    glm::vec3(
                        model.mesh.positions[position_offset + 0],
                        model.mesh.positions[position_offset + 1],
                        model.mesh.positions[position_offset + 2]),
                    glm::vec3(1.0, 1.0, 1.0),
                    glm::vec2(
                        model.mesh.texcoords[texture_coordinate_offset + 0],
                        1.0 - model.mesh.texcoords[texture_coordinate_offset + 1]));
                match vertex_table.get(&vertex) {
                    Some(idx) => vertex_index_s.push(VertexIndexU32::new(*idx as u32)),
                    None => {
                        let idx = vertex_s.len();
                        vertex_table.insert(vertex, idx);
                        vertex_s.push(vertex);
                        vertex_index_s.push(VertexIndexU32::new(idx as u32));
                    }
                }
            }
        }
        Ok(D3ModelMeshTutorialFormatObj::new(vertex_s, vertex_index_s))
    }
}