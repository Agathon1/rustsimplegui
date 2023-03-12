//! Grid layout - a geometry manager for arranging widgets.
//!
//! * also see the Tk [manual](https://www.tcl-lang.org/man/tcl8.6/TkCmd/grid.htm)
//!
//! The grid-layout is used in a "builder" style to layout a single widget:
//!
//! ```
//! widget.grid()
//!   .OPTION(VALUE) // 0 or more
//!   .layout();
//! ```
//!
//! 1. `grid` is called first, to get the GridLayout instance.
//! 2. `layout` must be called last, to perform the layout.
//! 3. zero or more options are added to the GridLayout, to control the position
//!   and layout of the widget.
//! 
//! ## Example
//!
//! The following example places a label:
//!
//! ```
//! label.grid()                    // starts layout definition
//!     .row(1)                     // at row 1
//!     .column(2)                  // at column 2
//!     .column_span(3)             // spanning three columns (i.e. 2,3,4)
//!     .padx(5)                    // with 5 pixels horizontal spacing
//!     .sticky(rstk::Sticky::E)    // "attached" to the right-hand side
//!     .layout();                  // completes the layout
//! ```
//!

use super::widget;
use super::wish;

/// Refers to the settings for a GridLayout.
///
/// Apart from `layout`, the methods on this struct set the values of different
/// options in a builder style: call `layout` to finish the layout.
///
#[derive(Clone, Debug)]
pub struct GridLayout {
    id: String,
    column: Option<u64>,
    column_span: Option<u64>,
    ipadx: Option<u64>,
    ipady: Option<u64>,
    padx: Option<u64>,
    pady: Option<u64>,
    row: Option<u64>,
    row_span: Option<u64>,
    sticky: widget::Sticky,
}

impl GridLayout {
    pub(super) fn new(wid: &str) -> GridLayout {
        GridLayout {
            id: String::from(wid),
            column: None,
            column_span: None,
            ipadx: None,
            ipady: None,
            padx: None,
            pady: None,
            row: None,
            row_span: None,
            sticky: widget::Sticky::None,
        }
    }

    /// Specifies the (0-indexed) column in which to place this widget.
    pub fn column(&mut self, column: u64) -> &mut Self {
        self.column = Some(column);
        self
    }

    /// The number of columns this widget should span.
    pub fn column_span(&mut self, span: u64) -> &mut Self {
        self.column_span = Some(span);
        self
    }

    /// Horizontal padding (inside content border).
    pub fn ipadx(&mut self, pad: u64) -> &mut Self {
        self.ipadx = Some(pad);
        self
    }

    /// Vertical padding (inside content border).
    pub fn ipady(&mut self, pad: u64) -> &mut Self {
        self.ipady = Some(pad);
        self
    }

    /// Horizontal padding (outside content border).
    pub fn padx(&mut self, pad: u64) -> &mut Self {
        self.padx = Some(pad);
        self
    }

    /// Vertical padding (outside content border).
    pub fn pady(&mut self, pad: u64) -> &mut Self {
        self.pady = Some(pad);
        self
    }

    /// Specifies the (0-indexed) row in which to place this widget.
    pub fn row(&mut self, row: u64) -> &mut Self {
        self.row = Some(row);
        self
    }

    /// Number of rows this widget should span.
    pub fn row_span(&mut self, span: u64) -> &mut Self {
        self.row_span = Some(span);
        self
    }

    /// When a widget is smaller than its containing space, this
    /// setting controls how the widget is expanded or positioned
    /// within that space.
    pub fn sticky(&mut self, sticky: widget::Sticky) -> &mut Self {
        self.sticky = sticky;
        self
    }

    /// Called last to finally create the layout with the parameter values
    /// set up by the builder.
    pub fn layout(&self) {
        let mut msg = format!("grid {} ", self.id);
        if let Some(column) = self.column {
            msg.push_str(&format!("-column {} ", column));
        }
        if let Some(span) = self.column_span {
            msg.push_str(&format!("-columnspan {} ", span));
        }
        if let Some(pad) = self.ipadx {
            msg.push_str(&format!("-ipadx {} ", pad));
        }
        if let Some(pad) = self.ipady {
            msg.push_str(&format!("-ipady {} ", pad));
        }
        if let Some(pad) = self.padx {
            msg.push_str(&format!("-padx {} ", pad));
        }
        if let Some(pad) = self.pady {
            msg.push_str(&format!("-pady {} ", pad));
        }
        if let Some(row) = self.row {
            msg.push_str(&format!("-row {} ", row));
        }
        if let Some(span) = self.row_span {
            msg.push_str(&format!("-rowspan {} ", span));
        }

        if self.sticky != widget::Sticky::None {
            msg.push_str(&format!("-sticky {} ", self.sticky));
        }

        wish::tell_wish(&msg);
    }
}

/// Common functions for widgets that can be arranged using GridLayouts
pub trait TkGridLayout: widget::TkWidget {
    /// Creates a GridLayout instance for placing this widget within its parent.
    fn grid(&self) -> GridLayout {
        GridLayout::new(self.id())
    }

    /// Sets properties for widget layout directly through Tk: see Tk 
    /// [manual](https://www.tcl-lang.org/man/tcl8.6/TkCmd/grid.htm#M9) for 
    /// options.
    ///
    /// e.g. to set the "padx" value to "5" on a label:
    ///
    /// ```
    /// label.grid_configure("padx", "5");
    /// ```
    ///
    fn grid_configure(&self, option: &str, value: &str) {
        let msg = format!("grid configure {} -{} {{{}}}", self.id(), option, value);
        wish::tell_wish(&msg);
    }

    /// Removes this widget from layout.
    fn grid_forget(&self) {
        let msg = format!("grid forget {}", self.id());
        wish::tell_wish(&msg);
    }
}
