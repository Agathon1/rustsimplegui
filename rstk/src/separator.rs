//! Separator widget - displays a separating line in its parent.
//!
//! The separator is either oriented vertically or horizontally.
//!
//! * also see the Tk [manual](https://www.tcl-lang.org/man/tcl8.6/TkCmd/ttk_separator.htm)
//!

use super::grid;
use super::pack;
use super::widget;
use super::wish;

/// Refers to a separator widget
#[derive(Clone, Debug, PartialEq)]
pub struct TkSeparator {
    pub id: String,
}

/// Creates an instance of a separator in given parent.
pub fn make_separator(
    parent: &impl widget::TkWidget,
    orientation: widget::Orientation,
) -> TkSeparator {
    let id = wish::next_wid(parent.id());
    let msg = format!("ttk::separator {} -orient {}", id, orientation);
    wish::tell_wish(&msg);

    TkSeparator { id }
}

impl widget::TkWidget for TkSeparator {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}

impl grid::TkGridLayout for TkSeparator {}
impl pack::TkPackLayout for TkSeparator {}

