use crate::termination::TerminationProcessMain;
use crate::lib::d3_model_mesh::D3ModelMesh;
use crate::data::d3_model_mesh_tutorial_simple::DataD3ModelMeshTutorialSimple;
use crate::data::d3_model_mesh_tutorial_format_obj::DataD3ModelMeshTutorialFormatObj;
use crate::data::d3_model_resource::DataD3ModelResource;


#[allow(dead_code)]
pub struct DataD3ModelMesh {}

impl DataD3ModelMesh {
    #[allow(dead_code)]
    pub fn load(resource_name: DataD3ModelResource) -> Result<D3ModelMesh, TerminationProcessMain> {
        let d3_model_mesh =
            match resource_name {
                DataD3ModelResource::TutorialSimple(name) => {
                    let model_mesh = DataD3ModelMeshTutorialSimple::load(name)?;
                    D3ModelMesh::TutorialSimple(model_mesh)
                },
                DataD3ModelResource::TutorialFormatObj(name) => {
                    let model_mesh = DataD3ModelMeshTutorialFormatObj::load(name)?;
                    D3ModelMesh::TutorialFormatObj(model_mesh)
                },
            };
        Ok(d3_model_mesh)
    }
}