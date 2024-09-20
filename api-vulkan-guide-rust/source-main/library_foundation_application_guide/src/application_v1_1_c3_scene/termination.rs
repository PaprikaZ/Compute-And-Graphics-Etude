use ::library_foundation_reintroduction::window_uniform::WindowUniformWindow;

use crate::error::foundation_application_guide::ErrorFoundationApplicationGuide;
use crate::application_v1_1_c3_scene::self_::ApplicationPartMain;


pub struct ApplicationTermination {}

impl ApplicationTermination {
    pub fn terminate(_window: WindowUniformWindow, mp_application: ApplicationPartMain)
    -> Result<(), ErrorFoundationApplicationGuide>
    {
        mp_application.destroy()
    }
}
