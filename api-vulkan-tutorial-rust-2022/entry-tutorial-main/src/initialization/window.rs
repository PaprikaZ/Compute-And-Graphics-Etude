use ::window_uniform::prelude::*;

use crate::termination::TerminationProcessMain;
use crate::window::Window;


pub struct InitializationWindow {}

impl InitializationWindow {
    pub fn create_main_event_loop_default()
        -> Result<Window, TerminationProcessMain>
    {
        let window_title = "Vulkan Tutorial On Rust";
        let window_inner_size = WindowUniformDpiLogicalSize::new(1024, 768);
        Window::create(window_title, window_inner_size)
        .map_err(|err| TerminationProcessMain::InitializationWindowUniformFail(err))
    }
}