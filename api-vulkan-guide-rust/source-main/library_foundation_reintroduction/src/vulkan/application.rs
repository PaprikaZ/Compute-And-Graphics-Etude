use std::ffi::CStr;

use crate::error::foundation_reintroduction::ErrorFoundationReintroductionOwn;
use crate::error::foundation_reintroduction::ErrorFoundationReintroduction;


#[derive(Debug, Clone, Copy)]
pub struct VulkanApplicationName<'t>(&'t CStr);

impl<'t> VulkanApplicationName<'t> {
    pub fn try_new(name_data: &'t [u8]) -> Result<Self, ErrorFoundationReintroduction> {
        let c_str_r = CStr::from_bytes_with_nul(name_data);
        match c_str_r {
            Ok(c_str) => Ok(Self(c_str)),
            Err(_) => Err(ErrorFoundationReintroductionOwn::VulkanApplicationNameInvalid)?,
        }
    }

    pub fn as_raw(self) -> &'t CStr {
        self.0
    }

    pub fn as_ref_byte_s_with_nul(self) -> &'t [u8] {
        self.0.to_bytes_with_nul()
    }
}
