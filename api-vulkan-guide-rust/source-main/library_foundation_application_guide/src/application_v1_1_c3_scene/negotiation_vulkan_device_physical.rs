use ::library_foundation_reintroduction::vulkan::VulkanInstance;
use ::library_foundation_reintroduction::vulkan::VulkanInstanceVersion1_0;
use ::library_foundation_reintroduction::vulkan::VulkanSurfaceKhr;
use ::library_foundation_reintroduction::vulkan::VulkanDevicePhysical;
use ::library_foundation_reintroduction::vulkan::VulkanVersionVanilla;
use ::library_foundation_reintroduction::vulkan::VulkanExtensionName;
use ::library_foundation_reintroduction::vulkan::VulkanSurfaceCapabilitySKhr;
use ::library_foundation_reintroduction::vulkan::VulkanSurfaceFormatKhr;
use ::library_foundation_reintroduction::vulkan::VulkanPresentModeKhr;
use ::library_foundation_reintroduction::vulkan::version::VulkanVersionApi;
use ::library_foundation_reintroduction::vulkan::queue::VulkanQueueFamilyIndexGraphic;
use ::library_foundation_reintroduction::vulkan::queue::VulkanQueueFamilyIndexPresent;
use ::library_foundation_vulkan_cooked::error::foundation_vulkan_cooked::ErrorFoundationVulkanCookedOwn;
use ::library_foundation_vulkan_cooked::error::foundation_vulkan_cooked::ErrorFoundationVulkanCooked;
use ::library_foundation_vulkan_cooked::vulkan_device_physical::feature::VulkanDevicePhysicalFeatureStandardName;
use ::library_foundation_vulkan_cooked::vulkan_requirement::device_physical::VulkanRequirementDevicePhysical;
use ::library_foundation_vulkan_cooked::vulkan_rank::score::VulkanRankScore;
use ::library_foundation_vulkan_cooked::vulkan_rank::VulkanRank;

use crate::error::foundation_application_guide::ErrorFoundationApplicationGuideOwn;
use crate::error::foundation_application_guide::ErrorFoundationApplicationGuide;
use crate::application_v1_1_c3_scene::config::ApplicationConfig;


#[derive(Debug)]
enum ApplicationNegotiationVulkanDevicePhysicalRequirementNotMatch {
    VersionApiNotFulfilled,
    QueueFamilyNotFulfilled,
    ExtensionNotFulfilled,
    SwapchainFormatNoneFulfilled,
    SwapchainPresentModeNoneFulfilled,
    FeatureNotFulfilled,
}

#[derive(Debug)]
enum ApplicationNegotiationVulkanDevicePhysicalRequirementPickError {
    RequirementNotMatch(ApplicationNegotiationVulkanDevicePhysicalRequirementNotMatch),
    Error(ErrorFoundationApplicationGuide),
}

pub struct ApplicationNegotiationVulkanDevicePhysical {}

impl ApplicationNegotiationVulkanDevicePhysical {
    fn try_pick_queue_family_index_s_graphic_rank_one_map_err(error: ErrorFoundationVulkanCooked)
    -> ApplicationNegotiationVulkanDevicePhysicalRequirementPickError
    {
        type EVC = ErrorFoundationVulkanCooked;
        type EVCO = ErrorFoundationVulkanCookedOwn;
        type RNM = ApplicationNegotiationVulkanDevicePhysicalRequirementNotMatch;
        type RPE = ApplicationNegotiationVulkanDevicePhysicalRequirementPickError;
        match error {
            EVC::Own(EVCO::VulkanRequirementVersionApiLeastDevicePhysicalNotFulfilled) =>
                RPE::RequirementNotMatch(RNM::VersionApiNotFulfilled),
            EVC::Own(EVCO::VulkanRequirementDevicePhysicalQueueFamilySNotFulfilled) =>
                RPE::RequirementNotMatch(RNM::QueueFamilyNotFulfilled),
            EVC::Own(EVCO::VulkanRequirementDevicePhysicalExtensionSNotFulfilled) =>
                RPE::RequirementNotMatch(RNM::ExtensionNotFulfilled),
            EVC::Own(EVCO::VulkanRequirementDevicePhysicalSurfaceFormatNoneFulfilled) =>
                RPE::RequirementNotMatch(RNM::SwapchainFormatNoneFulfilled),
            EVC::Own(EVCO::VulkanRequirementDevicePhysicalSurfacePresentModeNoneFulfilled) =>
                RPE::RequirementNotMatch(RNM::SwapchainPresentModeNoneFulfilled),
            EVC::Own(EVCO::VulkanRequirementDevicePhysicalFeatureSNotFulfilled) =>
                RPE::RequirementNotMatch(RNM::FeatureNotFulfilled),
            e => RPE::Error(e.into()),
        }
    }

    fn try_pick_queue_family_index_s_graphic_rank_one<'t>(
        config: &'t ApplicationConfig,
        vulkan_instance: &VulkanInstance,
        vulkan_surface: VulkanSurfaceKhr,
        vulkan_physical_device: VulkanDevicePhysical,
        vulkan_rank: &VulkanRank)
    -> Result<(VulkanQueueFamilyIndexGraphic, VulkanQueueFamilyIndexPresent,
               Vec<&'t VulkanExtensionName>, Vec<&'t VulkanDevicePhysicalFeatureStandardName>,
               VulkanSurfaceCapabilitySKhr, Vec<VulkanSurfaceFormatKhr>, Vec<VulkanPresentModeKhr>,
               VulkanRankScore),
              ApplicationNegotiationVulkanDevicePhysicalRequirementPickError>
    {
        let map_err = Self::try_pick_queue_family_index_s_graphic_rank_one_map_err;
        let vulkan_physical_device_property_s = unsafe { vulkan_instance.get_physical_device_properties(vulkan_physical_device) };
        VulkanRequirementDevicePhysical::fulfill_version(
            vulkan_instance, vulkan_physical_device, &config.vulkan.version_api_least_requirement)
            .map_err(map_err)?;
        let (graphic_queue_family_index, present_queue_family_index) =
            VulkanRequirementDevicePhysical::fulfill_queue_family_graphic_present(
                vulkan_instance, vulkan_physical_device, vulkan_surface)
            .map_err(map_err)?;
        let (matched_extension_name_s, matched_optional_extension_number) =
            VulkanRequirementDevicePhysical::fulfill_extension_s(
                vulkan_instance, vulkan_physical_device,
                &config.vulkan.device_physical_extension_name_s_required,
                &config.vulkan.device_physical_extension_name_s_optional)
            .map_err(map_err)?;
        let (surface_capability_s, surface_format_s, present_mode_s) =
            VulkanRequirementDevicePhysical::fulfill_surface(
                vulkan_instance, vulkan_physical_device, vulkan_surface)
            .map_err(map_err)?;
        let (matched_feature_name_s, matched_optional_feature_number) =
            VulkanRequirementDevicePhysical::fulfill_feature_s(
                vulkan_instance, vulkan_physical_device,
                &config.vulkan.device_physical_feature_name_s_required,
                &config.vulkan.device_physical_feature_name_s_optional)
            .map_err(map_err)?;
        //
        let vulkan_physical_device_rank_score =
            vulkan_rank.calculate_score_device_physical(
                vulkan_physical_device_property_s.device_type,
                config.vulkan.version_api_least_requirement,
                VulkanVersionApi::from(VulkanVersionVanilla::from(vulkan_physical_device_property_s.api_version)),
                matched_optional_extension_number,
                matched_optional_feature_number);
        Ok((graphic_queue_family_index, present_queue_family_index,
            matched_extension_name_s, matched_feature_name_s,
            surface_capability_s, surface_format_s, present_mode_s,
            vulkan_physical_device_rank_score))
    }

    pub fn try_pick_queue_family_index_s_graphic_rank<'t>(
        config: &'t ApplicationConfig,
        vulkan_instance: &VulkanInstance,
        vulkan_surface: VulkanSurfaceKhr)
    -> Result<(VulkanDevicePhysical,
               VulkanQueueFamilyIndexGraphic, VulkanQueueFamilyIndexPresent,
               Vec<&'t VulkanExtensionName>, Vec<&'t VulkanDevicePhysicalFeatureStandardName>,
               VulkanSurfaceCapabilitySKhr, Vec<VulkanSurfaceFormatKhr>, Vec<VulkanPresentModeKhr>),
              ErrorFoundationApplicationGuide>
    {
        type RPE = ApplicationNegotiationVulkanDevicePhysicalRequirementPickError;
        type RNM = ApplicationNegotiationVulkanDevicePhysicalRequirementNotMatch;
        let vulkan_physical_device_s =
            match unsafe { vulkan_instance.enumerate_physical_devices() } {
                Err(_e) => Err(ErrorFoundationApplicationGuideOwn::VulkanDevicePhysicalEnumerateFail)?,
                Ok(d_s) => d_s,
            };
        type TState<'t> = Option<(
            VulkanDevicePhysical,
            VulkanQueueFamilyIndexGraphic, VulkanQueueFamilyIndexPresent,
            Vec<&'t VulkanExtensionName>, Vec<&'t VulkanDevicePhysicalFeatureStandardName>,
            VulkanSurfaceCapabilitySKhr, Vec<VulkanSurfaceFormatKhr>, Vec<VulkanPresentModeKhr>,
            VulkanRankScore)>;
        let vulkan_rank = config.vulkan_rank.create_vulkan_rank();
        let vulkan_physical_device_queue_family_index_s_rank_score_o : TState<'t> =
            vulkan_physical_device_s
            .into_iter()
            .fold(Ok(None), |result_o_r, physical_device| {
                if result_o_r.is_err() { return result_o_r }
                match Self::try_pick_queue_family_index_s_graphic_rank_one(
                    config, vulkan_instance, vulkan_surface, physical_device, &vulkan_rank)
                {
                    Err(RPE::RequirementNotMatch(RNM::VersionApiNotFulfilled)) => //TODO logging
                        result_o_r,
                    Err(RPE::RequirementNotMatch(RNM::QueueFamilyNotFulfilled)) => //TODO logging
                        result_o_r,
                    Err(RPE::RequirementNotMatch(RNM::ExtensionNotFulfilled)) => //TODO logging
                        result_o_r,
                    Err(RPE::RequirementNotMatch(RNM::SwapchainFormatNoneFulfilled)) => //TODO logging
                        result_o_r,
                    Err(RPE::RequirementNotMatch(RNM::SwapchainPresentModeNoneFulfilled)) => //TODO logging
                        result_o_r,
                    Err(RPE::RequirementNotMatch(RNM::FeatureNotFulfilled)) => //TODO logging
                        result_o_r,
                    Err(RPE::Error(e)) =>
                        Err(e),
                    Ok((graphic_queue_family_index, surface_queue_family_index,
                        extension_name_s, feature_name_s,
                        surface_capability_s, surface_format_s, present_mode_s,
                        rank_score)) =>
                        match result_o_r.unwrap() {
                            None =>
                                Ok(Some((physical_device,
                                         graphic_queue_family_index, surface_queue_family_index,
                                         extension_name_s, feature_name_s,
                                         surface_capability_s, surface_format_s, present_mode_s,
                                         rank_score))),
                            Some((_, _, _, _, _, _, _, _, hit_rank_score)) if hit_rank_score < rank_score =>
                                Ok(Some((physical_device,
                                         graphic_queue_family_index, surface_queue_family_index,
                                         extension_name_s, feature_name_s,
                                         surface_capability_s, surface_format_s, present_mode_s,
                                         rank_score))),
                            Some(v) =>
                                Ok(Some(v)),
                        },
                }
            })?;
        match vulkan_physical_device_queue_family_index_s_rank_score_o {
            None =>
                Err(ErrorFoundationApplicationGuideOwn::VulkanDevicePhysicalRequirementNoneFulfilled)?,
            Some((pd, g_qf_i, s_qf_i, en_s, fn_s, sc_s, sf_s, pm_s, _rs)) =>
                Ok((pd, g_qf_i, s_qf_i, en_s, fn_s, sc_s, sf_s, pm_s)),
        }
    }
}
