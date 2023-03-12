//! Entry widget - text field for user input or editing.
//!
//! * also see the Tk [manual](https://www.tcl-lang.org/man/tcl8.6/TkCmd/ttk_entry.htm)

use super::grid;
use super::pack;
use super::widget;
use super::wish;

/// Refers to an entry widget
#[derive(Clone, Debug, PartialEq)]
pub struct TkEntry {
    pub id: String,
    var: String,
}

/// Creates an instance of an entry widget in given parent.
pub fn make_entry(parent: &impl widget::TkWidget) -> TkEntry {
    let id = wish::next_wid(parent.id());
    let var = format!("::en{}", wish::current_id());
    let msg = format!("ttk::entry {} -textvariable {}", id, var);
    wish::tell_wish(&msg);

    TkEntry { id, var }
}

impl widget::TkWidget for TkEntry {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}

impl grid::TkGridLayout for TkEntry {}
impl pack::TkPackLayout for TkEntry {}

impl TkEntry {
    /// Specifies the font to use for text.
    pub fn font(&self, definition: &str) {
        widget::configure(&self.id, "font", definition);
    }

    /// Specifies the foreground (text) colour.
    ///
    /// Colours are specified as a string, by either:
    ///
    /// * `name` - using one of the values in the tk [colours](https://tcl.tk/man/tcl8.6/TkCmd/colors.htm) list
    /// * `rgb` - as a 6-digit hexadecimal value in form "#RRGGBB"
    pub fn foreground(&self, colour: &str) {
        widget::configure(&self.id, "foreground", colour);
    }

    /// Alignment of text within widget
    pub fn justify(&self, value: widget::Justify) {
        widget::configure(&self.id, "justify", &value.to_string());
    }

    /// Used e.g. for a password, shows the given character instead of
    /// what is typed.
    pub fn show(&self, c: char) {
        widget::configure(&self.id, "show", &c.to_string());
    }

    /// Sets the state of the widget (readonly, normal or disabled).
    pub fn state(&self, value: widget::State) {
        widget::configure(&self.id, "state", &value.to_string());
    }

    /// Returns the current entry value
    pub fn value_get(&self) -> String {
        let msg = format!("puts ${} ; flush stdout", self.var);
        wish::ask_wish(&msg)
    }

    /// Sets the width of the widget, in characters
    pub fn width(&self, value: u64) {
        let msg = format!("{} configure -width {{{}}}", self.id, value);
        wish::tell_wish(&msg);
    }
}
