use ::library_foundation_reintroduction::window_uniform::WindowUniformEvent;
use ::library_foundation_reintroduction::window_uniform::WindowUniformEventWindow;

use crate::error::foundation_application_guide::ErrorFoundationApplicationGuideOwn;
use crate::error::foundation_application_guide::ErrorFoundationApplicationGuide;
use crate::application_v1_1_c0::self_::Application;


pub struct ApplicationContinuation {}

impl ApplicationContinuation {
    pub fn continue_loop_window_event(application: Application)
    -> Result<(), ErrorFoundationApplicationGuide>
    {
        type WE = WindowUniformEvent<()>;
        type WEW = WindowUniformEventWindow;
        let (wp_application, mut mp_application) = application.unwrap();
        wp_application.window_event_loop.run(move |event, window_target| {
            match event {
                WE::AboutToWait => {
                    todo!()
                },
                WE::WindowEvent { event: WEW::RedrawRequested, .. }
                if !window_target.exiting() && !mp_application.be_window_minimized =>
                {
                    let _render_return = mp_application.render(&wp_application.window);
                    //TODO terminate window
                },
                WE::WindowEvent { event: WEW::Resized(size), .. } => {
                    match (size.width, size.height) {
                        (0, 0) => { mp_application.be_window_minimized = true; },
                        _ => {
                            mp_application.be_window_minimized = false;
                            mp_application.signal_window_resized = true;
                        },
                    }
                },
                WE::WindowEvent { event: WEW::CloseRequested, .. } => {
                    window_target.exit();
                    let _mp_destroy_result = mp_application.terminate();
                },
                WE::WindowEvent { event: WEW::KeyboardInput { event: _event, .. }, .. } => {
                    ()
                },
                _ => (),
            }
        })
        .or(Err(ErrorFoundationApplicationGuideOwn::WindowEventLoopRunAbort))?;
        //let _wp_destroy_result = wp_application.terminate();
        Ok(())
    }
}