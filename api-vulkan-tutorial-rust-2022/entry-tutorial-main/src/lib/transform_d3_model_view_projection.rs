use ::nalgebra_glm as glm;


#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct TransformD3ModelViewProjection {
    model: glm::Mat4,
    view: glm::Mat4,
    projection: glm::Mat4,
}

impl TransformD3ModelViewProjection {
    pub fn new(model_transform: glm::Mat4, view_transform: glm::Mat4, projection_transform: glm::Mat4) -> Self {
        Self {
            model: model_transform,
            view: view_transform,
            projection: projection_transform,
        }
    }
}