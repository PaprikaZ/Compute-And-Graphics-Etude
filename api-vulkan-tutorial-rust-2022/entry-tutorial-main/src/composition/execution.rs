use ::window_uniform::prelude::*;

use crate::window::Window;
use crate::application::main::Application;
use crate::termination::TerminationProcessMain;
use crate::lib::window_viewport::WindowViewportLogicalNumber;


pub struct CompositionExecution {}

impl CompositionExecution {
    pub fn bind_window_event_loop_default(window: Window, mut application: Application)
    -> Result<(), TerminationProcessMain>
    {
        let mut be_destroying = false;
        let mut be_minimized = false;

        window.event_loop.run(move |event, _, control_flow| {
            *control_flow = WindowUniformEventLoopControlFlow::Poll;
            match event {
                WindowUniformEvent::MainEventsCleared if !be_destroying && !be_minimized => {
                    let _render_result = unsafe { application.render(&window.entity_main) };
                },
                WindowUniformEvent::WindowEvent { event: WindowUniformEventWindow::Resized(size), .. } =>
                    match (size.width, size.height) {
                        (0, 0) => { be_minimized = true; },
                        _ => {
                            be_minimized = false;
                            application.signal_window_resized = true;
                        }
                    },
                WindowUniformEvent::WindowEvent { event: WindowUniformEventWindow::CloseRequested, .. } => {
                    be_destroying = true;
                    *control_flow = WindowUniformEventLoopControlFlow::Exit;
                    let _destroy_result = unsafe { application.destroy() };
                },
                WindowUniformEvent::WindowEvent {
                    event: WindowUniformEventWindow::KeyboardInput { input: keyboard_input, .. }, ..  } =>
                {
                    match (keyboard_input.state, keyboard_input.virtual_keycode) {
                        (WindowUniformKeyState::Pressed, Some(WindowUniformKeyVirtualCode::Left)) =>
                            application.window_viewport_logical_number.decrease_one(),
                        (WindowUniformKeyState::Pressed, Some(WindowUniformKeyVirtualCode::Right)) =>
                            application.window_viewport_logical_number.increase_one(),
                        _ => (),
                    }
                },
                _ => {}
            }
        });
    }
}