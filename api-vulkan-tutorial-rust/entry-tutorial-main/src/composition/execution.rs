use ::window_uniform::prelude::*;

use crate::window::Window;
use crate::application::main::Application;
use crate::termination::TerminationProcessMain;


pub struct CompositionExecution {}

impl CompositionExecution {
    pub fn bind_window_event_loop_default(window: Window, mut application: Application)
     -> Result<(), TerminationProcessMain>
    {
        let mut be_destroying = false;

        window.event_loop.run(move |event, _, control_flow| {
            *control_flow = WindowUniformEventLoopControlFlow::Poll;
            match event {
                WindowUniformEvent::MainEventsCleared if !be_destroying => {
                    let _render_result = unsafe { application.render(&window.entity_main) };
                },
                WindowUniformEvent::WindowEvent { event: WindowUniformEventWindow::CloseRequested, .. } => {
                    be_destroying = true;
                    *control_flow = WindowUniformEventLoopControlFlow::Exit;
                    let _destroy_result = unsafe { application.destroy() };
                },
                _ => {}
            }
        });
    }
}