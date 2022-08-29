pub use ::winit::dpi::LogicalSize as WindowUniformDpiLogicalSize;
pub use ::winit::event::Event as WindowUniformEvent;
pub use ::winit::event::WindowEvent as WindowUniformEventWindow;
pub use ::winit::event_loop::ControlFlow as WindowUniformEventLoopControlFlow;
pub use ::winit::event_loop::EventLoop as WindowUniformEventLoop;
pub use ::winit::window::Window as WindowUniformWindow;
pub use ::winit::window::WindowBuilder as WindowUniformBuilderWindow;
pub use ::winit::error::OsError as WindowUniformErrorOperationSystem;

pub mod prelude {
    pub use ::winit::dpi::LogicalSize as WindowUniformDpiLogicalSize;
    pub use ::winit::event::Event as WindowUniformEvent;
    pub use ::winit::event::WindowEvent as WindowUniformEventWindow;
    pub use ::winit::event_loop::ControlFlow as WindowUniformEventLoopControlFlow;
    pub use ::winit::event_loop::EventLoop as WindowUniformEventLoop;
    pub use ::winit::window::Window as WindowUniformWindow;
    pub use ::winit::window::WindowBuilder as WindowUniformBuilderWindow;
    pub use ::winit::error::OsError as WindowUniformErrorOperationSystem;
}