use ::library_foundation_reintroduction::error::foundation_reintroduction::ErrorFoundationReintroductionOwn;
use ::library_foundation_reintroduction::error::foundation_reintroduction::ErrorFoundationReintroduction;
use ::library_foundation_vulkan_cooked::error::foundation_vulkan_cooked::ErrorFoundationVulkanCookedOwn;
use ::library_foundation_vulkan_cooked::error::foundation_vulkan_cooked::ErrorFoundationVulkanCooked;
use ::library_foundation_application_guide::error::foundation_application_guide::ErrorFoundationApplicationGuideOwn;
use ::library_foundation_application_guide::error::foundation_application_guide::ErrorFoundationApplicationGuide;


#[derive(Debug)]
pub enum ErrorEntryGuideOwn {
    EntryArgumentParseFail(String),
}

#[derive(Debug)]
pub enum ErrorEntryGuide {
    Own(ErrorEntryGuideOwn),
    ScopeReintroduction(ErrorFoundationReintroductionOwn),
    ScopeVulkanCooked(ErrorFoundationVulkanCookedOwn),
    ScopeApplicationGuide(ErrorFoundationApplicationGuideOwn),
}

impl From<ErrorEntryGuideOwn> for ErrorEntryGuide {
    fn from(error: ErrorEntryGuideOwn) -> Self {
        Self::Own(error)
    }
}

impl From<ErrorFoundationReintroduction> for ErrorEntryGuide {
    fn from(error: ErrorFoundationReintroduction) -> Self {
        match error {
            ErrorFoundationReintroduction::Own(e) =>
                Self::ScopeReintroduction(e),
        }
    }
}

impl From<ErrorFoundationReintroductionOwn> for ErrorEntryGuide {
    fn from(error: ErrorFoundationReintroductionOwn) -> Self {
        Self::ScopeReintroduction(error)
    }
}

impl From<ErrorFoundationVulkanCooked> for ErrorEntryGuide {
    fn from(error: ErrorFoundationVulkanCooked) -> Self {
        match error {
            ErrorFoundationVulkanCooked::Own(e) =>
                Self::ScopeVulkanCooked(e),
        }
    }
}

impl From<ErrorFoundationVulkanCookedOwn> for ErrorEntryGuide {
    fn from(error: ErrorFoundationVulkanCookedOwn) -> Self {
        Self::ScopeVulkanCooked(error)
    }
}

impl From<ErrorFoundationApplicationGuide> for ErrorEntryGuide {
    fn from(error: ErrorFoundationApplicationGuide) -> Self {
        match error {
            ErrorFoundationApplicationGuide::ScopeReintroduction(err) =>
                Self::ScopeReintroduction(err),
            ErrorFoundationApplicationGuide::ScopeVulkanCooked(err) =>
                Self::ScopeVulkanCooked(err),
            ErrorFoundationApplicationGuide::Own(err) =>
                Self::ScopeApplicationGuide(err),
        }
    }
}

impl From<ErrorFoundationApplicationGuideOwn> for ErrorEntryGuide {
    fn from(error: ErrorFoundationApplicationGuideOwn) -> Self {
        Self::ScopeApplicationGuide(error)
    }
}