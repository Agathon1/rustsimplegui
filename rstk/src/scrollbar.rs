//! Scrollbar widget - displays a scrollbar to control position within a range.
//!
//! * also see the Tk [manual](https://www.tcl-lang.org/man/tcl8.6/TkCmd/ttk_scrollbar.htm)
//!

use super::grid;
use super::pack;
use super::widget;
use super::wish;

/// Refers to a scrollbar widget
#[derive(Clone, Debug, PartialEq)]
pub struct TkScrollbar {
    pub id: String,
}

/// Creates an instance of an horizontal scrollbar in given parent item,
/// connected to associated widget.
///
pub fn make_horizontal_scrollbar(
    parent: &impl widget::TkWidget,
    widget: &impl widget::TkWidget,
) -> TkScrollbar {
    let id = wish::next_wid(parent.id());
    let msg = format!(
        "ttk::scrollbar {} -orient horizontal -command {{{} xview}}",
        id,
        widget.id()
    );
    wish::tell_wish(&msg);
    let msg = format!("{} configure -xscrollcommand {{{} set}}", widget.id(), id);
    wish::tell_wish(&msg);

    TkScrollbar { id }
}

/// Creates an instance of a vertical scrollbar in given parent item,
/// connected to associated widget.
///
pub fn make_vertical_scrollbar(
    parent: &impl widget::TkWidget,
    widget: &impl widget::TkWidget,
) -> TkScrollbar {
    let id = wish::next_wid(parent.id());
    let msg = format!(
        "ttk::scrollbar {} -orient vertical -command {{{} yview}}",
        id,
        widget.id()
    );
    wish::tell_wish(&msg);
    let msg = format!("{} configure -yscrollcommand {{{} set}}", widget.id(), id);
    wish::tell_wish(&msg);

    TkScrollbar { id }
}

impl widget::TkWidget for TkScrollbar {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}

impl grid::TkGridLayout for TkScrollbar {}
impl pack::TkPackLayout for TkScrollbar {}

impl TkScrollbar {}
