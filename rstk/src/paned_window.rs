//! Paned-window widget - a container widget which contains multiple panes.
//! Resizable sizers separate each pane.
//!
//! * also see the Tk [manual](https://www.tcl-lang.org/man/tcl8.6/TkCmd/ttk_panedwindow.htm)

use super::grid;
use super::pack;
use super::widget;
use super::wish;

/// Refers to a paned-window widget
#[derive(Clone, Debug, PartialEq)]
pub struct TkPanedWindow {
    pub id: String,
}

/// Creates an instance of a paned-window, in given parent.
/// Child panes will be stacked based on given orientation.
pub fn make_paned_window(
    parent: &impl widget::TkWidget,
    orientation: widget::Orientation,
) -> TkPanedWindow {
    let id = wish::next_wid(parent.id());
    let msg = format!("ttk::panedwindow {} -orient {}", id, orientation);
    wish::tell_wish(&msg);

    TkPanedWindow { id }
}

impl widget::TkWidget for TkPanedWindow {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}
impl grid::TkGridLayout for TkPanedWindow {}
impl pack::TkPackLayout for TkPanedWindow {}

impl TkPanedWindow {
    /// Adds given widget to the pane.
    pub fn add(&self, pane: &impl widget::TkWidget) {
        let msg = format!("{} add {}", self.id, pane.id());
        wish::tell_wish(&msg);
    }

    /// Adds given widget to the pane with given weight.
    pub fn add_weighted(&self, pane: &impl widget::TkWidget, weight: u64) {
        let msg = format!("{} add {} -weight {}", self.id, pane.id(), weight);
        wish::tell_wish(&msg);
    }

    /// Removes given widget from the pane.
    pub fn forget(&self, pane: &impl widget::TkWidget) {
        let msg = format!("{} forget {}", self.id, pane.id());
        wish::tell_wish(&msg);
    }

    /// Height of paned window, in rows
    pub fn height(&self, height: u64) {
        widget::configure(&self.id, "height", &height.to_string());
    }

    /// Inserts given widget to the pane at given index position.
    pub fn insert(&self, index: u64, pane: &impl widget::TkWidget) {
        let msg = format!("{} insert {} {}", self.id, index, pane.id());
        wish::tell_wish(&msg);
    }

    /// Inserts given widget to the pane at given index position with given weight.
    pub fn insert_weighted(&self, index: u64, pane: &impl widget::TkWidget, weight: u64) {
        let msg = format!(
            "{} insert {} {} -weight {}",
            self.id,
            index,
            pane.id(),
            weight
        );
        wish::tell_wish(&msg);
    }

    /// Width of paned window, in columns
    pub fn width(&self, width: u64) {
        widget::configure(&self.id, "width", &width.to_string());
    }
}
