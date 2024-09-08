use crate::error::entry_guide::ErrorEntryGuideOwn;
use crate::error::entry_guide::ErrorEntryGuide;
use crate::application_name::self_::ApplicationName;
use crate::entry_binary::name::EntryBinaryName;


#[derive(Debug, Clone)]
pub enum EntryArgument {
    RunApplication(ApplicationName)
}

impl EntryArgument {
    fn get_help_informating(binary_name: EntryBinaryName) -> String {
        let raw_binary_name = binary_name.as_raw();
        let mut first_line_text =
            format!("Usage:");
        let a_usage_line_text =
            format!("{raw_binary_name} <application-name>  -- run available application by name");
        let b_usage_line_text =
            format!("{raw_binary_name} help                -- print help information");
        let help_body_text = r#"

<application-name> available:
    example-chapter-0   -- vulkan guide chapter 0 example
    example-chapter-1   -- vulkan guide chapter 1 example
    example-chapter-2   -- vulkan guide chapter 2 example
"#;
        first_line_text.push('\n');
        first_line_text.push_str(&a_usage_line_text);
        first_line_text.push('\n');
        first_line_text.push_str(&b_usage_line_text);
        first_line_text.push_str(help_body_text);
        let help_information_text = first_line_text;
        help_information_text
    }

    fn try_parse_application_name(argument_s: &mut std::env::Args)
    -> Option<ApplicationName>
    {
        argument_s.next()
        .and_then(|first_argument| {
            match first_argument.as_str() {
                "example-chapter-0" => Some(ApplicationName::VulkanV1_1Chapter0),
                "example-chapter-1" => Some(ApplicationName::VulkanV1_1Chapter1),
                "example-chapter-2" => Some(ApplicationName::VulkanV1_1Chapter2),
                _ => None
            }
        })
    }

    pub fn try_parse(binary_name: EntryBinaryName, mut argument_s: std::env::Args)
    -> Result<Self, ErrorEntryGuide>
    {
        let _ = argument_s.next();
        Self::try_parse_application_name(&mut argument_s)
        .and_then(|application_name|
            if let Some(_) = argument_s.next() { None } else { Some(application_name) })
        .map(|application_name| EntryArgument::RunApplication(application_name))
        .ok_or_else(|| ErrorEntryGuideOwn::EntryArgumentParseFail(Self::get_help_informating(binary_name)))
        .map_err(|e| e.into())
    }
}