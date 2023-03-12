#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unreachable_patterns)]
#![allow(unreachable_code)]
#![allow(unused_mut)]


#[cfg(all(feature = "rsg_tk", feature = "rsg_relm4"))]
compile_error!("You can only use ONE backend with rustsimplegui!");

use std::collections::HashMap;

#[cfg(feature = "rsg_tk")]
use rsg_tk::*;
use rsg_tk::window as _window;
use rsg_tk::text as _text;
use rsg_tk::text_ex as _text_ex;
use rsg_tk::button as _button;
use rsg_tk::button_ex as _button_ex;
use rsg_tk::checkbox as _checkbox;
use rsg_tk::checkbox_ex as _checkbox_ex;
use rsg_tk::radio as _radio;
use rsg_tk::radio_ex as _radio_ex;
use rsg_tk::input as _input;
use rsg_tk::input_ex as _input_ex;
use rsg_tk::slider as _slider;
use rsg_tk::slider_ex as _slider_ex;
use rsg_tk::separator as _separator;
use rsg_tk::separator_ex as _separator_ex;
pub use rsg_tk::RsgColor as RsgColor;
pub use rsg_tk::RsgObjEx as RsgObjEx;
pub use rsg_tk::RsgOrientation as RsgOrientation;



pub fn text<T>(text_name: T) -> RsgObj where String: From<T> {
    return _text(text_name);
}
pub fn text_ex<T, U>(text_name: T, text_ex: U) -> RsgObj where String: From<T>, RsgObjEx: From<U>, U: Copy {
    return _text_ex(text_name, text_ex);
}


pub fn button<T>(button_name: T) -> RsgObj where String: From<T> {
    return _button(button_name);
}
pub fn button_ex<T, U>(button_name: T, button_ex: U) -> RsgObj where String: From<T>, RsgObjEx: From<U>, U: Copy {
    return _button_ex(button_name, button_ex);
}


pub fn checkbox<T>(checkbox_name: T) -> RsgObj where String: From<T> {
    return _checkbox(checkbox_name);
}
pub fn checkbox_ex<T, U>(checkbox_name: T, checkbox_ex: U) -> RsgObj where String: From<T>, RsgObjEx: From<U>, U: Copy {
    return _checkbox_ex(checkbox_name, checkbox_ex);
}


pub fn radio<T>(radio_name: T) -> RsgObj where String: From<T> {
    return _radio(radio_name);
}
pub fn radio_ex<T, U>(radio_name: T, radio_ex: U) -> RsgObj where String: From<T>, RsgObjEx: From<U>, U: Copy {
    return _radio_ex(radio_name, radio_ex);
}


pub fn input() -> RsgObj {
    return _input();
}
pub fn input_ex<T, U>(input_placeholder: T, input_ex: U) -> RsgObj where String: From<T>, RsgObjEx: From<U>, U: Copy {
    return _input_ex(input_placeholder, input_ex);
}


pub fn slider() -> RsgObj {
    return _slider();
}
pub fn slider_ex<T, U>(slider_orientation: T, slider_ex: U) -> RsgObj where RsgOrientation: From<T>, RsgObjEx: From<U>, U: Copy, {
    return _slider_ex(slider_orientation, slider_ex);
}


pub fn separator() -> RsgObj {
    return _separator();
}
pub fn separator_ex<T, U>(separator_orientaiton: T, separator_ex: U) -> RsgObj where RsgOrientation: From<T>, RsgObjEx: From<U>, U: Copy {
    return _separator_ex(separator_orientaiton, separator_ex);
}


pub fn window<T, U>(window_name: T, layout: U) -> Window where String: From<T>, Vec<Vec<RsgObj>>: From<U> {
    return _window(window_name, layout);
}