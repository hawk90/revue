//! Input widgets - User input components
//!
//! Widgets for collecting user input through various mechanisms.

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
