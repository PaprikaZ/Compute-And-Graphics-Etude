use ::library_foundation_application_guide::application_v1_1_c1::initialization::ApplicationInitialization
    as ApplicationV1_1Chapter1Initialization;
use ::library_foundation_application_guide::application_v1_1_c1::continuation::ApplicationContinuation
    as ApplicationV1_1Chapter1Continuation;
use ::library_foundation_application_guide::application_v1_1_c1::termination::ApplicationTermination
    as ApplicationV1_1Chapter1Termination;
use ::library_foundation_application_guide::predefine_config::application_v1_1_c1::PredefineConfigApplicationV1_1Chapter1;

use crate::error::entry_guide::ErrorEntryGuide;
use crate::application_name::self_::ApplicationName;
use crate::entry_argument::self_::EntryArgument;


pub struct Launcher {}

impl Launcher {
    pub fn launch(argument: EntryArgument)
    -> Result<(), ErrorEntryGuide>
    {
        match argument {
            EntryArgument::RunApplication(ApplicationName::VulkanV1_1Chapter0) => {
                println!("The example v1.1 chapter 0 is partial bootstraping code, no running");
                Ok(())
            },
            EntryArgument::RunApplication(ApplicationName::VulkanV1_1Chapter1) => {
                let application_config = PredefineConfigApplicationV1_1Chapter1::get();
                ApplicationV1_1Chapter1Initialization::initialize(application_config)
                .and_then(|application|
                    ApplicationV1_1Chapter1Continuation::continue_loop_window_event(application))
                .and_then(|(uniform_window, mp_application)|
                    ApplicationV1_1Chapter1Termination::terminate(uniform_window, mp_application))
                .map_err(|e| e.into())
            },
        }
    }
}