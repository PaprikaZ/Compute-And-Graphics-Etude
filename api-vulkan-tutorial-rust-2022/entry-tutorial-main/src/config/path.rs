use std::path::Path;
use std::path::PathBuf;


pub const DIRECTORY_ENTRY: &'static str = env!("CARGO_MANIFEST_DIR");
pub const PATH_FILE_NAME_TEXTURE_MAIN: &'static str = "texture.png";
pub const PATH_FILE_NAME_MODEL_VIKING_ROOM_MODEL: &'static str = "viking_room.obj";
pub const PATH_FILE_NAME_MODEL_VIKING_ROOM_IMAGE: &'static str = "viking_room.png";
pub const DIRECTORY_RELATIVE_ENTRY_RESOURCE: &'static str = "resource";


pub struct ConfigPathFile {}

impl ConfigPathFile {
    pub fn get_texture_main() -> PathBuf {
        Path::new(DIRECTORY_ENTRY)
        .join(DIRECTORY_RELATIVE_ENTRY_RESOURCE)
        .join(PATH_FILE_NAME_TEXTURE_MAIN)
    }

    pub fn get_model_viking_room() -> PathBuf {
        Path::new(DIRECTORY_ENTRY)
        .join(DIRECTORY_RELATIVE_ENTRY_RESOURCE)
        .join(PATH_FILE_NAME_MODEL_VIKING_ROOM_MODEL)
    }

    pub fn get_texture_viking_room() -> PathBuf {
        Path::new(DIRECTORY_ENTRY)
        .join(DIRECTORY_RELATIVE_ENTRY_RESOURCE)
        .join(PATH_FILE_NAME_MODEL_VIKING_ROOM_IMAGE)
    }
}