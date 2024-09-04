use ::library_foundation_reintroduction::window_uniform::WindowUniformEventLoop;
use ::library_foundation_reintroduction::window_uniform::WindowUniformWindow;
use ::library_foundation_reintroduction::vulkan::VulkanLibraryLoader;
use ::library_foundation_reintroduction::vulkan::VulkanEntry;


#[derive(Debug)]
pub struct ApplicationPartWindow {
    window_event_loop: WindowUniformEventLoop<()>,
    window: WindowUniformWindow,
}

impl ApplicationPartWindow {
    pub fn new(window: WindowUniformWindow, window_event_loop: WindowUniformEventLoop<()>)
    -> Self
    {
        Self {
            window: window,
            window_event_loop: window_event_loop,
        }
    }

    pub fn as_raw(self) -> (WindowUniformWindow, WindowUniformEventLoop<()>) {
        (self.window, self.window_event_loop)
    }

    pub fn get_window(&self) -> &WindowUniformWindow {
        &self.window
    }

    pub fn get_window_event_loop(&self) -> &WindowUniformEventLoop<()> {
        &self.window_event_loop
    }
}


#[derive(Debug)]
pub struct ApplicationPartMain {
    pub vulkan_library_loader: VulkanLibraryLoader,
    pub vulkan_entry: VulkanEntry,
    //
    pub be_destroying: bool,
    pub be_window_minimized: bool,
    pub signal_window_resized: bool,
}

impl ApplicationPartMain {
    pub fn render(&mut self, _window: &WindowUniformWindow) {
        todo!()
    }

    pub fn terminate(&self)
    {
        todo!()
    }
}


#[derive(Debug)]
pub struct Application(ApplicationPartWindow, ApplicationPartMain);

impl Application {
    pub fn as_raw(self) -> (ApplicationPartWindow, ApplicationPartMain) {
        (self.0, self.1)
    }

    pub fn terminate(self)
    {
        todo!()
    }
}