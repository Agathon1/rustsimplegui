//! Treeview widget - displays hierarchical data with multiple values.
//!
//! A combined tree/list-view widget, for displaying hierarchical data with
//! multiple values.
//!
//! * also see the Tk [manual](https://www.tcl-lang.org/man/tcl8.6/TkCmd/ttk_treeview.htm)
//!
//! # Columns
//!
//! The columns can be identified as follows (see Tk
//! [manual](https://www.tcl-lang.org/man/tcl8.6/TkCmd/ttk_treeview.htm#M77)):
//!
//! * using a string name, set using the `columns` method
//! * by index, counting from 1 (this must be given as a string)
//! * "#0" is a special index, referring to the left-most tree column
//!
//! # Events
//!
//! Use [bind](widget::TkWidget::bind) to call a function on following event:
//!
//! * `<<TreeviewSelect>>` - whenever selection is changed
//! * `<<TreeviewOpen>>` - when a node is opened
//! * `<<TreeviewClose>>` - when a node is closed

use super::grid;
use super::image;
use super::pack;
use super::widget;
use super::wish;

/// Refers to a treeview widget
#[derive(Clone, Debug, PartialEq)]
pub struct TkTreeview {
    pub id: String,
}

/// Refers to a treeview item
#[derive(Clone, Debug, PartialEq)]
pub struct TkTreeviewItem {
    pub treeview: String,
    pub id: String,
}

/// Creates an instance of a treeview widget in given parent.
pub fn make_treeview(parent: &impl widget::TkWidget) -> TkTreeview {
    let id = wish::next_wid(parent.id());
    let msg = format!("ttk::treeview {}", id);
    wish::tell_wish(&msg);

    TkTreeview { id }
}

impl widget::TkWidget for TkTreeview {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}

impl grid::TkGridLayout for TkTreeview {}
impl pack::TkPackLayout for TkTreeview {}

impl TkTreeview {
    /// Defines the names for the columns, used when referring to
    /// headings, columns or values.
    pub fn columns(&self, columns: &[&str]) {
        let mut columns_str = String::new();
        for column in columns {
            columns_str.push('{');
            columns_str.push_str(column);
            columns_str.push('}');
            columns_str.push(' ');
        }

        let msg = format!("{} configure -columns {{{}}}", &self.id, columns_str);
        wish::tell_wish(&msg);
    }

    /// Set the alignment for the given column.
    pub fn column_anchor(&self, column: &str, value: widget::Anchor) {
        let msg = format!("{} column {} -anchor {}", &self.id, column, value);
        wish::tell_wish(&msg);
    }

    /// Set whether the given column should 'stretch' when treeview is resized.
    pub fn column_stretch(&self, column: &str, value: bool) {
        let msg = format!(
            "{} column {} -stretch {}",
            &self.id,
            column,
            if value { "1" } else { "0" }
        );
        wish::tell_wish(&msg);
    }

    /// Set the minimum-width in pixels for the given column.
    pub fn column_min_width(&self, column: &str, value: u64) {
        let msg = format!("{} column {} -minwidth {}", &self.id, column, value);
        wish::tell_wish(&msg);
    }

    /// Set the width in pixels for the given column.
    pub fn column_width(&self, column: &str, value: u64) {
        let msg = format!("{} column {} -width {}", &self.id, column, value);
        wish::tell_wish(&msg);
    }

    /// Returns the focussed item.
    pub fn focus(&self) -> TkTreeviewItem {
        let msg = format!("puts [{} focus] ; flush stdout ", &self.id);
        let result = wish::ask_wish(&msg);

        TkTreeviewItem {
            treeview: self.id.clone(),
            id: result,
        }
    }

    /// Set the heading text for the given column.
    pub fn heading_text(&self, column: &str, title: &str) {
        let msg = format!("{} heading {} -text {{{}}}", &self.id, column, title);
        wish::tell_wish(&msg);
    }

    /// Set the heading image for the given column.
    pub fn heading_image(&self, column: &str, image: &image::TkImage) {
        let msg = format!("{} heading {} -image {}", &self.id, column, &image.id);
        wish::tell_wish(&msg);
    }

    /// Set the heading alignment for the given column.
    pub fn heading_anchor(&self, column: &str, value: widget::Anchor) {
        let msg = format!("{} heading {} -anchor {}", &self.id, column, value);
        wish::tell_wish(&msg);
    }

    /// Sets number of rows to display.
    pub fn height(&self, value: u64) {
        widget::configure(&self.id, "height", &value.to_string());
    }

    /// Create a new top-level item at end of treeview.
    pub fn insert_item(&self) -> TkTreeviewItem {
        let msg = format!("puts [{} insert {{}} end] ; flush stdout", &self.id);
        let result = wish::ask_wish(&msg);

        TkTreeviewItem {
            treeview: self.id.clone(),
            id: result,
        }
    }

    /// Create a new top-level item at given index position of treeview.
    pub fn insert_item_at(&self, index: u64) -> TkTreeviewItem {
        let msg = format!("puts [{} insert {{}} {}] ; flush stdout", &self.id, index);
        let result = wish::ask_wish(&msg);

        TkTreeviewItem {
            treeview: self.id.clone(),
            id: result,
        }
    }

    /// Moves a given item to become a child of given parent.
    pub fn move_item(&self, child: &TkTreeviewItem, parent: &TkTreeviewItem, index: u64) {
        let msg = format!("{} move {} {} {}", &self.id, &child.id, &parent.id, index);
        wish::tell_wish(&msg);
    }

    /// Selection mode.
    pub fn select_mode(&self, value: widget::Selection) {
        widget::configure(&self.id, "selectmode", &value.to_string());
    }

    /// Returns list of selected items.
    pub fn selected_items(&self) -> Vec<TkTreeviewItem> {
        let query = format!("puts [{} selection] ; flush stdout", &self.id);
        let values = wish::ask_wish(&query);

        let mut result: Vec<TkTreeviewItem> = vec![];
        for value in values.split_whitespace() {
            result.push(TkTreeviewItem {
                treeview: self.id.clone(),
                id: String::from(value),
            });
        }

        result
    }

    /// Shows both the tree and headers (default setting).
    pub fn show_all(&self) {
        widget::configure(&self.id, "show", "tree headings");
    }

    /// Shows the table part only.
    pub fn show_headings(&self) {
        widget::configure(&self.id, "show", "headings");
    }

    /// Shows the tree only.
    pub fn show_tree(&self) {
        widget::configure(&self.id, "show", "tree");
    }

    /// Binds event to given tag.
    pub fn tag_bind(
        &self,
        tag: &str,
        pattern: &str,
        command: impl Fn(widget::TkEvent) + Send + 'static,
    ) {
        // id+tag+pattern used as identifier
        let tag_pattern = format!("{}{}{}", &self.id, tag, pattern);
        wish::add_callback1_event(&tag_pattern, wish::mk_callback1_event(command));
        let msg = format!(
            "{} tag bind {} {} {{ puts cb1e:{}:%x:%y:%X:%Y:%h:%w:%k:%K:%b ; flush stdout }}",
            &self.id, tag, pattern, tag_pattern
        );
        wish::tell_wish(&msg);
    }

    /// Formatting is applied to tags using configuration options.
    ///
    /// For the available options, see the Tk
    /// [manual](https://www.tcl-lang.org/man/tcl8.6/TkCmd/ttk_treeview.htm#M72)
    pub fn tag_configure(&self, tag: &str, option: &str, value: &str) {
        let msg = format!("{} tag configure {} -{} {}", &self.id, tag, option, value);
        wish::tell_wish(&msg);
    }

    /// Returns a list of all the tag names defined in this text widget.
    pub fn tag_names(&self) -> Vec<String> {
        let msg = format!("puts [{} tag names] ; flush stdout", &self.id);
        let result = wish::ask_wish(&msg);
        wish::split_items(&result)
    }
}

impl TkTreeviewItem {
    /// Deletes this widget from tree.
    pub fn delete(&self) {
        let msg = format!("{} delete {}", &self.treeview, &self.id);
        wish::tell_wish(&msg);
    }

    /// Sets the text label for the item.
    pub fn text(&self, value: &str) {
        let msg = format!("{} item {} -text {{{}}}", &self.treeview, &self.id, value);
        wish::tell_wish(&msg);
    }

    /// Sets an image to display on the item.
    pub fn image(&self, image: &image::TkImage) {
        let msg = format!("{} item {} -image {}", &self.treeview, &self.id, &image.id);
        wish::tell_wish(&msg);
    }

    /// The list of values to display for this item.
    pub fn values(&self, values: &[&str]) {
        let mut values_str = String::new();
        for value in values {
            values_str.push('{');
            values_str.push_str(value);
            values_str.push('}');
            values_str.push(' ');
        }

        let msg = format!(
            "{} item {} -values {{{}}}",
            &self.treeview, &self.id, values_str
        );
        wish::tell_wish(&msg);
    }

    /// Sets item state to be open or closed.
    pub fn open(&self, value: bool) {
        let msg = format!(
            "{} item {} -open {}",
            &self.treeview,
            &self.id,
            if value { "1" } else { "0" }
        );
        wish::tell_wish(&msg);
    }

    /// Returns true/false if item is open or closed.
    pub fn is_open(&self) -> bool {
        let msg = format!(
            "puts [{} item {} -open] ; flush stdout",
            &self.treeview, &self.id
        );
        let result = wish::ask_wish(&msg);

        result == "1"
    }

    /// Create a new item at end of this treeview item.
    pub fn insert_item(&self) -> TkTreeviewItem {
        let msg = format!(
            "puts [{} insert {} end] ; flush stdout ",
            &self.treeview, &self.id
        );
        let result = wish::ask_wish(&msg);

        TkTreeviewItem {
            treeview: self.treeview.clone(),
            id: result,
        }
    }

    /// Create a new top-level item at given index position of this
    /// treeview item.
    pub fn insert_item_at(&self, index: u64) -> TkTreeviewItem {
        let msg = format!(
            "puts [{} insert {} {}] ; flush stdout",
            &self.treeview, &self.id, index
        );
        let result = wish::ask_wish(&msg);

        TkTreeviewItem {
            treeview: self.treeview.clone(),
            id: result,
        }
    }

    /// Returns an Option type containing the parent item if found, or
    /// None if this is a top-level item.
    pub fn parent(&self) -> Option<TkTreeviewItem> {
        let msg = format!(
            "puts [{} parent {}] ; flush stdout",
            &self.treeview, &self.id
        );
        let result = wish::ask_wish(&msg);

        if result.is_empty() || result == "{}" {
            None
        } else {
            Some(TkTreeviewItem {
                treeview: self.treeview.clone(),
                id: result,
            })
        }
    }

    /// Returns an Option type containing the previous item to
    /// this one in its list, if found, or None if this is the
    /// first child of its parent.
    pub fn previous(&self) -> Option<TkTreeviewItem> {
        let msg = format!("puts [{} prev {}] ; flush stdout", &self.treeview, &self.id);
        let result = wish::ask_wish(&msg);

        if result.is_empty() || result == "{}" {
            None
        } else {
            Some(TkTreeviewItem {
                treeview: self.treeview.clone(),
                id: result,
            })
        }
    }

    /// Returns an Option type containing the next item to
    /// this one in its list, if found, or None if this is the
    /// last child of its parent.
    pub fn next(&self) -> Option<TkTreeviewItem> {
        let msg = format!("puts [{} next {}] ; flush stdout", &self.treeview, &self.id);
        let result = wish::ask_wish(&msg);

        if result.is_empty() || result == "{}" {
            None
        } else {
            Some(TkTreeviewItem {
                treeview: self.treeview.clone(),
                id: result,
            })
        }
    }

    /// Returns a list of child items of given node.
    pub fn children(&self) -> Vec<TkTreeviewItem> {
        let msg = format!(
            "puts [{} children {}] ; flush stdout",
            &self.treeview, &self.id
        );
        let result = wish::ask_wish(&msg);

        let mut children: Vec<TkTreeviewItem> = vec![];

        for child in result.split_whitespace() {
            children.push(TkTreeviewItem {
                treeview: self.treeview.clone(),
                id: String::from(child),
            });
        }

        children
    }

    /// Adds a tag to this item.
    pub fn tag_add(&self, tag: &str) {
        let msg = format!("{} tag add {{{}}} {}", &self.treeview, tag, &self.id);
        wish::tell_wish(&msg);
    }

    /// Checks if this item has current tag.
    pub fn tag_has(&self, tag: &str) -> bool {
        let msg = format!(
            "puts [{} tag has {{{}}} {}] ; flush stdout",
            &self.treeview, tag, &self.id
        );
        let result = wish::ask_wish(&msg);

        result == "1"
    }

    /// Removes a tag from this item.
    pub fn tag_remove(&self, tag: &str) {
        let msg = format!("{} tag remove {{{}}} {}", &self.treeview, tag, &self.id);
        wish::tell_wish(&msg);
    }
}
