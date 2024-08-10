use std::fs::File;
use std::path::PathBuf;

use ::png::OutputInfo as FormatPngOutputInfomation;
use crate::termination::TerminationProcessMain;


pub struct DataD3ModelTexture {}

impl DataD3ModelTexture {
    pub fn load(texture_image_file_path: PathBuf)
     -> Result<(Vec<u8>, FormatPngOutputInfomation), TerminationProcessMain>
    {
        let open_texture_file_result = File::open(texture_image_file_path);
        let texture_file =
            match open_texture_file_result {
                Err(error) => return Err(TerminationProcessMain::InitializationFileOpenFail(error.to_string())),
                Ok(file) => file,
            };
        let png_format_decoder = png::Decoder::new(texture_file);
        let read_texture_file_information_result = png_format_decoder.read_info();
        let (texture_file_information, mut texture_file_reader) =
            match read_texture_file_information_result {
                Err(error) => return Err(TerminationProcessMain::InitializationFormatPngDecodingError(error)),
                Ok(information_and_reader) => information_and_reader,
            };
        let mut texture_file_pixel_s = vec![0; texture_file_information.buffer_size()];
        match texture_file_reader.next_frame(&mut texture_file_pixel_s) {
            Err(error) => return Err(TerminationProcessMain::InitializationFormatPngDecodingError(error)),
            Ok(()) => (),
        };
        Ok((texture_file_pixel_s, texture_file_information))
    }
}