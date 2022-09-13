use std::path::Path;
use std::path::PathBuf;


pub const DIRECTORY_ENTRY: &'static str = env!("CARGO_MANIFEST_DIR");
pub const PATH_FILE_NAME_TEXTURE_MAIN: &'static str = "texture.png";
pub const DIRECTORY_RELATIVE_ENTRY_RESOURCE: &'static str = "resource";


pub struct ConfigPath {}

impl ConfigPath {
    pub unsafe fn get_file_texture_image_main() -> PathBuf {
        Path::new(DIRECTORY_ENTRY)
        .join(DIRECTORY_RELATIVE_ENTRY_RESOURCE)
        .join(PATH_FILE_NAME_TEXTURE_MAIN)
    }
}