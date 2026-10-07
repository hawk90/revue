//! The pieces an app's stylesheet is built from, kept for hot reload.
//!
//! [`StyleSheet::merge`] appends rules, so merging a reloaded file into the
//! live stylesheet only ever adds: a declaration deleted from the file would
//! stay in effect until restart. Hot reload instead rebuilds the whole
//! stylesheet from these sources, in the order the builder added them.

use crate::constants::MAX_CSS_FILE_SIZE;
use crate::style::{parse_css, StyleSheet};
use std::fs;
use std::path::{Path, PathBuf};

/// One contribution to the app's stylesheet.
#[derive(Debug, Clone)]
enum StyleSource {
    /// Inline CSS or plugin styles: fixed for the app's lifetime.
    Fixed(StyleSheet),
    /// A stylesheet file, re-read on hot reload.
    File {
        path: PathBuf,
        /// The text `sheet` was parsed from; `None` until a read succeeds.
        text: Option<String>,
        /// The last version of the file that parsed.
        sheet: StyleSheet,
    },
}

/// The ordered sources of an app's stylesheet.
#[derive(Debug, Clone, Default)]
pub(crate) struct StyleSources {
    sources: Vec<StyleSource>,
}

impl StyleSources {
    /// Record inline CSS (or plugin styles).
    pub(crate) fn push_fixed(&mut self, sheet: StyleSheet) {
        self.sources.push(StyleSource::Fixed(sheet));
    }

    /// Record a stylesheet file and what it parsed to at build time (`None`
    /// when it could not be read or parsed then).
    pub(crate) fn push_file(&mut self, path: PathBuf, loaded: Option<(String, StyleSheet)>) {
        let (text, sheet) = match loaded {
            Some((text, sheet)) => (Some(text), sheet),
            None => (None, StyleSheet::new()),
        };
        self.sources.push(StyleSource::File { path, text, sheet });
    }

    /// Re-read every stylesheet file. Returns whether any of them changed.
    ///
    /// A file that cannot be read, is too large, or does not parse keeps its
    /// last good version - a half-typed rule should not wipe the styles - and
    /// a warning is logged.
    pub(crate) fn reload_files(&mut self) -> bool {
        let mut changed = false;
        for source in &mut self.sources {
            let StyleSource::File { path, text, sheet } = source else {
                continue;
            };
            let Some(new_text) = read_css(path) else {
                continue;
            };
            if text.as_deref() == Some(new_text.as_str()) {
                continue;
            }
            match parse_css(&new_text) {
                Ok(new_sheet) => {
                    *sheet = new_sheet;
                    *text = Some(new_text);
                    changed = true;
                    crate::log_debug!("Hot reload: reloaded {:?}", path);
                }
                Err(e) => {
                    crate::log_warn!("Hot reload: failed to parse CSS from {:?}: {}", path, e);
                }
            }
        }
        changed
    }

    /// The combined stylesheet: every source merged in order.
    pub(crate) fn stylesheet(&self) -> StyleSheet {
        let mut combined = StyleSheet::new();
        for source in &self.sources {
            let sheet = match source {
                StyleSource::Fixed(sheet) | StyleSource::File { sheet, .. } => sheet,
            };
            combined.merge(sheet.clone());
        }
        combined
    }
}

/// Read a stylesheet file, refusing one over [`MAX_CSS_FILE_SIZE`].
fn read_css(path: &Path) -> Option<String> {
    match fs::metadata(path) {
        Ok(meta) if meta.len() > MAX_CSS_FILE_SIZE => {
            crate::log_warn!(
                "Hot reload: CSS file too large ({} bytes, max {}): {:?}",
                meta.len(),
                MAX_CSS_FILE_SIZE,
                path
            );
            return None;
        }
        Ok(_) => {}
        Err(e) => {
            crate::log_warn!("Hot reload: failed to read {:?}: {}", path, e);
            return None;
        }
    }
    // Capped: the metadata length is not the read length for a FIFO or a
    // file still being written.
    match crate::utils::read_capped(path, MAX_CSS_FILE_SIZE).and_then(|bytes| {
        String::from_utf8(bytes)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }) {
        Ok(text) => Some(text),
        Err(e) => {
            crate::log_warn!("Hot reload: failed to read {:?}: {}", path, e);
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn declarations(sheet: &StyleSheet, selector: &str) -> Vec<(String, String)> {
        sheet
            .rules(selector)
            .iter()
            .flat_map(|rule| rule.declarations.iter())
            .map(|d| (d.property.clone(), d.value.clone()))
            .collect()
    }

    fn file_source(path: &Path) -> StyleSources {
        let text = fs::read_to_string(path).unwrap();
        let sheet = parse_css(&text).unwrap();
        let mut sources = StyleSources::default();
        sources.push_file(path.to_path_buf(), Some((text, sheet)));
        sources
    }

    #[test]
    fn reload_drops_a_deleted_declaration() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("main.css");
        fs::write(&path, ".count { color: red; font-weight: bold; }").unwrap();
        let mut sources = file_source(&path);

        fs::write(&path, ".count { color: blue; }").unwrap();
        assert!(sources.reload_files());

        assert_eq!(
            declarations(&sources.stylesheet(), ".count"),
            vec![("color".to_string(), "blue".to_string())]
        );
    }

    #[test]
    fn reload_of_an_unchanged_file_reports_no_change() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("main.css");
        fs::write(&path, ".a { color: red; }").unwrap();
        let mut sources = file_source(&path);

        assert!(!sources.reload_files());
    }

    #[test]
    fn unparsable_or_missing_file_keeps_the_last_good_version() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("main.css");
        fs::write(&path, ".a { color: red; }").unwrap();
        let mut sources = file_source(&path);

        fs::write(&path, ".a { color: ").unwrap();
        let _ = sources.reload_files();
        fs::remove_file(&path).unwrap();
        assert!(!sources.reload_files());

        assert_eq!(
            declarations(&sources.stylesheet(), ".a"),
            vec![("color".to_string(), "red".to_string())]
        );
    }

    #[test]
    fn file_missing_at_build_is_picked_up_once_it_exists() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("late.css");
        let mut sources = StyleSources::default();
        sources.push_file(path.clone(), None);
        assert!(sources.stylesheet().rules.is_empty());

        fs::write(&path, ".a { color: red; }").unwrap();
        assert!(sources.reload_files());
        assert_eq!(declarations(&sources.stylesheet(), ".a").len(), 1);
    }

    #[test]
    fn sources_combine_in_the_order_they_were_added() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("main.css");
        fs::write(&path, ".a { color: blue; }").unwrap();

        let mut sources = StyleSources::default();
        sources.push_fixed(parse_css(".a { color: red; }").unwrap());
        let text = fs::read_to_string(&path).unwrap();
        let sheet = parse_css(&text).unwrap();
        sources.push_file(path.clone(), Some((text, sheet)));
        sources.push_fixed(parse_css(".a { color: green; }").unwrap());

        let values: Vec<_> = declarations(&sources.stylesheet(), ".a")
            .into_iter()
            .map(|(_, v)| v)
            .collect();
        assert_eq!(values, ["red", "blue", "green"]);
    }
}
