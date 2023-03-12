//! Menu widget - for building menubars and menus.
//!
//! The menu widget is a versatile widget. It can take on any of these
//! roles:
//!
//! * a window's menubar, containing several menus, such as "File", "Edit",
//!   "Help" etc.
//! * a menu containing menu items, such as "Open File", or submenus, such
//!   as "Recent files...".
//! * a menu button, which can be clicked to invoke an action. This button
//!   can act as a button, a check button, or a radio button.
//!
//! This library uses separate structs for the different types of menu which can
//! be added to an existing menu. These are built up in standard "builder"
//! style, before being added or inserted.
//!
//! * also see the Tk [manual](https://www.tcl-lang.org/man/tcl8.6/TkCmd/menu.htm)
//!

use super::image;
use super::widget;
use super::wish;

/// Refers to a menu widget
#[derive(Clone, Debug, PartialEq)]
pub struct TkMenu {
    pub id: String,
}

/// Creates an instance of a menu widget in given parent.
pub fn make_menu(parent: &impl widget::TkWidget) -> TkMenu {
    let id = wish::next_wid(parent.id());
    let msg = format!("menu {}", id);
    wish::tell_wish(&msg);

    TkMenu { id }
}

// -- subtypes of menu

/// Refers to a submenu
#[derive(Clone, Debug)]
pub struct TkMenuCascade {
    parent: String,
    compound: widget::Compound,
    font: Option<String>,
    image: Option<String>,
    label: Option<String>,
    menu: Option<String>,
    state: widget::State,
    underline: Option<u64>,
}

/// Refers to a check-button menu-item
#[derive(Clone, Debug)]
pub struct TkMenuCheck {
    parent: String,
    accelerator: Option<String>,
    command: Option<String>,
    command_variable: Option<String>,
    compound: widget::Compound,
    font: Option<String>,
    image: Option<String>,
    label: Option<String>,
    state: widget::State,
    underline: Option<u64>,
}

/// Refers to a command (menu-item)
#[derive(Clone, Debug)]
pub struct TkMenuCommand {
    parent: String,
    accelerator: Option<String>,
    command: Option<String>,
    compound: widget::Compound,
    font: Option<String>,
    image: Option<String>,
    label: Option<String>,
    state: widget::State,
    underline: Option<u64>,
}

/// Refers to a radio-button menu-item
#[derive(Clone, Debug)]
pub struct TkMenuRadio {
    parent: String,
    group: String,
    value: String,
    accelerator: Option<String>,
    command: Option<String>,
    command_variable: Option<String>,
    compound: widget::Compound,
    font: Option<String>,
    image: Option<String>,
    label: Option<String>,
    state: widget::State,
    underline: Option<u64>,
}

/// Refers to a menu separator
#[derive(Clone, Debug)]
pub struct TkMenuSeparator {
    parent: String,
}

// -- implementations of each menu type

impl widget::TkWidget for TkMenu {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}

impl TkMenu {
    /// Start to create a submenu.
    pub fn cascade(&self) -> TkMenuCascade {
        TkMenuCascade {
            parent: self.id.clone(),
            compound: widget::Compound::None,
            font: None,
            image: None,
            label: None,
            menu: None,
            state: widget::State::Normal,
            underline: None,
        }
    }

    /// Start to create a check-button menu-item.
    pub fn check_button(&self) -> TkMenuCheck {
        TkMenuCheck {
            parent: self.id.clone(),
            accelerator: None,
            command: None,
            command_variable: None,
            compound: widget::Compound::None,
            font: None,
            image: None,
            label: None,
            state: widget::State::Normal,
            underline: None,
        }
    }

    /// Start to create a command (simple menu item).
    pub fn command(&self) -> TkMenuCommand {
        TkMenuCommand {
            parent: self.id.clone(),
            accelerator: None,
            command: None,
            compound: widget::Compound::None,
            font: None,
            image: None,
            label: None,
            state: widget::State::Normal,
            underline: None,
        }
    }

    /// Start to create a radio-button menu-item.
    ///
    /// Radio buttons are arranged in groups, with each button in the group
    /// having its own value. For this reason, the group and value are
    /// required when making a radio-button menu-item.
    pub fn radio_button(&self, group: &str, value: &str) -> TkMenuRadio {
        TkMenuRadio {
            parent: self.id.clone(),
            group: String::from(group),
            value: String::from(value),
            accelerator: None,
            command: None,
            command_variable: None,
            compound: widget::Compound::None,
            font: None,
            image: None,
            label: None,
            state: widget::State::Normal,
            underline: None,
        }
    }

    /// Returns the value for a given radio-button group.
    pub fn radio_button_value_get(&self, group: &str) -> String {
        let msg = format!("puts $::mrb_group_{} ; flush stdout", group);
        wish::ask_wish(&msg)
    }

    /// Sets the value for a given radio-button group.
    pub fn radio_button_value(&self, group: &str, value: &str) {
        let msg = format!("set ::mrb_group_{} {}", group, value);
        wish::tell_wish(&msg);
    }

    /// Start to create a separator.
    pub fn separator(&self) -> TkMenuSeparator {
        TkMenuSeparator {
            parent: self.id.clone(),
        }
    }

    /// Deletes the menu-item at given index position.
    pub fn delete(&self, index: u64) {
        let msg = format!("{} delete {}", &self.id, index);
        wish::tell_wish(&msg);
    }

    /// Returns the value (as a String) for given option for
    /// menu-item at given index position.
    pub fn entry_cget(&self, index: u64, option: &str) -> String {
        let msg = format!("{} entrycfig {} {{{}}}", &self.id, index, option);
        wish::ask_wish(&msg)
    }

    /// Sets the value (as a String) for given option for
    /// menu-item at given index position.
    pub fn entry_configure(&self, index: u64, option: &str, value: &str) {
        let msg = format!(
            "{} entryconfigure {} {{{}}} {{{}}}",
            &self.id, index, option, value
        );
        wish::tell_wish(&msg);
    }

    /// Invokes any associated command for the menu-item at given index
    /// position.
    pub fn invoke(&self, index: u64) {
        let msg = format!("{} invoke {}", &self.id, index);
        wish::tell_wish(&msg);
    }

    /// Shows this menu at screen coordinates x, y.
    pub fn popup(&self, x: i64, y: i64) {
        let msg = format!("tk_popup {} {} {}", &self.id, x, y);
        wish::tell_wish(&msg);
    }
}

// Convert core options into a string representation
fn common_option_string(
    compound: &widget::Compound,
    font: &Option<String>,
    image: &Option<String>,
    label: &Option<String>,
    state: &widget::State,
    underline: &Option<u64>,
) -> String {
    let mut msg = String::new();

    msg.push_str(&format!("-compound {} ", compound));
    if let Some(font) = &font {
        msg.push_str(&format!("-font {{{}}} ", font));
    }
    if let Some(image) = &image {
        msg.push_str(&format!("-image {{{}}} ", image));
    }
    if let Some(label) = &label {
        msg.push_str(&format!("-label {{{}}} ", label));
    }
    msg.push_str(&format!("-state {} ", state));
    if let Some(underline) = &underline {
        msg.push_str(&format!("-underline {} ", underline));
    }

    msg
}

impl TkMenuCascade {
    /// Sets arrangement if menu item has both an image and a label.
    pub fn compound(&mut self, value: widget::Compound) -> &mut Self {
        self.compound = value;
        self
    }

    /// Sets font.
    pub fn font(&mut self, value: &str) -> &mut Self {
        self.font = Some(String::from(value));
        self
    }

    /// Sets image to display.
    pub fn image(&mut self, image: &image::TkImage) -> &mut Self {
        self.image = Some(String::from(&image.id));
        self
    }

    /// Sets text label to display.
    pub fn label(&mut self, value: &str) -> &mut Self {
        self.label = Some(String::from(value));
        self
    }

    /// Sets submenu to display.
    pub fn menu(&mut self, submenu: &TkMenu) -> &mut Self {
        self.menu = Some(submenu.id.clone());
        self
    }

    /// Sets display state for menu item.
    pub fn state(&mut self, value: widget::State) -> &mut Self {
        self.state = value;
        self
    }

    /// Sets index of text label to underline.
    pub fn underline(&mut self, value: u64) -> &mut Self {
        self.underline = Some(value);
        self
    }

    // Convert options into a string representation
    fn option_string(&self) -> String {
        let mut msg = common_option_string(
            &self.compound,
            &self.font,
            &self.image,
            &self.label,
            &self.state,
            &self.underline,
        );

        if let Some(menu) = &self.menu {
            msg.push_str(&format!("-menu {} ", menu));
        }

        msg
    }

    /// Adds cascade menu-item to parent with current set of options.
    pub fn add(&self) {
        let msg = format!("{} add cascade {}", &self.parent, &self.option_string());
        wish::tell_wish(&msg);
    }

    /// Inserts cascade menu-item to parent at given index with current set of options.
    pub fn insert(&self, index: u64) {
        let msg = format!(
            "{} insert cascade {} {}",
            &self.parent,
            index,
            &self.option_string()
        );
        wish::tell_wish(&msg);
    }
}

impl TkMenuCheck {
    /// Sets value of accelerator key - displayed to right of menu.
    pub fn accelerator(&mut self, key_definition: &str) -> &mut Self {
        self.accelerator = Some(String::from(key_definition));
        self
    }

    /// Sets the function to be called when the menu item is clicked.
    pub fn command(&mut self, command: impl Fn(bool) + Send + 'static) -> &mut Self {
        let id = wish::next_wid(".");
        let var = format!("::mcb{}", wish::current_id());
        wish::add_callback1_bool(&id, wish::mk_callback1_bool(command));
        self.command = Some(id);
        self.command_variable = Some(var);
        self
    }

    /// Sets arrangement if menu item has both an image and a label.
    pub fn compound(&mut self, value: widget::Compound) -> &mut Self {
        self.compound = value;
        self
    }

    /// Sets font.
    pub fn font(&mut self, value: &str) -> &mut Self {
        self.font = Some(String::from(value));
        self
    }

    /// Sets image to display.
    pub fn image(&mut self, image: &image::TkImage) -> &mut Self {
        self.image = Some(String::from(&image.id));
        self
    }

    /// Sets text label to display.
    pub fn label(&mut self, value: &str) -> &mut Self {
        self.label = Some(String::from(value));
        self
    }

    /// Sets display state for menu item.
    pub fn state(&mut self, value: widget::State) -> &mut Self {
        self.state = value;
        self
    }

    /// Sets index of text label to underline.
    pub fn underline(&mut self, value: u64) -> &mut Self {
        self.underline = Some(value);
        self
    }

    // Convert options into a string representation
    fn option_string(&self) -> String {
        let mut msg = common_option_string(
            &self.compound,
            &self.font,
            &self.image,
            &self.label,
            &self.state,
            &self.underline,
        );

        if let Some(accelerator) = &self.accelerator {
            msg.push_str(&format!("-accelerator {{{}}} ", &accelerator));
        }
        if let Some(command) = &self.command {
            if let Some(command_variable) = &self.command_variable {
                msg.push_str(&format!(
                    "-command {{ puts cb1b-{}-${} ; flush stdout }} ",
                    &command, &command_variable
                ));
                msg.push_str(&format!("-variable {} ", &command_variable));
            }
        }

        msg
    }

    /// Adds check-button menu-item to parent with current set of options.
    pub fn add(&self) {
        let msg = format!("{} add checkbutton {}", &self.parent, &self.option_string());
        wish::tell_wish(&msg);
    }

    /// Inserts check-button menu-item to parent at given index with current set of options.
    pub fn insert(&self, index: u64) {
        let msg = format!(
            "{} insert checkbutton {} {}",
            &self.parent,
            index,
            &self.option_string()
        );
        wish::tell_wish(&msg);
    }
}

impl TkMenuCommand {
    /// Sets value of accelerator key - displayed to right of menu.
    pub fn accelerator(&mut self, key_definition: &str) -> &mut Self {
        self.accelerator = Some(String::from(key_definition));
        self
    }

    /// Sets command to invoke when menu-item clicked.
    pub fn command(&mut self, command: impl Fn() + Send + 'static) -> &mut Self {
        let id = wish::next_wid(".");
        wish::add_callback0(&id, wish::mk_callback0(command));
        self.command = Some(id);
        self
    }

    /// Sets arrangement if menu item has both an image and a label.
    pub fn compound(&mut self, value: widget::Compound) -> &mut Self {
        self.compound = value;
        self
    }

    /// Sets font.
    pub fn font(&mut self, value: &str) -> &mut Self {
        self.font = Some(String::from(value));
        self
    }

    /// Sets image to display.
    pub fn image(&mut self, image: &image::TkImage) -> &mut Self {
        self.image = Some(String::from(&image.id));
        self
    }

    /// Sets text label to display.
    pub fn label(&mut self, value: &str) -> &mut Self {
        self.label = Some(String::from(value));
        self
    }

    /// Sets display state for menu item.
    pub fn state(&mut self, value: widget::State) -> &mut Self {
        self.state = value;
        self
    }

    /// Sets index of text label to underline.
    pub fn underline(&mut self, value: u64) -> &mut Self {
        self.underline = Some(value);
        self
    }

    // Convert options into a string representation
    fn option_string(&self) -> String {
        let mut msg = common_option_string(
            &self.compound,
            &self.font,
            &self.image,
            &self.label,
            &self.state,
            &self.underline,
        );

        if let Some(accelerator) = &self.accelerator {
            msg.push_str(&format!("-accelerator {{{}}} ", &accelerator));
        }
        if let Some(command) = &self.command {
            msg.push_str(&format!(
                "-command {{ puts clicked-{} ; flush stdout }} ",
                &command
            ));
        }

        msg
    }

    /// Adds command menu-item to parent with current set of options.
    pub fn add(&self) {
        let msg = format!("{} add command {}", &self.parent, &self.option_string());
        wish::tell_wish(&msg);
    }

    /// Inserts command menu-item to parent at given index with current set of options.
    pub fn insert(&self, index: u64) {
        let msg = format!(
            "{} insert command {} {}",
            &self.parent,
            index,
            &self.option_string()
        );
        wish::tell_wish(&msg);
    }
}

impl TkMenuRadio {
    /// Sets value of accelerator key - displayed to right of menu.
    pub fn accelerator(&mut self, key_definition: &str) -> &mut Self {
        self.accelerator = Some(String::from(key_definition));
        self
    }

    /// Sets the function to be called when the menu item is clicked.
    pub fn command(&mut self, command: impl Fn(bool) + Send + 'static) -> &mut Self {
        let id = wish::next_wid(".");
        let var = format!("::mcb{}", wish::current_id());
        wish::add_callback1_bool(&id, wish::mk_callback1_bool(command));
        self.command = Some(id);
        self.command_variable = Some(var);
        self
    }

    /// Sets arrangement if menu item has both an image and a label.
    pub fn compound(&mut self, value: widget::Compound) -> &mut Self {
        self.compound = value;
        self
    }

    /// Sets font.
    pub fn font(&mut self, value: &str) -> &mut Self {
        self.font = Some(String::from(value));
        self
    }

    /// Sets image to display.
    pub fn image(&mut self, image: &image::TkImage) -> &mut Self {
        self.image = Some(String::from(&image.id));
        self
    }

    /// Sets text label to display.
    pub fn label(&mut self, value: &str) -> &mut Self {
        self.label = Some(String::from(value));
        self
    }

    /// Sets display state for menu item.
    pub fn state(&mut self, value: widget::State) -> &mut Self {
        self.state = value;
        self
    }

    /// Sets index of text label to underline.
    pub fn underline(&mut self, value: u64) -> &mut Self {
        self.underline = Some(value);
        self
    }

    // Convert options into a string representation
    fn option_string(&self) -> String {
        let mut msg = common_option_string(
            &self.compound,
            &self.font,
            &self.image,
            &self.label,
            &self.state,
            &self.underline,
        );

        msg.push_str(&format!(
            "-variable ::mrb_group_{} -value {{{}}} ",
            &self.group, &self.value
        ));
        if let Some(accelerator) = &self.accelerator {
            msg.push_str(&format!("-accelerator {{{}}} ", &accelerator));
        }
        if let Some(command) = &self.command {
            if let Some(command_variable) = &self.command_variable {
                msg.push_str(&format!(
                    "-command {{ puts cb1b-{}-{} ; flush stdout }} ",
                    &command, &command_variable
                ));
                msg.push_str(&format!("-variable {{{}}} ", &command_variable));
            }
        }

        msg
    }

    /// Adds radio-button menu-item to parent with current set of options.
    pub fn add(&self) {
        let msg = format!("{} add radiobutton {}", &self.parent, &self.option_string());
        wish::tell_wish(&msg);
    }

    /// Inserts radio-button menu-item to parent at given index with current set of options.
    pub fn insert(&self, index: u64) {
        let msg = format!(
            "{} insert radiobutton {} {}",
            &self.parent,
            index,
            &self.option_string()
        );
        wish::tell_wish(&msg);
    }
}

impl TkMenuSeparator {
    /// Adds separator to the parent menu.
    pub fn add(&self) {
        let msg = format!("{} add separator", &self.parent);
        wish::tell_wish(&msg);
    }

    /// Inserts separator into parent at given index.
    pub fn insert(&self, index: u64) {
        let msg = format!("{} insert sepator {}", &self.parent, index);
        wish::tell_wish(&msg);
    }
}
