#[derive(Debug)]
pub enum ErrorFoundationReintroductionOwn {
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