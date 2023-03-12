//! Dialogs - various built-in dialogs.
//!
//! All of the dialogs (apart from the font chooser) are built using a
//! "builder" style. The named function creates a relevant struct value,
//! and then functions on the relevant TkWIDGET struct alter the
//! default values in that struct, until finally calling `show`
//! will set up and display the dialog.
//!
//! There is only one font chooser dialog instance, and commands are
//! provided to interact with it directly.
//!
//! # Message boxes
//!
//! * also see the Tk [manual](https://www.tcl-lang.org/man/tcl8.6/TkCmd/messageBox.htm)
//!
//! The message-box is used to set up a simple alert, confirmation or
//! information dialog:
//!
//! ```
//! rstk::message_box()
//!   .OPTION(VALUE) // 0 or more
//!   .show();
//! ```
//!
//! 1. `message_box` is called first, to get the TkMessageBox instance.
//! 2. `show` must be called last, to set up and display the dialog.
//! 3. zero or more options are added to the message box.
//!
//! # Chooser dialogs
//!
//! For colours, directories, files and fonts!
//!
//! Each dialog returns an Option type, with value None if cancelled.
//!
//! Tk manual pages:
//!
//! * [chooseColor](https://www.tcl-lang.org/man/tcl8.6/TkCmd/chooseColor.htm)
//! * [chooseDirectory](https://www.tcl-lang.org/man/tcl8.6/TkCmd/chooseDirectory.htm)
//! * [getOpenFile](https://www.tcl-lang.org/man/tcl8.6/TkCmd/getOpenFile.htm) (and getSaveFile)
//! * [fontchooser](https://www.tcl-lang.org/man/tcl8.6/TkCmd/fontchooser.htm)
//!

use super::font;
use super::toplevel;
use super::widget;
use super::wish;

/// Refers to the settings for TkMessageBox.
#[derive(Clone, Debug)]
pub struct TkMessageBox {
    default: Option<String>,
    detail: Option<String>,
    icon: widget::IconImage,
    message: Option<String>,
    parent: Option<String>,
    title: Option<String>,
    type_buttons: widget::DialogType,
}

/// Creates a message box to complete in builder style.
pub fn message_box() -> TkMessageBox {
    TkMessageBox {
        default: None,
        detail: None,
        icon: widget::IconImage::Error,
        message: None,
        parent: None,
        title: None,
        type_buttons: widget::DialogType::Ok,
    }
}

impl TkMessageBox {
    /// Sets name used for default button.
    pub fn default(&mut self, name: &str) -> &mut Self {
        self.default = Some(String::from(name));
        self
    }

    /// Sets submessage to display, below message.
    pub fn detail(&mut self, text: &str) -> &mut Self {
        self.detail = Some(String::from(text));
        self
    }

    /// Sets icon type.
    pub fn icon(&mut self, value: widget::IconImage) -> &mut Self {
        self.icon = value;
        self
    }

    /// Sets message to display.
    pub fn message(&mut self, text: &str) -> &mut Self {
        self.message = Some(String::from(text));
        self
    }

    /// Sets parent widget - dialog is usually shown relative to parent.
    pub fn parent(&mut self, value: &toplevel::TkTopLevel) -> &mut Self {
        self.parent = Some(String::from(&value.id));
        self
    }

    /// Sets title of the dialog window.
    pub fn title(&mut self, text: &str) -> &mut Self {
        self.title = Some(String::from(text));
        self
    }

    /// Sets type of dialog, which specifies its buttons.
    pub fn type_buttons(&mut self, value: widget::DialogType) -> &mut Self {
        self.type_buttons = value;
        self
    }

    /// Once message box is defined, this function will finally show it.
    ///
    /// Returns a string for the name of the button pressed.
    ///
    pub fn show(&self) -> String {
        let mut msg = String::from("puts [tk_messageBox ");

        if let Some(default) = &self.default {
            msg.push_str(&format!("-default {{{}}} ", default));
        }

        if let Some(detail) = &self.detail {
            msg.push_str(&format!("-detail {{{}}} ", detail));
        }

        msg.push_str(&format!("-icon {} ", self.icon));

        if let Some(message) = &self.message {
            msg.push_str(&format!("-message {{{}}} ", message));
        }

        if let Some(parent) = &self.parent {
            msg.push_str(&format!("-parent {} ", parent));
        }

        if let Some(title) = &self.title {
            msg.push_str(&format!("-title {{{}}} ", title));
        }

        msg.push_str(&format!("-type {} ", self.type_buttons));
        msg.push_str("] ; flush stdout");

        wish::ask_wish(&msg)
    }
}

/// Refers to the settings for TkColourChooser.
#[derive(Clone, Debug)]
pub struct TkColourChooser {
    parent: Option<String>,
    title: Option<String>,
    initial: Option<String>,
}

/// Creates a colour-chooser to complete in builder style.
pub fn colour_chooser() -> TkColourChooser {
    TkColourChooser {
        parent: None,
        title: None,
        initial: None,
    }
}

impl TkColourChooser {
    /// Sets parent widget - dialog is usually shown relative to parent.
    pub fn parent(&mut self, value: &toplevel::TkTopLevel) -> &mut Self {
        self.parent = Some(String::from(&value.id));
        self
    }

    /// Sets title of the dialog window.
    pub fn title(&mut self, text: &str) -> &mut Self {
        self.title = Some(String::from(text));
        self
    }

    /// Sets initial color of chooser.
    pub fn initial_color(&mut self, value: &str) -> &mut Self {
        self.initial_colour(value)
    }

    /// Sets initial colour of chooser.
    pub fn initial_colour(&mut self, value: &str) -> &mut Self {
        self.initial = Some(String::from(value));
        self
    }

    /// Once dialog is defined, this function will finally show it.
    ///
    /// Returns an option:
    ///
    /// * `Some(string)` - for the chosen colour, or
    /// * `None` - if cancel pressed.
    ///
    pub fn show(&self) -> Option<String> {
        let mut msg = String::from("puts [tk_chooseColor ");

        if let Some(parent) = &self.parent {
            msg.push_str(&format!("-parent {} ", parent));
        }

        if let Some(title) = &self.title {
            msg.push_str(&format!("-title {{{}}} ", title));
        }

        if let Some(initial) = &self.initial {
            msg.push_str(&format!("-initialcolor {{{}}} ", initial));
        }

        msg.push_str("] ; flush stdout");

        let result = wish::ask_wish(&msg);
        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }
}

/// Refers to the settings for TkDirectoryChooser.
#[derive(Clone, Debug)]
pub struct TkDirectoryChooser {
    parent: Option<String>,
    title: Option<String>,
    initial: Option<String>,
    must_exist: bool,
}

/// Creates a directory-chooser to complete in builder style.
pub fn directory_chooser() -> TkDirectoryChooser {
    TkDirectoryChooser {
        parent: None,
        title: None,
        initial: None,
        must_exist: false,
    }
}

impl TkDirectoryChooser {
    /// Sets parent widget - dialog is usually shown relative to parent.
    pub fn parent(&mut self, value: &toplevel::TkTopLevel) -> &mut Self {
        self.parent = Some(String::from(&value.id));
        self
    }

    /// Sets title of the dialog window.
    pub fn title(&mut self, text: &str) -> &mut Self {
        self.title = Some(String::from(text));
        self
    }

    /// Sets initial directory of chooser.
    pub fn initial_directory(&mut self, value: &str) -> &mut Self {
        self.initial = Some(String::from(value));
        self
    }

    /// Specify if directory must exist.
    pub fn must_exist(&mut self, value: bool) -> &mut Self {
        self.must_exist = value;
        self
    }

    /// Once dialog is defined, this function will finally show it.
    ///
    /// Returns an option:
    ///
    /// * `Some(string)` - for the chosen directory, or
    /// * `None` - if cancel pressed.
    ///
    pub fn show(&self) -> Option<String> {
        let mut msg = String::from("puts [tk_chooseDirectory ");

        if let Some(parent) = &self.parent {
            msg.push_str(&format!("-parent {} ", parent));
        }

        if let Some(title) = &self.title {
            msg.push_str(&format!("-title {{{}}} ", title));
        }

        if let Some(initial) = &self.initial {
            msg.push_str(&format!("-initialdir {{{}}} ", initial));
        }

        if self.must_exist {
            // default is false, so only change for true
            msg.push_str("-mustexist 1 ");
        }

        msg.push_str("] ; flush stdout");

        let result = wish::ask_wish(&msg);
        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }
}

/// Refers to the settings for TkOpenFileChooser.
#[derive(Clone, Debug)]
pub struct TkOpenFileChooser {
    parent: Option<String>,
    title: Option<String>,
    file_types: Option<Vec<(String, String)>>,
    initial_directory: Option<String>,
    initial_filename: Option<String>,
}

/// Creates an open-file dialog to complete in builder style.
pub fn open_file_chooser() -> TkOpenFileChooser {
    TkOpenFileChooser {
        parent: None,
        title: None,
        file_types: None,
        initial_directory: None,
        initial_filename: None,
    }
}

impl TkOpenFileChooser {
    /// Sets parent widget - dialog is usually shown relative to parent.
    pub fn parent(&mut self, value: &toplevel::TkTopLevel) -> &mut Self {
        self.parent = Some(String::from(&value.id));
        self
    }

    /// Sets title of the dialog window.
    pub fn title(&mut self, text: &str) -> &mut Self {
        self.title = Some(String::from(text));
        self
    }

    /// Sets list of file types.
    ///
    /// File types are passed as a list of pairs, e.g.:
    ///
    ///```
    /// let file_types = [("C++", ".cpp"), ("Rust", ".rs"), ("Any", "*")];
    /// dialog.file_types(&file_types);
    ///```
    pub fn file_types(&mut self, file_types: &[(&str, &str)]) -> &mut Self {
        let mut types: Vec<(String, String)> = vec![];
        for (txt, pat) in file_types {
            types.push((String::from(*txt), String::from(*pat)));
        }
        self.file_types = Some(types);

        self
    }

    /// Sets initial directory of chooser.
    pub fn initial_directory(&mut self, value: &str) -> &mut Self {
        self.initial_directory = Some(String::from(value));
        self
    }

    /// Sets initial filename of chooser.
    pub fn initial_filename(&mut self, value: &str) -> &mut Self {
        self.initial_filename = Some(String::from(value));
        self
    }

    /// Once dialog is defined, this function will finally show it.
    ///
    /// Returns an option:
    ///
    /// * `Some(string)` - for the chosen filename, or
    /// * `None` - if cancel pressed.
    ///
    pub fn show(&self) -> Option<String> {
        let mut msg = String::from("puts [tk_getOpenFile ");

        if let Some(parent) = &self.parent {
            msg.push_str(&format!("-parent {} ", parent));
        }

        if let Some(title) = &self.title {
            msg.push_str(&format!("-title {{{}}} ", title));
        }

        if let Some(types) = &self.file_types {
            if !types.is_empty() {
                msg.push_str("-filetypes {");

                for (txt, pat) in types {
                    msg.push_str(&format!("{{{{{}}} {{{}}}}} ", *txt, *pat));
                }

                msg.push_str("} ");
            }
        }

        if let Some(initial) = &self.initial_directory {
            msg.push_str(&format!("-initialdir {{{}}} ", initial));
        }

        if let Some(initial) = &self.initial_filename {
            msg.push_str(&format!("-initialfile {{{}}} ", initial));
        }

        msg.push_str("] ; flush stdout");

        let result = wish::ask_wish(&msg);
        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }
}

/// Refers to the settings for TkSaveFileChooser.
#[derive(Clone, Debug)]
pub struct TkSaveFileChooser {
    parent: Option<String>,
    title: Option<String>,
    confirm_overwrite: bool,
    file_types: Option<Vec<(String, String)>>,
    initial_directory: Option<String>,
    initial_filename: Option<String>,
}

/// Creates a save-file dialog to complete in builder style.
pub fn save_file_chooser() -> TkSaveFileChooser {
    TkSaveFileChooser {
        parent: None,
        title: None,
        confirm_overwrite: true,
        file_types: None,
        initial_directory: None,
        initial_filename: None,
    }
}

impl TkSaveFileChooser {
    /// Sets parent widget - dialog is usually shown relative to parent.
    pub fn parent(&mut self, value: &toplevel::TkTopLevel) -> &mut Self {
        self.parent = Some(String::from(&value.id));
        self
    }

    /// Sets title of the dialog window.
    pub fn title(&mut self, text: &str) -> &mut Self {
        self.title = Some(String::from(text));
        self
    }

    /// Set (by default) to show a warning dialog if user attempts to
    /// select an existing filename. Call this to unset and remove the
    /// warning.
    pub fn confirm_overwrite(&mut self, value: bool) -> &mut Self {
        self.confirm_overwrite = value;
        self
    }

    /// Sets list of file types.
    ///
    /// File types are passed as a list of pairs, e.g.:
    ///
    ///```
    /// let file_types = [("C++", ".cpp"), ("Rust", ".rs"), ("Any", "*")];
    /// dialog.file_types(&file_types);
    ///```
    pub fn file_types(&mut self, file_types: &[(&str, &str)]) -> &mut Self {
        let mut types: Vec<(String, String)> = vec![];
        for (txt, pat) in file_types {
            types.push((String::from(*txt), String::from(*pat)));
        }
        self.file_types = Some(types);

        self
    }

    /// Sets initial directory of chooser.
    pub fn initial_directory(&mut self, value: &str) -> &mut Self {
        self.initial_directory = Some(String::from(value));
        self
    }

    /// Sets initial filename of chooser.
    pub fn initial_filename(&mut self, value: &str) -> &mut Self {
        self.initial_filename = Some(String::from(value));
        self
    }

    /// Once dialog is defined, this function will finally show it.
    ///
    /// Returns an option:
    ///
    /// * `Some(string)` - for the chosen filename, or
    /// * `None` - if cancel pressed.
    ///
    pub fn show(&self) -> Option<String> {
        let mut msg = String::from("puts [tk_getSaveFile ");

        if let Some(parent) = &self.parent {
            msg.push_str(&format!("-parent {} ", parent));
        }

        if let Some(title) = &self.title {
            msg.push_str(&format!("-title {{{}}} ", title));
        }

        msg.push_str(&format!(
            "-confirmoverwrite {} ",
            if self.confirm_overwrite { "1" } else { "0" }
        ));

        if let Some(types) = &self.file_types {
            if !types.is_empty() {
                msg.push_str("-filetypes {");

                for (txt, pat) in types {
                    msg.push_str(&format!("{{{{{}}} {{{}}}}} ", *txt, *pat));
                }

                msg.push_str("} ");
            }
        }

        if let Some(initial) = &self.initial_directory {
            msg.push_str(&format!("-initialdir {{{}}} ", initial));
        }

        if let Some(initial) = &self.initial_filename {
            msg.push_str(&format!("-initialfile {{{}}} ", initial));
        }

        msg.push_str("] ; flush stdout");

        let result = wish::ask_wish(&msg);
        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }
}

// -- font chooser is different - use individual functions

/// Set the parent widget for the font-chooser.
pub fn font_chooser_parent(parent: &impl widget::TkWidget) {
    let msg = format!("tk fontchooser configure -parent {}", parent.id());
    wish::tell_wish(&msg);
}

/// Set the title for the font-chooser.
pub fn font_chooser_title(title: &str) {
    let msg = format!("tk fontchooser configure -title {}", title);
    wish::tell_wish(&msg);
}

/// Set the command to be called when a font is chosen.
pub fn font_chooser_command(command: impl Fn(font::TkFont) + Send + 'static) {
    wish::add_callback1_font("font", wish::mk_callback1_font(command));
    let msg = "tk fontchooser configure -command [list font_choice font]";
    wish::tell_wish(&msg);
}

/// Get the font for the font-chooser.
pub fn font_chooser_font_get() -> String {
    let msg = "tk fontchooser configure -font ";
    wish::ask_wish(&msg)
}

/// Set the font for the font-chooser.
pub fn font_chooser_font_set(font: &str) {
    let msg = format!("tk fontchooser configure -font {}", font);
    wish::tell_wish(&msg);
}

/// Hide the font-chooser, making it not visible.
pub fn font_chooser_hide() {
    let msg = "tk fontchooser hide";
    wish::tell_wish(&msg);
}

/// Show the font-chooser, making it visible.
pub fn font_chooser_show() {
    let msg = "tk fontchooser show";
    wish::tell_wish(&msg);
}

/// Check if the font-chooser is currently visible.
pub fn font_chooser_visible() -> bool {
    let msg = "tk fontchooser configure -visible ";
    let result = wish::ask_wish(&msg);
    result == "1"
}
