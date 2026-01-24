//! Export format and options

/// Export format for clipboard/file export
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ExportFormat {
    #[default]
    Csv,
    Tsv,
    PlainText,
}

/// Export options
#[derive(Clone, Debug)]
pub struct ExportOptions {
    /// Output format
    pub format: ExportFormat,
    /// Include column headers
    pub include_headers: bool,
    /// Export only selected rows
    pub selected_only: bool,
    /// Export only visible columns
    pub visible_columns_only: bool,
}

impl Default for ExportOptions {
    fn default() -> Self {
        Self {
            format: ExportFormat::Csv,
            include_headers: true,
            selected_only: false,
            visible_columns_only: true,
        }
    }
}

impl ExportOptions {
    /// Create new export options with defaults
    pub fn new() -> Self {
        Self::default()
    }

    /// Set format
    pub fn format(mut self, format: ExportFormat) -> Self {
        self.format = format;
        self
    }

    /// Include headers
    pub fn include_headers(mut self, include: bool) -> Self {
        self.include_headers = include;
        self
    }

    /// Export selected rows only
    pub fn selected_only(mut self, selected: bool) -> Self {
        self.selected_only = selected;
        self
    }
}
