use ::library_foundation_application_guide::application_v1_1_c1::initialization::ApplicationInitialization
    as ApplicationV1_1Chapter1Initialization;
use ::library_foundation_application_guide::application_v1_1_c1::continuation::ApplicationContinuation
    as ApplicationV1_1Chapter1Continuation;
use ::library_foundation_application_guide::application_v1_1_c1::termination::ApplicationTermination
    as ApplicationV1_1Chapter1Termination;
use ::library_foundation_application_guide::predefine_config::application_v1_1_c1::PredefineConfigApplicationV1_1Chapter1;
//
use ::library_foundation_application_guide::application_v1_1_c2::initialization::ApplicationInitialization
    as ApplicationV1_1Chapter2Initialization;
use ::library_foundation_application_guide::application_v1_1_c2::continuation::ApplicationContinuation
    as ApplicationV1_1Chapter2Continuation;
use ::library_foundation_application_guide::application_v1_1_c2::termination::ApplicationTermination
    as ApplicationV1_1Chapter2Termination;
use ::library_foundation_application_guide::predefine_config::application_v1_1_c2::PredefineConfigApplicationV1_1Chapter2;
//
use ::library_foundation_application_guide::application_v1_1_c3::initialization::ApplicationInitialization
    as ApplicationV1_1Chapter3Initialization;
use ::library_foundation_application_guide::application_v1_1_c3::continuation::ApplicationContinuation
    as ApplicationV1_1Chapter3Continuation;
use ::library_foundation_application_guide::application_v1_1_c3::termination::ApplicationTermination
    as ApplicationV1_1Chapter3Termination;
use ::library_foundation_application_guide::predefine_config::application_v1_1_c3::PredefineConfigApplicationV1_1Chapter3;
//
use ::library_foundation_application_guide::application_v1_1_c3_scene::initialization::ApplicationInitialization
    as ApplicationV1_1Chapter3SceneInitialization;
use ::library_foundation_application_guide::application_v1_1_c3_scene::continuation::ApplicationContinuation
    as ApplicationV1_1Chapter3SceneContinuation;
use ::library_foundation_application_guide::application_v1_1_c3_scene::termination::ApplicationTermination
    as ApplicationV1_1Chapter3SceneTermination;
use ::library_foundation_application_guide::predefine_config::application_v1_1_c3_scene::PredefineConfigApplicationV1_1Chapter3Scene;

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
            },
            EntryArgument::RunApplication(ApplicationName::VulkanV1_1Chapter1) => {
                let application_config = PredefineConfigApplicationV1_1Chapter1::get();
                let application =
                    ApplicationV1_1Chapter1Initialization::initialize(application_config)?;
                let (uniform_window, mp_application) =
                    ApplicationV1_1Chapter1Continuation::continue_loop_window_event(application)?;
                ApplicationV1_1Chapter1Termination::terminate(uniform_window, mp_application)?;
            },
            EntryArgument::RunApplication(ApplicationName::VulkanV1_1Chapter2) => {
                let application_config = PredefineConfigApplicationV1_1Chapter2::get();
                let application =
                    ApplicationV1_1Chapter2Initialization::initialize(application_config)?;
                let (uniform_window, mp_application) =
                    ApplicationV1_1Chapter2Continuation::continue_loop_window_event(application)?;
                ApplicationV1_1Chapter2Termination::terminate(uniform_window, mp_application)?;
            },
            EntryArgument::RunApplication(ApplicationName::VulkanV1_1Chapter3) => {
                let application_config = PredefineConfigApplicationV1_1Chapter3::get();
                let application =
                    ApplicationV1_1Chapter3Initialization::initialize(application_config)?;
                let (uniform_window, mp_application) = ApplicationV1_1Chapter3Continuation::continue_loop_window_event(application)?;
                ApplicationV1_1Chapter3Termination::terminate(uniform_window, mp_application)?;
            },
            EntryArgument::RunApplication(ApplicationName::VulkanV1_1Chapter3Scene) => {
                let application_config = PredefineConfigApplicationV1_1Chapter3Scene::get();
                let application =
                    ApplicationV1_1Chapter3SceneInitialization::initialize(application_config)?;
                let (uniform_window, mp_application) =
                    ApplicationV1_1Chapter3SceneContinuation::continue_loop_window_event(application)?;
                ApplicationV1_1Chapter3SceneTermination::terminate(uniform_window, mp_application)?;
            },
        }
        Ok(())
    }
}