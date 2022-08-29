use crate::termination::TerminationProcessMain;


pub struct InitializationLoggerConsole {
    be_initialized: bool,
}

impl InitializationLoggerConsole {
    pub fn new() -> Self {
        Self { be_initialized: false }
    }

    pub fn initialize(&mut self) -> Result<(), TerminationProcessMain> {
        match self.be_initialized {
            true => {
                pretty_env_logger::init();
                self.be_initialized = true;
                Ok(())
            },
            false => {
                Err(TerminationProcessMain::InitializationLoggerConsoleFail)
            }
        }
    }
}