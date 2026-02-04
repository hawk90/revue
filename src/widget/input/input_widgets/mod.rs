//! Input widgets - User input components
//!
//! This module provides widgets for collecting user input through various mechanisms.
//! Input widgets handle text entry, selection, toggles, and specialized input patterns.
//!
//! # Widget Categories
//!
//! ## Text Input
//!
//! | Widget | Description | Constructor |
//! |--------|-------------|-------------|
//! | [`Input`] | Single-line text input | [`input()`] |
//! | [`TextArea`] | Multi-line text input | [`textarea()`] |
//! | [`SearchBar`] | Search input with filtering | [`search_bar()`] |
//!
//! ## Buttons & Toggles
//!
//! | Widget | Description | Constructor |
//! |--------|-------------|-------------|
//! | [`Button`] | Clickable button | [`button()`] |
//! | [`Switch`] | On/off toggle switch | [`switch()`], [`toggle()`] |
//! | [`Checkbox`] | Checkbox for binary choice | [`checkbox()`] |
//! | [`RadioGroup`] | Exclusive radio buttons | [`radio_group()`] |
//!
//! ## Selection
//!
//! | Widget | Description | Constructor |
//! |--------|-------------|-------------|
//! | [`Select`] | Dropdown selection | [`select()`] |
//! | [`Combobox`] | Editable dropdown | [`combobox()`] |
//! | [`SelectionList`] | Select from list | [`selection_list()`] |
//! | [`Autocomplete`] | Text with suggestions | [`autocomplete()`] |
//!
//! ## Numeric Input
//!
//! | Widget | Description | Constructor |
//! |--------|-------------|-------------|
//! | [`NumberInput`] | Numeric text input | [`number_input()`] |
//! | [`Slider`] | Range slider | [`slider()`] |
//! | [`Stepper`] | Increment/decrement steps | [`stepper()`] |
//! | [`Rating`] | Star rating widget | [`rating()`] |
//!
//! ## Specialized Input
//!
//! | Widget | Description | Constructor |
//! |--------|-------------|-------------|
//! | [`ColorPicker`] | Color selection | [`color_picker()`] |
//! | `slider_range()` | Dual-handle range slider | See function |
//!
//! # Quick Start
//!
//! ## Text Input
//!
//! ```rust,ignore
//! use revue::prelude::*;
//!
//! input()
//!     .placeholder("Enter your name...")
//!     .value("Default text");
//! ```
//!
//! ## Button
//!
//! ```rust,ignore
//! use revue::prelude::*;
//!
//! button("Click Me")
//!     .variant(ButtonVariant::Primary)
//!     .on_click(|_| println!("Clicked!"));
//! ```
//!
//! ## Checkbox
//!
//! ```rust,ignore
//! use revue::prelude::*;
//!
//! checkbox("Remember me")
//!     .checked(true)
//!     .on_change(|checked| println!("Checked: {}", checked));
//! ```
//!
//! ## Slider
//!
//! ```rust,ignore
//! use revue::prelude::*;
//!
//! slider()
//!     .min(0)
//!     .max(100)
//!     .value(50)
//!     .step(5);
//! ```
//!
//! ## Select
//!
//! ```rust,ignore
//! use revue::prelude::*;
//!
//! select()
//!     .options(vec![
//!         ("Option 1", "1"),
//!         ("Option 2", "2"),
//!         ("Option 3", "3"),
//!     ])
//!     .selected("1");
//! ```

pub mod autocomplete;
pub mod button;
pub mod checkbox;
pub mod color_picker;
pub mod combobox;
pub mod input;
pub mod number_input;
pub mod radio;
pub mod rating;
pub mod search_bar;
pub mod select;
pub mod selection_list;
pub mod slider;
pub mod stepper;
pub mod switch;
pub mod textarea;

// Re-exports for convenience
pub use autocomplete::{autocomplete, Autocomplete, Suggestion};
pub use button::{button, Button, ButtonVariant};
pub use checkbox::{checkbox, Checkbox, CheckboxStyle};
pub use color_picker::{color_picker, ColorPalette, ColorPicker, ColorPickerMode};
pub use combobox::{combobox, ComboOption, Combobox};
pub use input::{input, Input};
pub use number_input::{
    currency_input, integer_input, number_input, percentage_input, NumberInput,
};
pub use radio::{radio_group, RadioGroup, RadioLayout, RadioStyle};
pub use rating::{rating, Rating, RatingSize, RatingStyle};
pub use search_bar::{search_bar, SearchBar};
pub use select::{select, Select};
pub use selection_list::{
    selection_item, selection_list, SelectionItem, SelectionList, SelectionStyle,
};
pub use slider::{
    percentage_slider, slider, slider_range, volume_slider, Slider, SliderOrientation, SliderStyle,
};
pub use stepper::{step, stepper, Step, StepStatus, Stepper, StepperOrientation, StepperStyle};
pub use switch::{switch, toggle, Switch, SwitchStyle};
pub use textarea::{textarea, TextArea};
