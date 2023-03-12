//! Pack layout - a geometry manager for arranging widgets.
//!
//! * also see the Tk [manual](http://www.tcl-lang.org/man/tcl8.6/TkCmd/pack.htm)
//!
//! The pack-layout is used in a "builder" style to layout a single widget:
//!
//! ```
//! widget.pack()
//!   .OPTION(VALUE) // 0 or more
//!   .layout();
//! ```
//!
//! 1. `pack` is called first, to get the PackLayout instance.
//! 2. `layout` must be called last, to perform the layout.
//! 3. zero or more options are added to the PackLayout, to control the position
//!   and layout of the widget.

use super::widget;
use super::wish;

/// Refers to the settings for a PackLayout.
///
/// Apart from `layout`, the methods on this struct set the values of different
/// options in a builder style: call `layout` to finish the layout.
///
#[derive(Clone, Debug)]
pub struct PackLayout {
    id: String,
    after: Option<String>,
    anchor: widget::Anchor,
    before: Option<String>,
    expand: bool,
    fill: widget::PackFill, 
    inside: Option<String>,
    ipadx: Option<u64>,
    ipady: Option<u64>,
    padx: Option<u64>,
    pady: Option<u64>,
    side: widget::PackSide 
}

impl PackLayout {
    pub(super) fn new(wid: &str) -> PackLayout {
        PackLayout {
            id: String::from(wid),
            after: None,
            anchor: widget::Anchor::Centre,
            before: None,
            expand: false,
            fill: widget::PackFill::None,
            inside: None,
            ipadx: None,
            ipady: None,
            padx: None,
            pady: None,
            side: widget::PackSide::Top,
        }
    }

    /// Inserts this widget after given widget in its container and 
    /// in its packing order.
    pub fn after(&mut self, widget: &impl widget::TkWidget) -> &mut Self {
        self.after = Some(String::from(widget.id()));
        self
    }

    /// Specifies position of this widget with respect to its container.
    pub fn anchor(&mut self, value: widget::Anchor) -> &mut Self {
        self.anchor = value;
        self
    }

    /// Inserts this widget before given widget in its container and 
    /// in its packing order.
    pub fn before(&mut self, widget: &impl widget::TkWidget) -> &mut Self {
        self.before = Some(String::from(widget.id()));
        self
    }

    /// Specifies whether widget should expand to fit its container.
    pub fn expand(&mut self, value: bool) -> &mut Self {
        self.expand = value;
        self
    }

    /// Specifies how widget should expand to fill its container.
    pub fn fill(&mut self, value: widget::PackFill) -> &mut Self {
        self.fill = value;
        self
    }

    /// Inserts this widget into container of given widget at the end of 
    /// its packing order.
    pub fn inside(&mut self, widget: &impl widget::TkWidget) -> &mut Self {
        self.inside = Some(String::from(widget.id()));
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


    /// Specifies side of container that the widget is packed against.
    pub fn side(&mut self, value: widget::PackSide) -> &mut Self {
        self.side = value;
        self
    }

    /// Called last to finally create the layout with the parameter values
    /// set up by the builder.
    pub fn layout(&self) {
        let mut msg = format!("pack {} ", self.id);
        if let Some(after) = &self.after {
            msg.push_str(&format!("-after {} ", after));
        }
        msg.push_str(&format!("-anchor {} ", self.anchor));
        if let Some(before) = &self.before {
            msg.push_str(&format!("-before {} ", before));
        }
        if self.expand {
            msg.push_str("-expand 1 ");
        } else {
            msg.push_str("-expand 0 ");
        }
        msg.push_str(&format!("-fill {} ", self.fill));
        if let Some(inside) = &self.inside {
            msg.push_str(&format!("-inside {} ", inside));
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
        msg.push_str(&format!("-side {} ", self.side));

        wish::tell_wish(&msg);
    }
}

/// Common functions for widgets that can be arranged using GridLayouts
pub trait TkPackLayout: widget::TkWidget {
    /// Creates a PackLayout instance for placing this widget within its parent
    fn pack(&self) -> PackLayout {
        PackLayout::new(self.id())
    }

    /// Sets properties for widget layout
    fn pack_configure(&self, option: &str, value: &str) {
        let msg = format!("pack configure {} -{} {{{}}}", self.id(), option, value);
        wish::tell_wish(&msg);
    }

    /// Removes this widget from layout
    fn pack_forget(&self) {
        let msg = format!("pack forget {}", self.id());
        wish::tell_wish(&msg);
    }
}
