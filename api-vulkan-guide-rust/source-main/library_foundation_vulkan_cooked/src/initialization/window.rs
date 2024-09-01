use ::library_foundation_reintroduction::window_uniform::WindowUniformWindow;
use ::library_foundation_reintroduction::window_uniform::WindowUniformEventLoop;
use ::library_foundation_reintroduction::window_uniform::WindowUniformDpiLogicalSize;
use ::library_foundation_reintroduction::window_uniform::WindowUniformWindowBuilder;

use crate::error::foundation_vulkan_cooked::ErrorFoundationVulkanCookedOwn;
use crate::error::foundation_vulkan_cooked::ErrorFoundationVulkanCooked;


pub struct InitializationWindowUniform {}

impl InitializationWindowUniform {
    pub fn initialize_window_and_event_loop(
        window_title: &str,
        window_inner_size: WindowUniformDpiLogicalSize<i32>)
    -> Result<(WindowUniformWindow, WindowUniformEventLoop<()>), ErrorFoundationVulkanCooked>
    {
        let window_event_loop =
            WindowUniformEventLoop::new()
            .or(Err(ErrorFoundationVulkanCookedOwn::WindowUniformEventLoopCreateFail))?;
        let window =
            WindowUniformWindowBuilder::new()
            .with_title(window_title)
            .with_inner_size(window_inner_size)
            .build(&window_event_loop)
            .map_err(|_e| ErrorFoundationVulkanCookedOwn::WindowUniformCreateFail)?;
        Ok((window, window_event_loop))
    }
}