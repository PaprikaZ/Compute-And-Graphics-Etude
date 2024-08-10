#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum DataD3ModelResource {
    TutorialSimple(DataD3ModelResourceTutorialSimple),
    TutorialFormatObj(DataD3ModelResourceTutorialFormatObj),
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum DataD3ModelResourceTutorialSimple {
    Default,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum DataD3ModelResourceTutorialFormatObj {
    VikingRoom,
}
