//! Label widgets - displays text/image.
//!
//! * also see the Tk [manual](https://www.tcl-lang.org/man/tcl8.6/TkCmd/ttk_label.htm)
//!
//! ## Examples
//!
//! The simplest label has some text:
//!
//! ```
//! let label_1 = rstk::make_label(&root);
//! label_1.text("Label text");
//! ```
//!
//! Labels can also display images, with or without text, and how these are 
//! displayed can be controlled using [compound](widget::TkLabelOptions::compound).
//! In the following example, a label with both an image and text is set to show 
//! the image below the text:
//!
//! ```
//! let label_3 = rstk::make_label(&root);
//! label_3.image(&read_image("tcllogo.gif"));
//! label_3.text("Tcl Logo");
//! label_3.compound(rstk::Compound::Bottom);
//! ```
//! 
//! Labels can also show multi-line text. For this, specify a wrap-length (in pixels):
//!
//! ```
//! let label_7 = rstk::make_label(&root);
//! label_7.wrap_length(300);
//! label_7.text("Rust has great documentation, a friendly compiler with useful error messages, and
//! top-notch tooling - an integrated package manager and build tool, smart multi-editor support
//! with auto-completion and type inspections, an auto-formatter, and more. --
//! https://rust-lang.org");
//! ```
//!

use super::grid;
use super::pack;
use super::widget;
use super::wish;

/// Refers to a label widget
#[derive(Clone, Debug, PartialEq)]
pub struct TkLabel {
    pub id: String,
}

/// Creates an instance of a label widget in given parent.
pub fn make_label(parent: &impl widget::TkWidget) -> TkLabel {
    let id = wish::next_wid(parent.id());
    let msg = format!("ttk::label {}", id);
    wish::tell_wish(&msg);

    TkLabel { id }
}

impl widget::TkWidget for TkLabel {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}
impl grid::TkGridLayout for TkLabel {}
impl pack::TkPackLayout for TkLabel {}
impl widget::TkLabelOptions for TkLabel {}

impl TkLabel {
    /// Position of information with respect to internal margins.
    ///
    /// e.g. the space allocated to the label can be larger than its 
    /// text needs, so an Anchor value of E (east) will display the 
    /// text against the right-margin.
    pub fn anchor(&self, value: widget::Anchor) {
        widget::configure(&self.id, "anchor", &value.to_string());
    }

    /// Specifies the background colour.
    ///
    /// Colours are specified as a string, by either:
    ///
    /// * `name` - using one of the values in the tk [colours](https://tcl.tk/man/tcl8.6/TkCmd/colors.htm) list
    /// * `rgb` - as a 6-digit hexadecimal value in form "#RRGGBB"
    pub fn background(&self, colour: &str) {
        widget::configure(&self.id, "background", colour);
    }

    /// Alignment of text within widget.
    pub fn justify(&self, value: widget::Justify) {
        widget::configure(&self.id, "justify", &value.to_string());
    }

    /// Style of border around label.
    pub fn relief(&self, value: widget::Relief) {
        widget::configure(&self.id, "relief", &value.to_string());
    }

    /// Sets the maximum line length, in pixels.
    /// When set, displayed text is separated into lines, not exceeding
    /// the given length.
    pub fn wrap_length(&self, length: u64) {
        widget::configure(&self.id, "wraplength", &length.to_string());
    }
}
