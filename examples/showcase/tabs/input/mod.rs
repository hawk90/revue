//! Input widgets tab - Button, Input, Toggle, Select, Slider, Form, Picker, Autocomplete, Number, Masked

mod autocomplete;
mod button;
mod form;
mod input_field;
mod masked;
mod number;
mod picker;
mod select;
mod slider;
mod toggle;

pub use autocomplete::examples as autocomplete_examples;
pub use button::examples as button_examples;
pub use form::examples as form_examples;
pub use input_field::examples as input_field_examples;
pub use masked::examples as masked_examples;
pub use number::examples as number_examples;
pub use picker::examples as picker_examples;
pub use select::examples as select_examples;
pub use slider::examples as slider_examples;
pub use toggle::examples as toggle_examples;
