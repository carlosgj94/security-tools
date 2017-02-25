extern crate gdk;
extern crate gtk;

use gtk::prelude::*;
use gtk::{Label, Box, Orientation, InfoBar, MessageType};

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn get_box() -> Box {
    let v_box = Box::new(Orientation::Vertical, 10);
    let h_box = Box::new(Orientation::Horizontal, 10);

    let file_strings = get_hosts_file();
    let label_box = get_items_box(file_strings);

    //InfoBar
    let info_bar = InfoBar::new();
    info_bar.add_button("Unblock", 1);
    info_bar.set_message_type(MessageType::Warning);
    let content_info = info_bar.get_content_area().unwrap();
    content_info.activate();
    content_info.add_mnemonic_label(&Label::new("Do you want to unblock the app?"));

    v_box.pack_start(&info_bar, false, false, 0);
    h_box.pack_start(&label_box, false, false, 30);
    v_box.pack_start(&h_box, false, false, 0);
    
    return v_box;
}

fn get_hosts_file() -> Vec<String> {
    let file = match File::open("/etc/hosts"){
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", "/etc/hosts",
                                                   why.description()),
        Ok(file) => file,
    };
    let f = BufReader::new(file);
    let mut v = Vec::new();
    for line in f.lines() {
            let l = line.unwrap();
            v.push(l);
    }

    return v;
}

fn get_items_box(lines: Vec<String>) -> Box {
    
    let v_box = Box::new(Orientation::Vertical, 10);
    
    for item in lines {
        //let i: &str = &item;
        //let label = Label::new(Some(i));
        v_box.pack_start(&get_item_box(item), true, false, 0);
    }
    return v_box;
}

fn get_item_box(line :String) -> Box {
    let h_box = Box::new(Orientation::Horizontal, 10);
    let line_str: &str = &line;
    let label = Label::new(Some(line_str));
    h_box.pack_start(&label, false, false, 0);
    
    return h_box;
}
