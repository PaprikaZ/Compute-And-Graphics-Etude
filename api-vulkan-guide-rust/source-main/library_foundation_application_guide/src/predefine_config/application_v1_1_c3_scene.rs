use std::path::PathBuf;
use std::collections::HashSet;

use ::library_foundation_reintroduction::window_uniform::WindowUniformDpiLogicalSize;
use ::library_foundation_reintroduction::vulkan::VulkanExtensionDebugUtilityMessageTypeFlagS;
use ::library_foundation_reintroduction::vulkan::VulkanExtensionDebugUtilityMessageSeverityFlagS;
use ::library_foundation_reintroduction::vulkan::VulkanFormat;
use ::library_foundation_reintroduction::vulkan::VulkanColorSpaceKhr;
use ::library_foundation_reintroduction::vulkan::VulkanPresentModeKhr;
use ::library_foundation_reintroduction::vulkan::VULKAN_EXTENSION_SWAPCHAIN_KHR;
use ::library_foundation_reintroduction::vulkan::version::VulkanVersionEngine;
use ::library_foundation_reintroduction::vulkan::version::VulkanVersionApplication;
use ::library_foundation_reintroduction::vulkan::engine::VulkanEngineName;
use ::library_foundation_reintroduction::vulkan::application::VulkanApplicationName;
use ::library_foundation_reintroduction::vulkan::validation::VulkanValidationBeToEnable;
use ::library_foundation_vulkan_cooked::vulkan_requirement::version::VulkanRequirementVersionApiLeast;
use ::library_foundation_vulkan_cooked::vulkan_rank::weight::VulkanRankWeightFactorExponential;
use ::library_foundation_vulkan_cooked::config_vulkan::base::ConfigVulkanBase;
use ::library_foundation_vulkan_cooked::config_vulkan::rank::ConfigVulkanRank;
use ::library_foundation_vulkan_cooked::config_vulkan::swapchain::ConfigVulkanSwapchain;

use crate::application_v1_1_c3_scene::config::ApplicationConfig;


pub struct PredefineConfigApplicationV1_1Chapter3Scene {}

impl PredefineConfigApplicationV1_1Chapter3Scene {
    pub fn get<'t>() -> ApplicationConfig<'t> {
        let expect_message = "PredefineConfigApplicationV1_1Chapter3Scene: predefine hold";
        let vulkan_physical_device_extension_name_s = {
            let mut s = HashSet::new();
            s.insert(VULKAN_EXTENSION_SWAPCHAIN_KHR.name);
            s
        };
        let base_vulkan_config =
            ConfigVulkanBase::create(
                VulkanRequirementVersionApiLeast::new(1, 1, 0),
                VulkanEngineName::try_new(b"Vulkan Guide Engine\0").expect(expect_message),
                VulkanVersionEngine::new(1, 0, 0),
                VulkanApplicationName::try_new(b"Vulkan Guide Example\0").expect(expect_message),
                VulkanVersionApplication::new(1, 0, 0),
                HashSet::new(), HashSet::new(),
                HashSet::new(), HashSet::new(), HashSet::new(),
                vulkan_physical_device_extension_name_s, HashSet::new(),
                HashSet::new(), HashSet::new(),
                VulkanExtensionDebugUtilityMessageTypeFlagS::GENERAL
                | VulkanExtensionDebugUtilityMessageTypeFlagS::VALIDATION
                | VulkanExtensionDebugUtilityMessageTypeFlagS::PERFORMANCE,
                VulkanExtensionDebugUtilityMessageSeverityFlagS::all(),
                if cfg!(debug_assertions) { VulkanValidationBeToEnable::Y } else { VulkanValidationBeToEnable::N });
        let rank_vulkan_config =
            ConfigVulkanRank::new(
                VulkanRankWeightFactorExponential::try_new(1).expect(expect_message),
                VulkanRankWeightFactorExponential::try_new(4).expect(expect_message),
                VulkanRankWeightFactorExponential::try_new(1).expect(expect_message),
                VulkanRankWeightFactorExponential::try_new(0).expect(expect_message),
                VulkanRankWeightFactorExponential::try_new(0).expect(expect_message));
        let swapchain_vulkan_config =
            ConfigVulkanSwapchain::new(
                VulkanFormat::R8G8B8A8_SRGB,
                VulkanColorSpaceKhr::SRGB_NONLINEAR,
                VulkanPresentModeKhr::MAILBOX,
                VulkanPresentModeKhr::FIFO);
        let shader_source_directory_path: PathBuf =
            [env!("CARGO_MANIFEST_DIR"), r"..\..\", r"source-shader\"].iter().collect();
        let resource_directory_path: PathBuf =
            [env!("CARGO_MANIFEST_DIR"), r"..\..\", r"resource\"].iter().collect();
        ApplicationConfig::new(
            "Vulkan Guide Example Chapter 1",
            WindowUniformDpiLogicalSize::new(1280, 720),
            base_vulkan_config,
            rank_vulkan_config,
            swapchain_vulkan_config,
            shader_source_directory_path,
            PathBuf::from("triangle-dynamic.vert.spv"),
            PathBuf::from("triangle-color.frag.spv"),
            resource_directory_path,
            PathBuf::from("monkey_smooth.obj"))
    }
}