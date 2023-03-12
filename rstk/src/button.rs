//! Button widget - displays text/image. Executes a command when clicked.
//!
//! * also see the Tk [manual](https://www.tcl-lang.org/man/tcl8.6/TkCmd/ttk_button.htm)
//!
//! ## Examples
//!
//! The simplest button has some text and a command:
//!
//! ```
//! let button_1 = rstk::make_button(&root);
//! button_1.text("Button text");
//! button_1.command(|| { println!("Clicked button_1"); });
//! ```
//! 
//! Buttons can also display images, with or without text, and how these are 
//! displayed can be controlled using [compound](widget::TkLabelOptions::compound).
//! In the following example, a button with both an image and text is set to show 
//! the image below the text:
//!
//! ```
//! let button_3 = rstk::make_button(&root);
//! button_3.image(&read_image("tcllogo.gif"));
//! button_3.text("Tcl Logo");
//! button_3.command(|| { println!("Clicked button_3"); });
//! button_3.compound(rstk::Compound::Bottom);
//! ```
//! 

use super::grid;
use super::pack;
use super::widget;
use super::wish;

/// Refers to a button widget
#[derive(Clone, Debug, PartialEq)]
pub struct TkButton {
    pub id: String,
}

/// Creates an instance of a button widget in given parent.
pub fn make_button(parent: &impl widget::TkWidget) -> TkButton {
    let id = wish::next_wid(parent.id());
    let msg = format!("button {}", id);
    wish::tell_wish(&msg);

    TkButton { id }
}

impl widget::TkWidget for TkButton {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}

impl grid::TkGridLayout for TkButton {}
impl pack::TkPackLayout for TkButton {}

impl widget::TkLabelOptions for TkButton {}

impl TkButton {
    /// Sets the function to be called when the button is clicked.
    pub fn command(&self, command: impl Fn() + Send + 'static) {
        wish::add_callback0(&self.id, wish::mk_callback0(command));
        let msg = format!(
            "{} configure -command {{ puts clicked-{} ; flush stdout }}",
            self.id, self.id
        );
        wish::tell_wish(&msg);
    }

    /// Calls the button's command as if it were clicked.
    pub fn invoke(&self) {
        let msg = format!("{} invoke", self.id);
        wish::tell_wish(&msg);
    }

    /// Sets the state of the button, usually normal (clickable) 
    /// or disabled (unclickable).
    pub fn state(&self, value: widget::State) {
        widget::configure(&self.id, "state", &value.to_string());
    }
}
