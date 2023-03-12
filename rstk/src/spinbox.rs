//! Spinbox widget - displays a range with up/down buttons. 
//!
//! User can select between either a range of numbers or a list of values.
//!
//! * also see the Tk [manual](https://www.tcl-lang.org/man/tcl8.6/TkCmd/ttk_spinbox.htm)

use super::grid;
use super::pack;
use super::widget;
use super::wish;

/// Refers to a spinbox widget for numbers
#[derive(Clone, Debug, PartialEq)]
pub struct TkSpinboxRange {
    pub id: String,
}

/// Creates an instance of a numeric-spinbox widget in given parent.
///
/// This spinbox is used to select between a range of numbers, defined as [from, to].
/// The increment specifies how much the number changes as the up/down arrow is clicked.
pub fn make_spinbox_range(
    parent: &impl widget::TkWidget,
    from: f64,
    to: f64,
    increment: f64,
) -> TkSpinboxRange {
    let id = wish::next_wid(parent.id());
    let msg = format!(
        "ttk::spinbox {} -from {} -to {} -increment {} ",
        id, from, to, increment
    );
    wish::tell_wish(&msg);

    TkSpinboxRange { id }
}

impl widget::TkWidget for TkSpinboxRange {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}

impl grid::TkGridLayout for TkSpinboxRange {}
impl pack::TkPackLayout for TkSpinboxRange {}

impl TkSpinboxRange {
    /// Sets the state of the widget; Readonly means user cannot enter
    /// their own value, but must pick from the given selection.
    pub fn state(&self, value: widget::State) {
        widget::configure(&self.id, "state", &value.to_string());
    }

    /// Retrieves the spinbox's value.
    pub fn value_get(&self) -> f64 {
        let msg = format!("puts [{} get] ; flush stdout", self.id);
        let result = wish::ask_wish(&msg);
        if let Ok(value) = result.parse::<f64>() {
            value
        } else {
            0.0
        }
    }

    /// Set to true so spinbox 'wraps' around at top/bottom.
    pub fn wrap(&self, value: bool) {
        widget::configure(&self.id, "wrap", if value { "1" } else { "0" });
    }
}

/// Refers to a spinbox widget for discrete values
#[derive(Clone, Debug, PartialEq)]
pub struct TkSpinboxValues {
    pub id: String,
}

/// Creates an instance of a values-spinbox widget in given parent.
///
/// This spinbox is used to select between a given list of string values.
pub fn make_spinbox_values(parent: &impl widget::TkWidget, values: &[&str]) -> TkSpinboxValues {
    let id = wish::next_wid(parent.id());
    let mut values_str = String::new();
    for value in values {
        values_str.push('{');
        values_str.push_str(value);
        values_str.push('}');
        values_str.push(' ');
    }

    let msg = format!("ttk::spinbox {} -values {{{}}} ", id, values_str);
    wish::tell_wish(&msg);

    TkSpinboxValues { id }
}

impl widget::TkWidget for TkSpinboxValues {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}

impl grid::TkGridLayout for TkSpinboxValues {}
impl pack::TkPackLayout for TkSpinboxValues {}

impl TkSpinboxValues {
    /// Sets the state of the widget; Readonly means user cannot enter
    /// their own value, but must pick from the given selection.
    pub fn state(&self, value: widget::State) {
        widget::configure(&self.id, "state", &value.to_string());
    }

    /// Retrieves the spinbox's value.
    pub fn value_get(&self) -> String {
        let msg = format!("puts [{} get] ; flush stdout", self.id);
        wish::ask_wish(&msg)
    }

    /// Set to true so spinbox 'wraps' around at top/bottom.
    pub fn wrap(&self, value: bool) {
        widget::configure(&self.id, "wrap", if value { "1" } else { "0" });
    }
}
