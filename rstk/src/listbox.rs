//! Listbox widget - displays a list of items from which the user can select 
//! one or more.
//!
//! * also see the Tk [manual](https://www.tcl-lang.org/man/tcl8.6/TkCmd/listbox.htm)
//!
//! # Events
//!
//! Use [bind](widget::TkWidget::bind) to call a function on following event:
//!
//! * `<<ListboxSelect>>` - whenever selection is changed

use super::grid;
use super::pack;
use super::widget;
use super::wish;

/// Refers to a listbox widget
#[derive(Clone, Debug, PartialEq)]
pub struct TkListbox {
    pub id: String,
}

/// Creates an instance of a listbox widget in given parent
/// populating the listbox with the given set of values.
pub fn make_listbox(parent: &impl widget::TkWidget, values: &[&str]) -> TkListbox {
    let id = wish::next_wid(parent.id());

    let msg = format!("listbox {}", id);
    wish::tell_wish(&msg);

    // - add values to listbox
    for value in values {
        let msg = format!("{} insert end {{{}}}", id, value);
        wish::tell_wish(&msg);
    }
    // - select first item at start
    let msg = format!("{} selection set 0", id);
    wish::tell_wish(&msg);

    TkListbox { id }
}

impl widget::TkWidget for TkListbox {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}

impl grid::TkGridLayout for TkListbox {}
impl pack::TkPackLayout for TkListbox {}

impl TkListbox {
    /// Adds item to end of list.
    pub fn append(&self, item: &str) {
        let msg = format!("{} insert end {{{}}}", &self.id, item);
        wish::tell_wish(&msg);
    }

    /// Size of border around listbox.
    pub fn border_width(&self, width: u64) {
        widget::configure(&self.id, "borderwidth", &width.to_string());
    }

    /// Delete item at given index.
    pub fn delete(&self, index: u64) {
        let msg = format!("{} delete {}", &self.id, index);
        wish::tell_wish(&msg);
    }

    /// Specifies the font to use for text.
    pub fn font(&self, definition: &str) {
        widget::configure(&self.id, "font", definition);
    }

    /// Height of listbox, in rows.
    pub fn height(&self, height: u64) {
        widget::configure(&self.id, "height", &height.to_string());
    }

    /// Insert item at given index.
    pub fn insert_at(&self, index: u64, item: &str) {
        let msg = format!("{} insert {} {{{}}}", &self.id, index, item);
        wish::tell_wish(&msg);
    }

    /// Set configuration option for given item index.
    pub fn item_configure(&self, index: u64, option: &str, value: &str) {
        let msg = format!(
            "{} itemconfigure {} -{} {{{}}}",
            &self.id, index, option, value
        );
        wish::tell_wish(&msg);
    }

    /// Alignment of text within widget.
    pub fn justify(&self, value: widget::Justify) {
        widget::configure(&self.id, "justify", &value.to_string());
    }

    /// Style of border around listbox.
    pub fn relief(&self, value: widget::Relief) {
        widget::configure(&self.id, "relief", &value.to_string());
    }

    /// Selection mode, one of "single" or "multiple" ("none" is made "single").
    pub fn selection_mode(&self, value: widget::Selection) {
        let value = if value == widget::Selection::None {
            widget::Selection::Single.to_string()
        } else {
            value.to_string()
        };
        widget::configure(&self.id, "selectmode", &value);
    }

    /// Returns list of indices for selected items.
    pub fn selected_items(&self) -> Vec<u64> {
        let query = format!("puts [{} curselection] ; flush stdout", &self.id);
        let values = wish::ask_wish(&query);

        let mut result: Vec<u64> = vec![];
        for value in values.split_whitespace() {
            if let Ok(value) = value.parse::<u64>() {
                result.push(value);
            }
        }

        result
    }

    /// Sets the state of the listbox (normal or disabled).
    pub fn state(&self, value: widget::State) {
        widget::configure(&self.id, "state", &value.to_string());
    }

    /// Width of listbox, in characters.
    pub fn width(&self, width: u64) {
        widget::configure(&self.id, "width", &width.to_string());
    }
}
