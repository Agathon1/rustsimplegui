//! A Rust binding for the Tk graphics toolkit.
//!
//! rstk opens and communicates with Tk's wish program as a separate process.
//! The library provides:
//!
//! * low-level functions to directly communicate with wish, suitable for
//!   writing additional extensions
//! * high-level API to write GUI applications with minimal knowledge of Tk.
//!
//! The top-level functions to start/stop the GUI are contained
//! in the [wish] module.
//!
//! The remaining modules describe a widget or supporting component
//! (such as a font or image). Each widget has a constructor function, 
//! usually named "make_WIDGET", and this returns a struct of name "TkWIDGET". 
//!
//! Click on the struct name to get a list of methods supported by the
//! widget; functionality is divided between various traits, such as 
//! [TkWidget](widget::TkWidget).
//!
//! For examples and additional documentation, see the project
//! [webpage](https://peterlane.netlify.app/rstk/).
//!
//! # Example
//!
//! A simple hello-world example:
//!
//! ```
//! use rstk::*;
//!
//! fn main() {
//!   let root = rstk::start_wish().unwrap();
//!
//!   let hello = rstk::make_label(&root);
//!   hello.text("Hello from Rust/Tk");
//!
//!   hello.grid().layout();
//!
//!   rstk::mainloop();
//! }
//! ```
//!
//! ## Widget lifetimes
//!
//! The Tk process operates independently of your rust program. All references
//! to widgets in rust are merely string names used to 'lookup' the widget
//! when calling out to Tk. As such, widgets will remain live and visible even
//! if a variable referring to them goes out of scope in your rust code.
//! If you wish to destroy a widget, use the [destroy](widget::TkWidget::destroy)
//! method available on all widgets.
//!

pub mod button;
pub use button::*;

pub mod canvas;
pub use canvas::*;

pub mod chart;
pub use chart::*;

pub mod check_button;
pub use check_button::*;

pub mod combobox;
pub use combobox::*;

pub mod dialog;
pub use dialog::*;

pub mod entry;
pub use entry::*;

pub mod font;
pub use font::*;

pub mod frame;
pub use frame::*;

pub mod grid;
pub use grid::*;

pub mod image;
pub use image::*;

pub mod label;
pub use label::*;

pub mod label_frame;
pub use label_frame::*;

pub mod listbox;
pub use listbox::*;

pub mod menu;
pub use menu::*;

pub mod notebook;
pub use notebook::*;

pub mod pack;
pub use pack::*;

pub mod paned_window;
pub use paned_window::*;

pub mod progressbar;
pub use progressbar::*;

pub mod radio_button;
pub use radio_button::*;

pub mod scale;
pub use scale::*;

pub mod scrollbar;
pub use scrollbar::*;

pub mod separator;
pub use separator::*;

pub mod spinbox;
pub use spinbox::*;

pub mod text;
pub use text::*;

pub mod theme;
pub use theme::*;

pub mod toplevel;
pub use toplevel::*;

pub mod treeview;
pub use treeview::*;

pub mod widget;
pub use widget::*;

pub mod wish;
pub use wish::*;
