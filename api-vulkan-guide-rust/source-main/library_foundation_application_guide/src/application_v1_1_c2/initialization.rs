use ::library_foundation_reintroduction::window_uniform::WindowUniformWindow;
use ::library_foundation_reintroduction::vulkan::VulkanDeviceLogicalCreateInformation;
use ::library_foundation_reintroduction::vulkan::VulkanDeviceVersion1_0;
use ::library_foundation_reintroduction::vulkan::VulkanBuilderHas;
use ::library_foundation_reintroduction::vulkan::VulkanEntry;
use ::library_foundation_reintroduction::vulkan::VulkanInstance;
use ::library_foundation_reintroduction::vulkan::VulkanExtensionDebugUtility;
use ::library_foundation_reintroduction::vulkan::VulkanExtensionDebugUtilityMessengerCreateInformation;
use ::library_foundation_reintroduction::vulkan::VulkanInstanceCreateInformation;
use ::library_foundation_reintroduction::vulkan::VulkanInstanceCreateFlagS;
use ::library_foundation_reintroduction::vulkan::VulkanWindow;
use ::library_foundation_reintroduction::vulkan::VulkanSurfaceKhr;
use ::library_foundation_reintroduction::vulkan::VulkanFormat;
use ::library_foundation_reintroduction::vulkan::VulkanDeviceLogical;
use ::library_foundation_reintroduction::vulkan::VulkanRenderPass;
use ::library_foundation_reintroduction::vulkan::VulkanAttachmentDescription;
use ::library_foundation_reintroduction::vulkan::VulkanAttachmentReference;
use ::library_foundation_reintroduction::vulkan::VulkanAttachmentLoadOperation;
use ::library_foundation_reintroduction::vulkan::VulkanAttachmentStoreOperation;
use ::library_foundation_reintroduction::vulkan::VulkanImageLayout;
use ::library_foundation_reintroduction::vulkan::VulkanSampleCountFlagS;
use ::library_foundation_reintroduction::vulkan::VulkanSubpassDescription;
use ::library_foundation_reintroduction::vulkan::VulkanSubpassDependency;
use ::library_foundation_reintroduction::vulkan::VulkanPipelineBindPoint;
use ::library_foundation_reintroduction::vulkan::VulkanAccessFlagS;
use ::library_foundation_reintroduction::vulkan::VulkanPipelineStageFlagS;
use ::library_foundation_reintroduction::vulkan::VULKAN_SUBPASS_EXTERNAL;
use ::library_foundation_reintroduction::vulkan::VulkanImageView;
use ::library_foundation_reintroduction::vulkan::VulkanExtentD2;
use ::library_foundation_reintroduction::vulkan::VulkanFrameBuffer;
use ::library_foundation_reintroduction::vulkan::VulkanFrameBufferCreateInformation;
use ::library_foundation_reintroduction::vulkan::VulkanRenderPassCreateInformation;
use ::library_foundation_reintroduction::vulkan::VulkanCommandPool;
use ::library_foundation_reintroduction::vulkan::VulkanCommandPoolCreateInformation;
use ::library_foundation_reintroduction::vulkan::VulkanCommandPoolCreateFlagS;
use ::library_foundation_reintroduction::vulkan::VulkanCommandBufferAllocateInformation;
use ::library_foundation_reintroduction::vulkan::VulkanCommandBufferLevel;
use ::library_foundation_reintroduction::vulkan::VulkanCommandBuffer;
use ::library_foundation_reintroduction::vulkan::VulkanFence;
use ::library_foundation_reintroduction::vulkan::VulkanFenceCreateInformation;
use ::library_foundation_reintroduction::vulkan::VulkanFenceCreateFlagS;
use ::library_foundation_reintroduction::vulkan::VulkanSemaphore;
use ::library_foundation_reintroduction::vulkan::VulkanSemaphoreCreateInformation;
use ::library_foundation_reintroduction::vulkan::VulkanExtensionDebugUtilityMessenger;
use ::library_foundation_reintroduction::vulkan::VULKAN_EXTENSION_GET_PHYSICAL_DEVICE_PROPERTIES2_KHR;
use ::library_foundation_reintroduction::vulkan::VULKAN_EXTENSION_PORTABILITY_ENUMERATION_KHR;
use ::library_foundation_reintroduction::vulkan::queue::VulkanQueueFamilyIndexGraphic;
use ::library_foundation_reintroduction::vulkan::version::VulkanVersionEntry;
use ::library_foundation_reintroduction::vulkan::version::VulkanVersionApi;
use ::library_foundation_reintroduction::vulkan::validation::VulkanValidationBeToEnable;
use ::library_foundation_reintroduction::vulkan::portability::VULKAN_PORTABILITY_VERSION_ENTRY_MACOS_MIN;
use ::library_foundation_vulkan_cooked::vulkan_requirement::instance::VulkanRequirementInstance;
use ::library_foundation_vulkan_cooked::initialization::window::InitializationWindowUniform;
use ::library_foundation_vulkan_cooked::initialization::vulkan_library_loader::InitializationVulkanLibraryLoader;
use ::library_foundation_vulkan_cooked::initialization::vulkan_entry::InitializationVulkanEntry;
use ::library_foundation_vulkan_cooked::initialization::vulkan_device_logical::InitializationVulkanDeviceLogical;
use ::library_foundation_vulkan_cooked::initialization::vulkan_swapchain::InitializationVulkanSwapchain;
use ::library_foundation_vulkan_cooked::negotiation::vulkan_swapchain::NegotiationVulkanSwapchain;

use crate::error::foundation_application_guide::ErrorFoundationApplicationGuideOwn;
use crate::error::foundation_application_guide::ErrorFoundationApplicationGuide;
use crate::application_v1_1_c2::config::ApplicationConfig;
use crate::application_v1_1_c2::vulkan_debug::ApplicationVulkanDebug;
use crate::application_v1_1_c2::negotiation_vulkan_device_physical::ApplicationNegotiationVulkanDevicePhysical;
use crate::application_v1_1_c2::negotiation_vulkan_swapchain::ApplicationNegotiationVulkanSwapchain;
use crate::application_v1_1_c2::self_::ApplicationPartWindow;
use crate::application_v1_1_c2::self_::ApplicationPartMain;
use crate::application_v1_1_c2::self_::Application;


pub struct ApplicationInitialization {}

impl ApplicationInitialization {
    fn patch_config_window_extension(config: &mut ApplicationConfig, window: &WindowUniformWindow)
    {
        VulkanWindow::get_required_instance_extensions(window)
        .iter()
        .for_each(|n| { let _ = config.vulkan.instance_extension_name_s_required.insert(**n); })
    }

    fn patch_config_portability_macos(config: &mut ApplicationConfig, vulkan_entry: &VulkanEntry)
    -> Result<(), ErrorFoundationApplicationGuide>
    {
        let vulkan_entry_version: VulkanVersionEntry =
            vulkan_entry.version()
            .or(Err(ErrorFoundationApplicationGuideOwn::VulkanEntryVersionGetFail))?.into();
        if cfg!(target_os = "macos") && VULKAN_PORTABILITY_VERSION_ENTRY_MACOS_MIN <= vulkan_entry_version {
            config.vulkan.instance_extension_name_s_required.insert(VULKAN_EXTENSION_GET_PHYSICAL_DEVICE_PROPERTIES2_KHR.name);
            config.vulkan.instance_extension_name_s_required.insert(VULKAN_EXTENSION_PORTABILITY_ENUMERATION_KHR.name);
            config.vulkan.instance_create_flag_s = VulkanInstanceCreateFlagS::ENUMERATE_PORTABILITY_KHR;
        }
        Ok(())
    }

    fn initialize_vulkan_instance(config: &ApplicationConfig, vulkan_entry: &VulkanEntry)
    -> Result<(VulkanInstance, Option<VulkanExtensionDebugUtilityMessenger>), ErrorFoundationApplicationGuide>
    {
        let vulkan_api_version: VulkanVersionApi =
            vulkan_entry
            .version()
            .map_err(|_| ErrorFoundationApplicationGuideOwn::VulkanInstanceVersionApiQueryFail)?.into();
        config.vulkan.version_api_least_requirement.fulfill_instance(&vulkan_api_version)?;
        let vulkan_application_information =
            config.vulkan.create_vulkan_application_information(&vulkan_api_version);
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
        let mut vulkan_debug_utility_messenger_create_information =
            VulkanExtensionDebugUtilityMessengerCreateInformation::builder()
            .message_severity(config.vulkan.extension_debug_utility_message_severity_flag_s)
            .message_type(config.vulkan.extension_debug_utility_message_type_flag_s)
            .user_callback(Some(ApplicationVulkanDebug::callback))
            .build();
        //
        let vulkan_instance_layer_name_ptr_s =
            vulkan_instance_layer_name_s.iter().map(|n| n.as_ptr()).collect::<Vec<_>>();
        let vulkan_instance_extension_name_ptr_s =
            vulkan_instance_extension_name_s.iter().map(|n| n.as_ptr()).collect::<Vec<_>>();
        let vulkan_instance_create_information =
            VulkanInstanceCreateInformation::builder()
            .application_info(&vulkan_application_information)
            .enabled_layer_names(&vulkan_instance_layer_name_ptr_s)
            .enabled_extension_names(&vulkan_instance_extension_name_ptr_s)
            .flags(config.vulkan.instance_create_flag_s)
            .push_next(&mut vulkan_debug_utility_messenger_create_information)
            .build();
        let vulkan_instance =
            unsafe { vulkan_entry.create_instance(&vulkan_instance_create_information, None) }
            .or(Err(ErrorFoundationApplicationGuideOwn::VulkanInstanceCreateFail))?;
        let vulkan_debug_utility_messenger_o =
            if let VulkanValidationBeToEnable::Y = config.vulkan.be_to_enable_validation {
                Some(
                unsafe { vulkan_instance.create_debug_utils_messenger_ext(&vulkan_debug_utility_messenger_create_information, None) }
                .or(Err(ErrorFoundationApplicationGuideOwn::VulkanDebugUtilityMessengerCreateFail))?)
            } else {
                None
            };
        Ok((vulkan_instance, vulkan_debug_utility_messenger_o))
    }

    fn initialize_vulkan_surface(vulkan_instance: &VulkanInstance, window: &WindowUniformWindow)
    -> Result<VulkanSurfaceKhr, ErrorFoundationApplicationGuide>
    {
        match unsafe { VulkanWindow::create_surface(vulkan_instance, window, window) } {
            Err(_e) => Err(ErrorFoundationApplicationGuideOwn::VulkanSurfaceCreateFail)?,
            Ok(s) => Ok(s),
        }
    }

    fn initialize_render_pass(vulkan_logical_device: &VulkanDeviceLogical, vulkan_swapchain_format: VulkanFormat)
    -> Result<VulkanRenderPass, ErrorFoundationApplicationGuide>
    {
        let vulkan_color_attachment =
            VulkanAttachmentDescription::builder()
            .format(vulkan_swapchain_format)
            .samples(VulkanSampleCountFlagS::_1)
            .load_op(VulkanAttachmentLoadOperation::CLEAR)
            .store_op(VulkanAttachmentStoreOperation::STORE)
            .stencil_load_op(VulkanAttachmentLoadOperation::DONT_CARE)
            .stencil_store_op(VulkanAttachmentStoreOperation::DONT_CARE)
            .initial_layout(VulkanImageLayout::UNDEFINED)
            .final_layout(VulkanImageLayout::PRESENT_SRC_KHR)
            .build();
        let vulkan_color_attachment_reference =
            VulkanAttachmentReference::builder()
            .attachment(0)
            .layout(VulkanImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .build();
        let vulkan_attachment_s = &[vulkan_color_attachment];
        let vulkan_color_attachment_reference_s = &[vulkan_color_attachment_reference];
        //
        let vulkan_subpass_description =
            VulkanSubpassDescription::builder()
            .pipeline_bind_point(VulkanPipelineBindPoint::GRAPHICS)
            .color_attachments(vulkan_color_attachment_reference_s)
            .build();
        let vulkan_subpass_dependency =
            VulkanSubpassDependency::builder()
            .src_subpass(VULKAN_SUBPASS_EXTERNAL)
            .dst_subpass(0)
            .src_stage_mask(VulkanPipelineStageFlagS::COLOR_ATTACHMENT_OUTPUT)
            .src_access_mask(VulkanAccessFlagS::empty())
            .dst_stage_mask(VulkanPipelineStageFlagS::COLOR_ATTACHMENT_OUTPUT)
            .dst_access_mask(VulkanAccessFlagS::COLOR_ATTACHMENT_WRITE)
            .build();
        let vulkan_subpass_description_s = &[vulkan_subpass_description];
        let vulkan_subpass_dependency_s = &[vulkan_subpass_dependency];
        //
        let vulkan_render_pass_create_information =
            VulkanRenderPassCreateInformation::builder()
            .attachments(vulkan_attachment_s)
            .subpasses(vulkan_subpass_description_s)
            .dependencies(vulkan_subpass_dependency_s)
            .build();
        unsafe { vulkan_logical_device.create_render_pass(&vulkan_render_pass_create_information, None) }
        .or(Err(ErrorFoundationApplicationGuideOwn::VulkanRenderPassCreateFail.into()))
    }

    fn initialize_frame_buffer_s(
        vulkan_logical_device: &VulkanDeviceLogical,
        vulkan_image_view_s: &[VulkanImageView],
        vulkan_render_pass: VulkanRenderPass,
        vulkan_extent: VulkanExtentD2)
    -> Result<Vec<VulkanFrameBuffer>, ErrorFoundationApplicationGuide>
    {
        vulkan_image_view_s
        .iter()
        .try_fold(Vec::new(), |mut result_frame_buffer_s, iv| {
            let vulkan_frame_buffer_create_information =
                VulkanFrameBufferCreateInformation::builder()
                .render_pass(vulkan_render_pass)
                .attachments(&[*iv])
                .width(vulkan_extent.width)
                .height(vulkan_extent.height)
                .layers(1)
                .build();
            unsafe { vulkan_logical_device.create_framebuffer(&vulkan_frame_buffer_create_information, None) }
            .map(|fb| { result_frame_buffer_s.push(fb); result_frame_buffer_s })
        })
        .or(Err(ErrorFoundationApplicationGuideOwn::VulkanFrameBufferCreateFail.into()))
    }

    fn initialize_command_pool_and_buffer_main(
        vulkan_logical_device: &VulkanDeviceLogical,
        vulkan_graphic_queue_family_index: VulkanQueueFamilyIndexGraphic)
    -> Result<(VulkanCommandPool, VulkanCommandBuffer), ErrorFoundationApplicationGuide>
    {
        let vulkan_command_pool_create_information =
            VulkanCommandPoolCreateInformation::builder()
            .flags(VulkanCommandPoolCreateFlagS::RESET_COMMAND_BUFFER)
            .queue_family_index(vulkan_graphic_queue_family_index.as_raw())
            .build();
        let main_vulkan_command_pool =
            unsafe { vulkan_logical_device.create_command_pool(&vulkan_command_pool_create_information, None) }
            .or(Err(ErrorFoundationApplicationGuideOwn::VulkanCommandPoolCreateFail))?;
        let vulkan_command_buffer_allocate_information =
            VulkanCommandBufferAllocateInformation::builder()
            .command_pool(main_vulkan_command_pool)
            .level(VulkanCommandBufferLevel::PRIMARY)
            .command_buffer_count(1)
            .build();
        let vulkan_command_buffer_s =
            unsafe { vulkan_logical_device.allocate_command_buffers(&vulkan_command_buffer_allocate_information) }
            .or(Err(ErrorFoundationApplicationGuideOwn::VulkanCommandBufferAllocateFail))?;
        assert!(vulkan_command_buffer_s.len() == 1);
        let main_vulkan_command_buffer = vulkan_command_buffer_s[0];
        Ok((main_vulkan_command_pool, main_vulkan_command_buffer))
    }

    fn initialize_synchronization_primitive_set(vulkan_logical_device: &VulkanDeviceLogical)
    -> Result<(VulkanFence, VulkanSemaphore, VulkanSemaphore), ErrorFoundationApplicationGuide>
    {
        let vulkan_fence_create_information =
            VulkanFenceCreateInformation::builder().flags(VulkanFenceCreateFlagS::SIGNALED).build();
        let frame_rendering_finished_vulkan_fence =
            unsafe { vulkan_logical_device.create_fence(&vulkan_fence_create_information, None) }
            .or(Err(ErrorFoundationApplicationGuideOwn::VulkanFenceCreateFail))?;
        let vulkan_semaphore_create_information = VulkanSemaphoreCreateInformation::builder().build();
        let image_available_vulkan_semaphore =
            unsafe { vulkan_logical_device.create_semaphore(&vulkan_semaphore_create_information, None) }
            .or(Err(ErrorFoundationApplicationGuideOwn::VulkanSemaphoreCreateFail))?;
        let render_finished_vulkan_semaphore =
            unsafe { vulkan_logical_device.create_semaphore(&vulkan_semaphore_create_information, None) }
            .or(Err(ErrorFoundationApplicationGuideOwn::VulkanSemaphoreCreateFail))?;
        Ok((frame_rendering_finished_vulkan_fence, image_available_vulkan_semaphore, render_finished_vulkan_semaphore))
    }

    //
    pub fn initialize<'t>(config: ApplicationConfig<'t>)
    -> Result<Application<'t>, ErrorFoundationApplicationGuide>
    {
        let (window, window_event_loop) =
            InitializationWindowUniform::initialize_window_and_event_loop(config.window_title, config.window_inner_size)?;
        let vulkan_library_loader = InitializationVulkanLibraryLoader::initialize()?;
        let vulkan_entry = InitializationVulkanEntry::initialize(vulkan_library_loader)?;
        let config = {
            let mut config = config;
            Self::patch_config_window_extension(&mut config, &window);
            Self::patch_config_portability_macos(&mut config, &vulkan_entry)?;
            config
        };
        let (vulkan_instance, vulkan_debug_utility_messenger_o) =
            Self::initialize_vulkan_instance(&config, &vulkan_entry)?;
        let vulkan_surface = Self::initialize_vulkan_surface(&vulkan_instance, &window)?;
        let (vulkan_physical_device,
             vulkan_graphic_queue_family_index, vulkan_present_queue_family_index,
             matched_vulkan_extension_name_s, matched_vulkan_physical_device_feature_name_s,
             vulkan_surface_capability_s, available_vulkan_surface_format_s, available_vulkan_present_mode_s) =
            ApplicationNegotiationVulkanDevicePhysical::try_pick_queue_family_index_s_graphic_rank(
                &config, &vulkan_instance, vulkan_surface)?;
        let matched_vulkan_extension_name_s =
            matched_vulkan_extension_name_s.into_iter() .map(|n| n.clone()).collect::<Vec<_>>();
        let matched_vulkan_physical_device_feature_name_s =
            matched_vulkan_physical_device_feature_name_s.into_iter().map(|n| n.clone()).collect::<Vec<_>>();
        let vulkan_logical_device =
            InitializationVulkanDeviceLogical::initialize(
                &vulkan_instance, vulkan_physical_device,
                &matched_vulkan_extension_name_s, &matched_vulkan_physical_device_feature_name_s,
                vulkan_graphic_queue_family_index, vulkan_present_queue_family_index,
                VulkanDeviceLogicalCreateInformation::builder())?;
        let vulkan_graphic_queue =
            unsafe { vulkan_logical_device.get_device_queue(vulkan_graphic_queue_family_index.as_raw(), 0) };
        let vulkan_present_queue =
            unsafe { vulkan_logical_device.get_device_queue(vulkan_present_queue_family_index.as_raw(), 0) };
        //
        let vulkan_surface_format =
            ApplicationNegotiationVulkanSwapchain::negotiate_surface_format(
                config.vulkan_swapchain.format_prioritized,
                config.vulkan_swapchain.color_space_prioritized,
                &available_vulkan_surface_format_s);
        let vulkan_present_mode =
            NegotiationVulkanSwapchain::negotiate_present_mode(
                config.vulkan_swapchain.present_mode_prioritized,
                config.vulkan_swapchain.present_mode_fallback,
                &available_vulkan_present_mode_s);
        let vulkan_2d_extent =
            NegotiationVulkanSwapchain::negotiate_extent(&window, &vulkan_surface_capability_s);
        let vulkan_swapchain_image_number =
            NegotiationVulkanSwapchain::negotiate_image_number(&vulkan_surface_capability_s);
        let (vulkan_swapchain_sharing_mode, vulkan_swapchain_queue_family_index_s) =
            NegotiationVulkanSwapchain::negotiate_sharing_mode_and_queue_family_index_s_graphic_present(
                vulkan_graphic_queue_family_index, vulkan_present_queue_family_index);
        let (vulkan_swapchain, vulkan_swapchain_image_s, vulkan_swapchain_image_view_s) =
            InitializationVulkanSwapchain::initialize_with_image_and_view_s(
                vulkan_surface, &vulkan_logical_device, vulkan_surface_capability_s,
                vulkan_swapchain_sharing_mode, &vulkan_swapchain_queue_family_index_s,
                vulkan_swapchain_image_number, vulkan_2d_extent, vulkan_surface_format, vulkan_present_mode, None)?;
        let vulkan_render_pass =
            Self::initialize_render_pass(&vulkan_logical_device, vulkan_surface_format.format)?;
        let vulkan_swapchain_frame_buffer_s =
            Self::initialize_frame_buffer_s(&vulkan_logical_device, &vulkan_swapchain_image_view_s, vulkan_render_pass, vulkan_2d_extent)?;
        let (main_vulkan_command_pool, main_vulkan_command_buffer) =
            Self::initialize_command_pool_and_buffer_main(&vulkan_logical_device, vulkan_graphic_queue_family_index)?;
        let (render_finished_vulkan_fence, render_finished_vulkan_semaphore, image_available_vulkan_semaphore) =
            Self::initialize_synchronization_primitive_set(&vulkan_logical_device)?;
        let wp_application = ApplicationPartWindow::new(window, window_event_loop);
        let mp_application =
            ApplicationPartMain::new(
                config, vulkan_entry, vulkan_instance, vulkan_surface,
                vulkan_physical_device, vulkan_graphic_queue_family_index, vulkan_present_queue_family_index,
                matched_vulkan_extension_name_s, matched_vulkan_physical_device_feature_name_s,
                vulkan_surface_capability_s, available_vulkan_surface_format_s, available_vulkan_present_mode_s,
                vulkan_logical_device, vulkan_graphic_queue, vulkan_present_queue,
                vulkan_surface_format, vulkan_present_mode, vulkan_2d_extent,
                vulkan_swapchain_image_number, vulkan_swapchain_sharing_mode,
                vulkan_swapchain, vulkan_swapchain_image_s, vulkan_swapchain_image_view_s,
                vulkan_render_pass, vulkan_swapchain_frame_buffer_s,
                main_vulkan_command_pool, main_vulkan_command_buffer,
                render_finished_vulkan_fence, render_finished_vulkan_semaphore, image_available_vulkan_semaphore,
                vulkan_debug_utility_messenger_o,
            );
        Ok(Application::<'t>::new(wp_application, mp_application))
    }
}