use ::library_foundation_reintroduction::vulkan::VulkanDeviceVersion1_0;
use ::library_foundation_reintroduction::vulkan::VulkanBuilderHas;
use ::library_foundation_reintroduction::vulkan::VulkanDeviceLogical;
use ::library_foundation_reintroduction::vulkan::VulkanInstance;
use ::library_foundation_reintroduction::vulkan::VulkanDevicePhysical;
use ::library_foundation_reintroduction::vulkan::VulkanInstanceVersion1_0;
use ::library_foundation_reintroduction::vulkan::VulkanSampleCountFlagS;
use ::library_foundation_reintroduction::vulkan::VulkanFormat;
use ::library_foundation_reintroduction::vulkan::VulkanImage;
use ::library_foundation_reintroduction::vulkan::VulkanImageType;
use ::library_foundation_reintroduction::vulkan::VulkanExtentD3;
use ::library_foundation_reintroduction::vulkan::VulkanImageTiling;
use ::library_foundation_reintroduction::vulkan::VulkanImageLayout;
use ::library_foundation_reintroduction::vulkan::VulkanImageUsageFlagS;
use ::library_foundation_reintroduction::vulkan::VulkanMemoryPropertyFlagS;
use ::library_foundation_reintroduction::vulkan::VulkanImageCreateInformation;
use ::library_foundation_reintroduction::vulkan::VulkanSharingMode;
use ::library_foundation_reintroduction::vulkan::VulkanDeviceMemory;
use ::library_foundation_reintroduction::vulkan::VulkanMemoryAllocateInformation;
use ::library_foundation_reintroduction::vulkan::VulkanFormatFeatureFlagS;
use ::library_foundation_reintroduction::vulkan::mipmap::VulkanMipmapLevel;

use crate::error::foundation_vulkan_cooked::ErrorFoundationVulkanCookedOwn;
use crate::error::foundation_vulkan_cooked::ErrorFoundationVulkanCooked;
use crate::vulkan_memory_raw_prefab::self_::VulkanMemoryRawPrefab;


pub struct VulkanMemoryRawPrefabImage {}

impl VulkanMemoryRawPrefabImage {
    pub fn create(
        vulkan_logical_device: &VulkanDeviceLogical,
        vulkan_image_usage_flag_s: VulkanImageUsageFlagS,
        vulkan_image_extent: (u32, u32),
        vulkan_image_format: VulkanFormat,
        vulkan_sample_count: VulkanSampleCountFlagS)
    -> Result<VulkanImage, ErrorFoundationVulkanCooked>
    {
        let vulkan_image_extent =
            VulkanExtentD3::builder()
            .width(vulkan_image_extent.0)
            .height(vulkan_image_extent.1)
            .depth(1)
            .build();
        let vulkan_image_create_information =
            VulkanImageCreateInformation::builder()
            .image_type(VulkanImageType::_2D)
            .extent(vulkan_image_extent)
            .mip_levels(VulkanMipmapLevel::new(1).as_raw())
            .array_layers(1)
            .format(vulkan_image_format)
            .tiling(VulkanImageTiling::OPTIMAL)
            .initial_layout(VulkanImageLayout::UNDEFINED)
            .usage(vulkan_image_usage_flag_s)
            .samples(vulkan_sample_count)
            .sharing_mode(VulkanSharingMode::EXCLUSIVE)
            .build();
        unsafe { vulkan_logical_device.create_image(&vulkan_image_create_information, None) }
        .or(Err(ErrorFoundationVulkanCookedOwn::VulkanImageCreateFail.into()))
    }

    pub fn create_with_memory_bound(
        vulkan_instance: &VulkanInstance,
        vulkan_physical_device: VulkanDevicePhysical,
        vulkan_logical_device: &VulkanDeviceLogical,
        vulkan_image_usage_flag_s: VulkanImageUsageFlagS,
        vulkan_image_extent: (u32, u32),
        vulkan_image_format: VulkanFormat,
        vulkan_sample_number: VulkanSampleCountFlagS)
    -> Result<(VulkanImage, VulkanDeviceMemory), ErrorFoundationVulkanCooked>
    {
        let depth_vulkan_image =
            Self::create(
                vulkan_logical_device,
                vulkan_image_usage_flag_s, vulkan_image_extent, vulkan_image_format, vulkan_sample_number)?;
        let depth_vulkan_image_memory_requirement_s =
            unsafe { vulkan_logical_device.get_image_memory_requirements(depth_vulkan_image) };
        let depth_vulkan_image_memory_type_index =
            VulkanMemoryRawPrefab::lookup_type_index(
                vulkan_instance,
                vulkan_physical_device,
                VulkanMemoryPropertyFlagS::DEVICE_LOCAL,
                depth_vulkan_image_memory_requirement_s)?;
        let depth_vulkan_image_memory_allocate_information =
            VulkanMemoryAllocateInformation::builder()
            .allocation_size(depth_vulkan_image_memory_requirement_s.size)
            .memory_type_index(depth_vulkan_image_memory_type_index.as_raw())
            .build();
        let depth_vulkan_image_memory =
            unsafe { vulkan_logical_device.allocate_memory(&depth_vulkan_image_memory_allocate_information, None) }
            .or(Err(ErrorFoundationVulkanCookedOwn::VulkanMemoryAllocateFail))?;
        unsafe { vulkan_logical_device.bind_image_memory(depth_vulkan_image, depth_vulkan_image_memory, 0) }
        .or(Err(ErrorFoundationVulkanCookedOwn::VulkanImageMemoryBindFail))?;
        Ok((depth_vulkan_image, depth_vulkan_image_memory))
    }
}


pub struct VulkanMemoryRawPrefabImageDepth {}

impl VulkanMemoryRawPrefabImageDepth {
    pub fn select_format_from_candidate_s(
        vulkan_instance: &VulkanInstance,
        vulkan_physical_device: VulkanDevicePhysical,
        vulkan_image_tiling: VulkanImageTiling,
        required_vulkan_format_feature_flag_s: VulkanFormatFeatureFlagS,
        candidating_vulkan_format_s: &[VulkanFormat])
    -> Result<VulkanFormat, ErrorFoundationVulkanCooked>
    {
        candidating_vulkan_format_s
        .iter()
        .find(|f| {
            let vulkan_physical_device_format_property_s =
                unsafe { vulkan_instance.get_physical_device_format_properties(vulkan_physical_device, **f) };
            let available_vulkan_format_feature_flag_s =
                match vulkan_image_tiling {
                    VulkanImageTiling::LINEAR => vulkan_physical_device_format_property_s.linear_tiling_features,
                    VulkanImageTiling::OPTIMAL => vulkan_physical_device_format_property_s.optimal_tiling_features,
                    _ => VulkanFormatFeatureFlagS::empty(),
                };
            available_vulkan_format_feature_flag_s.contains(required_vulkan_format_feature_flag_s)
        })
        .map(|f| f.clone())
        .ok_or(ErrorFoundationVulkanCookedOwn::VulkanImageDepthFormatFeatureNotSupport.into())
    }

    pub fn allocate(
        vulkan_instance: &VulkanInstance,
        vulkan_physical_device: VulkanDevicePhysical,
        vulkan_logical_device: &VulkanDeviceLogical,
        depth_vulkan_image_extent: (u32, u32))
    -> Result<(VulkanFormat, VulkanImage, VulkanDeviceMemory), ErrorFoundationVulkanCooked>
    {
        let candidating_vulkan_depth_image_format_s =
            [VulkanFormat::D32_SFLOAT, VulkanFormat::D32_SFLOAT_S8_UINT, VulkanFormat::D24_UNORM_S8_UINT];
        let selected_vulkan_depth_image_format_s =
            VulkanMemoryRawPrefabImageDepth::select_format_from_candidate_s(
                vulkan_instance,
                vulkan_physical_device,
                VulkanImageTiling::OPTIMAL,
                VulkanFormatFeatureFlagS::DEPTH_STENCIL_ATTACHMENT,
                candidating_vulkan_depth_image_format_s.as_slice())?;
        let (vulkan_depth_image, vulkan_depth_image_memory) =
            VulkanMemoryRawPrefabImage::create_with_memory_bound(
                vulkan_instance,
                vulkan_physical_device,
                vulkan_logical_device,
                VulkanImageUsageFlagS::DEPTH_STENCIL_ATTACHMENT,
                depth_vulkan_image_extent,
                selected_vulkan_depth_image_format_s,
                VulkanSampleCountFlagS::_1)?;
        Ok((selected_vulkan_depth_image_format_s, vulkan_depth_image, vulkan_depth_image_memory))
    }
}