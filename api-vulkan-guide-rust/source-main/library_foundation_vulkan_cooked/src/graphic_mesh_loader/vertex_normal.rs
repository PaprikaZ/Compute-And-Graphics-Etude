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
        let mut vertex_s: Vec<GraphicMeshVertexNormal> = Vec::new();
        let mut vertex_index_s: Vec<T> = Vec::new();
        assert_eq!(model_s.len(), 1);
        let mesh_model = &model_s[0];
        (0..mesh_model.mesh.indices.len())
        .into_iter()
        .for_each(|index_offset| {
            let vertex_index = mesh_model.mesh.indices[index_offset] as usize;
            let vertex_position = glm::vec3(
                mesh_model.mesh.positions[vertex_index * 3 + 0],
                mesh_model.mesh.positions[vertex_index * 3 + 1],
                mesh_model.mesh.positions[vertex_index * 3 + 2]);
            let vertex_normal = glm::vec3(
                mesh_model.mesh.normals[vertex_index * 3 + 0],
                mesh_model.mesh.normals[vertex_index * 3 + 1],
                mesh_model.mesh.normals[vertex_index * 3 + 2]);
            let new_vertex = GraphicMeshVertexNormal::new(vertex_position, vertex_normal);
            vertex_s.push(new_vertex);
            vertex_index_s.push(new_graphic_mesh_vertex_index(vertex_index));
        });
        Ok((vertex_s, vertex_index_s))
    }
}