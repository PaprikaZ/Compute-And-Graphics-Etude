use std::path::PathBuf;

use ::png::OutputInfo as FormatPngOutputInfomation;
use crate::termination::TerminationProcessMain;
use crate::config::path::ConfigPathFile;
use crate::data::d3_model_resource::DataD3ModelResourceTutorialSimple;
use crate::data::d3_model_texture::DataD3ModelTexture;


pub struct DataD3ModelTextureTutorialSimple {}

impl DataD3ModelTextureTutorialSimple {
    pub fn load(resource_name: DataD3ModelResourceTutorialSimple)
     -> Result<(Vec<u8>, FormatPngOutputInfomation), TerminationProcessMain>
    {
        let texture_image_file_path = Self::file_path_from_resource_name(resource_name);
        DataD3ModelTexture::load(texture_image_file_path)
    }

    fn file_path_from_resource_name(resource_name: DataD3ModelResourceTutorialSimple) -> PathBuf {
        match resource_name {
            DataD3ModelResourceTutorialSimple::Default =>
                ConfigPathFile::get_texture_main(),
        }
    }
}