//! Functions and definitions applying to all widgets or specific sub-classes
//! of widgets.
//!

use std::fmt;

use super::canvas;
use crate::chart::plotchart;
use super::font;
use super::image;
use super::wish;

/// Struct holding information from a bound event,
/// returned as a parameter to the bound closure.
#[derive(Clone, Debug)]
pub struct TkEvent {
    /// x-coordinate relative to current widget
    pub x: i64,
    /// y-coordinate relative to current widget
    pub y: i64,
    /// x-coordinate relative to screen
    pub root_x: i64,
    /// y-coordinate relative to screen
    pub root_y: i64,
    /// vertical screen distance, e.g. for a drag event
    pub height: i64,
    /// horizontal screen distance, e.g. for a drag event
    pub width: i64,
    /// Numeric code representing key for current event
    pub key_code: u64,
    /// Symbol representing key for current event, e.g. "space", "e".
    pub key_symbol: String,
    /// Number of mouse button in current event: 1 for left, 3 for right, etc.
    pub mouse_button: u64,
}

/// Common trait for container widgets. Child widgets should implement the `id`
/// method. The remaining methods are standard Tk methods and convenient,
/// type-safe versions of them.
pub trait TkWidget {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str;

    /// Binds a command to this widget to call on given event pattern
    fn bind(&self, pattern: &str, command: impl Fn(TkEvent) + Send + 'static) {
        bind_to(&self.id(), pattern, command);
    }

    /// Retrieve the value of a configuration option
    /// as a string.
    ///
    /// * `option` - the option to read
    ///
    fn cget(&self, option: &str) -> String {
        let msg = format!("puts [{} cget -{}] ; flush stdout", self.id(), option);
        wish::ask_wish(&msg)
    }

    /// Used to change properties of a widget.
    /// This function can be used to directly configure
    /// the widget using an option-value string pair:
    ///
    /// * `option` - the option to change
    /// * `value` - the value to change it to
    ///
    fn configure(&self, option: &str, value: &str) {
        configure(&self.id(), option, value);
    }

    /// Destroys a widget and its children.
    fn destroy(&self) {
        let msg = format!("destroy {}", self.id());
        wish::tell_wish(&msg);
    }

    /// winfo retrieves information about widget.
    ///
    fn winfo(&self, option: &str) -> String {
        let msg = format!("winfo {} {}", option, self.id());
        wish::ask_wish(&msg)
    }

    // -- TODO should be here, or more specific?

    /// Makes this widget the focus window (e.g. for key presses)
    fn focus(&self) {
        let msg = format!("focus {}", self.id());
        wish::tell_wish(&msg);
    }

    // -- winfo functions

    /// Returns the widget x position in pixels, within its parent.
    fn position_x(&self) -> u64 {
        let msg = format!("winfo x {}", self.id());
        let result = wish::ask_wish(&msg);
        if let Ok(value) = result.parse::<u64>() {
            value
        } else {
            0
        }
    }

    /// Returns the widget y position in pixels, within its parent.
    fn position_y(&self) -> u64 {
        let msg = format!("winfo y {}", self.id());
        let result = wish::ask_wish(&msg);
        if let Ok(value) = result.parse::<u64>() {
            value
        } else {
            0
        }
    }

    /// Returns the widget height in pixels.
    fn widget_height(&self) -> u64 {
        let msg = format!("winfo height {}", self.id());
        let result = wish::ask_wish(&msg);
        if let Ok(value) = result.parse::<u64>() {
            value
        } else {
            0
        }
    }

    /// Returns the widget width in pixels.
    fn widget_width(&self) -> u64 {
        let msg = format!("winfo width {}", self.id());
        let result = wish::ask_wish(&msg);
        if let Ok(value) = result.parse::<u64>() {
            value
        } else {
            0
        }
    }

    /// Returns the position of the mouse on screen of widget as (x,y).
    fn mouse_position(&self) -> (i64, i64) {
        (self.mouse_x(), self.mouse_y())
    }

    /// Gives the x position of the mouse on screen of widget.
    fn mouse_x(&self) -> i64 {
        let msg = format!("winfo pointerx {}", self.id());
        let result = wish::ask_wish(&msg);
        if let Ok(value) = result.parse::<i64>() {
            value
        } else {
            -1
        }
    }

    /// Gives the y position of the mouse on screen of widget.
    fn mouse_y(&self) -> i64 {
        let msg = format!("winfo pointery {}", self.id());
        let result = wish::ask_wish(&msg);
        if let Ok(value) = result.parse::<i64>() {
            value
        } else {
            -1
        }
    }

    /// Height of screen of widget in pixels.
    fn screen_height(&self) -> u64 {
        let msg = format!("winfo screenheight {}", self.id());
        let result = wish::ask_wish(&msg);
        if let Ok(value) = result.parse::<u64>() {
            value
        } else {
            0
        }
    }

    /// Height of screen of widget in millimetres.
    fn screen_height_mm(&self) -> u64 {
        let msg = format!("winfo screenmmheight {}", self.id());
        let result = wish::ask_wish(&msg);
        if let Ok(value) = result.parse::<u64>() {
            value
        } else {
            0
        }
    }

    /// Width of screen of widget in pixels.
    fn screen_width(&self) -> u64 {
        let msg = format!("winfo screenwidth {}", self.id());
        let result = wish::ask_wish(&msg);
        if let Ok(value) = result.parse::<u64>() {
            value
        } else {
            0
        }
    }

    /// Width of screen of widget in millimetres.
    fn screen_width_mm(&self) -> u64 {
        let msg = format!("winfo screenmmwidth {}", self.id());
        let result = wish::ask_wish(&msg);
        if let Ok(value) = result.parse::<u64>() {
            value
        } else {
            0
        }
    }

    // -- stacking order

    /// Lowers the widget in stacking order.
    fn lower(&self) {
        let msg = format!("lower {}", self.id());
        wish::tell_wish(&msg);
    }

    /// Raises the widget in stacking order.
    fn raise(&self) {
        let msg = format!("raise {}", self.id());
        wish::tell_wish(&msg);
    }

    // -- for widgets that can contain other widgets

    /// Sets property for a given column of the grid layout
    /// contained within this widget.
    fn grid_configure_column(&self, index: u64, option: &str, value: &str) {
        let msg = format!(
            "grid columnconfigure {} {} -{} {{{}}}",
            self.id(),
            index,
            option,
            value
        );
        wish::tell_wish(&msg);
    }

    /// Sets property for a given row of the grid layout
    /// contained within this widget.
    fn grid_configure_row(&self, index: u64, option: &str, value: &str) {
        let msg = format!(
            "grid rowconfigure {} {} -{} {{{}}}",
            self.id(),
            index,
            option,
            value
        );
        wish::tell_wish(&msg);
    }
}

/// A set of common functions used in all label, button and similar widgets.
///
/// * also see the Tk [manual](https://tcl.tk/man/tcl/TkCmd/ttk_widget.htm#M6)
///
/// Colours are specified as strings, by either:
///
/// * `name` - using one of the values in the tk [colours](https://tcl.tk/man/tcl8.6/TkCmd/colors.htm) list
/// * `rgb` - as a 6-digit hexadecimal value in form "#RRGGBB"
///
pub trait TkLabelOptions: TkWidget {
    /// Sets how to arrange the image relative to the text.
    fn compound(&self, value: Compound) {
        configure(&self.id(), "compound", &value.to_string());
    }

    /// Sets the font to use for text.
    fn font(&self, definition: &font::TkFont) {
        configure(&self.id(), "font", &definition.to_string());
    }

    /// Sets the foreground (text) colour.
    fn foreground(&self, colour: &str) {
        configure(&self.id(), "foreground", colour);
    }

    /// Sets an image to display on the widget.
    fn image(&self, image: &image::TkImage) {
        configure(&self.id(), "image", &image.id);
    }

    /// Sets space around the widget. Takes an array of up to four values, 
    /// specifying the number of pixels on the different sides:
    ///
    /// * \[all]
    /// * [left-right top-bottom]
    /// * [left top-bottom right]
    /// * [left top right bottom]
    fn padding(&self, values: &[u64]) {
        padding(&self.id(), values);
    }

    /// Sets the text label for the widget.
    fn text(&self, value: &str) {
        configure(&self.id(), "text", value);
    }

    /// Underlines the character at the given index position.
    fn underline(&self, index: u64) {
        configure(&self.id(), "underline", &index.to_string());
    }

    /// Sets the width of the widget, in characters
    fn width(&self, value: i64) {
        let msg = format!("{} configure -width {{{}}}", self.id(), value);
        wish::tell_wish(&msg);
    }

    fn height(&self, value: i64) {
        let msg = format!("{} configure -height {}", self.id(), value);
        wish::tell_wish(&msg);
    }
}

// --------------------------------------------------------------------------
// Enums to type-check values

/// Defines position of a displayed item within some bounds.
#[derive(Clone, Debug, PartialEq)]
pub enum Anchor {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
    Center,
    Centre,
}

impl fmt::Display for Anchor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Anchor::N => "n",
            Anchor::NE => "ne",
            Anchor::E => "e",
            Anchor::SE => "se",
            Anchor::S => "s",
            Anchor::SW => "sw",
            Anchor::W => "w",
            Anchor::NW => "nw",
            Anchor::Center | Anchor::Centre => "center",
        };
        write!(f, "{}", &value)
    }
}

/// Arrangement of image relative to text in a
/// label-like widget.
///
/// So `Bottom` places the image below its text, etc.
///
#[derive(Clone, Debug, PartialEq)]
pub enum Compound {
    Bottom,
    Center,
    Centre,
    /// Show only the image
    Image,
    Left,
    /// Shows image if present, otherwise the text
    None,
    Right,
    /// Show only the text
    Text,
    Top,
}

impl fmt::Display for Compound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Compound::Bottom => "bottom",
            Compound::Center | Compound::Centre => "center",
            Compound::Image => "image",
            Compound::Left => "left",
            Compound::None => "none",
            Compound::Right => "right",
            Compound::Text => "text",
            Compound::Top => "top",
        };
        write!(f, "{}", &value)
    }
}

/// Type of message-box dialog.
#[derive(Clone, Debug, PartialEq)]
pub enum DialogType {
    AbortRetryIgnore,
    Ok,
    OkCancel,
    RetryCancel,
    YesNo,
    YesNoCancel,
}

impl fmt::Display for DialogType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            DialogType::AbortRetryIgnore => "abortretryignore",
            DialogType::Ok => "ok",
            DialogType::OkCancel => "okcancel",
            DialogType::RetryCancel => "retrycancel",
            DialogType::YesNo => "yesno",
            DialogType::YesNoCancel => "yesnocancel",
        };
        write!(f, "{}", &value)
    }
}

/// Type of icon to use in message-box dialog.
#[derive(Clone, Debug, PartialEq)]
pub enum IconImage {
    Error,
    Information,
    Question,
    Warning,
}

impl fmt::Display for IconImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            IconImage::Error => "error",
            IconImage::Information => "info",
            IconImage::Question => "question",
            IconImage::Warning => "warning",
        };
        write!(f, "{}", &value)
    }
}

/// Arrangement of text
#[derive(Clone, Debug, PartialEq)]
pub enum Justify {
    Center,
    Centre,
    Left,
    Right,
}

impl fmt::Display for Justify {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Justify::Left => "left",
            Justify::Center | Justify::Centre => "center",
            Justify::Right => "right",
        };
        write!(f, "{}", &value)
    }
}

/// Defines orientation of widget.
#[derive(Clone, Debug, PartialEq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

impl fmt::Display for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Orientation::Horizontal => "horizontal",
            Orientation::Vertical => "vertical",
        };
        write!(f, "{}", &value)
    }
}

/// Defines fill property for pack layouts: whether 
/// to expand in the x or y or both directions.
#[derive(Clone, Debug, PartialEq)]
pub enum PackFill {
    Both,
    None,
    X,
    Y
}

impl fmt::Display for PackFill {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            PackFill::Both => "both",
            PackFill::None => "none",
            PackFill::X => "x",
            PackFill::Y => "y",
        };
        write!(f, "{}", &value)
    }
}

/// Defines side property for pack layouts: whether 
/// widget is packed against top, bottom, left or right.
#[derive(Clone, Debug, PartialEq)]
pub enum PackSide {
    Bottom,
    Left,
    Right,
    Top,
}

impl fmt::Display for PackSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            PackSide::Bottom => "bottom",
            PackSide::Left => "left",
            PackSide::Right => "right",
            PackSide::Top => "top",
        };
        write!(f, "{}", &value)
    }
}

/// Defines mode of progressbar.
#[derive(Clone, Debug, PartialEq)]
pub enum ProgressMode {
    Determinate,
    Indeterminate,
}

impl fmt::Display for ProgressMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            ProgressMode::Determinate => "determinate",
            ProgressMode::Indeterminate => "indeterminate",
        };
        write!(f, "{}", &value)
    }
}

/// Defines shape of border around widget.
#[derive(Clone, Debug, PartialEq)]
pub enum Relief {
    Flat,
    Groove,
    Raised,
    Ridge,
    Solid,
    Sunken,
}

impl fmt::Display for Relief {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Relief::Flat => "flat",
            Relief::Groove => "groove",
            Relief::Raised => "raised",
            Relief::Ridge => "ridge",
            Relief::Solid => "solid",
            Relief::Sunken => "sunken",
        };
        write!(f, "{}", &value)
    }
}

/// Defines mode of selection.
///
/// Note: rstk diverges from the Tk terminology:
///
/// * 'Single' is equivalent to Tk's "browse"
/// * 'Multiple' is equivalent to Tk's "extended"
///
/// (Tk's "single" and "multiple" options are now rarely used (see
/// [manual](https://www.tcl-lang.org/man/tcl8.6/TkCmd/listbox.htm#M56)).)
///
#[derive(Clone, Debug, PartialEq)]
pub enum Selection {
    Multiple,
    Single,
    None,
}

impl fmt::Display for Selection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Selection::Multiple => "extended",
            Selection::None => "none",
            Selection::Single => "browse",
        };
        write!(f, "{}", &value)
    }
}

/// Specifies which sides of its container a widget 'sticks' to,
/// especially when it is resized.
#[derive(Clone, Debug, PartialEq)]
pub enum Sticky {
    N,
    NE,
    NES,
    NEW,
    NESW,
    NS,
    NSW,
    NW,
    E,
    ES,
    ESW,
    EW,
    S,
    SW,
    W,
    None,
}

impl fmt::Display for Sticky {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Sticky::N => "n",
            Sticky::NE => "ne",
            Sticky::NES => "nes",
            Sticky::NEW => "new",
            Sticky::NESW => "nesw",
            Sticky::NS => "ns",
            Sticky::NSW => "nsw",
            Sticky::NW => "nw",
            Sticky::E => "e",
            Sticky::ES => "es",
            Sticky::EW => "ew",
            Sticky::ESW => "esw",
            Sticky::S => "s",
            Sticky::SW => "sw",
            Sticky::W => "w",
            Sticky::None => "",
        };
        write!(f, "{}", &value)
    }
}

/// The kinds of activity state for a widget, e.g. if it is currently
/// available to use or disabled.
#[derive(Clone, Debug, PartialEq)]
pub enum State {
    /// Used, e.g., for buttons, to highlight when a mouse pointer is over them.
    Active,
    /// Used to prevent user-interaction with a widget.
    Disabled,
    /// The usual state of a widget, permitting user interactions.
    Normal,
    /// State cannot be changed, for those widgets with editable state.
    Readonly,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            State::Active => "active",
            State::Disabled => "disabled",
            State::Normal => "normal",
            State::Readonly => "readonly",
        };
        write!(f, "{}", &value)
    }
}

/// Types of word wrapping.
#[derive(Clone, Debug, PartialEq)]
pub enum Wrapping {
    Char,
    None,
    Word,
}

impl fmt::Display for Wrapping {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Wrapping::Char => "char",
            Wrapping::None => "none",
            Wrapping::Word => "word",
        };
        write!(f, "{}", &value)
    }
}

// --------------------------------------------------------------------------
// Internal functions for within crate use

pub(super) fn bind_to(tag: &str, pattern: &str, command: impl Fn(TkEvent) + Send + 'static) {
    // tag+pattern used as identifier, as multiple commands can be bound to each entity
    let tag_pattern = format!("{}{}", tag, pattern); // TODO ? remove ':' ?
    wish::add_callback1_event(&tag_pattern, wish::mk_callback1_event(command));
    let msg = format!(
        "bind {} {} {{ puts cb1e:{}:%x:%y:%X:%Y:%h:%w:%k:%K:%b ; flush stdout }}",
        tag, pattern, tag_pattern
    );
    wish::tell_wish(&msg);
}

pub(super) fn configure(wid: &str, option: &str, value: &str) {
    let msg = format!("{} configure -{} {{{}}}", wid, option, value);
    wish::tell_wish(&msg);
}

pub(super) fn padding(wid: &str, values: &[u64]) {
    let mut value_str = String::new();
    for i in values.iter() {
        value_str.push_str(&i.to_string());
        value_str.push(' ');
    }
    configure(wid, "padding", &value_str);
}

pub(super) fn strings_list(values: &[&str]) -> String {
    let mut values_str = String::new();

    for value in values {
        values_str.push_str(&format!("{} ", value));
    }

    values_str
}

pub(super) fn str_list(values: &[f64]) -> String {
    let mut values_str = String::new();

    for value in values {
        values_str.push_str(&format!("{} ", value));
    }

    values_str
}

pub(super) fn str_list_lists<M: AsRef<[R]>, R: AsRef<[f64]>>(values: M) -> String {
    let mut values_str = String::new();

    for vs in values.as_ref() {
        values_str.push_str("{");
        values_str.push_str(&str_list(vs.as_ref()));
        values_str.push_str("} ");
    }

    values_str
}

// --------------------------------------------------------------------------

/// Triggers given command after 'time' milliseconds.
pub fn after(time: u64, command: impl Fn() + Send + 'static) {
    wish::next_wid(".");
    let name = format!("after{}", wish::current_id()); 
    wish::add_callback0(&name, wish::mk_callback0(command));
    let msg = format!("after {} {{ puts clicked-{} ; flush stdout }}",
                      time, name);
    wish::tell_wish(&msg);
}

/// Binds command for event pattern to _all_ widgets.
pub fn bind(pattern: &str, command: impl Fn(TkEvent) + Send + 'static) {
    bind_to("all", pattern, command);
}

/// Sets colour map (for xy_plots).
pub fn colour_map(map: plotchart::ColourMap) {
    let msg = format!("::Plotchart::colorMap {}", map);
    wish::tell_wish(&msg);
}

/// Copies contents of one or more plots onto another canvas widget.
pub fn plot_pack(canvas: &canvas::TkCanvas, 
                 direction: plotchart::PlotDirection,
                 charts: &[&impl plotchart::TkPlotchart]) {
    let mut charts_str = String::new();
    for chart in charts {
        charts_str.push_str(&format!("${} ", chart.id()));
    }

    let msg = format!("::Plotchart::plotpack {} {} {}",
                      &canvas.id, direction, charts_str);
    wish::tell_wish(&msg);
}

/// Checks what the current OS system is: see
/// Tk [manual](https://www.tcl-lang.org/man/tcl8.6/TkCmd/tk.htm#M12).
///
/// Returns one of x11, win32, aqua
pub fn windowing_system() -> String {
    wish::ask_wish("tk windowingsystem")
}
