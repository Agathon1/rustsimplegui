#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unreachable_patterns)]
#![allow(unreachable_code)]
#![allow(unused_mut)]




use rstk::{self, TkLabelOptions, TkGridLayout, TkWidget};

use std::collections::HashMap;

pub use rsg_core::*;


pub fn text<T>(text_name: T) -> RsgObj where String: From<T> {
    return RsgObj{
        r#type: RsgObjType::Text,
        name: String::from(text_name),
        size: (0, 0),
        color: (RsgColor::None, RsgColor::None),
        pad: (10, 4),
        range: (0, 0)
    }
}
pub fn text_ex<T, U>(text_name: T, text_ex: U) -> RsgObj where String: From<T>, RsgObjEx: From<U>, U: Copy {
    return RsgObj{
        r#type: RsgObjType::Text,
        name: String::from(text_name),
        size: RsgObjEx::from(text_ex).size,
        color: RsgObjEx::from(text_ex).color,
        pad: RsgObjEx::from(text_ex).pad,
        range: (0, 0)
    }
}


pub fn button<T>(button_name: T) -> RsgObj where String: From<T> {
    return RsgObj{
        r#type: RsgObjType::Button,
        name: String::from(button_name),
        size: (0, 0),
        color: (RsgColor::None, RsgColor::None),
        pad: (10, 4),
        range: (0, 0)
    }
}
pub fn button_ex<T, U>(button_name: T, button_ex: U) -> RsgObj where String: From<T>, RsgObjEx: From<U>, U: Copy {
    return RsgObj{
        r#type: RsgObjType::Button,
        name: String::from(button_name),
        size: RsgObjEx::from(button_ex).size,
        color: RsgObjEx::from(button_ex).color,
        pad: RsgObjEx::from(button_ex).pad,
        range: (0, 0)
    }
}


pub fn checkbox<T>(checkbox_name: T) -> RsgObj where String: From<T> {
    return RsgObj{
        r#type: RsgObjType::CheckBox,
        name: String::from(checkbox_name),
        size: (0, 0),
        color: (RsgColor::None, RsgColor::None),
        pad: (10, 4),
        range: (0, 0)
    }
}


pub fn checkbox_ex<T, U>(checkbox_name: T, checkbox_ex: U) -> RsgObj where String: From<T>, RsgObjEx: From<U>, U: Copy {
    return RsgObj{
        r#type: RsgObjType::CheckBox,
        name: String::from(checkbox_name),
        size: RsgObjEx::from(checkbox_ex).size,
        color: RsgObjEx::from(checkbox_ex).color,
        pad: RsgObjEx::from(checkbox_ex).pad,
        range: (0, 0)
    }
}


pub fn radio<T>(radio_name: T) -> RsgObj where String: From<T> {
    return RsgObj{
        r#type: RsgObjType::Radio,
        name: String::from(radio_name),
        size: (0, 0),
        color: (RsgColor::None, RsgColor::None),
        pad: (10, 4),
        range: (0, 0)
    }
}
pub fn radio_ex<T, U>(radio_name: T, radio_ex: U) -> RsgObj where String: From<T>, RsgObjEx: From<U>, U: Copy {
    return RsgObj{
        r#type: RsgObjType::Radio,
        name: String::from(radio_name),
        size: RsgObjEx::from(radio_ex).size,
        color: RsgObjEx::from(radio_ex).color,
        pad: RsgObjEx::from(radio_ex).pad,
        range: RsgObjEx::from(radio_ex).range
    }
}


pub fn input() -> RsgObj {
    return RsgObj{
        r#type: RsgObjType::Input,
        name: String::from(""),
        size: (0, 0),
        color: (RsgColor::None, RsgColor::None),
        pad: (10, 4),
        range: (0, 0)
    }
}
pub fn input_ex<T, U>(input_placeholder: T, input_ex: U) -> RsgObj where String: From<T>, RsgObjEx: From<U>, U: Copy {
    return RsgObj{
        r#type: RsgObjType::Input,
        name: String::from(input_placeholder),
        size: RsgObjEx::from(input_ex).size,
        color: RsgObjEx::from(input_ex).color,
        pad: RsgObjEx::from(input_ex).pad,
        range: (0, 0)
    }
}


pub fn slider() -> RsgObj {
    return RsgObj{
        r#type: RsgObjType::Slider,
        name: RsgOrientation::Horizontal.to_string(),
        size: (0, 0),
        color: (RsgColor::None, RsgColor::None),
        pad: (10, 4),
        range: (0, 100)
    }
}
pub fn slider_ex<T, U>(slider_orientation: T, slider_ex: U) -> RsgObj where RsgOrientation: From<T>, RsgObjEx: From<U>, U: Copy {
    return RsgObj{
        r#type: RsgObjType::Slider,
        name: RsgOrientation::from(slider_orientation).to_string(),
        size: RsgObjEx::from(slider_ex).size,
        color: RsgObjEx::from(slider_ex).color,
        pad: RsgObjEx::from(slider_ex).pad,
        range: RsgObjEx::from(slider_ex).range
    }
}

// TODO ? Split into HSeparator, VSeparator...
pub fn separator() -> RsgObj {
    return RsgObj{
        r#type: RsgObjType::Separator,
        name: RsgOrientation::Horizontal.to_string(),
        size: (0, 0),
        color: (RsgColor::None, RsgColor::None),
        pad: (10, 4),
        range: (0, 0)
    }
}
pub fn separator_ex<T, U>(separator_orientaiton: T, separator_ex: U) -> RsgObj where RsgOrientation: From<T>, RsgObjEx: From<U>, U: Copy {
    return RsgObj{
        r#type: RsgObjType::Separator,
        name: RsgOrientation::from(separator_orientaiton).to_string(),
        size: RsgObjEx::from(separator_ex).size,
        color: RsgObjEx::from(separator_ex).color,
        pad: RsgObjEx::from(separator_ex).pad,
        range: RsgObjEx::from(separator_ex).range
    }
}



pub struct Window{
    widget_ids_to_names: HashMap<String, String>,
    inputs: Vec<String>,
    name: String,
    layout: Vec<Vec<RsgObj>>,
    root: rstk::TkTopLevel,
    sliders: Vec<String>
}


pub fn window<T, U>(window_name: T, layout: U) -> Window where String: From<T>, Vec<Vec<RsgObj>>: From<U> {
    let mut new = Window{
        widget_ids_to_names: HashMap::new(),
        inputs: Vec::new(),
        name: String::from(window_name),
        layout: layout.into(),
        root: rstk::start_wish().unwrap(),
        sliders: vec![]
    };

    for i in 0..new.layout.len() {
        for j in 0..new.layout[i].len() {
            let x = &new.layout[i][j];

            match x.r#type {
                RsgObjType::Text => {
                    let n = rstk::make_label(&new.root);
                    n.text(&x.name);

                    if x.size.0 != 0 && x.size.1 != 0 {
                        n.font(&rstk::TkFont{
                            size: ((x.size.0 + x.size.1) / 2) as u64,
                            ..Default::default()
                        });
                    }

                    n.grid()
                    .row(i as u64).column(j as u64)
                    .padx(x.pad.0 as u64).pady(x.pad.1 as u64)
                    .layout();

                    if let RsgColor::None = x.color.0 {
                        if let RsgColor::None = x.color.1 {
                        } else { rstk::tell_wish(&format!("{} configure -activebackground {}",n.id(), n.cget("fg"))) }
                    } else {
                        rstk::tell_wish(&format!("{} configure -fg {}", n.id(), get_rsg_color(x.color.0)));
                        rstk::tell_wish(&format!("{} configure -activebackground {}", n.id(), get_rsg_color(x.color.0)))
                    }

                    if let RsgColor::None = x.color.1 {
                        if let RsgColor::None = x.color.0 {
                        } else { rstk::tell_wish(&format!("{} configure -activeforeground {}", n.id(), n.cget("bg"))) }
                    } else {
                        rstk::tell_wish(&format!("{} configure -bg {}",n.id(), get_rsg_color(x.color.1)));
                        rstk::tell_wish(&format!("{} configure -activeforeground {}",n.id(), get_rsg_color(x.color.1)))
                    }
                }
                RsgObjType::Button => {
                    let n = rstk::make_button(&new.root);
                    n.text(&x.name);

                    if x.size.0 != 0 { n.width(x.size.0 as i64); }
                    if x.size.1 != 0 { n.height(x.size.1 as i64); }

                    n.grid()
                    .row(i as u64).column(j as u64)
                    .padx(x.pad.0 as u64).pady(x.pad.1 as u64)
                    .layout();


                    if let RsgColor::None = x.color.0 {
                        if let RsgColor::None = x.color.1 {
                        } else { rstk::tell_wish(&format!("{} configure -activebackground {}",n.id(), n.cget("fg"))) }
                    } else {
                        rstk::tell_wish(&format!("{} configure -fg {}", n.id(), get_rsg_color(x.color.0)));
                        rstk::tell_wish(&format!("{} configure -activebackground {}", n.id(), get_rsg_color(x.color.0)))
                    }

                    if let RsgColor::None = x.color.1 {
                        if let RsgColor::None = x.color.0 {
                        } else { rstk::tell_wish(&format!("{} configure -activeforeground {}", n.id(), n.cget("bg"))) }
                    } else {
                        rstk::tell_wish(&format!("{} configure -bg {}",n.id(), get_rsg_color(x.color.1)));
                        rstk::tell_wish(&format!("{} configure -activeforeground {}",n.id(), get_rsg_color(x.color.1)))
                    }

                    n.command(||{});
                    new.widget_ids_to_names
                    .entry(n.id().to_string()).or_insert(x.name.clone());
                }
                RsgObjType::CheckBox => {
                    let n = rstk::make_check_button(&new.root);
                    n.text(&x.name);

                    if x.size.0 != 0 { n.width(x.size.0 as i64); }
                    if x.size.1 != 0 { n.height(x.size.1 as i64); }

                    n.grid()
                    .row(i as u64).column(j as u64)
                    .padx(x.pad.0 as u64).pady(x.pad.1 as u64)
                    .layout();

                    if let RsgColor::None = x.color.0 {
                        if let RsgColor::None = x.color.1 {
                        } else { rstk::tell_wish(&format!("{} configure -activebackground {}",n.id(), n.cget("fg"))) }
                    } else {
                        rstk::tell_wish(&format!("{} configure -fg {}", n.id(), get_rsg_color(x.color.0)));
                        rstk::tell_wish(&format!("{} configure -activebackground {}", n.id(), get_rsg_color(x.color.0)))
                    }

                    if let RsgColor::None = x.color.1 {
                        if let RsgColor::None = x.color.0 {
                        } else { rstk::tell_wish(&format!("{} configure -activeforeground {}", n.id(), n.cget("bg"))) }
                    } else {
                        rstk::tell_wish(&format!("{} configure -bg {}",n.id(), get_rsg_color(x.color.1)));
                        rstk::tell_wish(&format!("{} configure -activeforeground {}",n.id(), get_rsg_color(x.color.1)))
                    }

                    n.command(|_|{});
                    new.widget_ids_to_names
                    .entry(n.id().to_string()).or_insert(x.name.clone());
                }
                RsgObjType::Radio => {
                    let mut group: (u64, u64) = (0, 0);

                    if x.pad.0 != 0 { group.0 = x.pad.0 }
                    if x.pad.1 != 0 { group.0 = x.pad.0 }
                    if x.pad.0 == 0 && x.pad.1 == 0 { group = (0, i as u64) }

                    let n = rstk::make_radio_button(&new.root, &format!("{}x{}", group.0, group.1), &x.name);
                    n.text(&x.name);

                    if x.size.0 != 0 { n.width(x.size.0 as i64); };
                    if x.size.1 != 0 { n.width(x.size.0 as i64); };

                    n.grid()
                    .row(i as u64).column(j as u64)
                    .padx(x.pad.0 as u64).pady(x.pad.1 as u64)
                    .layout();

                    if let RsgColor::None = x.color.0 {
                        if let RsgColor::None = x.color.1 {
                        } else { rstk::tell_wish(&format!("{} configure -activebackground {}",n.id(), n.cget("fg"))) }
                    } else {
                        rstk::tell_wish(&format!("{} configure -fg {}", n.id(), get_rsg_color(x.color.0)));
                        rstk::tell_wish(&format!("{} configure -activebackground {}", n.id(), get_rsg_color(x.color.0)))
                    }

                    if let RsgColor::None = x.color.1 {
                        if let RsgColor::None = x.color.0 {
                        } else { rstk::tell_wish(&format!("{} configure -activeforeground {}", n.id(), n.cget("bg"))) }
                    } else {
                        rstk::tell_wish(&format!("{} configure -bg {}",n.id(), get_rsg_color(x.color.1)));
                        rstk::tell_wish(&format!("{} configure -activeforeground {}",n.id(), get_rsg_color(x.color.1)))
                    }

                    n.command(|_|{});
                    new.widget_ids_to_names
                    .entry(n.id().to_string()).or_insert(x.name.clone());
                }
                RsgObjType::Input => {
                    let n = rstk::make_text(&new.root);
                    n.insert((0, 0), &x.name);

                    let new_name = x.name.clone();
                    if new_name != "".to_string() { n.insert((0, 0), &new_name); }

                    if x.size.0 == 0 { n.width(10); }
                    else { n.width(x.size.0 as u64); }
                    if x.size.1 == 0 { n.height(1); }
                    else { n.height(x.size.1 as u64); }

                    new.inputs.push(n.id().to_string());

                    n.grid()
                    .row(i as u64).column(j as u64)
                    .padx(x.pad.0 as u64).pady(x.pad.1 as u64)
                    .layout();

                    if let RsgColor::None = x.color.0 {
                        if let RsgColor::None = x.color.1 {
                        } else { rstk::tell_wish(&format!("{} configure -activebackground {}",n.id(), n.cget("bg"))) }
                    } else {
                        rstk::tell_wish(&format!("{} configure -fg {}", n.id(), get_rsg_color(x.color.0)));
                        rstk::tell_wish(&format!("{} configure -activebackground {}", n.id(), get_rsg_color(x.color.0)))
                    }

                    if let RsgColor::None = x.color.1 {
                        if let RsgColor::None = x.color.0 {
                        } else { rstk::tell_wish(&format!("{} configure -activeforeground {}", n.id(), n.cget("fg"))) }
                    } else {
                        rstk::tell_wish(&format!("{} configure -bg {}",n.id(), get_rsg_color(x.color.1)));
                        rstk::tell_wish(&format!("{} configure -activeforeground {}",n.id(), get_rsg_color(x.color.1)))
                    }
                }
                RsgObjType::Slider => {
                    let rsg_orientation = RsgOrientation::to_enum(&x.name);
                    let mut rstk_orientation: rstk::Orientation;
                    if let RsgOrientation::Horizontal = rsg_orientation {
                        rstk_orientation = rstk::Orientation::Horizontal;
                    } else { rstk_orientation = rstk::Orientation::Vertical; }
                    
                    let n = rstk::make_scale(&new.root, rstk_orientation);


                    rstk::tell_wish(&format!("{} configure -from {} -to {}", n.id(), x.range.0, x.range.1));

                    //if x.size.0 != 0 { n.width(x.size.0); }
                    //if x.size.1 != 0 { n.height(x.size.1); }

                    n.grid()
                    .row(i as u64).column(j as u64)
                    .padx(x.pad.0).pady(x.pad.1)
                    .layout();

                    if let RsgColor::None = x.color.0 {
                        if let RsgColor::None = x.color.1 {
                        } else { rstk::tell_wish(&format!("{} configure -activebackground {}",n.id(), n.cget("bg"))) }
                    } else {
                        rstk::tell_wish(&format!("{} configure -fg {}", n.id(), get_rsg_color(x.color.0)));
                        rstk::tell_wish(&format!("{} configure -activebackground {}", n.id(), get_rsg_color(x.color.0)))
                    }

                    if let RsgColor::None = x.color.1 {
                        if let RsgColor::None = x.color.0 {
                        } else { rstk::tell_wish(&format!("{} configure -activeforeground {}", n.id(), n.cget("fg"))) }
                    } else {
                        rstk::tell_wish(&format!("{} configure -bg {}",n.id(), get_rsg_color(x.color.1)));
                        rstk::tell_wish(&format!("{} configure -activeforeground {}",n.id(), get_rsg_color(x.color.1)))
                    }

                    new.sliders.push(n.id().to_string());
                }
                RsgObjType::Separator => {
                    let rsg_orientation = RsgOrientation::to_enum(&x.name);
                    let rstk_orientation: rstk::Orientation;
                    if let RsgOrientation::Horizontal = rsg_orientation {
                        rstk_orientation = rstk::Orientation::Horizontal;
                    } else { rstk_orientation = rstk::Orientation::Vertical; }

                    let n = rstk::make_separator(&new.root, rstk_orientation);

                    n.grid()
                    .row(i as u64).column(j as u64)
                    .padx(x.pad.0).pady(x.pad.1)
                    .layout();

                    if let RsgColor::None = x.color.0 {
                        if let RsgColor::None = x.color.1 {
                        } else { rstk::tell_wish(&format!("{} configure -activebackground {}",n.id(), n.cget("bg"))) }
                    } else {
                        rstk::tell_wish(&format!("{} configure -fg {}", n.id(), get_rsg_color(x.color.0)));
                        rstk::tell_wish(&format!("{} configure -activebackground {}", n.id(), get_rsg_color(x.color.0)))
                    }

                    if let RsgColor::None = x.color.1 {
                        if let RsgColor::None = x.color.0 {
                        } else { rstk::tell_wish(&format!("{} configure -activeforeground {}", n.id(), n.cget("fg"))) }
                    } else {
                        rstk::tell_wish(&format!("{} configure -bg {}",n.id(), get_rsg_color(x.color.1)));
                        rstk::tell_wish(&format!("{} configure -activeforeground {}",n.id(), get_rsg_color(x.color.1)))
                    }
                }
                _ => {}
            }
        }
    }
    return new;
}

impl Window {
    pub fn read(&self) -> (String, Vec<String>) {
        let event = rstk::mainloop().unwrap_or(String::from(""));

        if Some(event.clone()).is_some() {

            let or = String::from("None");

            //println!("{}", event);

            let ev: String; 

            if event.contains("-cbsep-") {
                let parts: Vec<&str> = event.split("-cbsep-").collect();
                let widget = self.widget_ids_to_names.get(parts[0].trim()).unwrap_or(&or);
                let value = parts[1].trim();
                ev = widget.to_owned() + ":::" + value;
            } else {
                ev = self.widget_ids_to_names.get(&event).unwrap_or(&or).clone();
            }

            let mut ret_values: Vec<String> = Vec::new();

            for each in &self.inputs {
                let x = rstk::ask_wish(&format!(
                    "puts [{} get {}.{} end] ; flush stdout",
                    each, 0, 0
                ));
                ret_values.push(x);            
            }
            for each in &self.sliders {
                let x = rstk::ask_wish(&format!(
                        "puts [{} get] ; flush stdout",
                        each
                ));
                ret_values.push(x.to_string());
            }
            return (ev.to_string(), ret_values);
        } else {
            return ("".to_string(), vec!["".to_string()])
        }
    }

    pub fn close(&self) {
        rstk::end_wish()
    }
}