use ::nalgebra_glm as glm;


#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct TransformD3ViewProjection {
    view: glm::Mat4,
    projection: glm::Mat4,
}

impl TransformD3ViewProjection {
    pub fn new(view_transform: glm::Mat4, projection_transform: glm::Mat4) -> Self {
        Self {
            view: view_transform,
            projection: projection_transform,
        }
    }
}