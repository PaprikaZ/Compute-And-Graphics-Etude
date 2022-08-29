use ::window_uniform::prelude::*;

use crate::window::Window;
use crate::application::main::Application;


pub struct CompositionExecution {}

impl CompositionExecution {
    pub fn bind_window_event_loop_default(window: Window, mut application: Application) -> () {
        let mut be_destroying = false;

        window.event_loop.run(move |event, _, control_flow| {
            *control_flow = WindowUniformEventLoopControlFlow::Poll;
            match event {
                WindowUniformEvent::MainEventsCleared if !be_destroying =>
                    unsafe { application.render(&window.entity_main).unwrap(); },
                WindowUniformEvent::WindowEvent { event: WindowUniformEventWindow::CloseRequested, .. } => {
                    be_destroying = true;
                    *control_flow = WindowUniformEventLoopControlFlow::Exit;
                    unsafe { application.destroy(); }
                },
                _ => {}
            }
        });
    }
}