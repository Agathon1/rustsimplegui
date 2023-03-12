//! Themes - functions to list and set the overall theme (look and feel) of 
//! the Tk program.
//!
//! * also see the Tk [manual](https://www.tcl-lang.org/man/tcl8.6/TkCmd/ttk_style.htm#M17)

use super::wish;

/// Returns a list of the current themes.
///
/// For example:
///
/// ```
/// let themes = rstk::theme_names();
/// println!("{} available themes: ", themes.len());
/// for theme in themes {
///     println!(" - {}", theme);
/// }
/// ```
///
/// Lists the themes (on Linux):
///
/// ```text
/// 4 available themes:
///  - clam
///  - alt
///  - default
///  - classic
/// ```
///
pub fn theme_names() -> Vec<String> {
    let themes = wish::ask_wish("puts [ttk::style theme names] ; flush stdout");

    let mut result: Vec<String> = vec![];
    for theme in themes.split_whitespace() {
        result.push(String::from(theme));
    }

    result
}

/// Sets the current theme to the given theme-name
pub fn use_theme(name: &str) {
    let msg = format!("ttk::style theme use {}", name);
    wish::tell_wish(&msg);
}
