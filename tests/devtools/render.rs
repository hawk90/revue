//! The DevTools panel draws the inspected widget's outline only on the
//! Inspector tab.

use crate::{corners, devtools_with, screen};
use revue::devtools::{DevToolsTab, Inspector};

#[test]
fn no_outline_on_another_tab() {
    let mut devtools = devtools_with(Inspector::new());
    devtools.set_tab(DevToolsTab::Styles);
    assert_eq!(corners(&screen(&devtools)), [None; 4]);
}
