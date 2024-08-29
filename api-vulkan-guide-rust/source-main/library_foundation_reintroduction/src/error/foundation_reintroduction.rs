#[derive(Debug)]
pub enum ErrorFoundationReintroductionOwn {
    VulkanApplicationNameInvalid,
    VulkanEngineNameInvalid,
}


#[derive(Debug)]
pub enum ErrorFoundationReintroduction {
    Own(ErrorFoundationReintroductionOwn),
}

impl From<ErrorFoundationReintroductionOwn> for ErrorFoundationReintroduction {
    fn from(error: ErrorFoundationReintroductionOwn) -> Self {
        Self::Own(error)
    }
}