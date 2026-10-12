//! LogViewer export: the filtered entries as text

use super::LogViewer;

impl LogViewer {
    /// Export filtered entries as text
    pub fn export_filtered(&self) -> String {
        self.filtered_entries()
            .map(|(_, entry)| entry.raw.as_str())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Export filtered entries with formatting
    pub fn export_formatted(&self) -> String {
        self.filtered_entries()
            .map(|(_, entry)| {
                let mut parts = Vec::new();

                if let Some(ref ts) = entry.timestamp {
                    parts.push(format!("[{}]", ts));
                }

                parts.push(format!("[{}]", entry.level.label()));

                if let Some(ref src) = entry.source {
                    parts.push(format!("[{}]", src));
                }

                parts.push(entry.message.clone());

                parts.join(" ")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}
