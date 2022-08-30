mod config;
mod window;
mod termination;
mod application;
mod initialization;
mod composition;

use crate::termination::TerminationProcessMain;
use crate::application::main::Application;
use crate::initialization::Initialization;
use crate::initialization::window::InitializationWindow;
use crate::composition::execution::CompositionExecution;
use crate::config::VULKAN_VALIDATION_LAYER;


fn main() -> TerminationProcessMain {
    let mut initialization = Initialization::create();
    if let Err(termination) = initialization.logger_console.initialize() {
        return termination;
    }
    let window =
        match InitializationWindow::create_main_event_loop_default() {
            Err(termination) => return termination,
            Ok(uniform_window_application) => uniform_window_application,
        };
    let application = unsafe {
        match Application::create(&window.entity_main, Some(&VULKAN_VALIDATION_LAYER)) {
            Err(termination) => return termination,
            Ok(application) => application,
        }
    };
    CompositionExecution::bind_window_event_loop_default(window, application);
    TerminationProcessMain::Ok
}
