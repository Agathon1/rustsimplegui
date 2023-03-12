//! Check button widget - displays text/image with an on/off widget. 
//! Executes a command when clicked.
//!
//! * also see the Tk [manual](https://tcl.tk/man/tcl/TkCmd/ttk_checkbutton.htm)
//!
//! ## Examples
//!
//! The check button works like a regular [button](crate::button), except that it 
//! has a _state_. 
//!
//! The simplest check button has some text and a command - the command 
//! accepts a single boolean, which is the _new_ state of the button:
//!
//! ```
//! let button_1 = rstk::make_check_button(&root);
//! button_1.text("Button label");
//! button_1.command(|value| { println!("button_1 now {}", value); });
//! ```
//! 
//! Check buttons can also display images, with or without text, and how these 
//! are displayed can be controlled using 
//! [compound](widget::TkLabelOptions::compound). In the following example, a 
//! check button with both an image and text is set to show the image below the 
//! text:
//!
//! ```
//! let button_3 = rstk::make_check_button(&root);
//! button_3.image(&read_image("tcllogo.gif"));
//! button_3.text("Tcl Logo");
//! button_3.command(|state| { println!("Clicked button_3"); });
//! button_3.compound(rstk::Compound::Bottom);
//! ```
//! 
//! The check button's state can be changed or checked using the 
//! [selected](TkCheckButton::selected) and 
//! [is_selected](TkCheckButton::is_selected) methods.
//!

use super::grid;
use super::pack;
use super::widget;
use super::wish;

/// Refers to a check-button widget
#[derive(Clone, Debug, PartialEq)]
pub struct TkCheckButton {
    pub id: String,
    var: String,
}

/// Creates an instance of a check-button widget in given parent.
pub fn make_check_button(parent: &impl widget::TkWidget) -> TkCheckButton {
    let id = wish::next_wid(parent.id());
    let var = format!("::cb{}", wish::current_id());
    let msg = format!("set {} 0 ; ttk::checkbutton {} -variable {}", var, id, var);
    wish::tell_wish(&msg);

    TkCheckButton { id, var }
}

impl widget::TkWidget for TkCheckButton {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}
impl grid::TkGridLayout for TkCheckButton {}
impl pack::TkPackLayout for TkCheckButton {}
impl widget::TkLabelOptions for TkCheckButton {}

impl TkCheckButton {
    /// Sets the function to be called when the button is clicked.
    /// This function takes one boolean parameter, which is the _new_ state
    /// of the check button.
    pub fn command(&self, command: impl Fn(bool) + Send + 'static) {
        wish::add_callback1_bool(&self.id, wish::mk_callback1_bool(command));
        let msg = format!(
            "{} configure -command {{ puts cb1b-{}-${} ; flush stdout }}",
            self.id, self.id, self.var
        );
        wish::tell_wish(&msg);
    }

    /// Toggles the button's state and calls the button's command, 
    /// as if it were clicked.
    pub fn invoke(&self) {
        let msg = format!("{} invoke", self.id);
        wish::tell_wish(&msg);
    }

    /// Returns true/false if button is selected (checked) or not.
    pub fn is_selected(&self) -> bool {
        let msg = format!("puts ${} ; flush stdout", self.var);
        let result = wish::ask_wish(&msg);
        result == "1"
    }

    /// Sets the selected (checked) state.
    pub fn selected(&self, value: bool) {
        let msg = format!("set {} {}", self.var, if value { "1" } else { "0" });
        wish::tell_wish(&msg);
    }

    /// Sets the state of the button, usually normal (clickable) 
    /// or disabled (unclickable).
    pub fn state(&self, value: widget::State) {
        widget::configure(&self.id, "state", &value.to_string());
    }
}
