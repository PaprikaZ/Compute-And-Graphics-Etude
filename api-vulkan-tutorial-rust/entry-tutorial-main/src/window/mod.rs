use ::window_uniform::prelude::*;


pub struct Window {
    pub event_loop: WindowUniformEventLoop<()>,
    pub entity_main: WindowUniformWindow,
}

impl Window {
    pub fn create(
        window_title: &str,
        window_inner_size: WindowUniformDpiLogicalSize<i32>)
     -> Result<Self, WindowUniformErrorOperationSystem>
    {
        let (main_window, event_loop) = Self::create_window(window_title, window_inner_size)?;
        let window =
            Self {
                event_loop: event_loop,
                entity_main: main_window,
            };
        Ok(window)
    }

    fn create_window(
        window_title: &str,
        window_inner_size: WindowUniformDpiLogicalSize<i32>)
     -> Result<(WindowUniformWindow, WindowUniformEventLoop<()>), WindowUniformErrorOperationSystem>
    {
        let window_event_loop = WindowUniformEventLoop::new();
        let window =
            WindowUniformBuilderWindow::new()
            .with_title(window_title)
            .with_inner_size(window_inner_size)
            .build(&window_event_loop)?;
        Ok((window, window_event_loop))
    }
}