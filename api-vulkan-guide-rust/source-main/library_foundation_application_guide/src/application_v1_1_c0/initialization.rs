use ::library_foundation_reintroduction::window_uniform::WindowUniformWindow;
use ::library_foundation_reintroduction::vulkan::VulkanBuilderHas;
use ::library_foundation_reintroduction::vulkan::VulkanEntry;
use ::library_foundation_reintroduction::vulkan::VulkanInstance;
use ::library_foundation_reintroduction::vulkan::VulkanExtensionDebugUtilityMessengerCreateInformation;
use ::library_foundation_reintroduction::vulkan::VulkanInstanceCreateInformation;
use ::library_foundation_reintroduction::vulkan::VulkanWindow;
use ::library_foundation_reintroduction::vulkan::VulkanSurfaceKhr;
use ::library_foundation_reintroduction::vulkan::version::VulkanVersionApi;
use ::library_foundation_vulkan_cooked::vulkan_requirement::instance::VulkanRequirementInstance;
use ::library_foundation_vulkan_cooked::initialization::window::InitializationWindowUniform;

use crate::error::foundation_application_guide::ErrorFoundationApplicationGuideOwn;
use crate::error::foundation_application_guide::ErrorFoundationApplicationGuide;
use crate::application_v1_1_c0::config::ApplicationConfig;
use crate::application_v1_1_c0::vulkan_debug::ApplicationVulkanDebug;
use crate::application_v1_1_c0::Application;


pub struct ApplicationInitialization {}

impl ApplicationInitialization {
    pub fn initialize_vulkan_instance(config: &ApplicationConfig, vulkan_entry: &VulkanEntry)
    -> Result<VulkanInstance, ErrorFoundationApplicationGuide>
    {
        let api_version: VulkanVersionApi =
            vulkan_entry
            .version()
            .map_err(|_| ErrorFoundationApplicationGuideOwn::VulkanInstanceVersionApiQueryFail)?.into();
        config.vulkan.version_api_least_requirement.fulfill_instance(&api_version)?;
        let vulkan_application_information =
            config.vulkan.create_vulkan_application_information(&api_version);
        let vulkan_instance_layer_name_s =
            VulkanRequirementInstance::fulfill_layer_s(
                vulkan_entry,
                &config.vulkan.instance_layer_name_s_required,
                &config.vulkan.instance_layer_name_s_optional)?;
        let vulkan_instance_extension_name_s =
            VulkanRequirementInstance::fulfill_extension_s(
                vulkan_entry,
                &config.vulkan.instance_extension_window_name_s,
                &config.vulkan.instance_extension_name_s_required,
                &config.vulkan.instance_extension_name_s_optional)?;
        let mut vulkan_debug_messager_create_information =
            VulkanExtensionDebugUtilityMessengerCreateInformation::builder()
            .message_severity(config.vulkan.extension_debug_utility_message_severity_flag_s)
            .message_type(config.vulkan.extension_debug_utility_message_type_flag_s)
            .user_callback(Some(ApplicationVulkanDebug::callback))
            .build();
        //
        let vulkan_instance_create_information =
            VulkanInstanceCreateInformation::builder()
            .application_info(&vulkan_application_information)
            .enabled_layer_names(vulkan_instance_layer_name_s.iter().map(|n| n.as_ptr()).collect::<Vec<_>>().as_ref())
            .enabled_extension_names(vulkan_instance_extension_name_s.iter().map(|n| n.as_ptr()).collect::<Vec<_>>().as_ref())
            .push_next(&mut vulkan_debug_messager_create_information)
            .build();
        match unsafe { vulkan_entry.create_instance(&vulkan_instance_create_information, None) } {
            Err(_e) => Err(ErrorFoundationApplicationGuideOwn::VulkanInstanceCreateFail)?,
            Ok(i) => Ok(i),
        }
    }

    pub fn initialize_vulkan_surface(vulkan_instance: &VulkanInstance, window: &WindowUniformWindow)
    -> Result<VulkanSurfaceKhr, ErrorFoundationApplicationGuide>
    {
        match unsafe { VulkanWindow::create_surface(&vulkan_instance, window, window) } {
            Err(_e) => Err(ErrorFoundationApplicationGuideOwn::VulkanSurfaceCreateFail)?,
            Ok(s) => Ok(s),
        }
    }

    pub fn initialize(config: &ApplicationConfig)
    -> Result<Application, ErrorFoundationApplicationGuide>
    {
        let (_window, _window_event_loop) =
            InitializationWindowUniform::initialize_window_and_event_loop(
                config.window_title,
                config.window_inner_size)?;
        todo!()
    }
}
