//! Combobox widgets - text field with a drop-down list of options.
//!
//! * also see the Tk [manual](https://www.tcl-lang.org/man/tcl8.6/TkCmd/ttk_combobox.htm)
//! # Events
//!
//! Use [bind](widget::TkWidget::bind) to call a function on following event:
//!
//! * `<<ComboboxSelected>>` - when an element is selected

use super::grid;
use super::pack;
use super::widget;
use super::wish;

/// Refers to a combobox widget
#[derive(Clone, Debug, PartialEq)]
pub struct TkCombobox {
    pub id: String,
}

/// Creates an instance of a combo-box widget in given parent,
/// populating the drop-down list with the given set of values.
pub fn make_combobox(parent: &impl widget::TkWidget, values: &[&str]) -> TkCombobox {
    let id = wish::next_wid(parent.id());
    let mut values_str = String::new();
    for value in values {
        values_str.push('{');
        values_str.push_str(value);
        values_str.push('}');
        values_str.push(' ');
    }

    let msg = format!("ttk::combobox {} -values {{{}}}", id, values_str);
    wish::tell_wish(&msg);

    TkCombobox { id }
}

impl widget::TkWidget for TkCombobox {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}
impl grid::TkGridLayout for TkCombobox {}
impl pack::TkPackLayout for TkCombobox {}

impl TkCombobox {
    /// Sets the height of the widget, in rows
    pub fn height(&self, value: u64) {
        let msg = format!("{} configure -height {{{}}}", self.id, value);
        wish::tell_wish(&msg);
    }

    /// Alignment of text within widget
    pub fn justify(&self, value: widget::Justify) {
        widget::configure(&self.id, "justify", &value.to_string());
    }

    /// Sets the state of the widget (readonly, normal or disabled).
    pub fn state(&self, value: widget::State) {
        widget::configure(&self.id, "state", &value.to_string());
    }

    /// Sets the current value
    pub fn value(&self, value: &str) {
        let msg = format!("puts [{} set {{{}}}] ; flush stdout", self.id, value);
        wish::tell_wish(&msg);
    }

    /// Returns the current value
    pub fn value_get(&self) -> String {
        let msg = format!("puts [{} get] ; flush stdout", self.id);
        wish::ask_wish(&msg)
    }

    /// Sets the width of the widget, in characters
    pub fn width(&self, value: u64) {
        let msg = format!("{} configure -width {{{}}}", self.id, value);
        wish::tell_wish(&msg);
    }
}
