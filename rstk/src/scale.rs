//! Scale widget - displays a slider over a range.
//!
//! * also see the Tk [manual](https://www.tcl-lang.org/man/tcl8.6/TkCmd/ttk_scale.htm)
//!

use super::grid;
use super::pack;
use super::widget;
use super::wish;

/// Refers to a scale widget
#[derive(Clone, Debug, PartialEq)]
pub struct TkScale {
    pub id: String,
}

/// Creates an instance of a scale widget in given parent,
/// with given orientation.
pub fn make_scale(parent: &impl widget::TkWidget, orientation: widget::Orientation) -> TkScale {
    let id = wish::next_wid(parent.id());
    let msg = format!("ttk::scale {} -orient {}", id, orientation);
    wish::tell_wish(&msg);

    TkScale { id }
}

impl widget::TkWidget for TkScale {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}

impl grid::TkGridLayout for TkScale {}
impl pack::TkPackLayout for TkScale {}

impl TkScale {
    /// Sets the function to be called whenever the scale value is changed.
    pub fn command(&self, command: impl Fn(f64) + Send + 'static) {
        wish::add_callback1_float(&self.id, wish::mk_callback1_float(command));
        let msg = format!(
            "{} configure -command [list scale_value {}]",
            self.id, self.id
        );
        wish::tell_wish(&msg);
    }

    /// Sets the minimum value for the scale.
    pub fn from(&self, value: f64) {
        widget::configure(&self.id, "from", &value.to_string());
    }

    /// Sets the maximum value for the scale.
    pub fn to(&self, value: f64) {
        widget::configure(&self.id, "to", &value.to_string());
    }

    /// Retrieves the scale's value.
    pub fn value_get(&self) -> f64 {
        let msg = format!("puts [{} get] ; flush stdout", self.id);
        let result = wish::ask_wish(&msg);
        if let Ok(value) = result.parse::<f64>() {
            value
        } else {
            0.0
        }
    }

    /// Set the scale's value.
    pub fn value(&self, value: f64) {
        widget::configure(&self.id, "value", &value.to_string());
    }

    /// Displayed length of scale in pixels.
    pub fn length(&self, value: u64) {
        widget::configure(&self.id, "length", &value.to_string());
    }

    /// Sets the state of the widget (normal or disabled).
    pub fn state(&self, value: widget::State) {
        widget::configure(&self.id, "state", &value.to_string());
    }
}
