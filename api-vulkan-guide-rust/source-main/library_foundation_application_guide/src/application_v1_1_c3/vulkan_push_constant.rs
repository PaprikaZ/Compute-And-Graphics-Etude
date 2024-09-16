use ::library_foundation_reintroduction::nalgebra_glm as glm;


#[derive(Debug, Clone, Copy)]
pub struct ApplicationVulkanPushConstantData {
    pub mvp_transform: glm::Mat4,
}

impl ApplicationVulkanPushConstantData {
    pub(super) fn create(render_frame_number: u32) -> Self {
        let camera_position = glm::vec3(0.0, 0.0, -2.0);
        let view_transform_matrix =
            glm::translate(&glm::Mat4::identity(), &camera_position);
        let mut projection_transform_matrix =
            glm::perspective_rh_zo(1700.0 / 900.0, glm::radians(&glm::vec1(70.0))[0], 0.1, 200.0);
        projection_transform_matrix[(1, 1)] *= -1.0;
        let model_transform_matrix =
            glm::rotate(
                &glm::identity(),
                glm::radians(&glm::vec1((render_frame_number as f32) * 0.4))[0],
                &glm::vec3(0.0, 1.0, 0.0));
        let new_main_mvp_transform: glm::Mat4 =
            projection_transform_matrix * view_transform_matrix * model_transform_matrix;
        Self {
            mvp_transform: new_main_mvp_transform,
        }
    }
}