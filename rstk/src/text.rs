//! Text widget - displays text.
//!
//! * also see the Tk [manual](https://www.tcl-lang.org/man/tcl8.6/TkCmd/text.htm)

use super::grid;
use super::image;
use super::pack;
use super::widget;
use super::wish;

/// Refers to a text widget
#[derive(Clone, Debug, PartialEq)]
pub struct TkText {
    pub id: String,
}

/// Creates an instance of a text widget in given parent.
pub fn make_text(parent: &impl widget::TkWidget) -> TkText {
    let id = wish::next_wid(parent.id());
    let msg = format!("text {}", id);
    wish::tell_wish(&msg);

    TkText { id }
}

impl widget::TkWidget for TkText {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}

impl grid::TkGridLayout for TkText {}
impl pack::TkPackLayout for TkText {}

impl TkText {
    /// Specifies the background colour.
    ///
    /// Colours are specified as a string, by either:
    ///
    /// * `name` - using one of the values in the tk [colours](https://tcl.tk/man/tcl8.6/TkCmd/colors.htm) list
    /// * `rgb` - as a 6-digit hexadecimal value in form "#RRGGBB"
    pub fn background(&self, colour: &str) {
        widget::configure(&self.id, "background", colour);
    }

    /// Size of border around widget.
    pub fn border_width(&self, width: u64) {
        widget::configure(&self.id, "borderwidth", &width.to_string());
    }

    /// Delete a range of text.
    pub fn delete(
        &self,
        (from_line, from_character): (u64, u64),
        (to_line, to_character): (u64, u64),
    ) {
        let msg = format!(
            "{} delete {}.{} {}.{}",
            &self.id, from_line, from_character, to_line, to_character
        );
        wish::tell_wish(&msg);
    }

    /// Delete a single character in text.
    pub fn delete_char(&self, (line, character): (u64, u64)) {
        let msg = format!("{} delete {}.{}", &self.id, line, character);
        wish::tell_wish(&msg);
    }

    /// Specifies the font to use for text.
    pub fn font(&self, definition: &str) {
        widget::configure(&self.id, "font", definition);
    }

    /// Specifies the foreground (text) colour.
    ///
    /// Colours are specified as a string, by either:
    ///
    /// * `name` - using one of the values in the tk [colours](https://tcl.tk/man/tcl8.6/TkCmd/colors.htm) list
    /// * `rgb` - as a 6-digit hexadecimal value in form "#RRGGBB"
    pub fn foreground(&self, colour: &str) {
        widget::configure(&self.id, "foreground", colour);
    }

    /// Get a range of text.
    pub fn get(
        &self,
        (from_line, from_character): (u64, u64),
        (to_line, to_character): (u64, u64),
    ) -> String {
        let msg = format!(
            "puts [{} get {}.{} {}.{}] ; flush stdout",
            &self.id, from_line, from_character, to_line, to_character
        );
        wish::ask_wish(&msg)
    }

    /// Get a range of text from a position to end.
    pub fn get_to_end(&self, (from_line, from_character): (u64, u64)) -> String {
        let msg = format!(
            "puts [{} get {}.{} end] ; flush stdout",
            &self.id, from_line, from_character
        );
        wish::ask_wish(&msg)
    }

    /// Height of text, in rows
    pub fn height(&self, height: u64) {
        widget::configure(&self.id, "height", &height.to_string());
    }

    /// Insert at given (line, character) position of text.
    pub fn insert(&self, (line, character): (u64, u64), text: &str) {
        let msg = format!("{} insert {}.{} {{{}}}", &self.id, line, character, text);
        wish::tell_wish(&msg);
    }

    /// Inserts at end of text.
    pub fn insert_end(&self, text: &str) {
        let msg = format!("{} insert end {{{}}}", &self.id, text);
        wish::tell_wish(&msg);
    }

    /// Inserts at end of text, with given tags.
    pub fn insert_end_with_tags(&self, text: &str, tags: &[&str]) {
        let mut tags_str = String::new();
        for tag in tags {
            tags_str.push('{');
            tags_str.push_str(tag);
            tags_str.push('}');
            tags_str.push(' ');
        }
        let msg = format!("{} insert end {{{}}} {{{}}}", &self.id, text, tags_str);
        wish::tell_wish(&msg);
    }

    /// Inserts an image at given (line, character) position of text.
    pub fn insert_image(&self, (line, character): (u64, u64), image: &image::TkImage) {
        let msg = format!(
            "{} image create {}.{} -image {{{}}}",
            &self.id, line, character, &image.id
        );
        wish::tell_wish(&msg);
    }

    /// Inserts a Tk widget at given (line, character) position of text.
    pub fn insert_widget(&self, (line, character): (u64, u64), widget: &impl widget::TkWidget) {
        let msg = format!(
            "{} window create {}.{} -window {{{}}}",
            &self.id,
            line,
            character,
            widget.id()
        );
        wish::tell_wish(&msg);
    }

    /// Insert at given (line, character) position of text,
    /// with given tags.
    pub fn insert_with_tags(&self, (line, character): (u64, u64), text: &str, tags: &[&str]) {
        let mut tags_str = String::new();
        for tag in tags {
            tags_str.push('{');
            tags_str.push_str(tag);
            tags_str.push('}');
            tags_str.push(' ');
        }
        let msg = format!(
            "{} insert {}.{} {{{}}} {{{}}}",
            self.id, line, character, text, tags_str
        );
        wish::tell_wish(&msg);
    }

    /// Sets named mark's gravity to left.
    pub fn mark_gravity_left(&self, mark: &str) {
        let msg = format!("{} mark gravity {} left", &self.id, mark);
        wish::tell_wish(&msg);
    }

    /// Sets named mark's gravity to right.
    pub fn mark_gravity_right(&self, mark: &str) {
        let msg = format!("{} mark gravity {} right", &self.id, mark);
        wish::tell_wish(&msg);
    }

    /// Returns a (line, character) tuple for the given mark's position.
    pub fn mark_index(&self, mark: &str) -> (u64, u64) {
        let msg = format!("puts [{} index {}] ; flush stdout", &self.id, mark);
        let result = wish::ask_wish(&msg);
        let parts: Vec<&str> = result.split('.').collect();
        if parts.len() == 2 {
            let line = parts[0].parse::<u64>().unwrap_or(1);
            let character = parts[1].parse::<u64>().unwrap_or(0);

            (line, character)
        } else {
            // TODO - can this fail?
            (1, 0)
        }
    }

    /// Returns a list of all the mark names defined in this text widget.
    pub fn mark_names(&self) -> Vec<String> {
        let msg = format!("puts [{} mark names] ; flush stdout", &self.id);
        let result = wish::ask_wish(&msg);
        wish::split_items(&result)
    }

    /// Returns name of next mark from given position.
    pub fn mark_next(&self, (line, character): (u64, u64)) -> String {
        let msg = format!("{} mark next {}.{}", &self.id, line, character);
        wish::ask_wish(&msg)
    }

    /// Returns name of previous mark to given position.
    pub fn mark_previous(&self, (line, character): (u64, u64)) -> String {
        let msg = format!("{} mark prev {}.{}", &self.id, line, character);
        wish::ask_wish(&msg)
    }

    /// Sets named mark to given position.
    pub fn mark_set(&self, mark: &str, (line, character): (u64, u64)) {
        let msg = format!("{} mark set {} {}.{}", &self.id, mark, line, character);
        wish::tell_wish(&msg);
    }

    /// Removes named mark.
    pub fn mark_unset(&self, mark: &str) {
        let msg = format!("{} mark unset {}", &self.id, mark);
        wish::tell_wish(&msg);
    }

    /// Amount of horizontal padding for widget.
    pub fn padx(&self, value: u64) {
        widget::configure(&self.id, "padx", &value.to_string());
    }

    /// Amount of vertical padding for widget.
    pub fn pady(&self, value: u64) {
        widget::configure(&self.id, "pady", &value.to_string());
    }

    /// Style of border around label.
    pub fn relief(&self, value: widget::Relief) {
        widget::configure(&self.id, "relief", &value.to_string());
    }

    /// Replaces a range of text with new text.
    pub fn replace(
        &self,
        (from_line, from_character): (u64, u64),
        (to_line, to_character): (u64, u64),
        text: &str,
    ) {
        let msg = format!(
            "{} replace {}.{} {}.{} {{{}}}",
            &self.id, from_line, from_character, to_line, to_character, text
        );
        wish::tell_wish(&msg);
    }

    /// Searches the text widget from given position for the
    /// text, returning an Option type containing either the
    /// position of the found text or none.
    pub fn search(&self, text: &str, (line, character): (u64, u64)) -> Option<(u64, u64)> {
        let msg = format!(
            "puts [{} search {{{}}} {}.{}] ; flush stdout",
            &self.id, text, line, character
        );
        let result = wish::ask_wish(&msg);
        let parts: Vec<&str> = result.split('.').collect();
        if parts.len() == 2 {
            let line = parts[0].parse::<u64>().unwrap_or(1);
            let character = parts[1].parse::<u64>().unwrap_or(0);

            Some((line, character))
        } else {
            None
        }
    }

    /// Arranges text widget display to ensure the given line, character
    /// is visible.
    pub fn see(&self, (line, character): (u64, u64)) {
        let msg = format!("{} see {}.{}", self.id, line, character);
        wish::tell_wish(&msg);
    }

    /// Sets the state of the widget (`normal` or `disabled` only).
    pub fn state(&self, value: widget::State) {
        widget::configure(&self.id, "state", &value.to_string());
    }

    /// Associates given tag with text in specified range.
    pub fn tag_add(
        &self,
        tag: &str,
        (from_line, from_character): (u64, u64),
        (to_line, to_character): (u64, u64),
    ) {
        let msg = format!(
            "{} tag add {{{}}} {}.{} {}.{}",
            &self.id, tag, from_line, from_character, to_line, to_character
        );
        wish::tell_wish(&msg);
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

    /// Deletes a tag.
    pub fn tag_delete(&self, tag: &str) {
        let msg = format!("{} tag delete {{{}}}", &self.id, tag);
        wish::tell_wish(&msg);
    }

    /// Formatting is applied to tags using configuration options.
    ///
    /// For the available options, see the Tk
    /// [manual](https://www.tcl-lang.org/man/tcl8.6/TkCmd/text.htm#M43)
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

    /// Returns a list of all the tag names defined in this text widget
    /// at the given location.
    pub fn tag_names_at(&self, (line, character): (u64, u64)) -> Vec<String> {
        let msg = format!(
            "puts [{} tag names {}.{}] ; flush stdout",
            &self.id, line, character
        );
        let result = wish::ask_wish(&msg);
        wish::split_items(&result)
    }

    /// De-associates given tag with text in specified range.
    pub fn tag_remove(
        &self,
        tag: &str,
        (from_line, from_character): (u64, u64),
        (to_line, to_character): (u64, u64),
    ) {
        let msg = format!(
            "{} tag remove {{{}}} {}.{} {}.{}",
            &self.id, tag, from_line, from_character, to_line, to_character
        );
        wish::tell_wish(&msg);
    }

    /// Width of text, in columns
    pub fn width(&self, width: u64) {
        widget::configure(&self.id, "width", &width.to_string());
    }

    /// How wrapping should be performed of long lines.
    pub fn wrap(&self, value: widget::Wrapping) {
        widget::configure(&self.id, "wrap", &value.to_string());
    }
}
