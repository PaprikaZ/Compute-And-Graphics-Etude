use std::path::PathBuf;

use ::png::OutputInfo as FormatPngOutputInfomation;
use crate::termination::TerminationProcessMain;
use crate::config::path::ConfigPathFile;
use crate::data::d3_model_texture::DataD3ModelTexture;

use super::d3_model_resource::DataD3ModelResourceTutorialFormatObj;


pub struct DataD3ModelTextureTutorialFormatObj {}

impl DataD3ModelTextureTutorialFormatObj {
    pub fn load(resource_name: DataD3ModelResourceTutorialFormatObj)
     -> Result<(Vec<u8>, FormatPngOutputInfomation), TerminationProcessMain>
    {
        let texture_image_file_path = Self::file_path_from_resource_name(resource_name);
        DataD3ModelTexture::load(texture_image_file_path)
    }

    fn file_path_from_resource_name(resource_name: DataD3ModelResourceTutorialFormatObj) -> PathBuf {
        match resource_name {
            DataD3ModelResourceTutorialFormatObj::VikingRoom =>
                ConfigPathFile::get_texture_viking_room(),
        }
    }
}