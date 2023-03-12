#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unreachable_patterns)]
#![allow(unreachable_code)]
#![allow(unused_mut)]

pub use crate::colors::*;

mod colors;


#[derive(Clone)]
pub enum RsgObjType {
    Text,
    Button,
    CheckBox,
    Radio,
    Input,
    Slider,
    Separator
}

#[derive(Clone)]
pub enum RsgOrientation {
    Horizontal,
    Vertical
}

impl RsgOrientation {
    pub fn to_string(&self) -> String {
        match &self {
            RsgOrientation::Horizontal => return String::from("horizontal"),
            RsgOrientation::Vertical   => return String::from("vertical"),
        }
    }
    pub fn to_enum(input: &String) -> RsgOrientation {
        match input.as_str() {
            "horizontal" => return RsgOrientation::Horizontal,
            "vertical" => return RsgOrientation::Vertical,
            _ => panic!("ERR: Somehow, RsgOrientation.to_enum() got an invalid string as its input...")
        }
    }
}

#[derive(Clone)]
pub struct RsgObj {
    pub r#type: RsgObjType,
    pub name: String,
    pub size: (u64, u64),
    pub color: (RsgColor, RsgColor),
    pub pad: (u64, u64),
    pub range: (i64, u64)

}


#[derive(Clone)]
#[derive(Copy)]
pub struct RsgObjEx {
    pub size: (u64, u64),
    pub color: (RsgColor, RsgColor),
    pub pad: (u64, u64),
    pub range: (i64, u64)
}

impl Default for RsgObjEx {
    fn default() -> RsgObjEx {
        return RsgObjEx{
            size: (0, 0),
            color: (RsgColor::None, RsgColor::None),
            pad: (10, 4),
            range: (0, 100)
        }
    }
}