//! Radio button widget - displays text/image with an on/off widget, arranged
//! in a group of mutually exclusive buttons. Executes a command when clicked.
//!
//! * also see the Tk [manual](https://tcl.tk/man/tcl/TkCmd/ttk_radiobutton.htm)
//!

use super::grid;
use super::pack;
use super::widget;
use super::wish;

/// Refers to a radio-button widget
#[derive(Clone, Debug, PartialEq)]
pub struct TkRadioButton {
    pub id: String,
    var: String,
}

/// Creates an instance of a radio-button widget in given parent.
///
/// Radio buttons are arranged in groups, with each button in the group
/// having its own value. For this reason, the group and value are
/// required when making a radio button.
pub fn make_radio_button(
    parent: &impl widget::TkWidget,
    group: &str,
    value: &str,
) -> TkRadioButton {
    let id = wish::next_wid(parent.id());
    let var = format!("::rb_group_{}", group);
    let msg = format!("ttk::radiobutton {} -value {} -variable {}", id, value, var);
    wish::tell_wish(&msg);

    TkRadioButton { id, var }
}

impl widget::TkWidget for TkRadioButton {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}
impl grid::TkGridLayout for TkRadioButton {}
impl pack::TkPackLayout for TkRadioButton {}
impl widget::TkLabelOptions for TkRadioButton {}

impl TkRadioButton {
    /// Sets the function to be called when the button is clicked.
    pub fn command(&self, command: impl Fn(bool) + Send + 'static) {
        wish::add_callback1_bool(&self.id, wish::mk_callback1_bool(command));
        let msg = format!(
            "{} configure -command {{ puts cb1-{}-${} ; flush stdout }}",
            self.id, self.id, self.var
        );
        wish::tell_wish(&msg);
    }

    /// Calls the button's command.
    pub fn invoke(&self) {
        let msg = format!("{} invoke", self.id);
        wish::tell_wish(&msg);
    }

    /// Sets the state of the button (normal or disabled).
    pub fn state(&self, value: widget::State) {
        widget::configure(&self.id, "state", &value.to_string());
    }

    /// Returns the selected value from this radio-button's group.
    pub fn value_get(&self) -> String {
        let msg = format!("puts ${} ; flush stdout", self.var);
        wish::ask_wish(&msg)
    }

    pub fn value(&self, value: &str) {
        let msg = format!("set {} {}", self.var, value);
        wish::tell_wish(&msg);
    }
}
