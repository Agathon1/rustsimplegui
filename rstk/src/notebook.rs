//! Notebook widget - a container widget which contains multiple panes, but 
//! displays one pane at a time.
//!
//! * also see the Tk [manual](https://www.tcl-lang.org/man/tcl8.6/TkCmd/ttk_notebook.htm)
//!
//! # Events
//!
//! Use [bind](widget::TkWidget::bind) to call a function on following event:
//!
//! * `<<NotebookTabChanged>>` - when new tab selected

use super::grid;
use super::pack;
use super::widget;
use super::wish;

/// Refers to a notebook widget
#[derive(Clone, Debug, PartialEq)]
pub struct TkNotebook {
    pub id: String,
}

/// Creates an instance of a notebook in given parent.
pub fn make_notebook(parent: &impl widget::TkWidget) -> TkNotebook {
    let id = wish::next_wid(parent.id());
    let msg = format!("ttk::notebook {}", id);
    wish::tell_wish(&msg);

    TkNotebook { id }
}

impl widget::TkWidget for TkNotebook {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}

impl grid::TkGridLayout for TkNotebook {}
impl pack::TkPackLayout for TkNotebook {}

impl TkNotebook {
    /// TODO: use builder pattern to support images+text
    pub fn add(&self, widget: &impl widget::TkWidget, title: &str) {
        let msg = format!("{} add {} -text {{{}}}", self.id, widget.id(), title);
        wish::tell_wish(&msg);
    }

    /// Height of notebook, in rows
    pub fn height(&self, height: u64) {
        widget::configure(&self.id, "height", &height.to_string());
    }

    /// Sets space around the widget. Takes
    /// an array of up to four values, specifying:
    ///
    /// * \[all]
    /// * [left-right top-bottom]
    /// * [left top-bottom right]
    /// * [left top right bottom]
    pub fn padding(&self, values: &[u64]) {
        widget::padding(&self.id, values);
    }

    /// Width of notebook, in columns
    pub fn width(&self, width: u64) {
        widget::configure(&self.id, "width", &width.to_string());
    }
}
