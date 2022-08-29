pub mod logger_console;
pub mod window;

use logger_console::InitializationLoggerConsole;


pub struct Initialization {
    pub logger_console: InitializationLoggerConsole,
}

impl Initialization {
    pub fn create() -> Self {
        Self { logger_console: InitializationLoggerConsole::new() }
    }
}