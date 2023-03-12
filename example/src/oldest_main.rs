#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use rustsimplegui as rsg;

fn main() {
    let mut layout : Vec<Vec<rsg::RsgObj>> = vec![];

    let mut row1 : Vec<rsg::RsgObj> = vec![];
    let mut row2 : Vec<rsg::RsgObj> = vec![];

    for i in 0..10 {
        //row1.push(rsg::text_ex("Hello!", rsg::RsgObjEx{
        //    size: 4
        //}));
        //row2.push(rsg::button_ex(format!("Button [{}]", i), rsg::RsgObjEx{
        //    size: 4
        //}));
        row1.push(rsg::text("Hello!"));
        row2.push(rsg::button(format!("Button [{}]", i)))
    }

    layout.push(row1);
    layout.push(row2);



    rsg::window(String::from("Test Windw"), layout);

    loop {
        let event = rsg::window_read();
        if event == "Quit" {
            break;
        }
        println!("{}", event);
    }
}
