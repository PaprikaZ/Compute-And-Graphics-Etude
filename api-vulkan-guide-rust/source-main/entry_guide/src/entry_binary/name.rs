#[derive(Debug, Clone)]
pub struct EntryBinaryName<'t>(&'t str);

impl<'t> EntryBinaryName<'t> {
    pub fn new(raw_binary_name: &'t str) -> Self {
        Self(raw_binary_name)
    }

    pub fn as_raw(self) -> &'t str {
        self.0
    }
}