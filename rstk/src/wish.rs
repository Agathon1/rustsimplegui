//! Core functions and data structures for interacting with the wish process.
//!
//! The basic structure of a program using wish is as follows:
//!
//! ```
//! fn main() {
//!   let root = rstk::start_wish().unwrap();
//!
//!   // -- add code here to create program
//!
//!   rstk::mainloop();
//! }
//! ```
//!
//! The call to `start_wish` starts the "wish" program and sets up some
//! internal structure to store information about your program's interaction
//! with wish. The return value is a `Result`, so must be unwrapped (or 
//! otherwise handled) to obtain the top-level window.
//!
//! If you are using a different program to "wish", e.g. a tclkit, then
//! call instead:
//!
//! ```
//!   let root = rstk::start_with("tclkit").unwrap();
//! ```
//!
//! All construction of the GUI must be done after starting a wish process.
//!
//! (For debugging purposes, [trace_with] additionally displays all 
//! messages to/from the wish program on stdout.)
//!
//! Tk is event-driven, so the code sets up the content and design
//! of various widgets and associates commands to particular events: events
//! can be button-clicks or the movement of a mouse onto a canvas.
//!
//! Once the GUI is created, then the [mainloop] must be started, which will
//! process and react to events: the call to `mainloop` is usually the last
//! statement in the program.
//!
//! The program will usually exit when the top-level window is closed. However,
//! that can be over-ridden or, to exit in another way, use [end_wish].
//!
//! ## Low-level API
//!
//! The modules in this crate aim to provide a rust-friendly, type-checked set 
//! of structs and methods for using the Tk library.
//!
//! However, there are many features in Tk and not all of them are likely to be
//! wrapped. If there is a feature missing they may be used by directly calling 
//! Tk commands through the low-level API.
//!
//! 1. every widget has an `id` field, which gives the Tk identifier.
//! 2. [tell_wish] sends a given string directly to wish
//! 3. [ask_wish] sends a given string directly to wish and
//!    returns, as a [String], the response.
//!
//! For example, label's
//! [takefocus](https://www.tcl-lang.org/man/tcl8.6/TkCmd/ttk_widget.htm#M-takefocus)
//! flag is not wrapped. You can nevertheless set its value using:
//!
//! ```
//! let label = rstk::make_label(&root);
//!
//! rstk::tell_wish(&format!("{} configure -takefocus 0", &label.id));
//! ```
//!
//! Also useful are:
//!
//! * [cget](widget::TkWidget::cget) - queries any option and returns its current value
//! * [configure](widget::TkWidget::configure) - used to set any option to a value
//! * [winfo](widget::TkWidget::winfo) - returns window-related information
//!
//! ## Extensions
//!
//! Extensions can be created with the help of [next_wid],
//! which returns a new, unique ID in Tk format. Writing an extension requires:
//!
//! 1. importing the tcl/tk library (using `tell_wish`)
//! 2. creating an instance of the underlying Tk widget using a unique id
//! 3. retaining that id in a struct, for later reference
//! 4. wrapping the widget's functions as methods, calling out to Tk with
//!    the stored id as a reference.
//!


use std::collections::HashMap;
use std::io::{Read, Write};
use std::process;
use std::sync::mpsc;
use std::sync::Mutex;
use std::thread;

use super::font;
use super::toplevel;
use super::widget;

// TODO - change when available from 'nightly'
use once_cell::sync::Lazy;
use once_cell::sync::OnceCell;

/// Reports an error in interacting with the Tk program.
#[derive(Debug)]
pub struct TkError {
    message: String,
}

static TRACE_WISH: OnceCell<bool> = OnceCell::new();
fn tracing() -> bool {
    *TRACE_WISH.get().unwrap_or(&false)
}

static mut WISH: OnceCell<process::Child> = OnceCell::new();
static mut OUTPUT: OnceCell<process::ChildStdout> = OnceCell::new();
static mut SENDER: OnceCell<mpsc::Sender<String>> = OnceCell::new();

// Kills the wish process - should be called to exit
pub(super) fn kill_wish() {
    unsafe {
        WISH.get_mut()
            .unwrap()
            .kill()
            .expect("Wish was unexpectedly already finished");
    }
}

/// Sends a message (tcl command) to wish.
///
/// Use with caution: the message must be valid tcl.
///
pub fn tell_wish(msg: &str) {
    if tracing() {
        println!("wish: {}", msg);
    }
    unsafe {
        SENDER.get_mut().unwrap().send(String::from(msg)).unwrap();
        SENDER.get_mut().unwrap().send(String::from("\n")).unwrap();
    }
}

/// Sends a message (tcl command) to wish and expects a result.
/// Returns a result as a string
///
/// Use with caution: the message must be valid tcl.
///
pub fn ask_wish(msg: &str) -> String {
    tell_wish(msg);

    unsafe {
        let mut input = [32; 10000]; // TODO - long inputs can get split?
        if OUTPUT.get_mut().unwrap().read(&mut input).is_ok() {
            if let Ok(input) = String::from_utf8(input.to_vec()) {
                if tracing() {
                    println!("---: {:?}", &input.trim());
                }
                return input.trim().to_string();
            }
        }
    }

    panic!("Eval-wish failed to get a result");
}

// -- Counter for making new ids

static NEXT_ID: Lazy<Mutex<i64>> = Lazy::new(|| Mutex::new(0));

/// Returns a new id string which can be used to name a new
/// widget instance. The new id will be in reference to the
/// parent, as is usual in Tk.
///
/// This is only for use when writing an extension library.
///
pub fn next_wid(parent: &str) -> String {
    let mut nid = NEXT_ID.lock().unwrap();
    *nid += 1;
    if parent == "." {
        format!(".r{}", nid)
    } else {
        format!("{}.r{}", parent, nid)
    }
}

/// Returns a new variable name. This is used in the chart
/// module to reference the chart instances in Tk.
///
/// This is only for use when writing an extension library.
///
pub fn next_var() -> String {
    let mut nid = NEXT_ID.lock().unwrap();
    *nid += 1;
    format!("::var{}", nid)
}

pub(super) fn current_id() -> i64 {
    let nid = NEXT_ID.lock().unwrap();
    *nid
}

// -- Store for callback functions, such as on button clicks

type Callback0 = Box<(dyn Fn() + Send + 'static)>;
pub(super) fn mk_callback0<F>(f: F) -> Callback0
where
    F: Fn() + Send + 'static,
{
    Box::new(f) as Callback0
}

static CALLBACKS0: Lazy<Mutex<HashMap<String, Callback0>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub(super) fn add_callback0(wid: &str, callback: Callback0) {
    CALLBACKS0
        .lock()
        .unwrap()
        .insert(String::from(wid), callback);
}

fn get_callback0(wid: &str) -> Option<Callback0> {
    if let Some((_, command)) = CALLBACKS0.lock().unwrap().remove_entry(wid) {
        Some(command)
    } else {
        None
    }
}

fn eval_callback0(wid: &str) {
    if let Some(command) = get_callback0(wid) {
        command();
        if !wid.contains("after") && // after commands apply once only
            !CALLBACKS0.lock().unwrap().contains_key(wid) // do not overwrite if a replacement command added
            {
            add_callback0(wid, command);
        }
    } // TODO - error?
}

type Callback1Bool = Box<(dyn Fn(bool) + Send + 'static)>;
pub(super) fn mk_callback1_bool<F>(f: F) -> Callback1Bool
where
    F: Fn(bool) + Send + 'static,
{
    Box::new(f) as Callback1Bool
}

static CALLBACKS1BOOL: Lazy<Mutex<HashMap<String, Callback1Bool>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub(super) fn add_callback1_bool(wid: &str, callback: Callback1Bool) {
    CALLBACKS1BOOL
        .lock()
        .unwrap()
        .insert(String::from(wid), callback);
}

fn get_callback1_bool(wid: &str) -> Option<Callback1Bool> {
    if let Some((_, command)) = CALLBACKS1BOOL.lock().unwrap().remove_entry(wid) {
        Some(command)
    } else {
        None
    }
}

fn eval_callback1_bool(wid: &str, value: bool) {
    if let Some(command) = get_callback1_bool(wid) {
        command(value);
        if !CALLBACKS1BOOL.lock().unwrap().contains_key(wid) {
            add_callback1_bool(wid, command);
        }
    } // TODO - error?
}

type Callback1Event = Box<(dyn Fn(widget::TkEvent) + Send + 'static)>;
pub(super) fn mk_callback1_event<F>(f: F) -> Callback1Event
where
    F: Fn(widget::TkEvent) + Send + 'static,
{
    Box::new(f) as Callback1Event
}

// for bound events, key is widgetid/all + pattern, as multiple events can be
// bound to same entity
static CALLBACKS1EVENT: Lazy<Mutex<HashMap<String, Callback1Event>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub(super) fn add_callback1_event(wid: &str, callback: Callback1Event) {
    CALLBACKS1EVENT
        .lock()
        .unwrap()
        .insert(String::from(wid), callback);
}

fn get_callback1_event(wid: &str) -> Option<Callback1Event> {
    if let Some((_, command)) = CALLBACKS1EVENT.lock().unwrap().remove_entry(wid) {
        Some(command)
    } else {
        None
    }
}

fn eval_callback1_event(wid: &str, value: widget::TkEvent) {
    if let Some(command) = get_callback1_event(wid) {
        command(value);
        if !CALLBACKS1EVENT.lock().unwrap().contains_key(wid) {
            add_callback1_event(wid, command);
        }
    } // TODO - error?
}

type Callback1Float = Box<(dyn Fn(f64) + Send + 'static)>;
pub(super) fn mk_callback1_float<F>(f: F) -> Callback1Float
where
    F: Fn(f64) + Send + 'static,
{
    Box::new(f) as Callback1Float
}

static CALLBACKS1FLOAT: Lazy<Mutex<HashMap<String, Callback1Float>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub(super) fn add_callback1_float(wid: &str, callback: Callback1Float) {
    CALLBACKS1FLOAT
        .lock()
        .unwrap()
        .insert(String::from(wid), callback);
}

fn get_callback1_float(wid: &str) -> Option<Callback1Float> {
    if let Some((_, command)) = CALLBACKS1FLOAT.lock().unwrap().remove_entry(wid) {
        Some(command)
    } else {
        None
    }
}

fn eval_callback1_float(wid: &str, value: f64) {
    if let Some(command) = get_callback1_float(wid) {
        command(value);
        if !CALLBACKS1FLOAT.lock().unwrap().contains_key(wid) {
            add_callback1_float(wid, command);
        }
    } // TODO - error?
}

type Callback1Font = Box<(dyn Fn(font::TkFont) + Send + 'static)>;
pub(super) fn mk_callback1_font<F>(f: F) -> Callback1Font
where
    F: Fn(font::TkFont) + Send + 'static,
{
    Box::new(f) as Callback1Font
}

static CALLBACKS1FONT: Lazy<Mutex<HashMap<String, Callback1Font>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub(super) fn add_callback1_font(wid: &str, callback: Callback1Font) {
    CALLBACKS1FONT
        .lock()
        .unwrap()
        .insert(String::from(wid), callback);
}

fn get_callback1_font(wid: &str) -> Option<Callback1Font> {
    if let Some((_, command)) = CALLBACKS1FONT.lock().unwrap().remove_entry(wid) {
        Some(command)
    } else {
        None
    }
}

fn eval_callback1_font(wid: &str, value: font::TkFont) {
    if let Some(command) = get_callback1_font(wid) {
        command(value);
        if !CALLBACKS1FONT.lock().unwrap().contains_key(wid) {
            add_callback1_font(wid, command);
        }
    } // TODO - error?
}

use crate::TkText;

static mut TEST: once_cell::sync::Lazy<String> = Lazy::<String>::new(|| String::from(""));

pub fn testerr(thing: TkText) {
    let x = thing.get_to_end((0, 0));
    unsafe {
        *TEST = x;
    }
}

/// Loops while GUI events occur
pub fn mainloop() -> Option<String> {
    unsafe {
            let mut input = [32; 10000];
            if OUTPUT.get_mut().unwrap().read(&mut input).is_ok() {
                if let Ok(input) = String::from_utf8(input.to_vec()) {
    
                    if input.starts_with("clicked") {
                        if let Some(n) = input.find('\n') {
                            let widget = &input[8..n];
                            return Some(widget.to_string());
                        }
                        return None;
                    } else if input.starts_with("cb1b") {
                        let parts: Vec<&str> = input.split("-").collect();
                        let widget = parts[1].trim();
                        let value = parts[2].trim();
                        return Some(widget.to_owned() + &format!("-cbsep-{}", value == "1"));
                    } else if input.starts_with("cb1") {
                        let parts: Vec<&str> = input.split("-").collect();
                        let widget = parts[1].trim();
                        return Some(widget.to_owned());
                    } else if input.starts_with("exit") {
                        kill_wish();
                        return Some("Quit".to_string())
                    }
                    return None; // exit loop and program
                    }
                }
                return None
            }
}

/// Creates a connection with the "wish" program.
pub fn start_wish() -> Result<toplevel::TkTopLevel, TkError> {
    start_with("wish")
}

/// Creates a connection with the given wish/tclkit program.
pub fn start_with(wish: &str) -> Result<toplevel::TkTopLevel, TkError> {
    if let Ok(_) = TRACE_WISH.set(false) {
        start_tk_connection(wish)
    } else {
        return Err(TkError { message: String::from("Failed to set trace option") })
    }
}

/// Creates a connection with the given wish/tclkit program with 
/// debugging output enabled (wish interactions are reported to stdout).
pub fn trace_with(wish: &str) -> Result<toplevel::TkTopLevel, TkError> {
    if let Ok(_) = TRACE_WISH.set(false) {
        start_tk_connection(wish)
    } else {
        return Err(TkError { message: String::from("Failed to set trace option") })
    }
}

/// Creates a connection with the given wish/tclkit program.
fn start_tk_connection(wish: &str)-> Result<toplevel::TkTopLevel, TkError> {

    let err_msg = format!("Do not start {} twice", wish);

    unsafe {
        if let Ok(wish_process) = process::Command::new(wish)
            .stdin(process::Stdio::piped())
                .stdout(process::Stdio::piped())
                .spawn()
                {
                    if WISH.set(wish_process).is_err() {
                        return Err(TkError { message: err_msg });
                    }
                } else {
                    return Err(TkError {
                        message: format!("Failed to start {} process", wish),
                    });
                };

        let mut input = WISH.get_mut().unwrap().stdin.take().unwrap();
        if OUTPUT
            .set(WISH.get_mut().unwrap().stdout.take().unwrap())
                .is_err()
                {
                    return Err(TkError { message: err_msg });
                }

        // -- initial setup of Tcl/Tk environment

        // load the plotchart package - TODO: give some indication if this fails
        input.write_all(b"package require Plotchart\n").unwrap();

        // set close button to output 'exit' message, so rust can close connection
        input
            .write_all(b"wm protocol . WM_DELETE_WINDOW { puts stdout {exit} ; flush stdout } \n")
            .unwrap();
        // remove the 'tearoff' menu option
        input.write_all(b"option add *tearOff 0\n").unwrap();
        // tcl function to help working with font chooser
        input
            .write_all(
                b"proc font_choice {w font args} {
            set res {font }
            append res [font actual $font]
                puts $res
                flush stdout
        }\n",
        )
            .unwrap();
        // tcl function to help working with scale widget
        input
            .write_all(
                b"proc scale_value {w value args} {
            puts cb1f-$w-$value
                flush stdout
        }\n",
        )
            .unwrap();

        let (sender, receiver) = mpsc::channel();
        SENDER.set(sender).expect(&err_msg);

        // create thread to receive strings to send on to wish
        thread::spawn(move || loop {
            let msg: Result<String, mpsc::RecvError> = receiver.recv();
            if let Ok(msg) = msg {
                input.write_all(msg.as_bytes()).unwrap();
                input.write_all(b"\n").unwrap();
            }
        });
    }

    Ok(toplevel::TkTopLevel {
        id: String::from("."),
    })
}

/// Used to cleanly end the wish process and current rust program.
pub fn end_wish() {
    kill_wish();
    process::exit(0);
}

// Splits tcl string where items can be single words or grouped in {..}
pub(super) fn split_items(text: &str) -> Vec<String> {
    let mut result: Vec<String> = vec![];

    let mut remaining = text.trim();
    while !remaining.is_empty() {
        if let Some(start) = remaining.find('{') {
            // -- add any words before first {
            for word in remaining[..start].split_whitespace() {
                result.push(String::from(word));
            }

            if let Some(end) = remaining.find('}') {
                result.push(String::from(&remaining[start + 1..end]));
                remaining = remaining[end + 1..].trim();
            } else {
                // TODO keep what we have
                break; // panic!("Incorrectly formed font family string");
            }
        } else {
            // no { }, so just split all the words and end
            for word in remaining.split_whitespace() {
                result.push(String::from(word));
            }
            break;
        }
        }

        result
    }

#[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn split_items_1() {
            let result = split_items("");
            assert_eq!(0, result.len());
        }

        #[test]
        fn split_items_2() {
            let result = split_items("abc");
            assert_eq!(1, result.len());
            assert_eq!("abc", result[0]);
        }

        #[test]
        fn split_items_3() {
            let result = split_items("  abc  def  ");
            assert_eq!(2, result.len());
            assert_eq!("abc", result[0]);
            assert_eq!("def", result[1]);
        }

        #[test]
        fn split_items_4() {
            let result = split_items("{abc def}");
            assert_eq!(1, result.len());
            assert_eq!("abc def", result[0]);
        }

        #[test]
        fn split_items_5() {
            let result = split_items("{abc def} xy_z {another}");
            assert_eq!(3, result.len());
            assert_eq!("abc def", result[0]);
            assert_eq!("xy_z", result[1]);
            assert_eq!("another", result[2]);
        }
    }
