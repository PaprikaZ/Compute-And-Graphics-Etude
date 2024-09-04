use ::library_foundation_reintroduction::window_uniform::WindowUniformEventLoop;
use ::library_foundation_reintroduction::window_uniform::WindowUniformWindow;
use ::library_foundation_reintroduction::vulkan::VulkanHandler;
use ::library_foundation_reintroduction::vulkan::VulkanBuilderHas;
use ::library_foundation_reintroduction::vulkan::VulkanInstanceVersion1_0;
use ::library_foundation_reintroduction::vulkan::VulkanDeviceVersion1_0;
use ::library_foundation_reintroduction::vulkan::VulkanEntry;
use ::library_foundation_reintroduction::vulkan::VulkanInstance;
use ::library_foundation_reintroduction::vulkan::VulkanSurfaceKhr;
use ::library_foundation_reintroduction::vulkan::VulkanDevicePhysical;
use ::library_foundation_reintroduction::vulkan::VulkanDeviceLogical;
use ::library_foundation_reintroduction::vulkan::VulkanExtensionName;
use ::library_foundation_reintroduction::vulkan::VulkanSurfaceCapabilitySKhr;
use ::library_foundation_reintroduction::vulkan::VulkanSurfaceFormatKhr;
use ::library_foundation_reintroduction::vulkan::VulkanSurfaceExtensionKhr;
use ::library_foundation_reintroduction::vulkan::VulkanPresentModeKhr;
use ::library_foundation_reintroduction::vulkan::VulkanQueue;
use ::library_foundation_reintroduction::vulkan::VulkanExtentD2;
use ::library_foundation_reintroduction::vulkan::VulkanSharingMode;
use ::library_foundation_reintroduction::vulkan::VulkanSwapchainKhr;
use ::library_foundation_reintroduction::vulkan::VulkanSwapchainExtensionKhr;
use ::library_foundation_reintroduction::vulkan::VulkanImage;
use ::library_foundation_reintroduction::vulkan::VulkanImageView;
use ::library_foundation_reintroduction::vulkan::VulkanRenderPass;
use ::library_foundation_reintroduction::vulkan::VulkanRenderPassBeginInformation;
use ::library_foundation_reintroduction::vulkan::VulkanFrameBuffer;
use ::library_foundation_reintroduction::vulkan::VulkanCommandPool;
use ::library_foundation_reintroduction::vulkan::VulkanCommandBuffer;
use ::library_foundation_reintroduction::vulkan::VulkanCommandBufferResetFlagS;
use ::library_foundation_reintroduction::vulkan::VulkanFence;
use ::library_foundation_reintroduction::vulkan::VulkanSemaphore;
use ::library_foundation_reintroduction::vulkan::VulkanExtensionDebugUtilityMessenger;
use ::library_foundation_reintroduction::vulkan::VulkanExtensionDebugUtility;
use ::library_foundation_reintroduction::vulkan::VulkanCommandBufferBeginInformation;
use ::library_foundation_reintroduction::vulkan::VulkanCommandBufferUsageFlagS;
use ::library_foundation_reintroduction::vulkan::VulkanClearValue;
use ::library_foundation_reintroduction::vulkan::VulkanClearColorValue;
use ::library_foundation_reintroduction::vulkan::VulkanRectangleD2;
use ::library_foundation_reintroduction::vulkan::VulkanOffsetD2;
use ::library_foundation_reintroduction::vulkan::VulkanSubpassContents;
use ::library_foundation_reintroduction::vulkan::VulkanPipelineStageFlagS;
use ::library_foundation_reintroduction::vulkan::VulkanSubmitInformation;
use ::library_foundation_reintroduction::vulkan::VulkanPresentInformationKhr;
use ::library_foundation_reintroduction::vulkan::VulkanErrorCode_;
use ::library_foundation_reintroduction::vulkan::VulkanSuccessCode_;
use ::library_foundation_reintroduction::vulkan::queue::VulkanQueueFamilyIndexGraphic;
use ::library_foundation_reintroduction::vulkan::queue::VulkanQueueFamilyIndexPresent;
use ::library_foundation_reintroduction::vulkan::swapchain::VulkanSwapchainImageNumber;
use ::library_foundation_reintroduction::vulkan::swapchain::VulkanSwapchainImageIndex;
use ::library_foundation_vulkan_cooked::vulkan_device_physical::feature::VulkanDevicePhysicalFeatureStandardName;

use crate::error::foundation_application_guide::ErrorFoundationApplicationGuideOwn;
use crate::error::foundation_application_guide::ErrorFoundationApplicationGuide;
use crate::application_v1_1_c1::config::ApplicationConfig;


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
pub struct ApplicationPartMain<'t> {
    config: ApplicationConfig<'t>,
    vulkan_entry: VulkanEntry,
    vulkan_instance: VulkanInstance,
    vulkan_surface: VulkanSurfaceKhr,
    vulkan_device_physical: VulkanDevicePhysical,
    vulkan_queue_family_index_graphic: VulkanQueueFamilyIndexGraphic,
    vulkan_queue_family_index_present: VulkanQueueFamilyIndexPresent,
    vulkan_device_physical_extension_name_s_matched: Vec<VulkanExtensionName>,
    vulkan_device_physical_feature_name_s_matched: Vec<VulkanDevicePhysicalFeatureStandardName>,
    vulkan_surface_capability_s: VulkanSurfaceCapabilitySKhr,
    vulkan_surface_format_s_available: Vec<VulkanSurfaceFormatKhr>,
    vulkan_present_mode_s_available: Vec<VulkanPresentModeKhr>,
    vulkan_device_logical: VulkanDeviceLogical,
    vulkan_queue_graphic: VulkanQueue,
    vulkan_queue_present: VulkanQueue,
    vulkan_surface_format: VulkanSurfaceFormatKhr,
    vulkan_present_mode: VulkanPresentModeKhr,
    vulkan_extent_d2: VulkanExtentD2,
    vulkan_swapchain_image_number: VulkanSwapchainImageNumber,
    vulkan_swapchain_sharing_mode: VulkanSharingMode,
    vulkan_swapchain: VulkanSwapchainKhr,
    vulkan_swapchain_image_s: Vec<VulkanImage>,
    vulkan_swapchain_image_view_s: Vec<VulkanImageView>,
    vulkan_render_pass: VulkanRenderPass,
    vulkan_swapchain_frame_buffer_s: Vec<VulkanFrameBuffer>,
    vulkan_command_pool_main: VulkanCommandPool,
    vulkan_command_buffer_main: VulkanCommandBuffer,
    vulkan_fence_render_finished: VulkanFence,
    vulkan_semaphore_render_finished: VulkanSemaphore,
    vulkan_semaphore_image_available: VulkanSemaphore,
    vulkan_debug_messenger_o: Option<VulkanExtensionDebugUtilityMessenger>,
    //
    number_frame_rendered: u32,
    //
    be_destroying: bool,
    be_window_minimized: bool,
    flag_signal_window_resized: bool,
}

impl<'t> ApplicationPartMain<'t> {
    pub fn new(
        config: ApplicationConfig<'t>,
        vulkan_entry: VulkanEntry,
        vulkan_instance: VulkanInstance,
        vulkan_surface: VulkanSurfaceKhr,
        vulkan_physical_device: VulkanDevicePhysical,
        vulkan_graphic_queue_family_index: VulkanQueueFamilyIndexGraphic,
        vulkan_present_queue_family_index: VulkanQueueFamilyIndexPresent,
        matched_vulkan_physical_device_extension_name_s: Vec<VulkanExtensionName>,
        matched_vulkan_physical_device_feature_name_s: Vec<VulkanDevicePhysicalFeatureStandardName>,
        vulkan_surface_capability_s: VulkanSurfaceCapabilitySKhr,
        available_vulkan_surface_format_s: Vec<VulkanSurfaceFormatKhr>,
        available_vulkan_present_mode_s: Vec<VulkanPresentModeKhr>,
        vulkan_logical_device: VulkanDeviceLogical,
        vulkan_graphic_queue: VulkanQueue,
        vulkan_present_queue: VulkanQueue,
        vulkan_surface_format: VulkanSurfaceFormatKhr,
        vulkan_present_mode: VulkanPresentModeKhr,
        vulkan_2d_extent: VulkanExtentD2,
        vulkan_swapchain_image_number: VulkanSwapchainImageNumber,
        vulkan_swapchain_sharing_mode: VulkanSharingMode,
        vulkan_swapchain: VulkanSwapchainKhr,
        vulkan_swapchain_image_s: Vec<VulkanImage>,
        vulkan_swapchain_image_view_s: Vec<VulkanImageView>,
        vulkan_render_pass: VulkanRenderPass,
        vulkan_swapchain_frame_buffer_s: Vec<VulkanFrameBuffer>,
        main_vulkan_command_pool: VulkanCommandPool,
        main_vulkan_command_buffer: VulkanCommandBuffer,
        render_finished_vulkan_fence: VulkanFence,
        render_finished_vulkan_semaphore: VulkanSemaphore,
        image_available_vulkan_semaphore: VulkanSemaphore,
        vulkan_debug_messenger_o: Option<VulkanExtensionDebugUtilityMessenger>)
    -> Self
    {
        Self {
            config: config,
            vulkan_entry: vulkan_entry,
            vulkan_instance: vulkan_instance,
            vulkan_surface: vulkan_surface,
            vulkan_device_physical: vulkan_physical_device,
            vulkan_queue_family_index_graphic: vulkan_graphic_queue_family_index,
            vulkan_queue_family_index_present: vulkan_present_queue_family_index,
            vulkan_device_physical_extension_name_s_matched: matched_vulkan_physical_device_extension_name_s,
            vulkan_device_physical_feature_name_s_matched: matched_vulkan_physical_device_feature_name_s,
            vulkan_surface_capability_s: vulkan_surface_capability_s,
            vulkan_surface_format_s_available: available_vulkan_surface_format_s,
            vulkan_present_mode_s_available: available_vulkan_present_mode_s,
            vulkan_device_logical: vulkan_logical_device,
            vulkan_queue_graphic: vulkan_graphic_queue,
            vulkan_queue_present: vulkan_present_queue,
            vulkan_surface_format: vulkan_surface_format,
            vulkan_present_mode: vulkan_present_mode,
            vulkan_extent_d2: vulkan_2d_extent,
            vulkan_swapchain_image_number: vulkan_swapchain_image_number,
            vulkan_swapchain_sharing_mode: vulkan_swapchain_sharing_mode,
            vulkan_swapchain: vulkan_swapchain,
            vulkan_swapchain_image_s: vulkan_swapchain_image_s,
            vulkan_swapchain_image_view_s: vulkan_swapchain_image_view_s,
            vulkan_render_pass: vulkan_render_pass,
            vulkan_swapchain_frame_buffer_s: vulkan_swapchain_frame_buffer_s,
            vulkan_command_pool_main: main_vulkan_command_pool,
            vulkan_command_buffer_main: main_vulkan_command_buffer,
            vulkan_fence_render_finished: render_finished_vulkan_fence,
            vulkan_semaphore_render_finished: render_finished_vulkan_semaphore,
            vulkan_semaphore_image_available: image_available_vulkan_semaphore,
            vulkan_debug_messenger_o: vulkan_debug_messenger_o,
            //
            number_frame_rendered: 0,
            //
            be_destroying: false,
            be_window_minimized: false,
            flag_signal_window_resized: false,
        }
    }

    pub fn get_config(&self) -> &ApplicationConfig {
        &self.config
    }

    pub fn get_vulkan_entry(&self) -> &VulkanEntry {
        &self.vulkan_entry
    }

    pub fn get_vulkan_instance(&self) -> &VulkanInstance {
        &self.vulkan_instance
    }

    pub fn get_vulkan_surface(&self) -> &VulkanSurfaceKhr {
        &self.vulkan_surface
    }

    pub fn get_vulkan_device_physical(&self) -> &VulkanDevicePhysical {
        &self.vulkan_device_physical
    }

    pub fn get_vulkan_queue_family_index_graphic(&self) -> &VulkanQueueFamilyIndexGraphic {
        &self.vulkan_queue_family_index_graphic
    }

    pub fn get_vulkan_queue_family_index_present(&self) -> &VulkanQueueFamilyIndexPresent {
        &self.vulkan_queue_family_index_present
    }

    pub fn get_vulkan_device_physical_extension_name_s_matched(&self) -> &Vec<VulkanExtensionName> {
        &self.vulkan_device_physical_extension_name_s_matched
    }

    pub fn get_vulkan_device_physical_feature_name_s_matched(&self) -> &Vec<VulkanDevicePhysicalFeatureStandardName> {
        &self.vulkan_device_physical_feature_name_s_matched
    }

    pub fn get_vulkan_surface_capability_s(&self) -> &VulkanSurfaceCapabilitySKhr {
        &self.vulkan_surface_capability_s
    }

    pub fn get_vulkan_surface_format_s_available(&self) -> &Vec<VulkanSurfaceFormatKhr> {
        &self.vulkan_surface_format_s_available
    }

    pub fn get_vulkan_present_mode_s_available(&self) -> &Vec<VulkanPresentModeKhr> {
        &self.vulkan_present_mode_s_available
    }

    pub fn get_vulkan_device_logical(&self) -> &VulkanDeviceLogical {
        &self.vulkan_device_logical
    }

    pub fn get_vulkan_queue_graphic(&self) -> &VulkanQueue {
        &self.vulkan_queue_graphic
    }

    pub fn get_vulkan_queue_present(&self) -> &VulkanQueue {
        &self.vulkan_queue_present
    }

    pub fn get_vulkan_surface_format(&self) -> &VulkanSurfaceFormatKhr {
        &self.vulkan_surface_format
    }

    pub fn get_vulkan_present_mode(&self) -> &VulkanPresentModeKhr {
        &self.vulkan_present_mode
    }

    pub fn get_vulkan_extent_d2(&self) -> &VulkanExtentD2 {
        &self.vulkan_extent_d2
    }

    pub fn get_vulkan_swapchain_image_number(&self) -> &VulkanSwapchainImageNumber {
        &self.vulkan_swapchain_image_number
    }

    pub fn get_vulkan_swapchain_sharing_mode(&self) -> &VulkanSharingMode {
        &self.vulkan_swapchain_sharing_mode
    }

    pub fn get_vulkan_swapchain(&self) -> &VulkanSwapchainKhr {
        &self.vulkan_swapchain
    }

    pub fn get_vulkan_swapchain_image_s(&self) -> &Vec<VulkanImage> {
        &self.vulkan_swapchain_image_s
    }

    pub fn get_vulkan_swapchain_image_view_s(&self) -> &Vec<VulkanImageView> {
        &self.vulkan_swapchain_image_view_s
    }

    pub fn get_vulkan_render_pass(&self) -> &VulkanRenderPass {
        &self.vulkan_render_pass
    }

    pub fn get_vulkan_swapchain_frame_buffer_s(&self) -> &Vec<VulkanFrameBuffer> {
        &self.vulkan_swapchain_frame_buffer_s
    }

    pub fn get_vulkan_command_pool_main(&self) -> &VulkanCommandPool {
        &self.vulkan_command_pool_main
    }

    pub fn get_vulkan_command_buffer_main(&self) -> &VulkanCommandBuffer {
        &self.vulkan_command_buffer_main
    }

    pub fn get_vulkan_fence_render_finished(&self) -> &VulkanFence {
        &self.vulkan_fence_render_finished
    }

    pub fn get_vulkan_semaphore_render_finished(&self) -> &VulkanSemaphore {
        &self.vulkan_semaphore_render_finished
    }

    pub fn get_vulkan_semaphore_image_available(&self) -> &VulkanSemaphore {
        &self.vulkan_semaphore_image_available
    }

    pub fn get_vulkan_debug_messenger_o(&self) -> &Option<VulkanExtensionDebugUtilityMessenger> {
        &self.vulkan_debug_messenger_o
    }

    pub fn is_destroying(&self) -> bool {
        self.be_destroying
    }

    pub fn set_be_destroying(&mut self, be_destroying: bool) {
        self.be_destroying = be_destroying;
    }

    pub fn is_window_minimized(&self) -> bool {
        self.be_window_minimized
    }

    pub fn set_be_window_minimized(&mut self, be_window_minimized: bool) {
        self.be_window_minimized = be_window_minimized;
    }

    pub fn get_flag_signal_window_resized(&self) -> bool {
        self.flag_signal_window_resized
    }

    pub fn set_flag_signal_window_resized(&mut self, window_resized_signal_flag: bool) {
        self.flag_signal_window_resized = window_resized_signal_flag;
    }

    pub fn recreate_swapchain(&mut self, _window: &WindowUniformWindow) -> Result<(), ErrorFoundationApplicationGuide>
    {
        todo!()
    }
}


#[derive(Debug)]
pub struct Application<'t>(ApplicationPartWindow, ApplicationPartMain<'t>);

impl<'t> Application<'t> {
    pub fn new(wp_application: ApplicationPartWindow, mp_application: ApplicationPartMain<'t>)
    -> Self
    {
        Self(wp_application, mp_application)
    }

    pub fn as_raw(self) -> (ApplicationPartWindow, ApplicationPartMain<'t>) {
        (self.0, self.1)
    }
}
