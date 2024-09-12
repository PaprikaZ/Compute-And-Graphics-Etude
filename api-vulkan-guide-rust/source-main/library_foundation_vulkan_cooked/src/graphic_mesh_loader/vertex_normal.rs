use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::collections::HashMap;

use ::library_foundation_reintroduction::tobj;
use ::library_foundation_reintroduction::nalgebra_glm as glm;

use crate::error::foundation_vulkan_cooked::ErrorFoundationVulkanCookedOwn;
use crate::error::foundation_vulkan_cooked::ErrorFoundationVulkanCooked;
use crate::graphic_mesh_vertex::normal::GraphicMeshVertexNormal;
use crate::graphic_mesh_loader::self_::GraphicMeshLoader;


impl GraphicMeshLoader<GraphicMeshVertexNormal> {
    pub fn load_from_path_file_obj<T>(
        file_path: &Path, new_graphic_mesh_vertex_index: Box<dyn Fn(usize) -> T>)
    -> Result<(Vec<GraphicMeshVertexNormal>, Vec<T>), ErrorFoundationVulkanCooked>
    {
        let opened_file =
            File::open(file_path)
            .or(Err(ErrorFoundationVulkanCookedOwn::PathFileGraphicMeshOpenFail))?;
        let mut file_reader = BufReader::new(opened_file);
        let (model_s, _material_s) =
            tobj::load_obj_buf(
                &mut file_reader,
                &tobj::LoadOptions { triangulate: true, ..Default::default() },
                |_| Ok(Default::default()))
            .or(Err(ErrorFoundationVulkanCookedOwn::PathFileGraphicMeshDataCorrupted))?;
        //
        let mut vertex_table: HashMap<GraphicMeshVertexNormal, usize> = HashMap::new();
        let mut vertex_s: Vec<GraphicMeshVertexNormal> = Vec::new();
        let mut vertex_index_s: Vec<T> = Vec::new();
        model_s
        .iter()
        .map(|model| {
            model.mesh.indices
            .iter()
            .map(move |index| (model, index))
        })
        .flatten()
        .for_each(|(model, index)| {
            let position_offset = (index * 3) as usize;
            let normal_offset = (index * 3) as usize;
            let vertex_position = glm::vec3(
                model.mesh.positions[position_offset + 0],
                model.mesh.positions[position_offset + 1],
                model.mesh.positions[position_offset + 2]);
            let vertex_normal = glm::vec3(
                model.mesh.normals[normal_offset + 0],
                model.mesh.normals[normal_offset + 1],
                model.mesh.normals[normal_offset + 2]);
            let vertex = GraphicMeshVertexNormal::new(vertex_position, vertex_normal);
            match vertex_table.get(&vertex) {
                //Some(idx) => vertex_index_s.push(GraphicMeshVertexIndexU16::new(*idx as u16)),
                Some(idx) => vertex_index_s.push(new_graphic_mesh_vertex_index(*idx)),
                None => {
                    let idx = vertex_s.len();
                    vertex_table.insert(vertex, idx);
                    vertex_s.push(vertex);
                    vertex_index_s.push(new_graphic_mesh_vertex_index(idx));
                },
            }
        });
        Ok((vertex_s, vertex_index_s))
    }
}