use ::library_foundation_reintroduction::window_uniform::WindowUniformEventLoop;
use ::library_foundation_reintroduction::window_uniform::WindowUniformWindow;
use ::library_foundation_reintroduction::vulkan::VulkanLibraryLoader;
use ::library_foundation_reintroduction::vulkan::VulkanEntry;


#[derive(Debug)]
pub struct ApplicationPartWindow {
    pub window_event_loop: WindowUniformEventLoop<()>,
    pub window: WindowUniformWindow,
}

impl ApplicationPartWindow {
    pub fn terminate(&self)
    {
        todo!()
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
