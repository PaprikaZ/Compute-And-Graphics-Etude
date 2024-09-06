use ::entry_guide::error::entry_guide::ErrorEntryGuide;
use ::entry_guide::error::entry_guide_handler::ErrorEntryGuideHandler;
use ::entry_guide::entry_binary::name::EntryBinaryName;
use ::entry_guide::entry_argument::self_::EntryArgument;
use ::entry_guide::launcher::self_::Launcher;


fn main() -> Result<(), ErrorEntryGuide> {
    let entry_binary_name = EntryBinaryName::new(env!("CARGO_BIN_NAME"));
    let result =
        EntryArgument
        ::try_parse(entry_binary_name, std::env::args())
        .and_then(Launcher::launch);
    if result.is_ok() { return Ok(()) }
    match ErrorEntryGuideHandler::handle(result.unwrap_err()) {
        None => Ok(()),
        Some(e) => Err(e),
    }
}