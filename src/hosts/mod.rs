extern crate gdk;
extern crate gtk;

use gtk::prelude::*;
use gtk::{Label, Box, Orientation, InfoBar, MessageType, Entry};

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
    let save_button = info_bar.add_button("Save", 1).unwrap();
    save_button.connect_clicked(|_| {
             println!("Save");
             //save_hosts();
    });
    //Not working yet
    let reset_button = info_bar.add_button("Reset", 1).unwrap();
    reset_button.connect_clicked(|_| {
             println!("Reset");
             //reset_hosts();
    });
    info_bar.set_message_type(MessageType::Info);

    //Description Box (Horizontal)
    let description_box = Box::new(Orientation::Horizontal, 10);
    let label1 = Label::new(None);
    let label2 = Label::new(None);
    label1.set_markup("<b>Web Pages:</b>");
    label2.set_markup("<b>Redirect to:</b>");
    description_box.pack_start(&label1, false, false, 35);
    description_box.pack_start(&label2, false, false, 55);

    v_box.pack_start(&info_bar, false, false, 0);
    v_box.pack_start(&description_box, false, false, 0);
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
        v_box.pack_start(&get_item_box(item), false, false, 0);
    }
    return v_box;
}

fn get_item_box(line :String) -> Box {
    let h_box = Box::new(Orientation::Horizontal, 10);
    //let line_str: &str = &line;
    if line.chars().next() != Some('#'){
        let mut text_iter = line.split("\t");
        if text_iter.clone().count() < 2{
            text_iter = line.split(" ");
        }
        for s in text_iter{
            if s != ""{
                let entry1 = Entry::new();
                entry1.set_text(s);
                h_box.pack_start(&entry1, false, false, 0);
            }
        }
    }
    return h_box;
}
