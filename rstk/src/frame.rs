//! Frame widget - a container widget for other widgets.
//!
//! * also see the Tk [manual](https://www.tcl-lang.org/man/tcl8.6/TkCmd/ttk_frame.htm)

use super::grid;
use super::pack;
use super::widget;
use super::wish;

/// Refers to a frame widget
#[derive(Clone, Debug, PartialEq)]
pub struct TkFrame {
    pub id: String,
}

/// Creates an instance of a frame widget in given parent.
pub fn make_frame(parent: &impl widget::TkWidget) -> TkFrame {
    let id = wish::next_wid(parent.id());
    let msg = format!("ttk::frame {}", id);
    wish::tell_wish(&msg);

    TkFrame { id }
}

impl widget::TkWidget for TkFrame {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}
impl grid::TkGridLayout for TkFrame {}
impl pack::TkPackLayout for TkFrame {}

impl TkFrame {
    /// Size of border around frame
    pub fn border_width(&self, width: u64) {
        widget::configure(&self.id, "borderwidth", &width.to_string());
    }

    /// Height of frame, in rows
    pub fn height(&self, height: u64) {
        widget::configure(&self.id, "height", &height.to_string());
    }

    /// Padding to place around the frame. Takes
    /// an array of up to four values, specifying:
    ///
    /// * \[all]
    /// * [left-right top-bottom]
    /// * [left top-bottom right]
    /// * [left top right bottom]
    pub fn padding(&self, values: &[u64]) {
        widget::padding(&self.id, values);
    }

    /// Style of border around frame
    pub fn relief(&self, value: widget::Relief) {
        widget::configure(&self.id, "relief", &value.to_string());
    }

    /// Width of frame, in columns
    pub fn width(&self, width: u64) {
        widget::configure(&self.id, "width", &width.to_string());
    }
}
